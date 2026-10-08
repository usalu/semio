//! 📐️ Validity of the elements of one storey: degenerate geometry, openings that do not fit their host (from `opening-frames`) and stairs that
//! break the code (from `stair-runs`).

use super::{Diagnostic, DiagnosticCode, Inputs};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged as corners, extents_of, seg};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::roofs::RoofFallback;
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::OpeningIssue;
use crate::standards::v1::subsets::any::schema::inferences::spaces::SpaceStatus;
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{BLONDEL_MAX, BLONDEL_MIN};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{target_of, vertical_of};
use crate::{Axis, ModelSnapshot, Point2, SpaceBoundary, Vertex};
use semio_framework_geometry::loops;

/// 📏️ Lengths below this (metres) are no length.
pub const LENGTH_EPS: f64 = 1e-6;

fn finite_point(point: &Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

fn finite_axis(axis: &Axis) -> bool {
    match axis {
        Axis::Line { start, end } => finite_point(start) && finite_point(end),
        Axis::Arc { start, end, bulge } => finite_point(start) && finite_point(end) && bulge.is_finite(),
    }
}

fn finite_loop(vertices: &[Vertex]) -> bool {
    vertices.iter().all(|vertex| finite_point(&vertex.point) && vertex.bulge.is_finite())
}

fn outline(found: &mut Vec<Diagnostic>, storey: &str, id: &str, vertices: &[Vertex]) {
    if !finite_loop(vertices) {
        found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
        return;
    }
    let ring = corners(vertices);
    let area = if ring.len() >= 3 { loops::area(&ring) } else { 0.0 };
    if area <= LENGTH_EPS * LENGTH_EPS {
        found.push(Diagnostic::new(DiagnosticCode::DegenerateLoop, &[id]).on(storey).with("area", area));
    } else {
        let crossings = loops::self_intersections(&ring).len();
        if crossings > 0 {
            found.push(Diagnostic::new(DiagnosticCode::SelfIntersectingLoop, &[id]).on(storey).with("crossings", crossings as f64));
        }
    }
}

fn axis(found: &mut Vec<Diagnostic>, storey: &str, id: &str, axis: &Axis) -> bool {
    if !finite_axis(axis) {
        found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
        return false;
    }
    let length = seg(axis).length();
    if length <= LENGTH_EPS {
        found.push(Diagnostic::new(DiagnosticCode::DegenerateAxis, &[id]).on(storey).with("length", length));
        return false;
    }
    true
}

fn height(found: &mut Vec<Diagnostic>, storey: &str, id: &str, height: f64) {
    if height <= LENGTH_EPS {
        found.push(Diagnostic::new(DiagnosticCode::DegenerateHeight, &[id]).on(storey).with("height", height));
    }
}

fn degenerate(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>, found: &mut Vec<Diagnostic>) {
    let Some(own) = inputs.levels.get(storey) else { return };
    if let Some(row) = snapshot.storeys.get(storey).filter(|row| !(row.height > LENGTH_EPS)) {
        found.push(Diagnostic::new(DiagnosticCode::DegenerateStorey, &[storey]).on(storey).with("height", row.height));
    }
    for (id, wall) in snapshot.walls.iter().filter(|(_, row)| row.storey == storey) {
        if axis(found, storey, id, &wall.axis) {
            let layout = inputs.layouts.get(id.as_str());
            if snapshot.wall_types.contains_key(&wall.wall_type) && layout.is_some_and(|layout| layout.thickness <= LENGTH_EPS) {
                found.push(Diagnostic::new(DiagnosticCode::DegenerateThickness, &[id]).on(storey));
            }
            if let Some(layout) = layout {
                height(found, storey, id, layout.height);
            }
        }
    }
    for (id, curtain) in snapshot.curtain_walls.iter().filter(|(_, row)| row.storey == storey) {
        if axis(found, storey, id, &curtain.axis) {
            let (base, top) = vertical_of(curtain.base_offset, &curtain.top, own, target_of(&curtain.top, &inputs.levels));
            height(found, storey, id, top - base);
        }
        if !(curtain.u_spacing > LENGTH_EPS && curtain.v_spacing > LENGTH_EPS) {
            found.push(Diagnostic::new(DiagnosticCode::DegenerateSpacing, &[id]).on(storey));
        }
        let (width, depth) = extents_of(&curtain.mullion);
        if !(width > LENGTH_EPS && depth > LENGTH_EPS) {
            found.push(Diagnostic::new(DiagnosticCode::DegenerateProfile, &[id]).on(storey));
        }
    }
    for (id, column) in snapshot.columns.iter().filter(|(_, row)| row.storey == storey) {
        if !finite_point(&column.position) || !column.rotation.is_finite() {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            continue;
        }
        if let Some(kind) = snapshot.column_types.get(&column.column_type) {
            let (width, depth) = extents_of(&kind.profile);
            if !(width > LENGTH_EPS && depth > LENGTH_EPS) {
                found.push(Diagnostic::new(DiagnosticCode::DegenerateProfile, &[id]).on(storey));
            }
        }
        let (base, top) = vertical_of(column.base_offset, &column.top, own, target_of(&column.top, &inputs.levels));
        height(found, storey, id, top - base);
    }
    for (id, beam) in snapshot.beams.iter().filter(|(_, row)| row.storey == storey) {
        if !(finite_point(&beam.start) && finite_point(&beam.end) && beam.top_offset.is_finite()) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            continue;
        }
        let length = (beam.end.x - beam.start.x).hypot(beam.end.y - beam.start.y);
        if length <= LENGTH_EPS {
            found.push(Diagnostic::new(DiagnosticCode::DegenerateAxis, &[id]).on(storey).with("length", length));
        }
        if let Some(kind) = snapshot.beam_types.get(&beam.beam_type) {
            let (width, depth) = extents_of(&kind.profile);
            if !(width > LENGTH_EPS && depth > LENGTH_EPS) {
                found.push(Diagnostic::new(DiagnosticCode::DegenerateProfile, &[id]).on(storey));
            }
        }
    }
    for (id, slab) in snapshot.slabs.iter().filter(|(_, row)| row.storey == storey) {
        outline(found, storey, id, &slab.boundary);
        slab.holes.iter().for_each(|hole| outline(found, storey, id, hole));
    }
    for (id, roof) in snapshot.roofs.iter().filter(|(_, row)| row.storey == storey) {
        outline(found, storey, id, &roof.footprint);
    }
    for (id, railing) in snapshot.railings.iter().filter(|(_, row)| row.storey == storey) {
        if !railing.path.iter().all(finite_point) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
        } else if railing.path.windows(2).map(|pair| (pair[1].x - pair[0].x).hypot(pair[1].y - pair[0].y)).sum::<f64>() <= LENGTH_EPS {
            found.push(Diagnostic::new(DiagnosticCode::DegeneratePath, &[id]).on(storey));
        }
    }
    for (id, space) in snapshot.spaces.iter().filter(|(_, row)| row.storey == storey) {
        match &space.boundary {
            SpaceBoundary::Explicit { outline: ring } => outline(found, storey, id, ring),
            SpaceBoundary::Bounded { seed } if !finite_point(seed) => found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey)),
            SpaceBoundary::Bounded { .. } => {}
        }
    }
}

fn openings(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>, found: &mut Vec<Diagnostic>) {
    let hosted = |host: &str| snapshot.walls.get(host).map(|wall| &wall.storey).or_else(|| snapshot.curtain_walls.get(host).map(|curtain| &curtain.storey));
    for (id, _) in snapshot.openings.iter().filter(|(_, opening)| hosted(&opening.host).is_some_and(|host_storey| host_storey == storey)) {
        let Some(frame) = inputs.frames.get(id.as_str()) else { continue };
        for issue in &frame.issues {
            match issue {
                OpeningIssue::OutsideHostExtent => found.push(Diagnostic::new(DiagnosticCode::OpeningOutsideHost, &[id]).on(storey)),
                OpeningIssue::BelowHostBase => found.push(Diagnostic::new(DiagnosticCode::OpeningBelowBase, &[id]).on(storey)),
                OpeningIssue::AboveHostTop => found.push(Diagnostic::new(DiagnosticCode::OpeningAboveTop, &[id]).on(storey)),
                OpeningIssue::OutsideTrimmedExtent => found.push(Diagnostic::new(DiagnosticCode::OpeningOutsideTrimmed, &[id]).on(storey)),
                OpeningIssue::NonPositiveSize => found.push(Diagnostic::new(DiagnosticCode::OpeningSize, &[id]).on(storey)),
                OpeningIssue::OverlapsSibling => {
                    for other in &frame.overlaps {
                        let (first, second) = if id <= other { (id.as_str(), other.as_str()) } else { (other.as_str(), id.as_str()) };
                        found.push(Diagnostic::new(DiagnosticCode::OpeningOverlap, &[first, second]).on(storey));
                    }
                }
                OpeningIssue::HostMissing | OpeningIssue::TypeMissing | OpeningIssue::HostDegenerate => {}
            }
        }
    }
}

fn stairs(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>, found: &mut Vec<Diagnostic>) {
    for (id, stair) in snapshot.stairs.iter().filter(|(_, row)| row.storey == storey) {
        if !(finite_point(&stair.start) && stair.direction.is_finite() && stair.width.is_finite()) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            continue;
        }
        let Some(run) = inputs.runs.get(id.as_str()) else { continue };
        if run.riser_count == 0 || !run.compliance.rise_positive {
            found.push(Diagnostic::new(DiagnosticCode::StairNoRise, &[id]).on(storey).with("rise", run.rise));
            continue;
        }
        if !run.compliance.riser_ok {
            found.push(Diagnostic::new(DiagnosticCode::StairRiserHeight, &[id]).on(storey).with("riser_height", run.riser_height).with("limit", stair.max_riser));
        }
        if !run.compliance.tread_ok {
            found.push(Diagnostic::new(DiagnosticCode::StairTreadDepth, &[id]).on(storey).with("tread", run.tread).with("limit", stair.min_tread));
        }
        if !run.compliance.blondel_ok {
            found.push(Diagnostic::new(DiagnosticCode::StairComfort, &[id]).on(storey).with("stride", run.stride).with("minimum", BLONDEL_MIN).with("maximum", BLONDEL_MAX));
        }
    }
}

fn spaces(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>, found: &mut Vec<Diagnostic>) {
    if !snapshot.spaces.values().any(|space| space.storey == storey && matches!(space.boundary, SpaceBoundary::Bounded { .. })) {
        return;
    }
    for (id, room) in inputs.rooms.into_iter().flatten() {
        match room.status {
            SpaceStatus::NotEnclosed => found.push(Diagnostic::new(DiagnosticCode::SpaceNotEnclosed, &[id]).on(storey)),
            SpaceStatus::SeedInsideWall => found.push(Diagnostic::new(DiagnosticCode::SpaceSeedInWall, &[id]).on(storey)),
            SpaceStatus::Inferred | SpaceStatus::Explicit | SpaceStatus::InvalidOutline => {}
        }
    }
}

fn roofs(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>, found: &mut Vec<Diagnostic>) {
    for (id, _) in snapshot.roofs.iter().filter(|(_, row)| row.storey == storey) {
        if let Some(fallback) = inputs.fallbacks.get(id.as_str()) {
            found.push(Diagnostic::new(fallback_code(*fallback), &[id]).on(storey));
        }
    }
}

/// 🏠️ The diagnostic code of a roof fallback: a roof that is flat although its shape asks for more, or whose overhang collapsed.
pub fn fallback_code(fallback: RoofFallback) -> DiagnosticCode {
    match fallback {
        RoofFallback::CurvedFootprint => DiagnosticCode::RoofFlatCurved,
        RoofFallback::NonConvexFootprint => DiagnosticCode::RoofFlatNonConvex,
        RoofFallback::DegenerateFootprint => DiagnosticCode::RoofFlatDegenerate,
        RoofFallback::InvalidPitch => DiagnosticCode::RoofFlatPitch,
        RoofFallback::OverhangCollapsed => DiagnosticCode::RoofOverhangCollapsed,
    }
}

/// 📐️ The validity findings of one storey from the values its parents inferred.
pub fn storey(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    degenerate(snapshot, storey, inputs, &mut found);
    openings(snapshot, storey, inputs, &mut found);
    stairs(snapshot, storey, inputs, &mut found);
    spaces(snapshot, storey, inputs, &mut found);
    roofs(snapshot, storey, inputs, &mut found);
    found
}

