//! ↩ Inverse constructor for `ChangeSceneShadowEnabled` — reconstructed from BASE state.

use super::ChangeSceneShadowEnabled;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::ChangeSceneShadowEnabled>,
{
    Ok((|| {
    vec![Op::from(ChangeSceneShadowEnabled { new_enabled: base.scene.shadow.enabled })]

    })())
}
