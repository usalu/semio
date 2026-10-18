//! 🔬️ tests-2 of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

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
                    pub mod diff {
                        #[path = "."]
                        pub mod patches {
                            #[path = "."]
                            pub mod building_elements {
                                #[path = "../../../../../../🔺️diff/🩹️patches/🧱️building-elements/🏚️walls-and-curtain-walls/🦀️.rs"]
                                pub mod walls_and_curtain_walls;
                                #[path = "../../../../../../🔺️diff/🩹️patches/🧱️building-elements/🏛️columns-beams-slabs/🦀️.rs"]
                                pub mod columns_beams_slabs;
                                #[path = "../../../../../../🔺️diff/🩹️patches/🧱️building-elements/🔲️ceilings-roofs-openings/🦀️.rs"]
                                pub mod ceilings_roofs_openings;
                            }
                            #[path = "../../../../../../🔺️diff/🩹️patches/📝️documentation/🦀️.rs"]
                            pub mod documentation;
                            #[path = "../../../../../../🔺️diff/🩹️patches/📚️library/🦀️.rs"]
                            pub mod library;
                            #[path = "../../../../../../🔺️diff/🩹️patches/🏢️site-and-spaces/🦀️.rs"]
                            pub mod site_and_spaces;
                        }
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra;
                    }
                    #[path = "."]
                    pub mod authored {
                        #[path = "."]
                        pub mod references {
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references::*;
                            #[cfg(test)]
                            #[path = "../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod sizes {
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::sizes::*;
                            #[cfg(test)]
                            #[path = "../../../../../🪟️sizes/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod storeys {
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::storeys::*;
                            #[cfg(test)]
                            #[path = "../../../../../🪜️storeys/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values;
                    }
                }
            }
        }
    }
}
