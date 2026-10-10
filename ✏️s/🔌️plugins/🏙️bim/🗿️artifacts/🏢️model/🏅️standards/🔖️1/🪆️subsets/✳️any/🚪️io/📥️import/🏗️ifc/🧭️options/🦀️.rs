//! 🧭️ IFC option groups, design options and worksets restore authored records and memberships.
use super::{Import, data::label, spatial::authoring_of, reader::{text, refs}};
use crate::{OptionGroup, DesignOption, Workset, ElementMembership};
use semio_s_artifact_stdio_ifc::part21::Part21Value;

/// 🏘️ Groups precede option children; malformed authored references are reported.
pub fn read(i: &mut Import<'_>) {
    for kind in ["DesignOptionGroup", "DesignOption", "Workset"] {
        for (instance, args) in i.doc.rows("IFCGROUP").into_iter().filter(|(_, args)| text(args, 4) == kind) {
            let rows = authoring_of(&i.doc, instance.id);
            let name = text(args, 2);
            let id = label(&rows, "Id").filter(|id| crate::mutations::elements::taken(&i.model, id).is_none()).unwrap_or_else(|| Import::unused(&format!("group-{}", Import::slug(&name)), |id| crate::mutations::elements::taken(&i.model, id).is_some()));
            match kind {
                "DesignOptionGroup" => { i.model.option_groups.insert(id.clone(), OptionGroup { name }); }
                "Workset" => { i.model.worksets.insert(id.clone(), Workset { name, default_visible: label(&rows, "DefaultVisible").as_deref() != Some("false") }); }
                _ => {
                    let Some(group) = label(&rows, "Group").filter(|id| i.model.option_groups.contains_key(id)) else { i.skip("IFCGROUP", &id, "Design option has no imported option group"); continue; };
                    let option = DesignOption { name, group, primary: label(&rows, "Primary").as_deref() == Some("true") };
                    if let Some(problem) = crate::mutations::option_rules::option_problem(&i.model, &id, &option) { i.skip("IFCGROUP", &id, problem); continue; }
                    i.model.design_options.insert(id.clone(), option);
                }
            }
            i.ids.insert(instance.id, id);
        }
    }
    let elements = crate::mutations::option_rules::element_ids(&i.model);
    for (_, args) in i.doc.rows("IFCRELASSIGNSTOGROUP") {
        let Some(group) = args.get(6).and_then(Part21Value::as_ref_id).and_then(|entity| i.ids.get(&entity)).cloned() else { continue; };
        let options = i.model.design_options.contains_key(&group);
        if !options && !i.model.worksets.contains_key(&group) { continue; }
        for entity in refs(args, 4) {
            let Some(id) = i.ids.get(&entity).filter(|id| elements.contains(*id)).cloned() else { continue; };
            let members = if options { &mut i.model.element_options } else { &mut i.model.element_worksets };
            if members.insert(id.clone(), ElementMembership { target: group.clone() }).is_some() { i.notes.push(format!("Multiple IFC memberships for {id}; later assignment chosen")); }
        }
    }
}
