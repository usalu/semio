//! ↩️ Inverse of `CreateTextNote`: the concrete `DeleteTextNote` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_text_note::DeleteTextNote;
use crate::standards::v1::subsets::any::schema::mutations::create_text_note::CreateTextNote;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateTextNote, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_text_note::DeleteTextNote>,
{
    if base.text_notes.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteTextNote { id: payload.id.clone() })]
}
