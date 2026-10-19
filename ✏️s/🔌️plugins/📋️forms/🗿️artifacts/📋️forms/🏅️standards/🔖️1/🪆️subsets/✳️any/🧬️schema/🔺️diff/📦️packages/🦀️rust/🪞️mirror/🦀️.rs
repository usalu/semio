//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod move_block_to_step {
                            #[path = "../../../../🧬️mutations/📦move-block-to-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/📦move-block-to-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../../🧬️mutations/📦move-block-to-step/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod reorder_step {
                            #[path = "../../../../🧬️mutations/🔀reorder-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🔀reorder-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../../🧬️mutations/🔀reorder-step/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod change_form_title {
                            #[path = "../../../../🧬️mutations/🏷️change-form-title/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🏷️change-form-title/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../../🧬️mutations/🏷️change-form-title/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod change_block_field {
                            #[path = "../../../../🧬️mutations/🎛️change-block-field/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🎛️change-block-field/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../../🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[path = "."]
                            pub mod response {
                                pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::io::text::snapshot::response::export;
                            }
                        }
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "."]
        pub mod questions {
            #[path = "../../../../../✏️editor/❓️questions/🫥️visibility/🦀️.rs"]
            pub mod visibility;
            #[path = "../../../../../✏️editor/❓️questions/🧩️extensions/🦀️.rs"]
            pub mod extensions;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod responses {
                #[path = "../../../../../✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🦀️.rs"]
                pub mod results;
            }
        }
    }
}
