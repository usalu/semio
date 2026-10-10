//! 🧭️ Authored-only validity of options, worksets and their memberships.
use crate::{DesignOption, ModelSnapshot};

/// 🛡️ Validates a design option against authored group and sibling references.
pub fn option_problem(base: &ModelSnapshot, id: &str, value: &DesignOption) -> Option<&'static str> {
    if !base.option_groups.contains_key(&value.group) { return Some("Option group is missing."); }
    if value.name.trim().is_empty() { return Some("Option name is empty."); }
    if value.primary && base.design_options.iter().any(|(key, option)| key != id && option.group == value.group && option.primary) { return Some("Option group already has a primary option."); }
    None
}

/// 🏗️ Physical element ids eligible for membership, in canonical order.
pub fn element_ids(base: &ModelSnapshot) -> std::collections::BTreeSet<String> {
    base.walls.keys().chain(base.curtain_walls.keys()).chain(base.columns.keys()).chain(base.beams.keys()).chain(base.slabs.keys()).chain(base.ceilings.keys()).chain(base.roofs.keys()).chain(base.openings.keys()).chain(base.stairs.keys()).chain(base.railings.keys()).chain(base.ramps.keys()).chain(base.components.keys()).chain(base.mep_elements.keys()).chain(base.spaces.keys()).chain(base.wall_sweeps.keys()).cloned().collect()
}

/// 🪟️ Authored host of dependent physical elements.
pub fn host<'a>(base: &'a ModelSnapshot, id: &str) -> Option<&'a str> {
    base.openings.get(id).map(|record| record.host.as_str()).or_else(|| base.wall_sweeps.get(id).map(|record| record.host.as_str()))
}
