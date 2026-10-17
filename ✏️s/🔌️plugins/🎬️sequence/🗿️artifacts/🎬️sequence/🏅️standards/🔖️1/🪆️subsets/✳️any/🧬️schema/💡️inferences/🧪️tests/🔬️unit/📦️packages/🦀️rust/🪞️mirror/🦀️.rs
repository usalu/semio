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
                        pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::schema::inferences::topology::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_sequence_sequence_schema::snapshot;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_sequence_sequence_inferences::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sequence {
        #[path = "."]
        pub mod edit_rules {
            pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::edit_rules::*;
            #[cfg(test)]
            #[path = "../../../../../../../✏️editor/📐️edit-rules/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::terminology::*;
            #[cfg(test)]
            #[path = "../../../../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "../../../../../../../✏️editor/🧱️base/🦀️.rs"]
        pub mod base;
        #[path = "../../../../../../../✏️editor/🖥️host/🦀️.rs"]
        pub mod host;
        #[cfg(target_arch = "wasm32")]
        #[path = "../../../../../../../✏️editor/🌉️wasm/🧩️component.rs"]
        pub mod wasm;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../✏️editor/🎮️commands/🔗️connection/🦀️.rs"]
            pub mod connection;
            #[path = "../../../../../../../✏️editor/🎮️commands/📚️example/🦀️.rs"]
            pub mod example;
            #[path = "../../../../../../../✏️editor/🎮️commands/🕸️node-graph/🦀️.rs"]
            pub mod node_graph;
            #[path = "../../../../../../../✏️editor/🎮️commands/🪜️step/🦀️.rs"]
            pub mod step;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sequence {
        #[path = "../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_sequence_sequence_inferences::viewer::sequence::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_sequence_sequence_inferences::viewer::sequence::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_sequence_sequence_inferences::viewer::sequence::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📽️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
