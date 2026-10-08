//! 🔺️ Sparse diff builder for `ClearWidgetLayout` — a real id-keyed removal from the fixture's
//! layout collection helper (never a whole-snapshot capture).

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dLayoutDelta};
use crate::Generation2dSnapshot;

pub fn diff(payload: &super::ClearWidgetLayout, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if !base.host_snapshot.layout.contains_key(&payload.id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Widget \"{}\" already has no layout entry.", payload.id));
    }
    protocol::MutationOutcome::new(Generation2dDiff { layout: Some(Generation2dLayoutDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
