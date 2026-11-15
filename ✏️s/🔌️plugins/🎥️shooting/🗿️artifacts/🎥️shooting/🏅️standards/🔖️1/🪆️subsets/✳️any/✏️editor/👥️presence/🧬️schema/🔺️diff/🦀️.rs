//! 🔺️ Shooting presence diff — sparse per-field delta over [`ShootingPresence`].

use super::super::ShootingPresence;
use crate::ShootingCamera;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};

/// 🔺️ Sparse field delta for the shooting presence; an absent field is untouched.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingPresenceDiff {
    pub selected_shot_ids: Option<Vec<String>>,
    pub camera: Option<ShootingCamera>,
}

protocol::retained_empty_option_patch!(ShootingPresenceDiff { selected_shot_ids, camera });
protocol::retained_row_patch!(pub retained_fields for ShootingPresenceDiff => ShootingPresence { selected_shot_ids: Vec<String>, camera: ShootingCamera });
protocol::retained_patch_diff!(ShootingPresenceDiff => ShootingPresence);

impl MutationDiff<ShootingPresence> for ShootingPresenceDiff {}

impl protocol::ColdMutationDiff<ShootingPresence> for ShootingPresenceDiff {
    fn absorb(&mut self, other: Self) {
        if other.selected_shot_ids.is_some() {
            self.selected_shot_ids = other.selected_shot_ids;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }

    fn apply(&self, unpublished: &mut ShootingPresence, _capability: ApplyCapability) -> MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(selected_shot_ids) = &self.selected_shot_ids {
            next.selected_shot_ids = selected_shot_ids.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        *unpublished = next;
        Ok(())
    }
}

impl DiffAlgebra<ShootingPresence> for ShootingPresenceDiff {
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

impl protocol::ColdDiffAlgebra<ShootingPresence> for ShootingPresenceDiff {
    fn inverse(&self, base: &ShootingPresence) -> Self {
        Self { selected_shot_ids: self.selected_shot_ids.as_ref().map(|_| base.selected_shot_ids.clone()), camera: self.camera.as_ref().map(|_| base.camera.clone()) }
    }
}
