//! 📊️ Native JSON inference table projection.
use std::collections::BTreeMap;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{StairRun};

/// 🧾️ The table the third-party oracle reproduces: the whole run of every stair.
pub fn table_json(runs: &BTreeMap<String, StairRun>) -> String {
    semio_framework_pack_json::to_json_string(runs)
}
