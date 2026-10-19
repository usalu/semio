//! ↩️ Inverse of `SetProjectInfo`: one `SetProjectInfo` restoring the base value of exactly the fields the forward really changes, none when nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_project_info::SetProjectInfo;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;

pub fn inverse<Op>(payload: &SetProjectInfo, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_project_info::SetProjectInfo>,
{
    let restore = payload.patch().minimal(&base.project).restoring(&base.project);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetProjectInfo::from_patch(restore))]
}
