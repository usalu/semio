//! 🧭️ Authored options/worksets as IfcGroup and IfcRelAssignsToGroup in IFC2x3 and IFC4.
use super::data::label;
use super::writer::{refs, rf, text, unset};
use super::Export;
use std::collections::BTreeMap;
fn group(x: &mut Export<'_>, id: &str, name: &str, kind: &str, rows: Vec<(&'static str, super::writer::V)>) -> u64 {
    let entity = x.ifc.rooted("IFCGROUP", id, name, "", vec![text(kind)]);
    x.links.elements.insert(id.to_owned(), entity);
    let mut authored = vec![("Id", label(id))];
    authored.extend(rows);
    x.links.authoring.push((entity, authored));
    entity
}
fn assign(x: &mut Export<'_>, id: &str, entity: u64, members: Vec<u64>) {
    if !members.is_empty() { x.ifc.rooted("IFCRELASSIGNSTOGROUP", &format!("{id}:members"), "", "", vec![refs(&members), unset(), rf(entity)]); }
}
/// 🏘️ Only authored memberships are exported; inherited visibility remains inference.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    let mut groups = BTreeMap::new();
    for (id, record) in &model.option_groups { groups.insert(id.clone(), group(x, id, &record.name, "DesignOptionGroup", Vec::new())); }
    for (id, option) in &model.design_options {
        let entity = group(x, id, &option.name, "DesignOption", vec![("Group", label(&option.group)), ("Primary", label(if option.primary { "true" } else { "false" }))]);
        let members = model.element_options.iter().filter(|(_, membership)| membership.target == *id).filter_map(|(element, _)| x.links.elements.get(element).copied()).collect();
        assign(x, id, entity, members);
    }
    for (id, entity) in groups {
        let members = model.design_options.iter().filter(|(_, option)| option.group == id).filter_map(|(option, _)| x.links.elements.get(option).copied()).collect();
        assign(x, &id, entity, members);
    }
    for (id, workset) in &model.worksets {
        let entity = group(x, id, &workset.name, "Workset", vec![("DefaultVisible", label(if workset.default_visible { "true" } else { "false" }))]);
        let members = model.element_worksets.iter().filter(|(_, membership)| membership.target == *id).filter_map(|(element, _)| x.links.elements.get(element).copied()).collect();
        assign(x, id, entity, members);
    }
}
