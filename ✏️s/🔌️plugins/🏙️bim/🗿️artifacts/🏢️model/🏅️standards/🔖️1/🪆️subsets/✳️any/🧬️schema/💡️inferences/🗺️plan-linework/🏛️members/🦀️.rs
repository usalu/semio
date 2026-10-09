//! 🏛️ Columns, beams, slabs, roofs and railings in the plan: poché where the plane cuts, projection below it, dashed hidden above it.

use super::{classify, outline_style, Context, Cutting, PlanKind, PlanStyle, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::bodies::{beam_outline, beam_span, column_outline, column_section, slab_span};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::{host_paths, Host};
use crate::RailingHost;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged as corners, point};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{target_of, vertical_of};
use semio_framework_geometry::loops::{self, Vertex as Corner};
use semio_framework_geometry::Point;

/// 🏛️ A column is poché when cut (its horizontal section at the cut height, which a leaning column moves), an outline otherwise (the extent of the whole column).
fn columns(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, column) in cx.snapshot.columns.iter().filter(|(_, column)| column.storey == cx.storey) {
        let (base, top) = vertical_of(column.base_offset, &column.top, &cx.own, target_of(&column.top, &cx.levels));
        match classify(base, top, cx.cut) {
            Cutting::Cut => {
                if let Some(section) = column_section(cx.snapshot, column, base, top, cx.cut) {
                    sheet.loop_region(id, PlanKind::ColumnCut, PlanStyle::Cut, &section, &[]);
                }
            }
            other => {
                if let Some(outline) = column_outline(cx.snapshot, column, base, top) {
                    sheet.loop_polyline(id, PlanKind::ColumnOutline, outline_style(other), &outline);
                }
            }
        }
    }
}

/// ➖️ A beam is an outline: dashed above the cut, as it hangs below the storey top.
fn beams(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, beam) in cx.snapshot.beams.iter().filter(|(_, beam)| beam.storey == cx.storey) {
        let (Some(outline), Some((low, high))) = (beam_outline(cx.snapshot, beam), beam_span(cx.snapshot, beam, &cx.own)) else { continue };
        sheet.loop_polyline(id, PlanKind::BeamOutline, outline_style(classify(low, high, cx.cut)), &outline);
    }
}

/// ⬜️ A slab shows its edge and its holes: projection below the cut, a region where the plane cuts it.
fn slabs(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, slab) in cx.snapshot.slabs.iter().filter(|(_, slab)| slab.storey == cx.storey) {
        if slab.boundary.len() < 3 {
            continue;
        }
        let (low, high) = slab_span(cx.snapshot, slab, &cx.own);
        let (edge, holes): (Vec<Corner>, Vec<Vec<Corner>>) = (corners(&slab.boundary), slab.holes.iter().map(|hole| corners(hole)).collect());
        match classify(low, high, cx.cut) {
            Cutting::Cut => sheet.loop_region(id, PlanKind::SlabEdge, PlanStyle::Cut, &edge, &holes),
            other => {
                let style = outline_style(other);
                sheet.loop_polyline(id, PlanKind::SlabEdge, style, &edge);
                holes.iter().for_each(|hole| sheet.loop_polyline(id, PlanKind::SlabHole, style, hole));
            }
        }
    }
}

/// 🔲️ A ceiling shows its edge and its holes like a slab: hidden (dashed) above the cut plane where it hangs, a region where the plane cuts it, projection when it hangs below it.
fn ceilings(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, ceiling) in cx.snapshot.ceilings.iter().filter(|(_, ceiling)| ceiling.storey == cx.storey) {
        if ceiling.boundary.len() < 3 {
            continue;
        }
        let (low, high) = ceilings::span(cx.snapshot, ceiling, &cx.own);
        let (edge, holes): (Vec<Corner>, Vec<Vec<Corner>>) = (corners(&ceiling.boundary), ceiling.holes.iter().map(|hole| corners(hole)).collect());
        match classify(low, high, cx.cut) {
            Cutting::Cut => sheet.loop_region(id, PlanKind::CeilingEdge, PlanStyle::Cut, &edge, &holes),
            other => {
                let style = outline_style(other);
                sheet.loop_polyline(id, PlanKind::CeilingEdge, style, &edge);
                holes.iter().for_each(|hole| sheet.loop_polyline(id, PlanKind::CeilingHole, style, hole));
            }
        }
    }
}

/// 🏠️ A roof is drawn with its overhang; it sits above the cut unless its base is below it.
fn roofs(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, roof) in cx.snapshot.roofs.iter().filter(|(_, roof)| roof.storey == cx.storey) {
        if roof.footprint.len() < 3 {
            continue;
        }
        let footprint = corners(&roof.footprint);
        let outline = if roof.overhang > 0.0 { loops::offset(&footprint, roof.overhang, 4.0).unwrap_or(footprint) } else { footprint };
        let base = cx.own.elevation + roof.base_offset;
        let cutting = if base >= cx.cut { Cutting::Above } else { Cutting::Below };
        sheet.loop_polyline(id, PlanKind::RoofOutline, outline_style(cutting), &outline);
    }
}

fn host_in<'a>(cx: &'a Context<'a>, spec: &RailingHost) -> Option<Host<'a>> {
    if let Some(run) = cx.inputs.runs.get(spec.element.as_str()) {
        return Some(Host::Stair(run));
    }
    if let (Some(ramp), Some(run)) = (cx.snapshot.ramps.get(&spec.element), cx.inputs.ramp_runs.get(spec.element.as_str())) {
        return Some(Host::Ramp { ramp, run });
    }
    let slab = cx.snapshot.slabs.get(&spec.element)?;
    Some(Host::Slab { slab, own: cx.levels.get(&slab.storey)? })
}

/// 🛤️ A railing is its path (a hosted railing follows its host): projection when its top is below the cut.
fn railings(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, railing) in cx.snapshot.railings.iter().filter(|(_, railing)| railing.storey == cx.storey) {
        let Some(spec) = railing.host.as_ref() else {
            let base = cx.own.elevation + railing.base_offset;
            let path: Vec<Point> = railing.path.iter().map(point).collect();
            sheet.path(id, PlanKind::RailingPath, outline_style(classify(base, base + railing.height, cx.cut)), &path);
            continue;
        };
        let Some(paths) = host_in(cx, spec).and_then(|host| host_paths(railing, spec, &host).ok()) else { continue };
        let (low, high) = paths.iter().flatten().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), p| (low.min(p.z), high.max(p.z)));
        for path in &paths {
            let points: Vec<Point> = path.iter().map(|p| Point::new(p.x, p.y)).collect();
            sheet.path(id, PlanKind::RailingPath, outline_style(classify(low, high + railing.height, cx.cut)), &points);
        }
    }
}

/// 🏛️ Draws the structural members, slabs, roofs and railings of the storey.
pub fn draw(sheet: &mut Sheet, cx: &Context<'_>) {
    slabs(sheet, cx);
    ceilings(sheet, cx);
    roofs(sheet, cx);
    columns(sheet, cx);
    beams(sheet, cx);
    railings(sheet, cx);
}
