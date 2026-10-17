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
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "../../../🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::io::binary::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
pub use semio_s_artifact_sequence_sequence_schema::diff;
pub use semio_s_artifact_sequence_sequence_schema::snapshot;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sequence {
        #[cfg(target_arch = "wasm32")]
        #[path = "."]
        pub mod wasm {
            #[path = "../../../../../✏️editor/🌉️wasm/📡️protocol.rs"]
            pub mod protocol;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../../✏️editor/📐️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "../../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sequence {
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
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📽️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
