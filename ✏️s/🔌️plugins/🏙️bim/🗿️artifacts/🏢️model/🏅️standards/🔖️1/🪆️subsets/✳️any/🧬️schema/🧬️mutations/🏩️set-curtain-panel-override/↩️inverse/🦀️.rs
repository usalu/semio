//! ↩️ Inverse of `SetCurtainPanelOverride`: an absolute `SetCurtainPanelOverride` back to the base panel, none when the override is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override::SetCurtainPanelOverride;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetCurtainPanelOverride, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override::SetCurtainPanelOverride>,
{
    match base.curtain_panel_overrides.get(&payload.id) {
        Some(record) => vec![Op::from(SetCurtainPanelOverride { id: payload.id.clone(), panel: record.panel.clone() })],
        None => Vec::new(),
    }
}
