//! 🔺️ Applies a camera control and publishes its owned slot and shot link together.

use super::PatchShotCamera;
use super::component::{apply_field, slot_id};
use crate::{ShootingAssigned, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingShotPatch, ShootingSnapshot};
use crate::diff::{ShootingDiff, ShootingEdit};

pub fn diff(payload: &PatchShotCamera, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let id = &payload.shot_id;
    let Some(shot) = base.shots.iter().find(|shot| shot.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "Shot does not exist.", [id.clone()]);
    };
    let slot = slot_id(id);
    let linked = shot.camera_id.as_ref().and_then(|id| base.saved_cameras.iter().find(|entry| entry.id == *id));
    let current = linked.map_or(&payload.fallback, |entry| &entry.camera);
    let mut camera = current.clone();
    if !apply_field(&mut camera, &payload.field, &payload.camera) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown shot camera control.", [id.clone()]);
    }
    let owned = linked.is_some_and(|entry| entry.id == slot);
    if payload.release.is_some() && !owned {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Shot camera slot is already released.");
    }
    let shared = base.shots.iter().any(|other| other.id != *id && other.camera_id.as_ref() == Some(&slot));
    if owned {
        if let Some(release) = &payload.release {
            if !shared && camera == release.seed {
                let index = base.saved_cameras.iter().position(|entry| entry.id == slot).expect("owned camera slot");
                let mut diff = ShootingDiff::camera_edit(ShootingEdit::Remove { id: slot, index });
                diff.shots = assign_camera(id, release.camera_id.clone()).shots;
                return protocol::MutationOutcome::new(diff);
            }
        }
    }
    if !PatchShotCamera::valid_camera(&camera) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Invalid shot camera pose.", [id.clone()]);
    }
    if owned {
        if shared {
            return protocol::MutationOutcome::fatal("mutation.invariant", "Shot camera slot is shared.", [slot]);
        }
        if camera == *current {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "Shot camera is unchanged.");
        }
        return protocol::MutationOutcome::new(ShootingDiff::camera_patches([(slot, ShootingSavedCameraPatch {
            label: None,
            camera: Some(camera),
        })]));
    }
    if base.saved_cameras.iter().any(|entry| entry.id == slot) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "Shot camera slot is occupied.", [slot]);
    }
    let mut diff = ShootingDiff::camera_edit(ShootingEdit::Add {
        index: base.saved_cameras.len(),
        item: ShootingSavedCamera { id: slot.clone(), label: shot.label.clone(), camera },
    });
    diff.shots = assign_camera(id, Some(slot)).shots;
    protocol::MutationOutcome::new(diff)
}

fn assign_camera(id: &str, camera_id: Option<String>) -> ShootingDiff {
    ShootingDiff::shot_patches([(id.to_owned(), ShootingShotPatch {
        camera_id: Some(ShootingAssigned::new(camera_id)),
        ..Default::default()
    })])
}
