//! 🚪️ Opening filler solids. A window is a frame ring (sill, head, two jambs), `panes - 1` muntins and `panes` glass panes; a door is a frame (two jambs and a head, no
//! threshold) and one or two leaves; a void has no filler. Everything is built in the local frame of `opening-frames` (`x` across the opening, `y` through the host, `z`
//! up from the sill) and centred on the thickness of the host, then placed with that frame. Invalid placements and missing types produce no solid; a filler is built for exactly
//! the frames the host cuts (`OpeningFrame::valid`).

use super::super::super::opening_frames::OpeningFrame;
use super::super::{dep_object, dep_value, parts, ElementSolid, SolidBuilder, SolidFamily};
use crate::{DoorLeaves, DoorType, ModelSnapshot, Opening, OpeningKind, WindowType};
use semio_framework_geometry::mesh::{extrude, TriMesh};
use semio_framework_geometry::placement::{Affine3, ZPlane};
use semio_framework_geometry::Point;
use semio_framework_value::DslValue;

/// 🪟️ Thickness of a window pane in metres.
pub const GLASS_THICKNESS: f64 = 0.02;

/// 🚪️ Thickness of a door leaf in metres.
pub const LEAF_THICKNESS: f64 = 0.04;

const EPS: f64 = 1e-9;

//#region 🔖️Parts
/// 📦️ An axis-aligned box in the local frame: `x`, `y`, `z` ranges.
fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> TriMesh {
    extrude(&[Point::new(x.0, y.0), Point::new(x.1, y.0), Point::new(x.1, y.1), Point::new(x.0, y.1)], &[], ZPlane::flat(z.0), ZPlane::flat(z.1))
}

fn empty(range: (f64, f64)) -> bool {
    range.1 - range.0 <= EPS
}

/// 🪟️ The parts of a window of overall size `width × height`, frame depth centred at lateral `centre`: `(part, is glazing, mesh)`.
pub fn window_parts(window: &WindowType, width: f64, height: f64, centre: f64) -> Vec<(&'static str, bool, TriMesh)> {
    let frame = window.frame_width.clamp(0.0, (width / 2.0).min(height / 2.0));
    let (half, depth) = (width / 2.0, window.frame_depth.max(0.0));
    let lateral = (centre - depth / 2.0, centre + depth / 2.0);
    let mut out = Vec::new();
    let mut add = |part: &'static str, glazing: bool, x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
        if !empty(x) && !empty(y) && !empty(z) {
            out.push((part, glazing, boxed(x, y, z)));
        }
    };
    add(parts::FRAME, false, (-half, half), lateral, (0.0, frame));
    add(parts::FRAME, false, (-half, half), lateral, (height - frame, height));
    add(parts::FRAME, false, (-half, -half + frame), lateral, (frame, height - frame));
    add(parts::FRAME, false, (half - frame, half), lateral, (frame, height - frame));
    let (inner, rise) = (width - 2.0 * frame, (frame, height - frame));
    if window.panes > 0 && inner > EPS && !empty(rise) {
        let panes = window.panes as f64;
        let muntin = (frame / 2.0).min(inner / panes / 2.0);
        let pane = (inner - (panes - 1.0) * muntin) / panes;
        let glass = GLASS_THICKNESS.min(depth);
        for k in 0..window.panes {
            let start = -half + frame + k as f64 * (pane + muntin);
            add(parts::GLASS, true, (start, start + pane), (centre - glass / 2.0, centre + glass / 2.0), rise);
            if k + 1 < window.panes {
                add(parts::MUNTIN, false, (start + pane, start + pane + muntin), lateral, rise);
            }
        }
    }
    out
}

/// 🚪️ The parts of a door of overall size `width × height`, frame depth centred at lateral `centre`: `(part, mesh)`.
pub fn door_parts(door: &DoorType, width: f64, height: f64, centre: f64) -> Vec<(&'static str, TriMesh)> {
    let frame = door.frame_width.clamp(0.0, (width / 2.0).min(height / 2.0));
    let (half, depth) = (width / 2.0, door.frame_depth.max(0.0));
    let lateral = (centre - depth / 2.0, centre + depth / 2.0);
    let mut out = Vec::new();
    let mut add = |part: &'static str, x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
        if !empty(x) && !empty(y) && !empty(z) {
            out.push((part, boxed(x, y, z)));
        }
    };
    add(parts::FRAME, (-half, half), lateral, (height - frame, height));
    add(parts::FRAME, (-half, -half + frame), lateral, (0.0, height - frame));
    add(parts::FRAME, (half - frame, half), lateral, (0.0, height - frame));
    let leaf = LEAF_THICKNESS.min(depth);
    let (inner, rise) = (width - 2.0 * frame, (0.0, height - frame));
    let count = if door.leaves == DoorLeaves::Double { 2 } else { 1 };
    for k in 0..count {
        let start = -half + frame + k as f64 * inner / count as f64;
        add(parts::LEAF, (start, start + inner / count as f64), (centre - leaf / 2.0, centre + leaf / 2.0), rise);
    }
    out
}
//#endregion 🔖️Parts

//#region 🔖️Solid
/// 🚪️ The family of the filler of an opening: windows and doors have one, voids none.
pub fn family_of(kind: &OpeningKind) -> Option<SolidFamily> {
    match kind {
        OpeningKind::Window { .. } => Some(SolidFamily::Window),
        OpeningKind::Door { .. } => Some(SolidFamily::Door),
        OpeningKind::Void { .. } => None,
    }
}

/// 🚪️ The filler of an opening in the frame the host resolved for it: the parts of its window or door type placed in the local frame, centred on the host thickness (the lateral centre follows the faces of the frame, so a flipped opening centres correctly) or, with an authored reveal, set back by the reveal depth from the front face. Empty for an invalid frame or a missing type; `None` for a void, which has no filler.
pub fn filler_solid(snapshot: &ModelSnapshot, opening: &Opening, frame: &OpeningFrame) -> Option<ElementSolid> {
    let place: Affine3 = frame.local.affine();
    let placed = |depth: f64| frame.setback.map_or((frame.face_front - frame.face_back) / 2.0, |setback| frame.face_front - setback - depth.max(0.0) / 2.0);
    match &opening.kind {
        OpeningKind::Window { window_type } => {
            let mut builder = SolidBuilder::new(SolidFamily::Window);
            if let (true, Some(window)) = (frame.valid, snapshot.window_types.get(window_type)) {
                for (part, glazing, mesh) in window_parts(window, frame.width, frame.height, placed(window.frame_depth)) {
                    builder.add(part, if glazing { "" } else { window.material.as_str() }, 0, &mesh.transformed(&place));
                }
            }
            Some(builder.build())
        }
        OpeningKind::Door { door_type } => {
            let mut builder = SolidBuilder::new(SolidFamily::Door);
            if let (true, Some(door)) = (frame.valid, snapshot.door_types.get(door_type)) {
                for (part, mesh) in door_parts(door, frame.width, frame.height, placed(door.frame_depth)) {
                    builder.add(part, &door.material, 0, &mesh.transformed(&place));
                }
            }
            Some(builder.build())
        }
        OpeningKind::Void { .. } => None,
    }
}

/// 🔑️ What `filler_solid` reads of an opening besides its frame: the window or door type it names.
pub fn dependency(snapshot: &ModelSnapshot, opening: &Opening) -> DslValue {
    let window = if let OpeningKind::Window { window_type } = &opening.kind { snapshot.window_types.get(window_type).cloned() } else { None };
    let door = if let OpeningKind::Door { door_type } = &opening.kind { snapshot.door_types.get(door_type).cloned() } else { None };
    dep_object([("window_type", dep_value(&window)), ("door_type", dep_value(&door))])
}
//#endregion 🔖️Solid

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
