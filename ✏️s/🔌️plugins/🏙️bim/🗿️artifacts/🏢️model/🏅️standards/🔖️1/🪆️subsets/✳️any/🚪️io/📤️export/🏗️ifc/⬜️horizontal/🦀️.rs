//! ⬜️ Slabs and roofs. A level slab is an `IfcSlab` (`FLOOR`) whose boundary, with its holes, is extruded by the sum of its layers downward from the storey elevation plus `offset`;
//! a sloped slab is the faceted brep of its inferred solid. A roof is an `IfcRoof` aggregating one `IfcSlab` (`ROOF`) per layer, each a faceted brep of the roof solid.

use super::brep::{brep_definition, mesh_where};
use super::data::label;
use super::frames::{ccw, cw};
use super::writer::{en, real, rf, unset};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use crate::standards::v1::subsets::any::schema::inferences::quantities::LayerQuantity;
use crate::{RoofShape, Slab};

/// 🏷️ The `Semio_Authoring` row that holds the authored roof record: the shape, overhang and offsets IFC has no slot for.
pub const RECORD_ROW: &str = "Roof";
/// 🏷️ The `Semio_Authoring` row that holds the authored record of a sloped slab (a brep has no boundary to read back).
pub const SLAB_ROW: &str = "Slab";

fn shape_of(shape: &RoofShape) -> &'static str {
    match shape {
        RoofShape::Flat => "FLAT_ROOF",
        RoofShape::Shed { .. } => "SHED_ROOF",
        RoofShape::Gable { .. } => "GABLE_ROOF",
        RoofShape::Hip { .. } => "HIP_ROOF",
        RoofShape::Mansard { .. } => "MANSARD_ROOF",
    }
}

/// 🍰️ The take-off volume of every group of a roof solid: the rows are one per distinct (layer, material) of the groups, in that order.
pub fn group_volumes(solid: &ElementSolid, rows: &[LayerQuantity]) -> Vec<f64> {
    let mut keys: Vec<(u32, &str)> = solid.groups.iter().map(|group| (group.layer, group.material.as_str())).collect();
    keys.sort();
    keys.dedup();
    solid.groups.iter().map(|group| keys.iter().position(|key| *key == (group.layer, group.material.as_str())).and_then(|at| rows.get(at)).map_or(0.0, |row| row.volume)).collect()
}

fn slab(x: &mut Export<'_>, id: &str, row: &Slab) {
    let model = x.model;
    let (Some(storey), Some(kind), Some(bounds)) = (x.storeys.get(&row.storey).copied(), model.slab_types.get(&row.slab_type), x.solid(id).map(|solid| solid.bounds)) else {
        x.skip("slab", id, "its storey, type or inferred solid is missing");
        return;
    };
    let layers: f64 = kind.layers.iter().map(|layer| layer.thickness).sum();
    if layers <= 0.0 || row.boundary.len() < 3 {
        x.skip("slab", id, "it has no thickness or boundary");
        return;
    }
    let sloped = row.slope.is_some_and(|slope| slope.angle.abs() > 1e-12);
    let thickness = if sloped { layers } else { bounds.max.z - bounds.min.z };
    let origin = x.ifc.axis3([0.0, 0.0, if sloped { 0.0 } else { bounds.min.z - storey.elevation }], None, None);
    let placement = x.ifc.place(Some(storey.placement), origin);
    let shape = if sloped {
        let Some(solid) = x.solid(id) else {
            x.skip("slab", id, "its sloped solid is not inferred");
            return;
        };
        brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |_, _| true))
    } else {
        let outer = x.ifc.loop_curve(&ccw(&row.boundary));
        let holes: Vec<u64> = row.holes.iter().map(|hole| x.ifc.loop_curve(&cw(hole))).collect();
        let profile = x.ifc.curve_profile(outer, &holes);
        let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], thickness);
        let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
        Some(x.ifc.definition(&[body]))
    };
    let element = x.product("IFCSLAB", id, &row.name, placement, shape, vec![en("FLOOR")]);
    x.contain(&row.storey, id, element);
    if sloped {
        x.links.authoring.push((element, vec![(SLAB_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    }
    if let Some(object) = x.links.types.get(&("slab", row.slab_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    if let Some(set) = x.links.layer_sets.get(&("slab", row.slab_type.clone())).copied() {
        let usage = x.ifc.add("IFCMATERIALLAYERSETUSAGE", x.by(vec![rf(set), en("AXIS3"), en("NEGATIVE"), real(layers)], vec![rf(set), en("AXIS3"), en("NEGATIVE"), real(layers), unset()]));
        x.links.materials.entry(usage).or_default().push(element);
    }
    x.quantify(element, "Qto_SlabBaseQuantities", id, |row| {
        vec![
            Quantity::Length("Width", row.width),
            Quantity::Length("Perimeter", row.perimeter),
            Quantity::Area("GrossArea", row.gross_area),
            Quantity::Area("NetArea", row.net_area),
            Quantity::Volume("GrossVolume", row.gross_volume),
            Quantity::Volume("NetVolume", row.net_volume),
        ]
    });
}

fn roof(x: &mut Export<'_>, id: &str, row: &crate::Roof) {
    let Some(storey) = x.storeys.get(&row.storey).copied() else {
        x.skip("roof", id, "its storey is missing");
        return;
    };
    let Some(solid) = x.solid(id) else {
        x.skip("roof", id, "its solid is not inferred");
        return;
    };
    let origin = x.ifc.place(Some(storey.placement), x.ifc.origin);
    let roof = x.product("IFCROOF", id, &row.name, origin, None, vec![en(shape_of(&row.shape))]);
    x.contain(&row.storey, id, roof);
    if let Some(object) = x.links.types.get(&("roof", row.roof_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(roof);
    }
    x.links.authoring.push((roof, vec![(RECORD_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    x.quantify(roof, "Qto_RoofBaseQuantities", id, |row| vec![Quantity::Area("ProjectedArea", row.gross_area)]);
    let volumes = x.measure(id).map(|row| group_volumes(solid, &row.layers));
    let mut parts = Vec::new();
    for (index, group) in solid.groups.iter().enumerate() {
        let mesh = mesh_where(solid, storey.elevation, |candidate, _| candidate == group);
        let Some(shape) = brep_definition(&mut x.ifc, &mesh) else { continue };
        let part_id = format!("{id}:layer{index}");
        let part_place = x.ifc.place(Some(storey.placement), x.ifc.origin);
        let part = x.product("IFCSLAB", &part_id, &row.name, part_place, Some(shape), vec![en("ROOF")]);
        if let Some(definition) = x.links.material_defs.get(&group.material).copied() {
            x.links.materials.entry(definition).or_default().push(part);
        }
        if let Some(volume) = volumes.as_ref().and_then(|volumes| volumes.get(index)) {
            x.links.quantities.push((part, "Qto_SlabBaseQuantities", vec![Quantity::Volume("GrossVolume", *volume)]));
        }
        parts.push(part);
    }
    x.links.aggregated.insert(roof, parts);
}

/// ⬜️ Writes every slab and roof.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.slabs {
        slab(x, id, row);
    }
    for (id, row) in &model.roofs {
        roof(x, id, row);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
