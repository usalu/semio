//! 🕹️ interaction of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        #[path = "."]
        pub mod entities {
            pub use semio_s_artifact_bim_model_entities::editor::bim::entities::*;
            pub use semio_s_artifact_bim_model_entities::editor::bim::entities::*;
            #[path = "."]
            pub mod families {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::families::*;
                #[cfg(test)]
                #[path = "../../../🧩️entities/🧬️families/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[cfg(test)]
            #[path = "../../../🧩️entities/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod interaction {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod schedule {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧮️schedule/🏷️vocabulary/🦀️.rs"]
                        pub mod vocabulary;
                    }
                }
            }
        }
    }
}
