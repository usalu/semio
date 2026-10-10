//! 🪜️ Stairs and railings of an IFC file. An `IfcStair` or `IfcRailing` that carries the authored record in the `Stair` or `Railing` row of its `Semio_Authoring` set is restored exactly: the record from the row, the
//! storey from the spatial structure; the flights, stringers, balusters and infill it aggregates are its inferred parts and are not imported on their own. A stair of a foreign file becomes a straight stair from its
//! first flight (placement, riser count and height, tread length, the width of a swept body, else one metre); a railing of a foreign file becomes a railing along its `Axis` or `FootPrint` polyline with a plain rail and
//! posts and the `Height` of its base quantities (else one metre). Whatever has no such flight or polyline is reported by the unsupported-class note.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgelements/lexical/ifcstair.htm>

use super::data::label;
use super::reader::{real, text, Doc, Section};
use super::spatial::{authoring_of, phase_of, quantity};
use super::Import;
use crate::{Infill, Point2, Profile, Railing, RiserKind, Stair, StairFlight, StairStringer, StringerKind, TopConstraint};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::Part21Value;

fn foreign_stair(i: &mut Import<'_>, ifc: u64, args: &[Part21Value], storey: &str) -> Option<Stair> {
    let flights: Vec<u64> = i.doc.index.parts.get(&ifc)?.iter().copied().filter(|part| i.doc.args(*part, "IFCSTAIRFLIGHT").is_some()).collect();
    let first = i.doc.args(*flights.first()?, "IFCSTAIRFLIGHT")?;
    let (risers, riser_height, tread) = (flights.iter().filter_map(|flight| real(i.doc.args(*flight, "IFCSTAIRFLIGHT")?, 8)).sum::<f64>(), real(first, 10)?, real(first, 11)?);
    if risers < 1.0 || riser_height <= 0.0 || tread <= 0.0 {
        return None;
    }
    let to_building = i.in_building(&first[5], storey);
    let width = match i.doc.body(first).map(|body| body.section) {
        Some(Section::Rectangle { depth, .. }) => depth,
        _ => 1.0,
    };
    Some(Stair {
        storey: storey.to_string(),
        start: Point2 { x: to_building.origin[0], y: to_building.origin[1] },
        direction: to_building.heading(),
        width,
        flight: StairFlight::Straight,
        top: TopConstraint::Unconnected { height: risers * riser_height },
        max_riser: riser_height,
        min_tread: tread,
        stringer: StairStringer { kind: StringerKind::None, width: 0.0, depth: 0.0 },
        nosing: 0.0,
        tread_thickness: 0.04,
        riser: RiserKind::Open,
        landing_depth: 0.0,
        phase: phase_of(&i.doc, ifc),
        name: text(args, 2),
    })
}

fn foreign_railing(i: &mut Import<'_>, ifc: u64, args: &[Part21Value], storey: &str) -> Option<Railing> {
    let line = i.doc.path(args, "Axis").or_else(|| i.doc.path(args, "FootPrint"))?;
    let to_building = i.in_building(&args[5], storey);
    let path = line
        .iter()
        .map(|point| {
            let world = to_building.point([point[0], point[1], 0.0]);
            Point2 { x: world[0], y: world[1] }
        })
        .collect();
    let elevation = i.levels.get(storey).map_or(0.0, |level| level.elevation);
    let plain = Profile::Rectangle { width: 0.05, depth: 0.05 };
    Some(Railing {
        storey: storey.to_string(),
        path,
        height: quantity(&i.doc, ifc, "Height").filter(|height| *height > 0.0).unwrap_or(1.0),
        post_spacing: 1.2,
        profile: plain.clone(),
        post_profile: plain,
        baluster: None,
        infill: Infill::None,
        material: i.doc.index.materials.get(&ifc).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default(),
        base_offset: to_building.origin[2] - elevation,
        host: None,
        phase: phase_of(&i.doc, ifc),
        name: text(args, 2),
    })
}

/// 🪜️ Reads the stairs and railings of the file into the model.
pub fn read(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCSTAIR") {
        let name = text(args, 2);
        let Some(storey) = i.storey_of(instance.id) else {
            i.skip("IFCSTAIR", &name, "it is not in a storey");
            continue;
        };
        let recorded = label(&authoring_of(&i.doc, instance.id), "Stair").and_then(|record| from_json_str::<Stair>(&record, JsonMemberPolicy::Reject).ok());
        let Some(stair) = recorded.map(|stair| Stair { storey: storey.clone(), ..stair }).or_else(|| foreign_stair(i, instance.id, args, &storey)) else {
            i.skip("IFCSTAIR", &name, "it carries no authored stair record and no flight with risers and treads");
            continue;
        };
        let id = Doc::identity(args, "IFCSTAIR").unwrap_or_else(|| format!("st-{}", instance.id));
        i.model.stairs.insert(id.clone(), stair);
        i.claim_parts(instance.id, &id);
        i.ids.insert(instance.id, id);
    }
    let railings: Vec<_> = i.doc.rows("IFCRAILING").into_iter().filter(|(instance, _)| !i.ids.contains_key(&instance.id) && !i.doc.index.whole.contains_key(&instance.id)).collect();
    for (instance, args) in railings {
        let name = text(args, 2);
        let Some(storey) = i.storey_of(instance.id) else {
            i.skip("IFCRAILING", &name, "it is not in a storey");
            continue;
        };
        let recorded = label(&authoring_of(&i.doc, instance.id), "Railing").and_then(|record| from_json_str::<Railing>(&record, JsonMemberPolicy::Reject).ok());
        let Some(railing) = recorded.map(|railing| Railing { storey: storey.clone(), ..railing }).or_else(|| foreign_railing(i, instance.id, args, &storey)) else {
            i.skip("IFCRAILING", &name, "it carries no authored railing record and no polyline path");
            continue;
        };
        let id = Doc::identity(args, "IFCRAILING").unwrap_or_else(|| format!("rl-{}", instance.id));
        i.model.railings.insert(id.clone(), railing);
        i.claim_parts(instance.id, &id);
        i.ids.insert(instance.id, id);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
