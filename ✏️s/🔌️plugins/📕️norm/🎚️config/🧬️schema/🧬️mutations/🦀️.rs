//! 🎚️ The closed semantic mutation vocabulary for norm editor configuration.

use crate::config::NormConfig;
use super::change_selected_check_index::ChangeSelectedCheckIndex;

#[derive(Clone, Debug, PartialEq, dsl::DslOps, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[mutations(snapshot = NormConfig, diff = NormConfig, schema = "s.norm.norm.config")]
pub enum NormConfigMutation {
    ChangeSelectedCheckIndex(ChangeSelectedCheckIndex),
}
