//! ↩️ Inverse of `SetTextNote`: an absolute `SetTextNote` restoring the base value of exactly the fields the forward really changes, none when the text note is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_text_note::SetTextNote;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetTextNote, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_text_note::SetTextNote>,
{
    let Some(record) = base.text_notes.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetTextNote::from_patch(payload.id.clone(), restore))]
}
