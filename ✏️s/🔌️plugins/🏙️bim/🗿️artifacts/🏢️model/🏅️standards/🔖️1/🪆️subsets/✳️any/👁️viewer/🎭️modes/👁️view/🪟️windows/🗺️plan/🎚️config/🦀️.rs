//! 🎚️ Persisted local configuration of ONE concrete BIM viewer plan window: the storey it shows and its pan/zoom. Per window instance, never
//! per app, and never on the artifact lane.

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🗺️ Storey id (`""` = the lowest storey of the model) and viewport of one plan window. `framed` turns true with the first pan/zoom gesture
/// and from then on suppresses the fit to the storey linework.
#[derive(semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.bim.model.viewer-plan-window.config", extension = "bimviewerplanwindowcfg")]
pub struct BimViewerPlanWindowConfig {
    pub storey: String,
    #[dsl(block)]
    pub viewport: store::Viewport2d,
    pub framed: bool,
}

impl Default for BimViewerPlanWindowConfig {
    fn default() -> Self {
        Self { storey: String::new(), viewport: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 }, framed: false }
    }
}

crate::bim_window_config! {
    config: BimViewerPlanWindowConfig,
    diff: BimViewerPlanWindowConfigDiff,
    mutation: BimViewerPlanWindowConfigMutation,
    owner: BimViewerPlanWindowConfigOwner,
    window: super::WINDOW_KIND_ID,
    schema: "bim.model.viewer-plan-window.config",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️plan/🎚️config",
    display: "Set BIM Viewer Plan Window Configuration",
    bytes: 4_096,
    fields: { storey: String, viewport: store::Viewport2d, framed: bool },
}
//#endregion 🔖️Config

//#region 🔖️Edits
impl BimViewerPlanWindowConfig {
    /// 🎥️ The viewport the user just produced; it also marks the window as framed.
    pub fn with_viewport(&self, viewport: store::Viewport2d) -> Self {
        Self { viewport, framed: true, ..self.clone() }
    }

    /// 🏢️ Shows another storey and requests a fresh fit to its linework.
    pub fn with_storey(&self, storey: &str) -> Self {
        Self { storey: storey.to_string(), framed: false, ..self.clone() }
    }
}
//#endregion 🔖️Edits

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
