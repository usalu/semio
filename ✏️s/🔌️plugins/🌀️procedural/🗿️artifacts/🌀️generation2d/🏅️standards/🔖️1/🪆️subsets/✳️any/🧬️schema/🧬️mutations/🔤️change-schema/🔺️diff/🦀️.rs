//! 🔺️ Sparse diff builder for `ChangeSchema` — a real scalar-field write on the fixture (never a
//! whole-snapshot capture).

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff};
use crate::Generation2dSnapshot;

pub fn diff(payload: &super::ChangeSchema, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if base.host_snapshot.schema == payload.schema {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fixture schema is already \"{}\".", payload.schema));
    }
    protocol::MutationOutcome::new(Generation2dDiff { schema: Some(payload.schema.clone()), ..Default::default() })
}
