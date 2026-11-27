//! 🧰️ kit of `semio-s-artifact-bim-model`: the facade that re-exports the carved kit core, the utility registry and the five window crates under their original paths.

pub mod editor {
    pub mod bim {
        pub use semio_s_artifact_bim_model_kit_core::editor::bim::kit;
        pub use semio_s_artifact_bim_model_utilities::editor::bim::utilities;
        pub mod modes {
            pub mod edit {
                pub mod windows {
                    pub use semio_s_artifact_bim_model_window_plan::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_window_schedule::editor::bim::modes::edit::windows::schedule;
                    pub use semio_s_artifact_bim_model_window_section::editor::bim::modes::edit::windows::section;
                    pub use semio_s_artifact_bim_model_window_sheet::editor::bim::modes::edit::windows::sheet;
                    pub use semio_s_artifact_bim_model_window_world::editor::bim::modes::edit::windows::world;
                }
            }
        }
    }
}
