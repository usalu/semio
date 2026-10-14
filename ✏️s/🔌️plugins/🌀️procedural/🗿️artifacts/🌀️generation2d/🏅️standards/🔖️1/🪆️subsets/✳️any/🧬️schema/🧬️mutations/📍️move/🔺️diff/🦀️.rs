//! 🔺️ Sparse diff builder for `MoveWidget` — a real id-keyed upsert into the fixture's layout
//! collection helper (never a whole-snapshot capture).

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::diff::Generation2dLayoutDelta;
use semio_s_artifact_procedural_2d_model::diff::Generation2dLayoutRow;
use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn diff(payload: &super::MoveWidget, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if !base.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Widget \"{}\" position must be finite.", payload.id), [payload.id.clone()]);
    }
    let row = Generation2dLayoutRow { id: payload.id.clone(), layout: payload.layout.clone() };
    let delta = if base.host_snapshot.layout.contains_key(&payload.id) { Generation2dLayoutDelta { patched: vec![row], ..Default::default() } } else { Generation2dLayoutDelta { added: vec![row], ..Default::default() } };
    protocol::MutationOutcome::new(Generation2dDiff { layout: Some(delta), ..Default::default() })
}
