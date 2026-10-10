//! 🌀️ The MEP element rows of the entity table: how a routed duct, pipe or cable tray reads off the snapshot (its system, its section as text, its path as `x, y, z` vertices), how an edited value becomes a `set-mep-element`
//! mutation, and what a new element is. What an element shows (its length, its volume, the findings that name it) is read off the inference.

use super::components::{system_choices, system_of};
use super::{container, keys, number, parse_number, partial, phasing, rename_element, size_pair, Created, EntityKind, FieldRow, InferredRow};
use crate::{MepElement, MepShape, MepSystem, ModelMutation, ModelSnapshot, Point3};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

/// 📏️ How far a new element runs beside the previous ones of its storey, in metres.
const ROW_PITCH: f64 = 0.5;
/// 📏️ The length of a new element along +X, in metres.
const LENGTH: f64 = 4.0;
/// 📏️ The height above the storey elevation a new element runs at, in metres.
pub const ELEVATION: f64 = 2.5;
/// 📏️ The section of a new duct, in metres.
pub const DUCT: (f64, f64) = (0.3, 0.2);
/// 📏️ The diameter of a new pipe, in metres.
pub const PIPE: f64 = 0.1;
/// 📏️ The section of a new cable tray, in metres.
pub const TRAY: (f64, f64) = (0.3, 0.06);

//#region 🔖️Shape
/// 🌀️ The section a new element of `system` starts with: a duct for the air systems, a pipe for the water, waste and gas systems, a tray for power, data and lighting.
pub fn default_shape(system: MepSystem) -> MepShape {
    match system {
        MepSystem::Supply | MepSystem::Return | MepSystem::Exhaust => MepShape::Duct { width: DUCT.0, height: DUCT.1 },
        MepSystem::DomesticWater | MepSystem::Waste | MepSystem::Gas => MepShape::Pipe { diameter: PIPE },
        MepSystem::Power | MepSystem::Data | MepSystem::Lighting => MepShape::Tray { width: TRAY.0, height: TRAY.1 },
    }
}

/// 🌀️ The token of a section kind: `duct`, `pipe` or `tray`.
pub fn shape_token(shape: &MepShape) -> &'static str {
    match shape {
        MepShape::Duct { .. } => "duct",
        MepShape::Pipe { .. } => "pipe",
        MepShape::Tray { .. } => "tray",
    }
}

/// 🌀️ The text of a section in metres: `duct 0.3 x 0.2`, `pipe 0.1` or `tray 0.3 x 0.06`.
pub fn shape_text(shape: &MepShape) -> String {
    match shape {
        MepShape::Duct { width, height } | MepShape::Tray { width, height } => format!("{} {} x {}", shape_token(shape), number(*width), number(*height)),
        MepShape::Pipe { diameter } => format!("pipe {}", number(*diameter)),
    }
}

/// 🌀️ The section a text names: `duct w x h`, `pipe d` or `tray w x h` in metres, every size above zero.
pub fn parse_shape(text: &str) -> Option<MepShape> {
    let text = text.trim();
    let (name, rest) = text.split_once(' ').map_or((text, ""), |(name, rest)| (name, rest.trim()));
    let shape = match name.to_ascii_lowercase().as_str() {
        "duct" => size_pair(rest).map(|(width, height)| MepShape::Duct { width, height }),
        "tray" => size_pair(rest).map(|(width, height)| MepShape::Tray { width, height }),
        "pipe" => parse_number(rest).map(|diameter| MepShape::Pipe { diameter }),
        _ => None,
    }?;
    let sizes = match &shape {
        MepShape::Duct { width, height } | MepShape::Tray { width, height } => vec![*width, *height],
        MepShape::Pipe { diameter } => vec![*diameter],
    };
    sizes.iter().all(|size| size.is_finite() && *size > 0.0).then_some(shape)
}

/// 🌀️ The short text of a section for a list: `300 × 200`, `Ø100` in millimetres.
pub fn size_text(shape: &MepShape) -> String {
    let millimetres = |metres: f64| number((metres * 1e6).round() / 1e3);
    match shape {
        MepShape::Duct { width, height } | MepShape::Tray { width, height } => format!("{} × {}", millimetres(*width), millimetres(*height)),
        MepShape::Pipe { diameter } => format!("Ø{}", millimetres(*diameter)),
    }
}
//#endregion 🔖️Shape

//#region 🔖️Path
/// 🌀️ The text of a path: one `x, y, z` per vertex, joined by `; `.
pub fn path_text(path: &[Point3]) -> String {
    path.iter().map(|vertex| format!("{}, {}, {}", number(vertex.x), number(vertex.y), number(vertex.z))).collect::<Vec<_>>().join("; ")
}

/// 🌀️ The path a text names: at least two `x, y, z` vertices joined by `;`, every number finite.
pub fn parse_path(text: &str) -> Option<Vec<Point3>> {
    let path = text
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut numbers = part.split(',').map(parse_number);
            let (x, y, z) = (numbers.next()??, numbers.next()??, numbers.next()??);
            (numbers.next().is_none() && x.is_finite() && y.is_finite() && z.is_finite()).then_some(Point3 { x, y, z })
        })
        .collect::<Option<Vec<_>>>()?;
    (path.len() >= 2).then_some(path)
}

fn parse_system(text: &str) -> Option<MepSystem> {
    system_of(text)
}
//#endregion 🔖️Path

//#region 🔖️Fields
/// 🧾️ The authored parameters of a routed MEP element.
pub static MEP_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.mep_elements.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.mep_elements.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("system", field_system, Text, |s, id| s.mep_elements.get(id).map(|row| format!("{:?}", row.system)), choices: system_choices, parse_system => set_mep_element::SetMepElement),
    field!("shape", field_cross_section, Text, |s, id| s.mep_elements.get(id).map(|row| shape_text(&row.shape)), parse_shape => set_mep_element::SetMepElement),
    field!("path", field_path, Text, |s, id| s.mep_elements.get(id).map(|row| path_text(&row.path)), parse_path => set_mep_element::SetMepElement),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
/// 💡️ What a routed element shows now: its section in millimetres, its length and volume, and the findings that name it.
pub static MEP_INFERRED: &[InferredRow] = &[
    inferred!("section", field_cross_section, |s, _, id| s.mep_elements.get(id).map(|row| size_text(&row.shape))),
    inferred!("length", field_length, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.length > 0.0).map(|row| number(row.length))),
    inferred!("volume", field_volume, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.net_volume > 0.0).map(|row| number(row.net_volume))),
    inferred!("issues", fam_field_issues, |s, inference, id| s.mep_elements.contains_key(id).then(|| inference.diagnostic_index.elements.get(id).map_or(0, |found| found.count).to_string())),
];
//#endregion 🔖️Inferred

//#region 🔖️Create
/// 🌀️ A new element: a supply duct of 300 by 200 millimetres, four metres along +X at 2.5 m above the storey elevation, beside the elements already on its storey.
pub fn create_mep(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let row = snapshot.mep_elements.values().filter(|element| element.storey == storey).count() as f64 * ROW_PITCH;
    let system = MepSystem::Supply;
    Ok(ModelMutation::CreateMepElement(crate::mutations::create_mep_element::CreateMepElement {
        id: id.into(),
        mep: MepElement { storey, system, shape: default_shape(system), path: vec![Point3 { x: 0.0, y: row, z: ELEVATION }, Point3 { x: LENGTH, y: row, z: ELEVATION }], name: name.into() },
    }))
}
//#endregion 🔖️Create

//#region 🔖️Kinds
/// 🎯️ The entity row of routed MEP elements: named by themselves, standing on their storey.
pub const MEP_ELEMENT: EntityKind = kind!("mep-element", "waypoints", false, kind_mep_element, group_mep_elements, mep_elements . name, parent: |s, id| s.mep_elements.get(id).map(|row| row.storey.clone()),
    delete: delete!(delete_mep_element::DeleteMepElement), rename: Some(rename_element), create: Some(create_mep), fields: MEP_FIELDS, inferred: MEP_INFERRED);
//#endregion 🔖️Kinds

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
