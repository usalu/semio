//! 📊️ Native JSON inference table projection.
use std::collections::BTreeMap;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanLinework, METRICS};

/// 📏️ The canonical JSON table `storey → measure → value` the plan oracle compares.
pub fn metrics_json(plans: &BTreeMap<String, PlanLinework>) -> String {
    let table: BTreeMap<String, BTreeMap<String, f64>> = plans.iter().map(|(storey, plan)| (storey.clone(), METRICS.iter().map(|(name, kind, area)| (name.to_string(), if *area { plan.area_of(*kind) } else { plan.length_of(*kind) })).collect())).collect();
    semio_framework_pack_json::to_json_string(&table)
}
