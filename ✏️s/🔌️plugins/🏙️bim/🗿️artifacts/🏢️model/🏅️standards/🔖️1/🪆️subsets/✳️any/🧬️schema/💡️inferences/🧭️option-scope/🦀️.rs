//! 🧭️ Inferred option visibility and takeoff from authored membership and quantity parents.
use crate::{ModelSnapshot, ElementMembership};
use super::super::quantities::{ElementQuantity, QuantityTotals, totals_of};
use crate::standards::v1::subsets::any::schema::mutations::option_rules::{element_ids, host};
use std::collections::{BTreeMap, BTreeSet};

pub const READS: &[&str] = &["option_groups", "design_options", "worksets", "element_options", "element_worksets", "walls", "curtain_walls", "columns", "beams", "slabs", "ceilings", "roofs", "openings", "stairs", "ramps", "railings", "spaces", "components", "mep_elements", "wall_sweeps"];

/// 🧭️ Derived references, visibility sets and quantity totals; no ownership or active choice is persisted here.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct OptionScope {
    pub elements: Vec<String>,
    pub defaults: BTreeMap<String, String>,
    pub groups: BTreeMap<String, String>,
    pub effective_options: BTreeMap<String, String>,
    pub effective_worksets: BTreeMap<String, String>,
    pub hidden_worksets: Vec<String>,
    pub hosts: BTreeMap<String, String>,
    pub visible: Vec<String>,
    pub options: BTreeMap<String, Vec<String>>,
    pub worksets: BTreeMap<String, Vec<String>>,
    pub takeoff: BTreeMap<String, QuantityTotals>,
}
fn effective(base: &ModelSnapshot, memberships: &BTreeMap<String, ElementMembership>, id: &str) -> Option<String> {
    memberships.get(id).or_else(|| host(base, id).and_then(|parent| memberships.get(parent))).map(|record| record.target.clone())
}
impl OptionScope {
    /// 👁️ One element visibility query without building a separate state holder.
    pub fn includes(&self, id: &str, selected: &BTreeMap<String, String>, visibility: &BTreeMap<String, bool>) -> bool {
        let accepts = |id: &str| {
            self.effective_options.get(id).is_none_or(|option| self.groups.get(option).is_some_and(|group| selected.get(group).or_else(|| self.defaults.get(group)) == Some(option)))
                && self.effective_worksets.get(id).is_none_or(|workset| visibility.get(workset).copied().unwrap_or_else(|| !self.hidden_worksets.contains(workset)))
        };
        accepts(id) && self.hosts.get(id).is_none_or(|parent| accepts(parent))
    }

    /// 👁️ Visible physical ids for local option choices and hidden worksets, including hosted visibility.
    pub fn visible_for(&self, selected: &BTreeMap<String, String>, hidden: &BTreeSet<String>) -> Vec<String> {
        let accepts = |id: &str| {
            let option = self.effective_options.get(id).is_none_or(|option| self.groups.get(option).is_some_and(|group| selected.get(group).or_else(|| self.defaults.get(group)) == Some(option)));
            let workset = self.effective_worksets.get(id).is_none_or(|workset| !hidden.contains(workset));
            option && workset
        };
        self.elements.iter().filter(|id| accepts(id) && self.hosts.get(*id).is_none_or(|parent| accepts(parent))).cloned().collect()
    }
}
/// 🧮️ One pure projection; quantities are parent values and never recomputed.
pub fn scope_of<'a>(base: &ModelSnapshot, quantities: impl IntoIterator<Item = (&'a str, &'a ElementQuantity)>) -> OptionScope {
    let ids = element_ids(base);
    let mut result = OptionScope { elements: ids.iter().cloned().collect(), groups: base.design_options.iter().map(|(id, option)| (id.clone(), option.group.clone())).collect(), hidden_worksets: base.worksets.iter().filter(|(_, workset)| !workset.default_visible).map(|(id, _)| id.clone()).collect(), ..Default::default() };
    for (id, option) in &base.design_options { if option.primary || !result.defaults.contains_key(&option.group) { result.defaults.insert(option.group.clone(), id.clone()); } }
    for id in &ids {
        if let Some(option) = effective(base, &base.element_options, id) { result.effective_options.insert(id.clone(), option); }
        if let Some(workset) = effective(base, &base.element_worksets, id) { result.effective_worksets.insert(id.clone(), workset); }
        if let Some(parent) = host(base, id) { result.hosts.insert(id.clone(), parent.to_owned()); }
    }
    let hidden = result.hidden_worksets.iter().cloned().collect();
    result.visible = result.visible_for(&BTreeMap::new(), &hidden);
    let quantities: BTreeMap<_, _> = quantities.into_iter().collect();
    for (id, option) in &base.design_options {
        let selected = BTreeMap::from([(option.group.clone(), id.clone())]);
        let visible = result.visible_for(&selected, &BTreeSet::new());
        result.takeoff.insert(id.clone(), totals_of(visible.iter().filter_map(|id| quantities.get(id.as_str()).copied())));
        result.options.insert(id.clone(), visible);
    }
    for id in base.worksets.keys() { result.worksets.insert(id.clone(), result.effective_worksets.iter().filter(|(_, workset)| *workset == id).map(|(element, _)| element.clone()).collect()); }
    result
}
/// 🔗️ Authored inputs of the visibility node; quantity changes arrive through graph parents.
pub fn dependency(base: &ModelSnapshot) -> semio_framework_value::DslValue {
    use super::super::element_solids::{dep_object, dep_value};
    dep_object([("option_groups", dep_value(&base.option_groups)), ("design_options", dep_value(&base.design_options)), ("worksets", dep_value(&base.worksets)), ("element_options", dep_value(&base.element_options)), ("element_worksets", dep_value(&base.element_worksets)), ("elements", dep_value(&element_ids(base).into_iter().collect::<Vec<_>>())), ("hosts", dep_value(&element_ids(base).iter().filter_map(|id| host(base, id).map(|parent| (id.clone(), parent.to_owned()))).collect::<BTreeMap<_, _>>()))])
}
