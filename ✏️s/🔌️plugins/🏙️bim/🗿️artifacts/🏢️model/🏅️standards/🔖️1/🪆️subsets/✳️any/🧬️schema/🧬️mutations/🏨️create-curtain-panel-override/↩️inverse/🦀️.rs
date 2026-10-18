//! ↩️ Inverse of `CreateCurtainPanelOverride`: the concrete `DeleteCurtainPanelOverride` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride;
use crate::standards::v1::subsets::any::schema::mutations::create_curtain_panel_override::CreateCurtainPanelOverride;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateCurtainPanelOverride, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride>,
{
    if base.curtain_panel_overrides.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteCurtainPanelOverride { id: payload.id.clone() })]
}
