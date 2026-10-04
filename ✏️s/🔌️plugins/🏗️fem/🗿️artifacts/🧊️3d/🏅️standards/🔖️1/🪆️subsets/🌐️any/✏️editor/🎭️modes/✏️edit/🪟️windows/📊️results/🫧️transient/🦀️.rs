//! 🫧️ The fem 3d results window's owner of the ONE FEM playback clock — the clock, its transient record, its
//! `set-playback-clock` leaf and every law live in the fem 2d crate beside the fem 2d results window
//! (`semio_s_artifact_fem_2d::editor::fem2d::modes::edit::windows::results::transient`); this window kind only
//! registers it under its own id.

use semio_framework_plugin::{WindowTransientOwner, WindowTransientOwnerBundle};
pub use semio_s_artifact_fem_2d::editor::fem2d::modes::edit::windows::results::transient::{addressed_to, captured_clock, required_clock, FemPlaybackClock, FemResultsWindowTransient, FemResultsWindowTransientMutation, SetPlaybackClock};

pub const WINDOW_KIND_ID: &str = super::FEM3D_WINDOW_RESULTS;

/// 🪟️ The fem 3d results window's owner of the clock.
pub struct Fem3dResultsWindowTransientOwner;

impl WindowTransientOwner for Fem3dResultsWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = WINDOW_KIND_ID;
    type State = FemResultsWindowTransient;
    type Mutation = FemResultsWindowTransientMutation;

    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        semio_s_artifact_fem_2d::editor::fem2d::modes::edit::windows::results::transient::owners()
    }
}
