//! 🪑️ The component rows of the entity table: how a placed instance of a family (a chair, a basin, a luminaire, a diffuser) and one of its parameter overrides read off the snapshot, how an edited value becomes a
//! `set-component` or `set-component-override` mutation, what a new component is, the pickers of the properties panel (family by category, host wall, system) and the placeable families the browser, the place tool and the
//! `create-entity` command share. What a component shows (its volume, its issues) is read off the inference; the evaluated parameters are rows of the properties panel.

use super::{container, keys, number, parse_flag, parse_number, parse_point, partial, phasing, point, rename_element, variant, Created, EntityKind, FieldRow, InferredRow};
use crate::editor::bim::modes::edit::windows::family::edit::CATEGORIES;
use crate::editor::bim::modes::edit::windows::family::vocabulary::category_label;
use crate::editor::bim::terminology::BimLabels;
use crate::mutations::remove_component_override::RemoveComponentOverride;
use crate::mutations::set_component_override::SetComponentOverride;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{Assigned, Component, FamilyCategory, MepSystem, ModelMutation, ModelSnapshot, Point2};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

/// 📏️ How far a new component stands from the previous ones of its storey, in metres.
const ROW_PITCH: f64 = 1.0;

//#region 🔖️Systems
/// 🌀️ The systems a terminal or a routed element can carry, in the order the pickers list them.
pub const SYSTEMS: [MepSystem; 9] = [MepSystem::Supply, MepSystem::Return, MepSystem::Exhaust, MepSystem::DomesticWater, MepSystem::Waste, MepSystem::Gas, MepSystem::Power, MepSystem::Data, MepSystem::Lighting];

/// 🌀️ The localized name of a system.
pub fn system_label(labels: &BimLabels, system: MepSystem) -> String {
    let label = match system {
        MepSystem::Supply => labels.sys_supply,
        MepSystem::Return => labels.sys_return,
        MepSystem::Exhaust => labels.sys_exhaust,
        MepSystem::DomesticWater => labels.sys_domestic_water,
        MepSystem::Waste => labels.sys_waste,
        MepSystem::Gas => labels.sys_gas,
        MepSystem::Power => labels.sys_power,
        MepSystem::Data => labels.sys_data,
        MepSystem::Lighting => labels.sys_lighting,
    };
    label.as_str().to_string()
}

/// 🌀️ The system a token names, without regard to case (`water` is the domestic water).
pub fn system_of(text: &str) -> Option<MepSystem> {
    let text = text.trim();
    if text.eq_ignore_ascii_case("water") {
        return Some(MepSystem::DomesticWater);
    }
    variant(text, &SYSTEMS)
}

/// 🌀️ The systems by token and localized name, for the pickers.
pub fn system_choices(_: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    SYSTEMS.iter().map(|system| (format!("{system:?}"), system_label(labels, *system))).collect()
}
//#endregion 🔖️Systems

//#region 🔖️Families
/// 🧬️ The families a component can be an instance of: every family that is no profile, by category (in the order of the category pickers), then name, then id.
pub fn placeable(snapshot: &ModelSnapshot) -> Vec<(String, FamilyCategory)> {
    let rank = |category: FamilyCategory| CATEGORIES.iter().position(|known| *known == category).unwrap_or(CATEGORIES.len());
    let mut rows: Vec<((usize, String, String), FamilyCategory)> = snapshot.families.iter().filter(|(_, family)| family.category != FamilyCategory::Profile).map(|(id, family)| ((rank(family.category), family.name.to_lowercase(), id.clone()), family.category)).collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows.into_iter().map(|((_, _, id), category)| (id, category)).collect()
}

/// 🔍️ The placeable families whose name, id or category (token or localized name) contains `query`, without regard to case; every placeable family for an empty query.
pub fn matching(snapshot: &ModelSnapshot, query: &str, labels: Option<&BimLabels>) -> Vec<String> {
    let needle = query.trim().to_lowercase();
    placeable(snapshot)
        .into_iter()
        .filter(|(id, category)| {
            let name = snapshot.families.get(id).map(|family| family.name.to_lowercase()).unwrap_or_default();
            needle.is_empty() || name.contains(&needle) || id.to_lowercase().contains(&needle) || format!("{category:?}").to_lowercase().contains(&needle) || labels.is_some_and(|labels| category_label(labels, *category).to_lowercase().contains(&needle))
        })
        .map(|(id, _)| id)
        .collect()
}

/// 🧬️ The placeable families by id and name with their category, for the family picker.
pub fn family_choices(snapshot: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    placeable(snapshot).into_iter().map(|(id, category)| (id.clone(), format!("{} · {}", snapshot.families.get(&id).map_or("", |family| family.name.as_str()), category_label(labels, category)))).collect()
}

/// 🧱️ The walls a component can be mounted on, by id and name with the storey they stand on.
pub fn host_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.walls.iter().map(|(id, wall)| (id.clone(), format!("{} · {}", wall.name, snapshot.storeys.get(&wall.storey).map_or(wall.storey.as_str(), |storey| storey.name.as_str())))).collect()
}

/// 🧬️ The categories whose instances cling to a wall: the plumbing, lighting, electrical and casework a wall carries.
pub fn wall_mounted(category: FamilyCategory) -> bool {
    matches!(category, FamilyCategory::Plumbing | FamilyCategory::Lighting | FamilyCategory::Electrical | FamilyCategory::Casework)
}
//#endregion 🔖️Families

//#region 🔖️Parse
fn parse_family(text: &str) -> Option<String> {
    Some(text.trim()).filter(|id| !id.is_empty()).map(str::to_string)
}

fn parse_degrees(text: &str) -> Option<f64> {
    parse_number(text).filter(|degrees| degrees.is_finite()).map(f64::to_radians)
}

/// 📐️ The text of an angle in radians as degrees, without the noise of the conversion.
pub fn degrees_text(radians: f64) -> String {
    number((radians.to_degrees() * 1e6).round() / 1e6)
}

fn write_host(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let host = Some(value.trim()).filter(|host| !host.is_empty()).map(str::to_string);
    set!(set_component::SetComponent, id, "host", &Assigned::new(host))
}

fn write_system(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let text = value.trim();
    let system = if text.is_empty() { None } else { Some(system_of(text)?) };
    set!(set_component::SetComponent, id, "system", &Assigned::new(system))
}

fn write_override(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let row = snapshot.component_overrides.get(id)?;
    let value = formula::canonical(value).ok()?;
    Some(ModelMutation::SetComponentOverride(SetComponentOverride { component: row.component.clone(), name: row.name.clone(), value }))
}

fn delete_override(id: &str) -> ModelMutation {
    let (component, name) = formula::split_parameter_id(id).unwrap_or((id, ""));
    ModelMutation::RemoveComponentOverride(RemoveComponentOverride { component: component.into(), name: name.into() })
}
//#endregion 🔖️Parse

//#region 🔖️Fields
/// 🧾️ The authored parameters of a component. The rotation is shown in degrees; the host and the system can be cleared.
pub static COMPONENT_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.components.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.components.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("family", field_family, Text, |s, id| s.components.get(id).map(|row| row.family.clone()), choices: family_choices, parse_family => set_component::SetComponent),
    field!("position", field_position, Text, |s, id| s.components.get(id).map(|row| point(row.position.x, row.position.y)), parse_point => set_component::SetComponent),
    field!("elevation", field_elevation, Number, |s, id| s.components.get(id).map(|row| number(row.elevation)), parse_number => set_component::SetComponent),
    field!("rotation", field_rotation_degrees, Number, |s, id| s.components.get(id).map(|row| degrees_text(row.rotation)), |_, id, value| set!(set_component::SetComponent, id, "rotation", &parse_degrees(value)?)),
    field!("mirrored", field_mirrored, Text, |s, id| s.components.get(id).map(|row| row.mirrored.to_string()), parse_flag => set_component::SetComponent),
    field!("host", field_host, Text, |s, id| s.components.get(id).map(|row| row.host.clone().unwrap_or_default()), choices: host_choices, write: write_host),
    field!("system", field_system, Text, |s, id| s.components.get(id).map(|row| row.system.map(|system| format!("{system:?}")).unwrap_or_default()), choices: system_choices, write: write_system),
];

/// 🧾️ The authored parameters of a parameter override: the component and the parameter it addresses are fixed, the formula is not.
pub static COMPONENT_OVERRIDE_FIELDS: &[FieldRow] = &[
    FieldRow { key: "component", label: |labels| labels.field_component, input: InputKind::Text, read: |s, id| s.component_overrides.get(id).map(|row| row.component.clone()), write: None, choices: None },
    FieldRow { key: "name", label: |labels| labels.field_parameter, input: InputKind::Text, read: |s, id| s.component_overrides.get(id).map(|row| row.name.clone()), write: None, choices: None },
    FieldRow { key: "value", label: |labels| labels.fam_formula, input: InputKind::Text, read: |s, id| s.component_overrides.get(id).map(|row| row.value.clone()), write: Some(write_override), choices: None },
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
/// 💡️ What a component shows now: the category of its family, how many parameters it overrides, the volume of its solids and the findings that name it.
pub static COMPONENT_INFERRED: &[InferredRow] = &[
    inferred!("category", field_category, |s, _, id| s.components.get(id).and_then(|row| s.families.get(&row.family)).map(|family| format!("{:?}", family.category))),
    inferred!("overrides", field_overrides, |s, _, id| s.components.contains_key(id).then(|| s.component_overrides.values().filter(|row| row.component == id).count().to_string())),
    inferred!("volume", field_volume, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.net_volume > 0.0).map(|row| number(row.net_volume))),
    inferred!("issues", fam_field_issues, |s, inference, id| s.components.contains_key(id).then(|| inference.diagnostic_index.elements.get(id).map_or(0, |found| found.count).to_string())),
];

/// 💡️ What a parameter override shows now: the formula the family itself gives the parameter.
pub static COMPONENT_OVERRIDE_INFERRED: &[InferredRow] = &[inferred!("family_formula", fam_formula, |s, _, id| {
    let row = s.component_overrides.get(id)?;
    let family = &s.components.get(&row.component)?.family;
    s.family_parameters.get(&formula::parameter_id(family, &row.name)).map(|parameter| parameter.value.clone())
})];
//#endregion 🔖️Inferred

//#region 🔖️Create
/// 🪑️ A new component: the first placeable family of the model, standing at the storey elevation beside the components already on its storey, not mounted, no terminal.
pub fn create_component(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let (family, _) = placeable(snapshot).into_iter().next().ok_or("bim.create.component-family-missing")?;
    let row = snapshot.components.values().filter(|component| component.storey == storey).count() as f64;
    Ok(ModelMutation::CreateComponent(crate::mutations::create_component::CreateComponent {
        id: id.into(),
        component: Component { storey, family, position: Point2 { x: row * ROW_PITCH, y: 0.0 }, elevation: 0.0, rotation: 0.0, mirrored: false, host: None, system: None, name: name.into() },
    }))
}
//#endregion 🔖️Create

//#region 🔖️Kinds
/// 🎯️ The entity row of components: named by themselves, standing on their storey.
pub const COMPONENT: EntityKind = kind!("component", "armchair", false, kind_component, group_components, components . name, parent: |s, id| s.components.get(id).map(|row| row.storey.clone()),
    delete: delete!(delete_component::DeleteComponent), rename: Some(rename_element), create: Some(create_component), fields: COMPONENT_FIELDS, inferred: COMPONENT_INFERRED);

/// 🎯️ The entity row of parameter overrides: named by their parameter, standing under their component. They are made by the override rows of the properties panel, never by `create-entity`.
pub const COMPONENT_OVERRIDE: EntityKind = kind!("component-override", "sliders-horizontal", false, kind_component_override, group_component_overrides, component_overrides . name, parent: |s, id| s.component_overrides.get(id).map(|row| row.component.clone()),
    delete: Some(delete_override), rename: None, create: None, fields: COMPONENT_OVERRIDE_FIELDS, inferred: COMPONENT_OVERRIDE_INFERRED);
//#endregion 🔖️Kinds

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
