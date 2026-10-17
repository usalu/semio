//! 💡️ inferences of `semio-s-artifact-demonstrator-playground` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use semio_s_artifact_demonstrator_playground_schema::{PLAYGROUND_DOCUMENT_SCHEMA};

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
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs"]
                        pub mod structural_correspondence_tests;
                        #[path = "."]
                        pub mod change_schema {
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::*;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/✒️change-schema/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::diff;
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::inverse;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/✒️change-schema/🧪️tests/📅️retags/🦀️.rs"]
                            pub mod tests_retags_the_playground_artifact_schema;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "."]
                            pub mod change_schema {
                                pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::io::text::mutations::change_schema::*;
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/📝️text/🧬️mutations/✒️change-schema/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod playground {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"]
            pub mod set_active_example;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playground {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
