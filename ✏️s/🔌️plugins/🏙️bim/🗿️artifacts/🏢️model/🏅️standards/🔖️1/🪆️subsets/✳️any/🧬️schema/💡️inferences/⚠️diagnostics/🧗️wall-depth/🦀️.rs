//! 🧗️ Findings of the wall depth package, per storey: an attach that loops, covers only part of the wall axis or lifts the top to the base, a sweep that reaches above the lowest point of the top of its wall or is interrupted over its whole
//! length, and a reveal that sets the frame of an opening back by more than the wall leaves room for. A missing attach target or host is a dangling reference, reported by `references`.

use super::{Diagnostic, DiagnosticCode, Inputs};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{dep_object, dep_records, dep_value, walls, wall_sweeps};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::attach::AttachState;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::WallLayout;
use crate::{ModelSnapshot, OpeningKind};
use semio_framework_value::DslValue;

const TOLERANCE: f64 = 1e-9;

fn frame_depth(snapshot: &ModelSnapshot, kind: &OpeningKind) -> Option<f64> {
    match kind {
        OpeningKind::Window { window_type } => snapshot.window_types.get(window_type).map(|row| row.frame_depth),
        OpeningKind::Door { door_type } => snapshot.door_types.get(door_type).map(|row| row.frame_depth),
        OpeningKind::Void { .. } => None,
    }
}

fn attaches(found: &mut Vec<Diagnostic>, storey: &str, wall: &str, states: [&Option<AttachState>; 2]) {
    for state in states.into_iter().flatten() {
        if state.cycle {
            found.push(Diagnostic::new(DiagnosticCode::WallAttachCycle, &[wall, &state.target]).on(storey));
        }
        if state.found && state.covered < 1.0 - TOLERANCE {
            found.push(Diagnostic::new(DiagnosticCode::WallAttachUnreached, &[wall, &state.target]).on(storey).with("covered", (state.covered * 100.0).round()));
        }
        if state.clamped {
            found.push(Diagnostic::new(DiagnosticCode::WallAttachCollapsed, &[wall, &state.target]).on(storey));
        }
    }
}

/// 📏️ The lowest height of the wall over its axis: the smallest difference of the top and the base edge at the ends and the breakpoints.
fn lowest(layout: &WallLayout) -> f64 {
    let marks = [0.0, layout.length].into_iter().chain(layout.top_profile.iter().map(|point| point.s)).chain(layout.base_profile.iter().map(|point| point.s)).filter(|s| *s >= -TOLERANCE && *s <= layout.length + TOLERANCE);
    marks.map(|s| layout.top_at(s) - layout.base_at(s)).fold(f64::INFINITY, f64::min)
}

/// ⚠️ The wall depth findings of one storey from the layouts and frames its `Diagnostics` node has as parents.
pub fn storey(snapshot: &ModelSnapshot, storey: &str, inputs: &Inputs<'_>) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    for (id, wall) in snapshot.walls.iter().filter(|(_, row)| row.storey == storey) {
        if let Some(layout) = inputs.layouts.get(id.as_str()) {
            attaches(&mut found, storey, id, [&layout.top_attach, &layout.base_attach]);
        }
    }
    for (id, sweep) in snapshot.wall_sweeps.iter().filter(|(_, row)| snapshot.walls.get(&row.host).is_some_and(|wall| wall.storey == storey)) {
        let (Some(wall), Some(layout)) = (snapshot.walls.get(&sweep.host), inputs.layouts.get(sweep.host.as_str())) else { continue };
        let rise = wall_sweeps::extents_of(sweep).1;
        let ceiling = lowest(layout);
        if ceiling.is_finite() && sweep.height + rise > ceiling + TOLERANCE {
            found.push(Diagnostic::new(DiagnosticCode::WallSweepAboveWall, &[id, &sweep.host]).on(storey).with("height", sweep.height + rise).with("wall_height", ceiling));
        }
        let hosted = snapshot.openings.iter().filter(|(_, opening)| opening.host == sweep.host).filter_map(|(opening, _)| inputs.frames.get(opening.as_str()).copied());
        if layout.length > TOLERANCE && wall_sweeps::runs(sweep, wall, layout, &walls::cuts_of(hosted)).is_empty() {
            found.push(Diagnostic::new(DiagnosticCode::WallSweepNoRun, &[id, &sweep.host]).on(storey));
        }
    }
    for (id, opening) in snapshot.openings.iter().filter(|(_, row)| row.reveal_depth.is_some() && snapshot.walls.get(&row.host).is_some_and(|wall| wall.storey == storey)) {
        let (Some(layout), Some(depth), Some(reveal)) = (inputs.layouts.get(opening.host.as_str()), frame_depth(snapshot, &opening.kind), opening.reveal_depth) else { continue };
        if reveal + depth > layout.thickness + TOLERANCE {
            found.push(Diagnostic::new(DiagnosticCode::OpeningRevealDepth, &[id]).on(storey).with("reveal", reveal).with("frame_depth", depth).with("thickness", layout.thickness));
        }
    }
    found
}

/// 🔑️ What `storey` reads of the snapshot besides the layouts and frames of its parents: the sweeps of its walls and the frame depth of the types of the openings that set their frame back.
pub fn dependency(snapshot: &ModelSnapshot, storey: &str) -> DslValue {
    let walls_of = |host: &str| snapshot.walls.get(host).is_some_and(|wall| wall.storey == storey);
    let depths = snapshot.openings.iter().filter(|(_, row)| row.reveal_depth.is_some() && walls_of(&row.host)).map(|(id, row)| (id.clone(), dep_value(&frame_depth(snapshot, &row.kind))));
    dep_object([("sweeps", dep_records(snapshot.wall_sweeps.iter().filter(|(_, row)| walls_of(&row.host)))), ("frame_depths", DslValue::object(depths))])
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
