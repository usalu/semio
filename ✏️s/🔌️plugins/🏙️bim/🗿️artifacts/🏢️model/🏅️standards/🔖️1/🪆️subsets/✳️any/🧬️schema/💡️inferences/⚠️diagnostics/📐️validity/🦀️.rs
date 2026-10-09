//! 📐️ Validity of the elements of one storey: degenerate geometry, openings that do not fit their host (from `opening-frames`) and stairs that
//! break the code (from `stair-runs`).

use super::{Diagnostic, DiagnosticCode, Inputs};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::MAX_TILT;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged as corners, extents_of, seg};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::roofs::RoofFallback;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::stairs::stringer_ignored;
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::OpeningIssue;
use crate::standards::v1::subsets::any::schema::inferences::spaces::SpaceStatus;
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{BLONDEL_MAX, BLONDEL_MIN};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{target_of, vertical_of};
use crate::{Axis, CurtainGrid, CurtainPanel, ModelSnapshot, Point2, SpaceBoundary, Vertex};
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
        let kind = snapshot.curtain_wall_types.get(&curtain.curtain_wall_type);
        let rules = [curtain.u_grid.as_ref().or(kind.map(|kind| &kind.u_grid)), curtain.v_grid.as_ref().or(kind.map(|kind| &kind.v_grid))];
        if rules.iter().flatten().any(|rule| matches!(rule, CurtainGrid::Spacing { spacing } if !(*spacing > LENGTH_EPS))) {
            found.push(Diagnostic::new(DiagnosticCode::DegenerateSpacing, &[id]).on(storey));
        }
        if kind.is_some_and(|kind| [&kind.interior_mullion, &kind.border_mullion].into_iter().any(|profile| { let (width, depth) = extents_of(profile); !(width > LENGTH_EPS && depth > LENGTH_EPS) })) {
            found.push(Diagnostic::new(DiagnosticCode::DegenerateProfile, &[id]).on(storey));
        }
    }
    for (id, column) in snapshot.columns.iter().filter(|(_, row)| row.storey == storey) {
        if !finite_point(&column.position) || !column.rotation.is_finite() || !column.tilt.is_none_or(|tilt| tilt.direction.is_finite() && tilt.angle.is_finite()) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            continue;
        }
        if let Some(tilt) = column.tilt.filter(|tilt| !(tilt.angle > 0.0 && tilt.angle <= MAX_TILT)) {
            found.push(Diagnostic::new(DiagnosticCode::ColumnTiltInvalid, &[id]).on(storey).with("angle", tilt.angle));
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
        if !(beam.top_offset.is_finite() && beam.end_top_offset.is_none_or(f64::is_finite)) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            continue;
        }
        axis(found, storey, id, &beam.axis);
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
    for (id, ceiling) in snapshot.ceilings.iter().filter(|(_, row)| row.storey == storey) {
        if !ceiling.offset.is_finite() || !ceiling.slope.is_none_or(|slope| slope.direction.is_finite() && slope.angle.is_finite()) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            continue;
        }
        outline(found, storey, id, &ceiling.boundary);
        ceiling.holes.iter().for_each(|hole| outline(found, storey, id, hole));
        if finite_loop(&ceiling.boundary) && snapshot.ceiling_types.contains_key(&ceiling.ceiling_type) {
            let (bottom, top) = ceilings::span(snapshot, ceiling, own);
            if top > own.top_elevation + LENGTH_EPS || bottom < own.elevation - LENGTH_EPS {
                found.push(Diagnostic::new(DiagnosticCode::CeilingOutsideStorey, &[id]).on(storey).with("top", top - own.elevation).with("bottom", bottom - own.elevation).with("height", own.top_elevation - own.elevation));
            }
        }
    }
    for (id, roof) in snapshot.roofs.iter().filter(|(_, row)| row.storey == storey) {
        outline(found, storey, id, &roof.footprint);
    }
    for (id, railing) in snapshot.railings.iter().filter(|(_, row)| row.storey == storey) {
        if let Some(host) = railing.host.as_ref() {
            if !host.inset.is_finite() {
                found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            } else if rail_hosts::unresolved(railing, snapshot.slabs.get(&host.element)) {
                found.push(Diagnostic::new(DiagnosticCode::RailingHostUnresolved, &[id, &host.element]).on(storey));
            }
        } else if !railing.path.iter().all(finite_point) {
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
        if stringer_ignored(stair) {
            found.push(Diagnostic::new(DiagnosticCode::StairStringerIgnored, &[id]).on(storey));
        }
    }
}

fn ramps(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>, found: &mut Vec<Diagnostic>) {
    for (id, ramp) in snapshot.ramps.iter().filter(|(_, row)| row.storey == storey) {
        if !(ramp.path.iter().all(|vertex| finite_point(&vertex.point) && vertex.bulge.is_finite()) && ramp.width.is_finite() && ramp.thickness.is_finite() && ramp.max_slope.is_finite() && ramp.base_offset.is_finite()) {
            found.push(Diagnostic::new(DiagnosticCode::NonFinite, &[id]).on(storey));
            continue;
        }
        let Some(run) = inputs.ramp_runs.get(id.as_str()) else { continue };
        if run.length <= LENGTH_EPS {
            found.push(Diagnostic::new(DiagnosticCode::DegenerateAxis, &[id]).on(storey).with("length", run.length));
        } else if !run.compliance.run_ok {
            found.push(Diagnostic::new(DiagnosticCode::RampNoRun, &[id]).on(storey).with("rise", run.rise));
        } else if !run.compliance.slope_ok {
            found.push(Diagnostic::new(DiagnosticCode::RampSlope, &[id]).on(storey).with("slope_percent", run.slope * 100.0).with("limit_percent", ramp.max_slope * 100.0).with("rise", run.rise.abs()).with("run", run.run_length));
        }
    }
}

fn curtains(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>, found: &mut Vec<Diagnostic>) {
    for (id, curtain) in snapshot.curtain_walls.iter().filter(|(_, row)| row.storey == storey) {
        let Some(layout) = inputs.curtains.get(id.as_str()) else { continue };
        for stray in &layout.stray {
            let row = &snapshot.curtain_panel_overrides[stray];
            found.push(Diagnostic::new(DiagnosticCode::CurtainOverrideOutOfGrid, &[stray, id]).on(storey).with("u", f64::from(row.u)).with("v", f64::from(row.v)).with("u_panels", f64::from(layout.u_panels)).with("v_panels", f64::from(layout.v_panels)));
        }
        if !layout.repeated.is_empty() {
            let mut ids: Vec<&str> = layout.repeated.iter().map(String::as_str).collect();
            ids.sort_unstable();
            found.push(Diagnostic::new(DiagnosticCode::CurtainDuplicateOverride, &ids).on(storey));
        }
        let ignored = layout.ignored_u.len() + layout.ignored_v.len();
        if ignored > 0 {
            found.push(Diagnostic::new(DiagnosticCode::CurtainGridLineOutside, &[id]).on(storey).with("ignored", ignored as f64));
        }
        for cell in layout.overrides.iter().filter(|cell| cell.v > 0 && matches!(cell.panel, CurtainPanel::Door { .. })) {
            found.push(Diagnostic::new(DiagnosticCode::CurtainDoorNotAtBase, &[&cell.id, id]).on(storey).with("v", f64::from(cell.v)));
        }
        if layout.v_panels > 1 && matches!(layout.panel, Some(CurtainPanel::Door { .. })) && snapshot.curtain_wall_types.contains_key(&curtain.curtain_wall_type) {
            found.push(Diagnostic::new(DiagnosticCode::CurtainDoorNotAtBase, &[id]).on(storey).with("v", 1.0));
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

/// 🏠️ The diagnostic code of a roof fallback: a roof that is flat although its shape asks for more, whose overhang collapsed, or whose gable ends turned into hips.
pub fn fallback_code(fallback: RoofFallback) -> DiagnosticCode {
    match fallback {
        RoofFallback::CurvedFootprint => DiagnosticCode::RoofFlatCurved,
        RoofFallback::SkeletonFailed => DiagnosticCode::RoofFlatSkeleton,
        RoofFallback::DegenerateFootprint => DiagnosticCode::RoofFlatDegenerate,
        RoofFallback::InvalidPitch => DiagnosticCode::RoofFlatPitch,
        RoofFallback::OverhangCollapsed => DiagnosticCode::RoofOverhangCollapsed,
        RoofFallback::GableEndsAdjustToHip => DiagnosticCode::RoofGableToHip,
    }
}

/// 📐️ The validity findings of one storey from the values its parents inferred.
pub fn storey(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    degenerate(snapshot, storey, inputs, &mut found);
    openings(snapshot, storey, inputs, &mut found);
    stairs(snapshot, storey, inputs, &mut found);
    ramps(snapshot, storey, inputs, &mut found);
    curtains(snapshot, storey, inputs, &mut found);
    spaces(snapshot, storey, inputs, &mut found);
    roofs(snapshot, storey, inputs, &mut found);
    found
}

