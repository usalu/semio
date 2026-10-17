//! 🧪️ tests of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

#[cfg(test)]
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
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
                pub mod schema {
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    #[path = "."]
                    pub mod validation {
                        pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::validation::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "."]
                            pub mod response {
                                #[path = "."]
                                pub mod export {
                                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::io::text::snapshot::response::export::*;
                                    #[cfg(test)]
                                    #[path = "../../../../../🚪️io/📝️text/📸️snapshot/📨️response/📤️export/🧪️tests/🦀️.rs"]
                                    pub mod tests;
                                }
                            }
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
    pub mod forms {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
            #[path = "."]
            pub mod io {
                pub use semio_s_artifact_forms_forms_config::editor::forms::config::io::*;
                #[path = "../../../../../✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../../✏️editor/🎚️config/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🎚️config/🧪️tests/🔬️contract-vectors/🦀️.rs"]
            pub mod contract_vectors;
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../../../../✏️editor/🧰️support/🦀️.rs"]
        pub mod support;
        #[path = "."]
        pub mod modes {
            #[path = "../../../../../✏️editor/🎭️modes/✍️fill/🦀️.rs"]
            pub mod fill;
            #[path = "."]
            pub mod blueprint {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod try_wizard {
                        #[path = "."]
                        pub mod config {
                            #[path = "../../../../../✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                        #[path = "."]
                        pub mod transient {
                            #[path = "../../../../../✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod inspection {
                #[path = "../../../../../✏️editor/📌️panels/🔍️inspection/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod forms {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
