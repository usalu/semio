//! 📦️ payload and the frame step are the ONE FEM implementation in the fem 2d crate
//! ⏱️ Fem3d play app command — `result-animation-tick`: one playback clock tick of a fem 3d results window. The
//! (`semio_s_artifact_fem_2d_editor::editor::fem2d::commands::result_animation_tick`), driven over
//! [`crate::editor::fem3d::commands::set_result_animation::Fem3dResultsPlayback`].

use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::ViewModel;
pub use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::result_animation_tick::result_animation_tick_step;
pub use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::result_animation_tick::ResultAnimationTick;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

pub fn handle(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem.result-animation-tick.window-context-required"))
}

/// 🚧️ The batch route has no transient authority to land a frame in — the tick is a retained
/// command by construction, and any other arrival is a routing error.
pub fn handle_window(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view: &ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem.result-animation-tick.retained-route-required"))
}

//#region 🧪️Tests
//#endregion 🧪️Tests
