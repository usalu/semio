//! 🔺️ Shooting config diff — sparse per-field delta over [`ShootingConfig`]: each leaf names exactly the fields it sets.

use super::super::ShootingConfig;
use crate::ShootingCamera;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};

/// 🔺️ Sparse field delta for the shooting config; an absent field is untouched.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingConfigDiff {
    pub default_shot_format: Option<String>,
    pub default_shot_shape: Option<String>,
    pub default_asset_format: Option<String>,
    pub selected_shot_ids: Option<Vec<String>>,
    pub center_model: Option<bool>,
    pub fit_revision: Option<u32>,
    pub camera: Option<ShootingCamera>,
}

impl MutationDiff<ShootingConfig> for ShootingConfigDiff {
    fn apply(&self, base: &ShootingConfig, _capability: ApplyCapability) -> MutationApplyResult<ShootingConfig> {
        let mut next = base.clone();
        macro_rules! write {
            ($($field:ident),+) => {
                $(if let Some(value) = &self.$field {
                    next.$field = value.clone();
                })+
            };
        }
        write!(default_shot_format, default_shot_shape, default_asset_format, selected_shot_ids, center_model, fit_revision, camera);
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($($field:ident),+) => {
                $(if other.$field.is_some() {
                    self.$field = other.$field;
                })+
            };
        }
        take!(default_shot_format, default_shot_shape, default_asset_format, selected_shot_ids, center_model, fit_revision, camera);
    }
}

impl DiffAlgebra<ShootingConfig> for ShootingConfigDiff {
    fn inverse(&self, base: &ShootingConfig) -> Self {
        Self {
            default_shot_format: self.default_shot_format.as_ref().map(|_| base.default_shot_format.clone()),
            default_shot_shape: self.default_shot_shape.as_ref().map(|_| base.default_shot_shape.clone()),
            default_asset_format: self.default_asset_format.as_ref().map(|_| base.default_asset_format.clone()),
            selected_shot_ids: self.selected_shot_ids.as_ref().map(|_| base.selected_shot_ids.clone()),
            center_model: self.center_model.as_ref().map(|_| base.center_model),
            fit_revision: self.fit_revision.as_ref().map(|_| base.fit_revision),
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
        }
    }

    fn between(base: &ShootingConfig, other: &ShootingConfig) -> Self {
        Self {
            default_shot_format: (base.default_shot_format != other.default_shot_format).then(|| other.default_shot_format.clone()),
            default_shot_shape: (base.default_shot_shape != other.default_shot_shape).then(|| other.default_shot_shape.clone()),
            default_asset_format: (base.default_asset_format != other.default_asset_format).then(|| other.default_asset_format.clone()),
            selected_shot_ids: (base.selected_shot_ids != other.selected_shot_ids).then(|| other.selected_shot_ids.clone()),
            center_model: (base.center_model != other.center_model).then_some(other.center_model),
            fit_revision: (base.fit_revision != other.fit_revision).then_some(other.fit_revision),
            camera: (base.camera != other.camera).then(|| other.camera.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
