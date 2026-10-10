//! ⬜️ Roofs of an IFC file. An `IfcRoof` that carries the authored roof record in the `Roof` row of its `Semio_Authoring` set is restored exactly: the record from the row, the storey from the spatial structure; the
//! `IfcSlab` layers it aggregates are its inferred parts and are not imported on their own. A flat roof of a foreign file (`FLAT_ROOF` with a swept outline) becomes a flat roof of that outline with the type its
//! association names (else one layer of the sweep depth); a pitched foreign roof has no pitch to read (its geometry is a mesh) and is reported by the unsupported-class note.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgelements/lexical/ifcroof.htm>

use super::data::label;
use super::reader::{text, Doc, Section};
use super::spatial::{authoring_of, phase_of};
use super::Import;
use crate::{Layer, LayerFunction, Point2, Roof, RoofShape, RoofType, Vertex};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::Part21Value;

fn roof_type_for(i: &mut Import<'_>, ifc: u64, thickness: f64) -> String {
    if let Some(known) = i.doc.index.types.get(&ifc).and_then(|kind| i.type_ids.get(kind)).filter(|id| i.model.roof_types.contains_key(*id)) {
        return known.clone();
    }
    let material = i.doc.index.materials.get(&ifc).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default();
    let layers = vec![Layer { material, thickness, function: LayerFunction::Structure }];
    if let Some((id, _)) = i.model.roof_types.iter().find(|(_, kind)| kind.layers == layers) {
        return id.clone();
    }
    let id = Import::unused("rt-imported", |candidate| i.model.roof_types.contains_key(candidate));
    i.model.roof_types.insert(id.clone(), RoofType { name: format!("Roof {} mm", (thickness * 1000.0).round() as i64), layers });
    id
}

fn foreign(i: &mut Import<'_>, ifc: u64, args: &[Part21Value], storey: &str) -> Option<Roof> {
    if args.get(8).and_then(Part21Value::as_enum) != Some("FLAT_ROOF") {
        return None;
    }
    let body = i.doc.body(args)?;
    let Section::Outline { outer, .. } = &body.section else { return None };
    let to_building = i.in_building(&args[5], storey);
    let footprint = outer
        .iter()
        .map(|(point, bulge)| {
            let world = to_building.point(body.position.point([point[0], point[1], 0.0]));
            Vertex { point: Point2 { x: world[0], y: world[1] }, bulge: *bulge }
        })
        .collect();
    let elevation = i.levels.get(storey).map_or(0.0, |level| level.elevation);
    let base_offset = to_building.point(body.position.origin)[2] - elevation;
    let roof_type = roof_type_for(i, ifc, body.depth * body.direction[2].abs());
    Some(Roof { storey: storey.to_string(), roof_type, footprint, shape: RoofShape::Flat, overhang: 0.0, base_offset, phase: phase_of(&i.doc, ifc), name: text(args, 2) })
}

/// ⬜️ Reads the roofs of the file into the model.
pub fn read(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCROOF") {
        let name = text(args, 2);
        let Some(storey) = i.storey_of(instance.id) else {
            i.skip("IFCROOF", &name, "it is not in a storey");
            continue;
        };
        let recorded = label(&authoring_of(&i.doc, instance.id), "Roof").and_then(|record| from_json_str::<Roof>(&record, JsonMemberPolicy::Reject).ok());
        let Some(roof) = recorded.map(|roof| Roof { storey: storey.clone(), ..roof }).or_else(|| foreign(i, instance.id, args, &storey)) else {
            i.skip("IFCROOF", &name, "it carries no authored roof record and no flat swept outline");
            continue;
        };
        let id = Doc::identity(args, "IFCROOF").unwrap_or_else(|| format!("r-{}", instance.id));
        i.model.roofs.insert(id.clone(), roof);
        i.claim_parts(instance.id, &id);
        i.ids.insert(instance.id, id);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
