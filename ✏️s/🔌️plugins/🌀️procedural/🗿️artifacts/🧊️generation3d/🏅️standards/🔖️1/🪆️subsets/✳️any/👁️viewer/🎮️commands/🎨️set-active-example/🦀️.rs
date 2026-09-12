//! 🎨️ Generation3d viewer command — `set-active-example`. The read-only twin of the sibling
//! surface's own example switch.
//!
//! 📚️ Opening an example is LOADING a document, not mutating one. The editor's `setActiveExample`
//! replaces the artifact's fixture through the document lane and records an undoable operation; a
//! viewer owns no write authority at all, so this one names the example on its own CONFIG lane and
//! `Generation3dViewer::viewed_document` resolves that id to the example's projection for every
//! read path. The emitted `ViewEmit` carries config operations only — the compile-time half of the
//! read-only guarantee — and `Generation3dViewCommandWork` re-arms every attached preview window's
//! `flowEvalTick` chain afterwards, so the switch's own consequence never depends on a host
//! `refresh-ui` round trip (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use crate::standards::v1::subsets::any::schema::is_generation3d_example_id;
use crate::viewer::generation3d::config;
use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Points the read-only surface at a bundled example — or, for the empty id, back at the
/// document this session actually opened. An id that names no bundled example is refused rather
/// than silently blanking the view, because the picker can only ever offer declared ids.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    if !payload.example_id.is_empty() && !is_generation3d_example_id(&payload.example_id) {
        return Err(Fault::from("generation3d-view-active-example-unknown"));
    }
    Ok(ViewEmit::config(vec![Generation3dViewConfigMutation::SetActiveExample(config::SetActiveExample { value: payload.example_id.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
