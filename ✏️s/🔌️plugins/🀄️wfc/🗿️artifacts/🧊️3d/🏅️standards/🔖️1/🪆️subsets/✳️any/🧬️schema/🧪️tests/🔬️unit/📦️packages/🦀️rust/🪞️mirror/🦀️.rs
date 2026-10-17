//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod host {
    pub use semio_s_artifact_wfc_3d_inferences::host::inferences;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_3d_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod tests;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                        }
                    }
                    pub use semio_s_artifact_wfc_3d_inferences::standards::v1::subsets::any::schema::inferences;
                }
                pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples;
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_wfc_3d_inferences::standards::v1::subsets::any::io::binary::*;
                        pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_3d_wfc3d::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::io::text::mutations;
                    }
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::import;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc3d {
        pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::*;
        pub use semio_s_artifact_wfc_3d_transient::editor::wfc3d::transient;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::*;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::*;
                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::tools::*;
                    #[path = "."]
                    pub mod fill {
                        pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::tools::fill::*;
                        #[cfg(test)]
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/📑️fill/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::windows::*;
                    #[path = "."]
                    pub mod preview {
                        pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::windows::preview::*;
                        #[cfg(test)]
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️preview/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wfc3d {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_3d_wfc3d::viewer::wfc3d::modes::view::*;
                pub use semio_s_artifact_wfc_3d_wfc3d::viewer::wfc3d::modes::view::windows;
            }
        }
    }
}
