//! 🧰️ kit of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod render {
    pub use semio_s_artifact_bim_model_tests::render::window_config;
    pub use semio_s_artifact_bim_model_tests_5::render::world;
    pub use semio_s_artifact_bim_model_plan_linework::render::plan;
    pub use semio_s_artifact_bim_model_modify::render::envelope;
    pub use semio_s_artifact_bim_model_modify::render::structure;
}
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
                    pub mod export {
                        #[path = "."]
                        pub mod sheets {
                            pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::*;
                            pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::sheets::ink;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                        }
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
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        #[path = "."]
        pub mod kit {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        #[path = "."]
        pub mod utilities {
            #[path = "../../../🪛️utilities/🦀️.rs"]
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
                    pub mod plan {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🗺️plan/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod world {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧊️world/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod section {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/📐️section/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod schedule {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧮️schedule/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod sheet {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/📄️sheet/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
