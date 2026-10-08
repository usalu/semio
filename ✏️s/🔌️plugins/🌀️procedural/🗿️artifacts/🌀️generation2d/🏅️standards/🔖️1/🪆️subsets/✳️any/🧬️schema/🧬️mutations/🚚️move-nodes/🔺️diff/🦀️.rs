//! 🔺️ Sparse diff builder for `MoveNodes` — every addressed widget's layout entry moves by the payload offset from its BASE
//! position; a widget without a stored position or without a widget is skipped (`mutation.partial`).

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dLayoutDelta, Generation2dLayoutRow};
use crate::standards::v1::subsets::any::schema::mutations::{generation2d_partial,generation2d_targets_invariant,widget_index};

use crate::Generation2dSnapshot;
use semio_framework_artifact_flow_flow::WidgetLayout;

/// 🏗️ None left to move is `target-missing` (no such widget) or `target-mismatch` (no stored position); a zero offset is
/// `mutation.no-op`.
pub fn diff(payload: &super::MoveNodes, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if let Err(reason) = generation2d_targets_invariant(&payload.ids) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, payload.ids.clone());
    }
    if !payload.dx.is_finite() || !payload.dy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node offset must be finite", payload.ids.clone());
    }
    let (mut missing, mut unplaced, mut moved) = (Vec::new(), Vec::new(), Vec::new());
    for id in &payload.ids {
        match (widget_index(&base.host_snapshot, id), base.host_snapshot.layout.get(id)) {
            (None, _) => missing.push(id.clone()),
            (Some(_), None) => unplaced.push(id.clone()),
            (Some(_), Some(layout)) => moved.push((id.clone(), WidgetLayout { x: layout.x + payload.dx, y: layout.y + payload.dy })),
        }
    }
    let messages: Vec<protocol::MutationMessage> = [generation2d_partial(missing.clone(), payload.ids.len(), "no such widget"), generation2d_partial(unplaced.clone(), payload.ids.len(), "no stored position")].into_iter().flatten().collect();
    if moved.is_empty() {
        return if unplaced.is_empty() {
            protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} widget(s) exists", payload.ids.len()), missing)
        } else {
            protocol::MutationOutcome::error("mutation.target-mismatch", format!("none of the {} widget(s) has a stored position", payload.ids.len()), unplaced)
        };
    }
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::empty().absorb_messages(messages.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "the drag offset is zero").at(payload.ids.clone())]));
    }
    if moved.iter().any(|(_, layout)| !layout.x.is_finite() || !layout.y.is_finite()) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "the moved position leaves the finite canvas", payload.ids.clone());
    }
    let patched = moved.into_iter().map(|(id, layout)| Generation2dLayoutRow { id, layout }).collect();
    protocol::MutationOutcome::new(Generation2dDiff { layout: Some(Generation2dLayoutDelta { patched, ..Default::default() }), ..Default::default() }).absorb_messages(messages)
}
