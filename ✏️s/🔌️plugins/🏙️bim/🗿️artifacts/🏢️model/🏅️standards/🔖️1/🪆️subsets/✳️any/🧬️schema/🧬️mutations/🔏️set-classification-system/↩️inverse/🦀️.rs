//! ↩️ Inverse of `SetClassificationSystem`: an absolute `SetClassificationSystem` restoring the base value of exactly the fields the forward really changes (a removed source assigned null), none when the system is absent or
//! nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_classification_system::SetClassificationSystem;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetClassificationSystem, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_classification_system::SetClassificationSystem>,
{
    let Some(record) = base.classification_systems.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetClassificationSystem::from_patch(payload.id.clone(), restore))]
}
