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
                    pub mod snapshot {
                        pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::schema::snapshot::*;
                        #[cfg(test)]
                        #[path = "../../../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "../../../🦀️.rs"]
                    pub mod diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "../../../../🧬️mutations/🎲️change-seed/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_cell_size {
                            #[path = "../../../../🧬️mutations/📏️change-cell-size/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/📏️change-cell-size/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/📏️change-cell-size/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            #[path = "../../../../🧬️mutations/🔁️change-periodicity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/🔁️change-periodicity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🔁️change-periodicity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
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
                                            #[path = "../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
    pub mod grid2d {
        #[path = "../../../../../📚️examples/🚰️pipes/🦀️.rs"]
        pub mod pipes;
        #[path = "../../../../../📚️examples/🏜️terrain/🦀️.rs"]
        pub mod terrain;

        /// 📇️ Every bundled example, in the order the editor's example picker offers them.
        pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
            vec![pipes::source(), terrain::source()]
        }

        /// 🧫️ The committed default document an editor or viewer boots with.
        pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
            static SOURCES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
            SOURCES.get_or_init(sources).as_slice()
        }

        #[cfg(test)]
        #[path = "../../../../../📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
        mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid2d {
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
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid2d {
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
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}
