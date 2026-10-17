//! 🔎️ Project and lookup of the BIM entity table: the project fields, the classification text and the kind, holding, id and storey lookups. See [`super`].

use super::*;

//#region 🔖️Project
/// 📇️ The id the project record answers to in `setField`: the project is no entity of the table, so its parameters are rows of their own.
pub const PROJECT_ID: &str = "project";

fn project_info(edit: impl FnOnce(&mut crate::standards::v1::subsets::any::schema::mutations::set_project_info::SetProjectInfo)) -> Option<ModelMutation> {
    let mut info = crate::standards::v1::subsets::any::schema::mutations::set_project_info::SetProjectInfo { name: None, description: None, author: None, organization: None, phase_names: None };
    edit(&mut info);
    Some(ModelMutation::SetProjectInfo(info))
}

/// 📇️ The editable parameters of the project record, each one a `set-project-info` patch of that single field.
pub static PROJECT_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, _| Some(s.project.name.clone()), |_, _, value| project_info(|info| info.name = Some(value.trim().to_string()))),
    field!("description", field_description, Text, |s, _| Some(s.project.description.clone()), |_, _, value| project_info(|info| info.description = Some(value.trim().to_string()))),
    field!("author", field_author, Text, |s, _| Some(s.project.author.clone()), |_, _, value| project_info(|info| info.author = Some(value.trim().to_string()))),
    field!("organization", field_organization, Text, |s, _| Some(s.project.organization.clone()), |_, _, value| project_info(|info| info.organization = Some(value.trim().to_string()))),
    field!("phase_names", field_phase_names, Text, |s, _| Some(s.project.phase_names.join(", ")), |_, _, value| project_info(|info| info.phase_names = Some(value.split(',').map(str::trim).filter(|name| !name.is_empty()).map(str::to_string).collect()))),
];
//#endregion 🔖️Project

//#region 🔖️Classification
/// 🗂️ The classifications of a holder as text: the name of the system and the code, one per system, joined by `; `.
pub fn classification_text(snapshot: &ModelSnapshot, id: &str) -> String {
    snapshot.classifications.get(id).into_iter().flatten().map(|(system, code)| format!("{} {code}", snapshot.classification_systems.get(system).map_or(system.as_str(), |row| row.name.as_str()))).collect::<Vec<_>>().join("; ")
}

/// 🧾️ Every parameter row of a kind.
pub fn fields_of(row: &EntityKind) -> impl Iterator<Item = &'static FieldRow> {
    let physical = matches!(row.kind, "wall" | "curtain-wall" | "column" | "beam" | "slab" | "ceiling" | "roof" | "opening" | "stair" | "railing" | "ramp" | "space" | "component" | "mep-element" | "wall-sweep");
    row.fields.iter().chain(options::MEMBERSHIP_FIELDS.iter().filter(move |_| physical))
}
//#endregion 🔖️Classification

//#region 🔖️Lookup
/// 🔎️ The kind row of a kind id.
pub fn kind_of(kind: &str) -> Option<&'static EntityKind> {
    ENTITIES.iter().find(|row| row.kind == kind)
}

/// 🔎️ The kind row that holds `id`, searched in table order.
pub fn kind_holding(snapshot: &ModelSnapshot, id: &str) -> Option<&'static EntityKind> {
    ENTITIES.iter().find(|row| (row.name)(snapshot, id).is_some())
}

/// 🪪️ Whether any collection of the snapshot already holds `id`.
pub fn id_taken(snapshot: &ModelSnapshot, id: &str) -> bool {
    kind_holding(snapshot, id).is_some()
}

/// 🪜️ The storey an entity is drawn on: its own, or its host's for an opening.
pub fn storey_of(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    let row = kind_holding(snapshot, id)?;
    match row.kind {
        "storey" => Some(id.to_string()),
        "opening" | "wall-sweep" | "component-override" => (row.parent)(snapshot, id).and_then(|host| storey_of(snapshot, &host)),
        "site" | "building" | "grid" | "view" | "zone" | "area-scheme" | "sheet" | "viewport" | "sheet-revision" | "clash-set" | "rule" | "issue" | "issue-comment" => None,
        _ if row.library => None,
        _ => (row.parent)(snapshot, id),
    }
}
//#endregion 🔖️Lookup
