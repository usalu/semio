//! 🧬️ The family rows of the entity table: how a family (a library entry) and a family solid read off the snapshot, how an edited value becomes a `set-family` or `set-family-solid` mutation, what a new
//! family and a new solid are, and the category picker of the properties panel. What a family shows (its volume, its issues) is read off the `families` inference; parameters are rows of the family window.

use super::{partial, Created, FieldRow, InferredRow};
use crate::editor::bim::modes::edit::windows::family::edit::{apply, category_of, Edit, CATEGORIES};
use crate::editor::bim::modes::edit::windows::family::vocabulary::category_label;
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{Family, FamilyCategory, ModelMutation, ModelSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

fn parse_category(text: &str) -> Option<FamilyCategory> {
    category_of(text)
}

fn parse_name(text: &str) -> Option<String> {
    Some(text.trim()).filter(|name| !name.is_empty()).map(str::to_string)
}

fn parse_formula(text: &str) -> Option<String> {
    formula::canonical(text).ok()
}

//#region 🔖️Choices
/// 🏷️ The categories a family can take by token and localized name.
pub fn category_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    CATEGORIES.iter().map(|category| (format!("{category:?}"), category_label(labels, *category))).collect()
}
//#endregion 🔖️Choices

//#region 🔖️Fields
/// 🧾️ The authored parameters of a family.
pub static FAMILY_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.families.get(id).map(|row| row.name.clone()), parse_name => set_family::SetFamily),
    field!("category", field_category, Text, |s, id| s.families.get(id).map(|row| format!("{:?}", row.category)), choices: category_choices, parse_category => set_family::SetFamily),
];

/// 🧾️ The authored parameters of a family solid: its name and the formulas of its material and its visibility.
pub static FAMILY_SOLID_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.family_solids.get(id).map(|row| row.name.clone()), parse_name => set_family_solid::SetFamilySolid),
    field!("material", field_material, Text, |s, id| s.family_solids.get(id).map(|row| row.material.clone()), parse_formula => set_family_solid::SetFamilySolid),
    field!("visible", fam_field_visible, Text, |s, id| s.family_solids.get(id).map(|row| row.visible.clone()), parse_formula => set_family_solid::SetFamilySolid),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
/// 💡️ What a family shows now: the volume of its visible solids and the number of its issues.
pub static FAMILY_INFERRED: &[InferredRow] = &[
    inferred!("volume", field_volume, |_, inference, id| inference.families.get(id).map(|row| format!("{:.6}", row.volume()))),
    inferred!("issues", fam_field_issues, |_, inference, id| inference.families.get(id).map(|row| row.issues.len().to_string())),
];

/// 💡️ What a family solid shows now: its volume.
pub static FAMILY_SOLID_INFERRED: &[InferredRow] =
    &[inferred!("volume", field_volume, |s, inference, id| s.family_solids.get(id).and_then(|row| inference.families.get(&row.family)?.solids.get(id)).map(|row| format!("{:.6}", row.volume)))];
//#endregion 🔖️Inferred

//#region 🔖️Create
/// 🧬️ A new family: empty, in the category the parent names (generic when it names none).
pub fn create_family(_: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let category = category_of(parent).unwrap_or(FamilyCategory::Generic);
    Ok(ModelMutation::CreateFamily(crate::mutations::create_family::CreateFamily { id: id.into(), family: Family { name: name.into(), category } }))
}

/// 🧊️ A new solid: a cuboid of a tenth of a metre in the first material of the model, in the family the parent names.
pub fn create_family_solid(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let edit = Edit { part: "solid".into(), op: "add".into(), key: "Cuboid".into(), value: String::new() };
    let mutations = apply(snapshot, parent, &edit, id).map_err(|code| if code == "bim.family.missing" { "bim.create.family-missing" } else { "bim.create.material-missing" })?;
    match mutations.into_iter().next() {
        Some(ModelMutation::CreateFamilySolid(mut leaf)) => {
            leaf.solid.name = name.to_string();
            Ok(ModelMutation::CreateFamilySolid(leaf))
        }
        _ => Err("bim.create.family-missing"),
    }
}
//#endregion 🔖️Create

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
