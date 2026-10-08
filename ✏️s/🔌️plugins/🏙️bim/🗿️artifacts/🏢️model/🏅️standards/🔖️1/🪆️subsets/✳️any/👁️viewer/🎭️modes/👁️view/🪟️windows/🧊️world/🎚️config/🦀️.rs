//! 🎚️ Persisted local configuration of ONE concrete BIM viewer world window: its orbit pose, its projection preset bank and the storeys
//! the user hid. A window config, not an app config: two open world windows orbit and filter independently, and a read-only surface
//! writes nothing to the artifact lane.

use semio_framework_plugin::{WorldProjectionConfig, apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🧊️ Orbit pose, projection preset bank and hidden storey ids of one world window. `framed` turns true with the first camera gesture and
/// from then on suppresses the one-shot fit to the model.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.bim.model.viewer-world-window.config", extension = "bimviewerworldwindowcfg")]
pub struct BimViewerWorldWindowConfig {
    #[dsl(block)]
    pub orbit: store::Viewport3dOrbit,
    #[dsl(block)]
    pub projection: WorldProjectionConfig,
    pub hidden_storeys: Vec<String>,
    pub framed: bool,
}

impl Default for BimViewerWorldWindowConfig {
    fn default() -> Self {
        Self { orbit: store::Viewport3dOrbit { position: [12.0, -12.0, 9.0], target: [0.0; 3], zoom: 1.0, up: None }, projection: WorldProjectionConfig::default(), hidden_storeys: Vec::new(), framed: false }
    }
}

crate::bim_window_config! {
    config: BimViewerWorldWindowConfig,
    diff: BimViewerWorldWindowConfigDiff,
    mutation: BimViewerWorldWindowConfigMutation,
    owner: BimViewerWorldWindowConfigOwner,
    window: super::WINDOW_KIND_ID,
    schema: "bim.model.viewer-world-window.config",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️world/🎚️config",
    display: "Set BIM Viewer World Window Configuration",
    bytes: 16_384,
    fields: { orbit: store::Viewport3dOrbit, projection: WorldProjectionConfig, hidden_storeys: Vec<String>, framed: bool },
}
//#endregion 🔖️Config

//#region 🔖️Edits
impl BimViewerWorldWindowConfig {
    /// 🎥️ The orbit pose the user just produced; it also marks the window as framed.
    pub fn with_orbit(&self, orbit: store::Viewport3dOrbit) -> Self {
        Self { orbit, framed: true, ..self.clone() }
    }

    /// 📐️ Applies a `setProjection` / `setProjectionParam` action; a kind/view change re-poses the orbit around its target at the current distance.
    pub fn with_projection_action(&self, action: &str, args: &semio_framework_pack_json::Value) -> Option<Self> {
        let mut projection = self.projection.clone();
        if !apply_world3d_projection_action(&mut projection, action, Some(args)) {
            return None;
        }
        let mut orbit = self.orbit;
        if world3d_projection_action_moves_pose(action, Some(args)) {
            let distance = orbit.position.iter().zip(orbit.target.iter()).map(|(position, target)| (position - target).powi(2)).sum::<f64>().sqrt().max(1.0);
            let (position, up) = world3d_projection_pose(&projection, orbit.target, distance);
            orbit = store::Viewport3dOrbit { position, up: Some(up), ..orbit };
        }
        Some(Self { orbit, projection, framed: true, ..self.clone() })
    }

    /// 👁️ Shows or hides one storey; the hidden list stays sorted and free of duplicates.
    pub fn with_storey_visible(&self, storey: &str, visible: bool) -> Self {
        let mut hidden: Vec<String> = self.hidden_storeys.iter().filter(|id| id.as_str() != storey).cloned().collect();
        if !visible {
            hidden.push(storey.to_string());
            hidden.sort();
        }
        Self { hidden_storeys: hidden, ..self.clone() }
    }

    pub fn shows(&self, storey: &str) -> bool {
        !self.hidden_storeys.iter().any(|id| id == storey)
    }
}
//#endregion 🔖️Edits

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
