//! 🔺️ `create-object` — sparse diff construction (delegates to the existing objects-add field-delta
//! constructor); Fatal `duplicate-id` guards against overwriting an existing object.

use super::CreateObject;
use semio_s_artifact_lowpoly_lowpoly_model_diff::diff_objects_add;
use semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateObject, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if base.objects.iter().any(|object| object.id == payload.object.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An object with id \"{}\" already exists.", payload.object.id), [payload.object.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_objects_add(payload.index, payload.object.clone(), base))
}
//#endregion 🔖️Diff
