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
                    pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod relocated_engine_tests;
                    pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::schema::terrain_descriptor;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::imported_map;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::inferences;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod operations {
                        pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::operations::*;
                        #[cfg(test)]
                        #[path = "../../../../../⚙️operations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[cfg(test)]
                        #[path = "../../../../../⚙️operations/🧪️tests/🔬️kinds-conformance/🦀️.rs"]
                        pub mod kinds_conformance;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::las;
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::ply;
                                pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::io::export::serializers::artifacts::stl;
                                #[path = "."]
                                pub mod gltf {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
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
        #[cfg(test)]
        #[path = "../../../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis3d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../✏️editor/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️.rs"]
                    pub mod terrain;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gisterrain {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️.rs"]
                    pub mod terrain;
                }
            }
        }
    }
}
