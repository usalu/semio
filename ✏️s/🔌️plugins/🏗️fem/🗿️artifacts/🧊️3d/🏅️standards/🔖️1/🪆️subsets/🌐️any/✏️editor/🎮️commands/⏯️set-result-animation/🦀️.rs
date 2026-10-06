//! ⏯️ Fem3d play app command — `result-animation`: the fem 3d results window as a [`FemPlaybackTransport`]. The
//! payload, the field vocabulary and the playback step are the ONE FEM implementation in the fem 2d crate
//! (`semio_s_artifact_fem_2d::editor::fem2d::commands::set_result_animation`).

use crate::app_surface::FemResultsAnimation;
use crate::editor::fem3d::modes::edit::windows::results;
use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation, ViewModel, WindowConfigMutation};
pub use semio_s_artifact_fem_2d::editor::fem2d::commands::set_result_animation::{apply_field, merge, rearm_effect, set_result_animation_step, FemPlaybackStep, FemPlaybackTransport, SetResultAnimation, TICK_ACTION};

type Fem3dSnapshot = crate::Fem3dSnapshot;

/// 🧊️ The fem 3d results window as a playback transport.
pub struct Fem3dResultsPlayback;

impl FemPlaybackTransport for Fem3dResultsPlayback {
    type Mutation = Fem3dMutation;
    type Config = results::config::Fem3dResultsWindowConfig;
    type Owner = results::transient::Fem3dResultsWindowTransientOwner;
    const REARM_REQUEST: u64 = 143;

    fn current(cfg: &ConfigView<'_, NoConfig>) -> Self::Config {
        results::config::current(cfg)
    }

    fn animation(config: &Self::Config) -> FemResultsAnimation {
        config.animation
    }

    fn with_animation(config: &Self::Config, animation: FemResultsAnimation) -> Self::Config {
        Self::Config { animation, ..config.clone() }
    }

    fn addressed_window_id(cfg: &ConfigView<'_, NoConfig>, view: &ViewModel, requested: Option<&str>) -> Result<String, Fault> {
        results::config::addressed_window_id(cfg, view, requested)
    }

    fn addressed_to(window_id: &str, config: Self::Config) -> WindowConfigMutation {
        results::config::addressed_to(window_id, config)
    }

    /// 🪟️ The results body and the panel that reads its transport.
    fn dirty_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: vec![results::FEM3D_BODY_RESULTS.to_owned()], panel_bodies: vec![crate::editor::fem3d::panels::results::BODY_KEY.to_owned()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }
}

pub fn handle(_payload: &SetResultAnimation, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem.result-animation.window-context-required"))
}

/// 🗂️ Merges the gesture into the addressed fem 3d results window's playback state (batch route: no clock).
pub fn handle_window(payload: &SetResultAnimation, _doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    set_result_animation_step::<Fem3dResultsPlayback>(payload, cfg, view, None).map(|step| step.emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
