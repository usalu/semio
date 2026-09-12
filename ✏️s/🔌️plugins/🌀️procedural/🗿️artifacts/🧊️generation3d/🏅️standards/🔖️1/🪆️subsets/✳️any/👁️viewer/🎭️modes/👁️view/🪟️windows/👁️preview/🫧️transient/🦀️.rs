//! 🫧️ Computed evaluation owned by one exact generation3d viewer preview window.
//!
//! ⏱️ This is the publication end of the shared `flowEvalTick` chain (`🧵️preview-eval`): the tick
//! is ADDRESSED at a concrete preview window, `retained_window_transient_target` captures that
//! window's mutation authority, and the tick's `CompleteWithEphemeral` writes the evaluation here.
//! The window's own `render_with_request_context` then reads it back. Without a window-scoped owner
//! the tick has no authority to capture and every dispatch fails its retained preflight — which is
//! exactly the shape the editor's two preview windows already run
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
//!
//! 🪪️ The STATE is this surface's own `🫧️transient` type, not a second schema: one evaluated
//! output, one shape, one set of format leaves. The window owner narrows WHO may write it, never
//! WHAT is written — the same relation `🧬️generate`'s preview owner has to `✏️edit`'s on the
//! sibling surface.

use crate::viewer::generation3d::transient::{Generation3dViewTransient, Generation3dViewTransientMutation, SetPreviewEval};

/// 🪟️ The viewer preview window's retained evaluation publication.
pub struct Generation3dViewPreviewWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for Generation3dViewPreviewWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::WINDOW_KIND_ID;
    type State = Generation3dViewTransient;
    type Mutation = Generation3dViewTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

/// 🎒️ One evaluation publication's real retained footprint: the state header plus the evaluation
/// text's own capacity, so a 250 KB eval is admitted as 250 KB and not as one anonymous row.
fn preflight(mutation: &Generation3dViewTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Generation3dViewTransientMutation::SetPreviewEval(SetPreviewEval { eval_text }) = mutation;
    let retained_bytes = size_of::<Generation3dViewTransient>().checked_add(eval_text.as_ref().map_or(0, String::capacity)).ok_or_else(|| "Generation3d viewer preview window transient footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Generation3d viewer preview window transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: Generation3dViewTransientMutation) -> Generation3dViewTransient {
    let Generation3dViewTransientMutation::SetPreviewEval(SetPreviewEval { eval_text }) = mutation;
    Generation3dViewTransient { preview_eval_text: eval_text }
}

/// 🎯️ One publication addressed to the window the tick named — refused unless that window really
/// is a viewer preview window that really carries this owner.
pub fn addressed(snapshot: &semio_framework_plugin::WindowTransientSnapshot, eval_text: Option<String>) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    if snapshot.window_kind_id() != super::WINDOW_KIND_ID || snapshot.get::<Generation3dViewPreviewWindowTransientOwner>().is_none() {
        return Err(semio_framework_plugin::Fault::from("generation3d-view-preview-window-transient-required"));
    }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<Generation3dViewPreviewWindowTransientOwner>(snapshot.window_id(), Generation3dViewTransientMutation::SetPreviewEval(SetPreviewEval { eval_text })))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
