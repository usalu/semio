//! 🔲️ Ceilings of an IFC file. An `IfcCovering` of type `CEILING` that is a swept outline is read like a slab: the outline and its islands give the boundary and holes, the extent of the sweep the thickness, and the distance
//! from the top of the sweep to the top of its storey the `offset`; the ceiling type is the `IfcCoveringType` it is typed by, else a single-layer type of that thickness. A sloped ceiling is a faceted brep with no outline to
//! read, so it is restored from the `Ceiling` row of its `Semio_Authoring` set (storey, type and material still come from the file). A covering of another predefined type, or one that is neither, is reported by the
//! unsupported-class note.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifccovering.htm>

use super::data::label;
use super::reader::{text, Doc, Loop, Section};
use super::spatial::authoring_of;
use super::Import;
use crate::standards::v1::subsets::any::io::export::ifc::ceilings::RECORD_ROW;
use crate::{Ceiling, CeilingType, Layer, LayerFunction, Point2, Vertex};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::Part21Value;

fn vertices(ring: &Loop, to_building: &impl Fn([f64; 2]) -> Point2) -> Vec<Vertex> {
    ring.iter().map(|(point, bulge)| Vertex { point: to_building(*point), bulge: *bulge }).collect()
}

fn layered_ceiling_type(i: &mut Import<'_>, ifc: u64, thickness: f64) -> String {
    if let Some(known) = i.doc.index.types.get(&ifc).and_then(|kind| i.type_ids.get(kind)).filter(|id| i.model.ceiling_types.contains_key(*id)) {
        return known.clone();
    }
    let material = i.doc.index.materials.get(&ifc).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default();
    if let Some((id, _)) = i.model.ceiling_types.iter().find(|(_, kind)| kind.layers.len() == 1 && (kind.layers[0].thickness - thickness).abs() < 1e-9 && kind.layers[0].material == material) {
        return id.clone();
    }
    let millimetres = (thickness * 1000.0).round() as i64;
    let id = Import::unused(&format!("cet-{millimetres}"), |candidate| i.model.ceiling_types.contains_key(candidate));
    i.model.ceiling_types.insert(id.clone(), CeilingType { name: format!("Ceiling {millimetres} mm"), layers: vec![Layer { material, thickness, function: LayerFunction::Finish }] });
    id
}

fn authored(i: &mut Import<'_>, ifc: u64, record: &str, storey: String, name: &str) -> Option<Ceiling> {
    let Ok(mut ceiling) = from_json_str::<Ceiling>(record, JsonMemberPolicy::Reject) else {
        i.skip("IFCCOVERING", name, "its authored ceiling record is not valid");
        return None;
    };
    ceiling.storey = storey;
    if let Some(known) = i.doc.index.types.get(&ifc).and_then(|kind| i.type_ids.get(kind)).filter(|id| i.model.ceiling_types.contains_key(*id)) {
        ceiling.ceiling_type = known.clone();
    }
    Some(ceiling)
}

fn swept(i: &mut Import<'_>, ifc: u64, args: &[Part21Value], storey: &str, name: &str) -> Option<Ceiling> {
    let Some(body) = i.doc.body(args) else {
        i.skip("IFCCOVERING", name, "it has no swept body");
        return None;
    };
    let Section::Outline { outer, holes } = &body.section else {
        i.skip("IFCCOVERING", name, "its profile is not an outline");
        return None;
    };
    let to_building = i.in_building(&args[5], storey);
    let map = |point: [f64; 2]| {
        let world = to_building.point(body.position.point([point[0], point[1], 0.0]));
        Point2 { x: world[0], y: world[1] }
    };
    let thickness = body.depth * body.direction[2].abs();
    let origin = to_building.point(body.position.origin)[2];
    let top = if body.direction[2] < 0.0 { origin } else { origin + thickness };
    let storey_top = i.levels.get(storey).map_or(0.0, |level| level.top_elevation);
    let (boundary, holes) = (vertices(outer, &map), holes.iter().map(|hole| vertices(hole, &map)).collect());
    let ceiling_type = layered_ceiling_type(i, ifc, thickness);
    Some(Ceiling { storey: storey.to_string(), ceiling_type, boundary, holes, offset: storey_top - top, slope: None, name: String::new() })
}

/// 🔲️ Reads the ceilings of the file into the model.
pub fn read(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCCOVERING") {
        if args.get(8).and_then(Part21Value::as_enum) != Some("CEILING") {
            continue;
        }
        let name = text(args, 2);
        let Some(storey) = i.storey_of(instance.id) else {
            i.skip("IFCCOVERING", &name, "it is not in a storey");
            continue;
        };
        let record = label(&authoring_of(&i.doc, instance.id), RECORD_ROW);
        let found = match record {
            Some(record) => authored(i, instance.id, &record, storey, &name),
            None => swept(i, instance.id, args, &storey, &name),
        };
        let Some(mut ceiling) = found else { continue };
        let id = Doc::identity(args, "IFCCOVERING").unwrap_or_else(|| format!("ce-{}", instance.id));
        ceiling.name = if name == id { String::new() } else { name };
        i.model.ceilings.insert(id.clone(), ceiling);
        i.ids.insert(instance.id, id);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
