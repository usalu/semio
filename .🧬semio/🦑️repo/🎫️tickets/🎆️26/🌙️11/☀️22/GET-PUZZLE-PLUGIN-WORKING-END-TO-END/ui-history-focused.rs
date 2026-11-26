extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

pub use semio_s_artifact_puzzle_2d_mutations::standards;

pub mod editor {
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_v1::editor::puzzle2d::*;
        pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::{commands, snapshot};
        pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::{kit, recorder};
        pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::retained;
        pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::{config, presence};
        pub use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window;
        pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::precompute;
        pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels;

        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
        }

        pub mod modes {
            pub mod edit {
                pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::*;
                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::modes::edit::tools;

                pub mod windows {
                    pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::*;

                    pub mod overview {
                        pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::overview::*;
                        pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities;
                    }
                }
            }
        }
    }
}

#[path = "@HISTORY@"]
mod history_edit_runtime_tests;
