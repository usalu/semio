//! ↩️ Inverse of `CreateTextNote`: the concrete `DeleteTextNote` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_97::standards::v1::subsets::any::schema::mutations::delete_text_note::DeleteTextNote;
use crate::standards::v1::subsets::any::schema::mutations::create_text_note::CreateTextNote;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateTextNote, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_97::standards::v1::subsets::any::schema::mutations::delete_text_note::DeleteTextNote>,
{
    if base.text_notes.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteTextNote { id: payload.id.clone() })]
}
