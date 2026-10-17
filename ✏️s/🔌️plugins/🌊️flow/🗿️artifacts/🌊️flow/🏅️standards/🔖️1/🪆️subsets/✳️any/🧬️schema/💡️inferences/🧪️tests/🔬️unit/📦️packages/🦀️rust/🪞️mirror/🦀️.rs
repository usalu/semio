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
                    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_flow_flow_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_flow_flow_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_flow_flow_inferences::standards::v1::subsets::any::schema::inferences::topology::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::io::binary::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "../../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
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
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::base;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::presence::*;
            #[cfg(test)]
            #[path = "../../../../../../../✏️editor/👥️presence/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod edit_rules {
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::edit_rules::*;
            #[cfg(test)]
            #[path = "../../../../../../../✏️editor/📐️edit-rules/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::terminology::*;
            #[cfg(test)]
            #[path = "../../../../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🦀️.rs"]
                        pub mod transient;
                    }
                }
            }
            #[path = "."]
            pub mod generate {
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../../✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs"]
                    pub mod generations;
                    #[path = "../../../../../../../✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../../../../✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
