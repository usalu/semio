//! 🪑️ Components. A placed instance of a family is an `IfcFurnishingElement` (`IfcFurniture` in IFC4) for furniture, an `IfcFlowTerminal` (`IfcSanitaryTerminal`, `IfcLightFixture` in IFC4) for plumbing and lighting fixtures and for
//! every mechanical or electrical family whose instance is a terminal (it names a system), and an `IfcBuildingElementProxy` for the rest (equipment, casework, generic objects). Each family becomes one type object per kind of
//! occurrence (`IfcFurnitureType`, `IfcSanitaryTerminalType`, `IfcLightFixtureType`, `IfcFlowTerminalType`, `IfcBuildingElementProxyType`) that carries the authored family, its parameters and its solids in the `Semio_Authoring`
//! set; the instance carries its authored record as the `Component` row, its per-instance overrides in the `Semio_ComponentOverrides` set and `Qto_ComponentBaseQuantities` (volume of the visible solids, plan area of
//! their bounds). The placement is the inferred one (host fit, yaw, origin in the storey), the body the faceted brep or tessellation of the inferred solid in that frame (a mirror is baked into the body, IFC placements cannot mirror).
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgserviceelements/lexical/ifcflowterminal.htm>

use super::brep::{body_kind, mesh_item};
use super::data::{define, label, property_set};
use super::writer::{en, opt_text, refs, rf, unset, V};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::components::{overrides_of, ComponentPlacement};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use crate::{FamilyCategory, FamilyParameter, FamilySolid};
use semio_framework_geometry::mesh::TriMesh;
use std::collections::BTreeMap;

/// 🏷️ The `Semio_Authoring` row that holds the authored component record.
pub const RECORD_ROW: &str = "Component";
/// 🏷️ The property set that holds the formulas of the per-instance overrides.
pub const OVERRIDES_SET: &str = "Semio_ComponentOverrides";
/// 🧮️ The base quantity set of a component.
pub const QUANTITY_SET: &str = "Qto_ComponentBaseQuantities";
/// 🧬️ The `Semio_Authoring` rows of a family type object: the family record, its parameters by name and its solids by id.
pub const FAMILY_ROWS: [&str; 3] = ["Family", "Parameters", "Solids"];

/// 🪑️ What an occurrence of a family is in IFC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Furniture,
    Sanitary,
    Light,
    Terminal,
    Proxy,
}

impl Kind {
    /// 🪑️ The kind of an instance of a family of `category`: plumbing and lighting fixtures and the terminals of a mechanical or electrical family are flow terminals.
    pub fn of(category: FamilyCategory, terminal: bool) -> Self {
        match category {
            FamilyCategory::Furniture => Self::Furniture,
            FamilyCategory::Plumbing => Self::Sanitary,
            FamilyCategory::Lighting => Self::Light,
            FamilyCategory::Mechanical | FamilyCategory::Electrical if terminal => Self::Terminal,
            _ => Self::Proxy,
        }
    }

    /// 🏷️ The occurrence entity and its entity-specific tail in the schema of `x`.
    pub fn occurrence(self, x: &Export<'_>) -> (&'static str, Vec<V>) {
        match self {
            Self::Furniture => x.by(("IFCFURNISHINGELEMENT", Vec::new()), ("IFCFURNITURE", vec![unset()])),
            Self::Sanitary => x.by(("IFCFLOWTERMINAL", Vec::new()), ("IFCSANITARYTERMINAL", vec![unset()])),
            Self::Light => x.by(("IFCFLOWTERMINAL", Vec::new()), ("IFCLIGHTFIXTURE", vec![unset()])),
            Self::Terminal => ("IFCFLOWTERMINAL", Vec::new()),
            Self::Proxy => ("IFCBUILDINGELEMENTPROXY", vec![unset()]),
        }
    }

    /// 🏷️ The type entity and its entity-specific tail in the schema of `x`.
    pub fn type_object(self, x: &Export<'_>) -> (&'static str, Vec<V>) {
        match self {
            Self::Furniture => x.by(("IFCFURNITURETYPE", vec![en("NOTDEFINED")]), ("IFCFURNITURETYPE", vec![en("NOTDEFINED"), en("NOTDEFINED")])),
            Self::Sanitary => ("IFCSANITARYTERMINALTYPE", vec![en("NOTDEFINED")]),
            Self::Light => ("IFCLIGHTFIXTURETYPE", vec![en("NOTDEFINED")]),
            Self::Terminal => x.by(("IFCFLOWTERMINALTYPE", Vec::new()), ("IFCAIRTERMINALTYPE", vec![en("NOTDEFINED")])),
            Self::Proxy => ("IFCBUILDINGELEMENTPROXYTYPE", vec![en("NOTDEFINED")]),
        }
    }
}

/// 🧭️ The mesh of a solid of the building frame in the frame of the instance: the inverse of the yaw about the vertical axis and of the translation (the mirror stays in the mesh).
pub fn local_mesh(solid: &ElementSolid, placement: &ComponentPlacement) -> TriMesh {
    let (sin, cos) = placement.yaw.sin_cos();
    let corner = |index: u32| {
        let at = index as usize * 3;
        let (dx, dy, dz) = (solid.positions[at] - placement.x, solid.positions[at + 1] - placement.y, solid.positions[at + 2] - placement.z);
        [cos * dx + sin * dy, -sin * dx + cos * dy, dz]
    };
    let mut mesh = TriMesh::new();
    for triangle in solid.indices.chunks_exact(3) {
        mesh.push_triangle(corner(triangle[0]), corner(triangle[1]), corner(triangle[2]));
    }
    mesh
}


/// 🧬️ The authored rows of a family type: the family record, its parameters by name and its solids by id, as canonical JSON.
fn family_rows(family: &crate::Family, parameters: &BTreeMap<String, FamilyParameter>, solids: Option<&BTreeMap<String, FamilySolid>>) -> Vec<(&'static str, V)> {
    let empty = BTreeMap::new();
    vec![
        (FAMILY_ROWS[0], label(&semio_framework_pack_json::to_json_string(family))),
        (FAMILY_ROWS[1], label(&semio_framework_pack_json::to_json_string(parameters))),
        (FAMILY_ROWS[2], label(&semio_framework_pack_json::to_json_string(solids.unwrap_or(&empty)))),
    ]
}

fn family_type(x: &mut Export<'_>, family_id: &str, family: &crate::Family, kind: Kind, primary: bool, solids: &BTreeMap<&str, BTreeMap<String, FamilySolid>>) -> u64 {
    let key = if primary { family_id.to_string() } else { format!("{family_id}|{kind:?}") };
    let prefix = format!("{family_id}.");
    let parameters: BTreeMap<String, FamilyParameter> = x.model.family_parameters.range(prefix.clone()..).take_while(|(name, _)| name.starts_with(&prefix)).filter(|(_, row)| row.family == family_id).map(|(_, row)| (row.name.clone(), row.clone())).collect();
    let authoring = property_set(x, &format!("component-type:{key}:authoring"), "Semio_Authoring", family_rows(family, &parameters, solids.get(family_id)));
    let (entity, tail) = kind.type_object(x);
    let mut args = vec![unset(), refs(&[authoring]), unset(), opt_text(family_id), unset()];
    args.extend(tail);
    let object = x.ifc.rooted(entity, &format!("component-type:{key}"), &family.name, "", args);
    x.links.types.insert(("component", key), object);
    object
}

fn component(x: &mut Export<'_>, id: &str, row: &crate::Component, kind: Kind, object: u64) {
    let model = x.model;
    let (Some(storey), Some(value)) = (x.storeys.get(&row.storey).copied(), x.inferred.components.get(id)) else { return };
    let placement = &value.placement;
    let shape = x.solid(id).filter(|solid| !solid.is_empty()).and_then(|solid| {
        let item = mesh_item(&mut x.ifc, &local_mesh(solid, placement))?;
        let body = x.ifc.shape(x.ifc.body, "Body", body_kind(&x.ifc), &[item]);
        Some(x.ifc.definition(&[body]))
    });
    let axis = x.ifc.axis3([placement.x, placement.y, placement.z - storey.elevation], None, Some([placement.yaw.cos(), placement.yaw.sin(), 0.0]));
    let place = x.ifc.place(Some(storey.placement), axis);
    let (entity, tail) = kind.occurrence(x);
    let element = x.product(entity, id, &row.name, place, shape, tail);
    x.contain(&row.storey, id, element);
    x.links.typed.entry(object).or_default().push(element);
    x.links.quantities.push((element, QUANTITY_SET, vec![Quantity::Volume("GrossVolume", value.volume), Quantity::Volume("NetVolume", value.volume), Quantity::Area("GrossFootprintArea", value.footprint_area)]));
    x.links.authoring.push((element, vec![(RECORD_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    let overrides = overrides_of(model, id);
    if !overrides.is_empty() {
        let rows: Vec<(&str, V)> = overrides.iter().map(|(name, formula)| (name.as_str(), label(formula))).collect();
        let set = property_set(x, &format!("{id}:overrides"), OVERRIDES_SET, rows);
        define(x, &format!("{id}:overrides"), &[element], set);
    }
}

/// 🪑️ Writes every component with the type objects of its families.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    let mut solids: BTreeMap<&str, BTreeMap<String, FamilySolid>> = BTreeMap::new();
    for (id, row) in &model.family_solids {
        solids.entry(row.family.as_str()).or_default().insert(id.clone(), row.clone());
    }
    let mut types: BTreeMap<(String, Kind), u64> = BTreeMap::new();
    for (id, row) in &model.components {
        if !x.storeys.contains_key(&row.storey) {
            x.skip("component", id, "its storey is missing");
            continue;
        }
        let Some(family) = model.families.get(&row.family).filter(|family| family.category != FamilyCategory::Profile) else {
            x.skip("component", id, "its family is missing or a profile");
            continue;
        };
        if !x.inferred.components.contains_key(id) {
            x.skip("component", id, "it is not inferred");
            continue;
        }
        let kind = Kind::of(family.category, row.system.is_some());
        let object = match types.get(&(row.family.clone(), kind)).copied() {
            Some(object) => object,
            None => {
                let primary = !types.keys().any(|(known, _)| *known == row.family);
                let object = family_type(x, &row.family, family, kind, primary, &solids);
                types.insert((row.family.clone(), kind), object);
                object
            }
        };
        component(x, id, row, kind, object);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
