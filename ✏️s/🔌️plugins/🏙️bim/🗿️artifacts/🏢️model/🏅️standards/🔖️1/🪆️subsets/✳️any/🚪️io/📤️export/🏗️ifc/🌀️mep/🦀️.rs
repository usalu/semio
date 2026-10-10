//! 🌀️ Routed MEP elements and their systems. A duct, pipe or cable tray is an `IfcFlowSegment` (IFC4: `IfcDuctSegment`, `IfcPipeSegment`, `IfcCableCarrierSegment`) typed by an `IfcDuctSegmentType`, `IfcPipeSegmentType` or
//! `IfcCableCarrierSegmentType` per distinct section; its `Axis` representation is the centre line as a 3D polyline in the storey frame and its `Body` the faceted brep (IFC4: tessellation) of the inferred mitred sweep. The base
//! quantities are the length, the cross-section area and the volume of the inference; the authored element is the `MepElement` row of its `Semio_Authoring` set. Every service in use is one `IfcSystem` (IFC4:
//! `IfcDistributionSystem` with its predefined type) that groups the segments and the terminals (components with a system) of that service with `IfcRelAssignsToGroup` and serves the buildings they stand in.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgserviceelements/lexical/ifcflowsegment.htm>

use super::brep::{body_kind, mesh_item, mesh_where};
use super::data::{label, property_set};
use super::writer::{en, opt_text, refs, rf, unset, V};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::mep::{key, MepSectionKind};
use crate::{MepSystem, MepShape};
use std::collections::{BTreeMap, BTreeSet};

/// 🏷️ The `Semio_Authoring` row that holds the authored MEP element record.
pub const RECORD_ROW: &str = "MepElement";

/// 🌀️ The IFC entity names of a section kind: `(IFC 2x3 occurrence, IFC4 occurrence, type entity, predefined type, quantity set)`.
pub fn entities(kind: MepSectionKind) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
    match kind {
        MepSectionKind::Duct => ("IFCFLOWSEGMENT", "IFCDUCTSEGMENT", "IFCDUCTSEGMENTTYPE", "RIGIDSEGMENT", "Qto_DuctSegmentBaseQuantities"),
        MepSectionKind::Pipe => ("IFCFLOWSEGMENT", "IFCPIPESEGMENT", "IFCPIPESEGMENTTYPE", "RIGIDSEGMENT", "Qto_PipeSegmentBaseQuantities"),
        MepSectionKind::Tray => ("IFCFLOWSEGMENT", "IFCCABLECARRIERSEGMENT", "IFCCABLECARRIERSEGMENTTYPE", "CABLETRAYSEGMENT", "Qto_CableCarrierSegmentBaseQuantities"),
    }
}

/// 🏷️ The name of a section kind in the type name.
pub fn kind_name(kind: MepSectionKind) -> &'static str {
    match kind {
        MepSectionKind::Duct => "Duct",
        MepSectionKind::Pipe => "Pipe",
        MepSectionKind::Tray => "Tray",
    }
}

/// 🌀️ The IFC4 `IfcDistributionSystemEnum` literal of a service.
pub fn predefined(system: MepSystem) -> &'static str {
    match system {
        MepSystem::Supply | MepSystem::Return => "VENTILATION",
        MepSystem::Exhaust => "EXHAUST",
        MepSystem::DomesticWater => "DOMESTICCOLDWATER",
        MepSystem::Waste => "WASTEWATER",
        MepSystem::Gas => "GAS",
        MepSystem::Power => "ELECTRICAL",
        MepSystem::Data => "DATA",
        MepSystem::Lighting => "LIGHTING",
    }
}

/// 🏷️ The name of the system group of a service.
pub fn system_name(system: MepSystem) -> String {
    format!("{system:?}")
}

fn segment_type(x: &mut Export<'_>, types: &mut BTreeMap<String, u64>, kind: MepSectionKind, section_label: &str, shape: &MepShape) -> u64 {
    let id = format!("mep-type:{}:{section_label}", kind.key());
    if let Some(object) = types.get(&id) {
        return *object;
    }
    let (_, _, entity, predefined_type, _) = entities(kind);
    let authoring = property_set(x, &format!("{id}:authoring"), "Semio_Authoring", vec![("Shape", label(&semio_framework_pack_json::to_json_string(shape)))]);
    let object = x.ifc.rooted(entity, &id, &format!("{} {section_label}", kind_name(kind)), "", vec![unset(), refs(&[authoring]), unset(), opt_text(&id), unset(), en(predefined_type)]);
    x.links.types.insert(("mep", id.clone()), object);
    types.insert(id, object);
    object
}

fn segment(x: &mut Export<'_>, types: &mut BTreeMap<String, u64>, id: &str, row: &crate::MepElement) {
    let Some(storey) = x.storeys.get(&row.storey).copied() else {
        x.skip("mep element", id, "its storey is missing");
        return;
    };
    let (Some(value), Some(solid)) = (x.inferred.mep.get(id), x.solid(id)) else {
        x.skip("mep element", id, "it is not inferred");
        return;
    };
    if !value.buildable() || solid.is_empty() {
        x.skip("mep element", id, "its section or path has no extent");
        return;
    }
    let kind = value.section.kind;
    let (occurrence_2x3, occurrence_4, _, _, quantity_set) = entities(kind);
    let Some(item) = mesh_item(&mut x.ifc, &mesh_where(solid, storey.elevation, |_, _| true)) else {
        x.skip("mep element", id, "its solid is empty");
        return;
    };
    let body = x.ifc.shape(x.ifc.body, "Body", body_kind(&x.ifc), &[item]);
    let points: Vec<u64> = row.path.iter().map(|point| x.ifc.point3([point.x, point.y, point.z])).collect();
    let curve = x.ifc.add("IFCPOLYLINE", vec![refs(&points)]);
    let axis = x.ifc.shape(x.ifc.axis, "Axis", "Curve3D", &[curve]);
    let shape = x.ifc.definition(&[axis, body]);
    let place = x.ifc.place(Some(storey.placement), x.ifc.origin);
    let (entity, tail) = x.by((occurrence_2x3, Vec::<V>::new()), (occurrence_4, vec![unset()]));
    let element = x.product(entity, id, &row.name, place, Some(shape), tail);
    x.contain(&row.storey, id, element);
    let object = segment_type(x, types, kind, &value.section.label, &row.shape);
    x.links.typed.entry(object).or_default().push(element);
    x.links.quantities.push((element, quantity_set, vec![Quantity::Length("Length", value.length), Quantity::Area("CrossSectionArea", value.section.area), Quantity::Volume("GrossVolume", value.volume), Quantity::Volume("NetVolume", value.volume)]));
    x.links.authoring.push((element, vec![(RECORD_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
}

/// 🌀️ Writes every routed MEP element with the type objects of its sections.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    let mut types: BTreeMap<String, u64> = BTreeMap::new();
    for (id, row) in &model.mep_elements {
        segment(x, &mut types, id, row);
    }
}

/// 🌀️ Writes one system group per service in use: the segments and terminals written for it, assigned with `IfcRelAssignsToGroup`, serving the buildings they stand in with `IfcRelServicesBuildings`.
pub fn emit_systems(x: &mut Export<'_>) {
    let model = x.model;
    let mut members: BTreeMap<&'static str, (MepSystem, Vec<(u64, String)>)> = BTreeMap::new();
    for (id, row) in &model.mep_elements {
        if let Some(entity) = x.links.elements.get(id).copied() {
            members.entry(key(row.system)).or_insert_with(|| (row.system, Vec::new())).1.push((entity, row.storey.clone()));
        }
    }
    for (id, row) in &model.components {
        if let (Some(system), Some(entity)) = (row.system, x.links.elements.get(id).copied()) {
            members.entry(key(system)).or_insert_with(|| (system, Vec::new())).1.push((entity, row.storey.clone()));
        }
    }
    for (token, (system, rows)) in members {
        let name = system_name(system);
        let guid = format!("system:{token}");
        let (entity, tail) = x.by(("IFCSYSTEM", vec![opt_text(&name)]), ("IFCDISTRIBUTIONSYSTEM", vec![opt_text(&name), unset(), en(predefined(system))]));
        let group = x.ifc.rooted(entity, &guid, &name, "", tail);
        let entities: Vec<u64> = rows.iter().map(|(entity, _)| *entity).collect();
        x.ifc.rooted("IFCRELASSIGNSTOGROUP", &guid, "", "", vec![refs(&entities), unset(), rf(group)]);
        let buildings: BTreeSet<u64> = rows.iter().filter_map(|(_, storey)| model.storeys.get(storey)).filter_map(|storey| x.buildings.get(&storey.building)).map(|building| building.ifc).collect();
        if !buildings.is_empty() {
            let served: Vec<u64> = buildings.into_iter().collect();
            x.ifc.rooted("IFCRELSERVICESBUILDINGS", &guid, "", "", vec![rf(group), refs(&served)]);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
