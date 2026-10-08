//! 🔺️ Shooting presence diff — sparse per-field delta over [`ShootingPresence`].

use super::super::ShootingPresence;
use crate::ShootingCamera;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};

/// 🔺️ Sparse field delta for the shooting presence; an absent field is untouched.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingPresenceDiff {
    pub selected_shot_ids: Option<Vec<String>>,
    pub camera: Option<ShootingCamera>,
}

impl MutationDiff<ShootingPresence> for ShootingPresenceDiff {
    fn apply(&self, base: &ShootingPresence, _capability: ApplyCapability) -> MutationApplyResult<ShootingPresence> {
        let mut next = base.clone();
        if let Some(selected_shot_ids) = &self.selected_shot_ids {
            next.selected_shot_ids = selected_shot_ids.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.selected_shot_ids.is_some() {
            self.selected_shot_ids = other.selected_shot_ids;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl DiffAlgebra<ShootingPresence> for ShootingPresenceDiff {
    fn inverse(&self, base: &ShootingPresence) -> Self {
        Self { selected_shot_ids: self.selected_shot_ids.as_ref().map(|_| base.selected_shot_ids.clone()), camera: self.camera.as_ref().map(|_| base.camera.clone()) }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
