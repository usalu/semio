//! 📌️ Owns the bitmap pin command, bounded materialization and its complete command laws.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;

pub use semio_s_artifact_wfc_bitmap_inferences::BitmapMutation;
pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot;

pub mod schema {
    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
}
pub mod inferences {
    pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::inferences::*;
    pub use semio_s_artifact_wfc_bitmap_inferences::host::inferences::*;
}
pub mod mutations {
    pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::mutations::*;
}
pub mod standards {
    pub mod v1 {
        pub mod subsets {
            pub mod any {
                pub mod io {
                    pub mod text {
                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
pub mod editor {
    pub mod bitmap {
        pub use semio_s_artifact_wfc_bitmap_transient::editor::bitmap::transient;
        pub mod commands {
            pub mod pin_solution {
                pub use crate::component::*;
            }
        }
    }
}

#[path = "../../🦀️.rs"]
mod component;
pub use component::{PinSolution, BitmapPinCursor, PIN_SOLUTION_ACTION_ID, BITMAP_PIN_WORK_UNITS_PER_TURN, BITMAP_MAXIMUM_OUTPUT_EDGE, BITMAP_MAXIMUM_OUTPUT_CELLS, handle, solution_operations, require_preview_window, preview_fault, preview_operations, pin_work_items, pin_cells, revision_matches, completion_matches, preview_pixels};

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
