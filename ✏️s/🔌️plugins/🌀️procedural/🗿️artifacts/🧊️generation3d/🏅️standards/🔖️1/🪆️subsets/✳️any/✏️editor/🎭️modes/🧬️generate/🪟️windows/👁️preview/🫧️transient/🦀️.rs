//! 🟚 Computed evaluation owned by one exact Generation3d generate-preview window.

use crate::editor::generation3d::modes::edit::windows::preview::transient::{Generation3dPreviewWindowTransient, Generation3dPreviewWindowTransientMutation, Generation3dPreviewWindowTransientOwner};

pub struct Generation3dGeneratePreviewWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for Generation3dGeneratePreviewWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW;
    type State = Generation3dPreviewWindowTransient;
    type Mutation = Generation3dPreviewWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        Generation3dPreviewWindowTransientOwner::build_owners()
    }
}

pub fn addressed(snapshot: &semio_framework_plugin::WindowTransientSnapshot, eval_text: Option<String>) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    if snapshot.window_kind_id() != super::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW || snapshot.get::<Generation3dGeneratePreviewWindowTransientOwner>().is_none() {
        return Err(semio_framework_plugin::Fault::from("generation3d-generate-preview-window-transient-required"));
    }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<Generation3dGeneratePreviewWindowTransientOwner>(
        snapshot.window_id(),
        Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text },
    ))
}
