//! 🍃️ `set-playback-clock` leaf and every law live in the fem 2d crate beside the fem 2d results window
//! 🫧️ The fem 3d results window's owner of the ONE FEM playback clock — the clock, its transient record, its
//! (`semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results::transient`); this window kind only
//! registers it under its own id.

use semio_framework_plugin::WindowTransientOwner;

use semio_framework_plugin::WindowTransientOwnerBundle;
pub use crate::editor::fem3d::modes::edit::windows::results::config::addressed_to;
pub use semio_s_artifact_fem_2d_results_state::transient::captured_clock;
pub use semio_s_artifact_fem_2d_results_state::transient::required_clock;
pub use semio_s_artifact_fem_2d_results_state::transient::FemPlaybackClock;
pub use semio_s_artifact_fem_2d_results_state::transient::FemResultsWindowTransient;
pub use semio_s_artifact_fem_2d_results_state::transient::FemResultsWindowTransientMutation;
pub use semio_s_artifact_fem_2d_results_state::transient::SetPlaybackClock;

pub const WINDOW_KIND_ID: &str = super::FEM3D_WINDOW_RESULTS;

/// 🪟️ The fem 3d results window's owner of the clock.
pub struct Fem3dResultsWindowTransientOwner;

impl WindowTransientOwner for Fem3dResultsWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = WINDOW_KIND_ID;
    type State = FemResultsWindowTransient;
    type Mutation = FemResultsWindowTransientMutation;

    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        semio_s_artifact_fem_2d_results_state::transient::owners()
    }
}
