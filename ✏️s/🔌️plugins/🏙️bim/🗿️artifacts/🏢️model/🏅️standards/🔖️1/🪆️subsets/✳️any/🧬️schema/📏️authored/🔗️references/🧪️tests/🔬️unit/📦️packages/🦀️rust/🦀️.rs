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
                    }
                    #[path = "."]
                    pub mod authored {
                        #[path = "."]
                        pub mod references {
                            #[cfg(test)]
                            #[path = "../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod sizes {
                            #[cfg(test)]
                            #[path = "../../../../../🪟️sizes/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod storeys {
                            #[cfg(test)]
                            #[path = "../../../../../🪜️storeys/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    
                }
            }
        }
    }
}
