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
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_schema {
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::*;
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::diff;
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "."]
                            pub mod change_schema {
                                pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::io::binary::mutations::change_schema::*;
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/💾️binary/🧬️mutations/✒️change-schema/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_demonstrator_playground_inferences::examples::demo::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod playground {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod commands {
            #[path = "../../../🎮️commands/🔧️change-schema/🦀️.rs"]
            pub mod change_schema;
            pub use semio_s_artifact_demonstrator_playground_inferences::editor::playground::commands::set_active_example;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playground {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
