//! `set-playback-clock` leaf and every law live in the fem 2d crate beside the fem 2d results window
//! 🫧️ The fem 3d results window's owner of the ONE FEM playback clock — the clock, its transient record, its
//! (`semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results::transient`); this window kind only
//! registers it under its own id.

use semio_framework_plugin::WindowTransientOwner;

use semio_framework_plugin::WindowTransientOwnerBundle;
pub use crate::transient::addressed_to;
pub use crate::transient::captured_clock;
pub use crate::transient::required_clock;
pub use crate::transient::FemPlaybackClock;
pub use crate::transient::FemResultsWindowTransient;
pub use crate::transient::FemResultsWindowTransientMutation;
pub use crate::transient::SetPlaybackClock;

pub const WINDOW_KIND_ID: &str = super::FEM3D_WINDOW_RESULTS;

/// 🪟️ The fem 3d results window's owner of the clock.
pub struct Fem3dResultsWindowTransientOwner;

impl WindowTransientOwner for Fem3dResultsWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = WINDOW_KIND_ID;
    type State = FemResultsWindowTransient;
    type Mutation = FemResultsWindowTransientMutation;

    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        crate::transient::owners()
    }
}
