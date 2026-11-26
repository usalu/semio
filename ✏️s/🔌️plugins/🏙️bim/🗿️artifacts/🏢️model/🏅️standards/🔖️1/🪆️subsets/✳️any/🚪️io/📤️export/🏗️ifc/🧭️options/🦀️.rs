//! 🧭️ Authored options/worksets as IfcGroup and IfcRelAssignsToGroup in IFC2x3 and IFC4.
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::label;
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::{refs, rf, text, unset};
use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context::Export;
use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::zoning::group_proxy;
fn group(x: &mut Export<'_>, id: &str, name: &str, kind: &str, members: Vec<u64>, rows: Vec<(&'static str, semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::V)>) {
    let entity = if members.is_empty() && x.by(true, false) {
        group_proxy(x, id, name, kind)
    } else {
        x.ifc.rooted("IFCGROUP", id, name, "", vec![text(kind)])
    };
    x.links.elements.insert(id.to_owned(), entity);
    let mut authored = vec![("Id", label(id))];
    authored.extend(rows);
    x.links.authoring.push((entity, authored));
    if !members.is_empty() { x.ifc.rooted("IFCRELASSIGNSTOGROUP", &format!("{id}:members"), "", "", vec![refs(&members), unset(), rf(entity)]); }
}
/// 🏘️ Only authored memberships are exported; inherited visibility remains inference.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, option) in &model.design_options {
        let members = model.element_options.iter().filter(|(_, membership)| membership.target == *id).filter_map(|(element, _)| x.links.elements.get(element).copied()).collect();
        group(x, id, &option.name, "DesignOption", members, vec![("Group", label(&option.group)), ("Primary", label(if option.primary { "true" } else { "false" }))]);
    }
    for (id, record) in &model.option_groups {
        let members = model.design_options.iter().filter(|(_, option)| option.group == *id).filter_map(|(option, _)| x.links.elements.get(option).copied()).collect();
        group(x, id, &record.name, "DesignOptionGroup", members, Vec::new());
    }
    for (id, workset) in &model.worksets {
        let members = model.element_worksets.iter().filter(|(_, membership)| membership.target == *id).filter_map(|(element, _)| x.links.elements.get(element).copied()).collect();
        group(x, id, &workset.name, "Workset", members, vec![("DefaultVisible", label(if workset.default_visible { "true" } else { "false" }))]);
    }
}
