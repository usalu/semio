//! 📊️ Native JSON inference table projection.
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::schema::inferences::ramp_runs::RampRun;

/// 🧾️ The table the third-party oracle reproduces: the whole run of every ramp.
pub fn table_json(runs: &BTreeMap<String, RampRun>) -> String {
    semio_framework_pack_json::to_json_string(runs)
}
