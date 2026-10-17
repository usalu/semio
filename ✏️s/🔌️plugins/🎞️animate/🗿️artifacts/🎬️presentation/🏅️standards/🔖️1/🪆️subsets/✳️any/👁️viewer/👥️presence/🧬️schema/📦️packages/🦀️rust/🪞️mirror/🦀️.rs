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
                pub mod io {
                    #[path = "../../../../../../🚪️io/📝️text/🦀️.rs"]
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
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
pub use semio_s_artifact_animate_presentation_inferences::snapshot;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod config {
            #[path = "."]
            pub mod io {
                #[path = "../../../../../../✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../../../✏️editor/🎚️config/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
            #[path = "../../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../../../../../✏️editor/👥️presence/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../../../../../✏️editor/👥️presence/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
            #[path = "../../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../../../✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod presence {
            #[path = "../../../🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
