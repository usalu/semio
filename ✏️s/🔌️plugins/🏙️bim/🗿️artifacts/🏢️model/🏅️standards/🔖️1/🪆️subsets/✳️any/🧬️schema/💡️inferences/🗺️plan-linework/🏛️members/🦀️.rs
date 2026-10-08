//! 🏛️ Columns, beams, slabs, roofs and railings in the plan: poché where the plane cuts, projection below it, dashed hidden above it.

use super::{classify, outline_style, Context, Cutting, PlanKind, PlanStyle, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::bodies::{beam_outline, beam_span, column_outline, slab_span};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged as corners, point};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{target_of, vertical_of};
use semio_framework_geometry::loops::{self, Vertex as Corner};
use semio_framework_geometry::Point;

/// 🏛️ A column is poché when cut, an outline otherwise.
fn columns(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, column) in cx.snapshot.columns.iter().filter(|(_, column)| column.storey == cx.storey) {
        let Some(outline) = column_outline(cx.snapshot, column) else { continue };
        let (base, top) = vertical_of(column.base_offset, &column.top, &cx.own, target_of(&column.top, &cx.levels));
        match classify(base, top, cx.cut) {
            Cutting::Cut => sheet.loop_region(id, PlanKind::ColumnCut, PlanStyle::Cut, &outline, &[]),
            other => sheet.loop_polyline(id, PlanKind::ColumnOutline, outline_style(other), &outline),
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

/// 🛤️ A railing is its path: projection when its top is below the cut.
fn railings(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, railing) in cx.snapshot.railings.iter().filter(|(_, railing)| railing.storey == cx.storey) {
        let base = cx.own.elevation + railing.base_offset;
        let path: Vec<Point> = railing.path.iter().map(point).collect();
        sheet.path(id, PlanKind::RailingPath, outline_style(classify(base, base + railing.height, cx.cut)), &path);
    }
}

/// 🏛️ Draws the structural members, slabs, roofs and railings of the storey.
pub fn draw(sheet: &mut Sheet, cx: &Context<'_>) {
    slabs(sheet, cx);
    roofs(sheet, cx);
    columns(sheet, cx);
    beams(sheet, cx);
    railings(sheet, cx);
}
