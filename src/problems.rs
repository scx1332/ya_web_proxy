use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSimulateProblems {
    pub timeout_chance: f64,
    pub error_chance: f64,
    pub malformed_response_chance: f64,
    pub skip_sending_raw_transaction_chance: f64,
    pub allow_only_parsed_calls: bool,
    pub allow_only_single_calls: bool,
}

impl Default for EndpointSimulateProblems {
    fn default() -> Self {
        Self {
            timeout_chance: 0.0,
            error_chance: 0.4,
            malformed_response_chance: 0.4,
            skip_sending_raw_transaction_chance: 1.0,
            allow_only_parsed_calls: true,
            allow_only_single_calls: true,
        }
    }
}