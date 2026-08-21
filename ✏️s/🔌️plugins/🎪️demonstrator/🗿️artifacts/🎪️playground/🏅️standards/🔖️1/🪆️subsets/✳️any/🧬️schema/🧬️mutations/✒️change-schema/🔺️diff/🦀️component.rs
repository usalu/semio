//! 🔺️ `change-schema` — sparse diff construction. Root-scoped `change-<artifact>-<field>` on the
//! document's single metadata string — no target to be missing; Warning `no-op` when the value is
//! unchanged.

use super::mutation::ChangeSchema;
use crate::artifacts::playground::standards::v1::subsets::any::schema::{diff::PlaygroundDiff, snapshot::PlaygroundSnapshot};

//#region 🔖️Diff
/// 🔺️ The `schema` slot is the only sparse field this payload ever touches.
pub async fn diff(payload: &ChangeSchema, base: &PlaygroundSnapshot) -> protocol::MutationOutcome<PlaygroundDiff> {
    if base.schema == payload.new_schema {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Playground schema is already \"{}\".", payload.new_schema));
    }
    protocol::MutationOutcome::new(PlaygroundDiff { schema: Some(payload.new_schema.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
