//! ↩️ Inverse of `DeleteCurtainPanelOverride`: the concrete create of the removed override, in storage order (see the shared cascade).

use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteCurtainPanelOverride, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::inverse(base, std::slice::from_ref(&payload.id))
}
