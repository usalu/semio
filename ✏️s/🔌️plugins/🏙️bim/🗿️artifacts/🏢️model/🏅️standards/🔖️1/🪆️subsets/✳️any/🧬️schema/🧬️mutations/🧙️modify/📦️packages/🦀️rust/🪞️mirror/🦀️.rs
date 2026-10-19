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
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod svg {
                            #[path = "."]
                            pub mod codec {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🎨️svg/🧱️codec/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path;
                            #[path = "."]
                            pub mod sheet {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🎨️svg/📐️sheet/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod drawing {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🎨️svg/🖍️drawing/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod energy {
                            #[path = "../../../../../../🚪️io/📤️export/🔋️energy/🏗️build/🦀️.rs"]
                            pub mod build;
                            #[path = "../../../../../../🚪️io/📤️export/🔋️energy/📊️table/🦀️.rs"]
                            pub mod table;
                        }
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧬️data/🦀️.rs"]
                            pub mod data;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏗️frame/🦀️.rs"]
                            pub mod frame;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/⬜️horizontal/🦀️.rs"]
                            pub mod horizontal;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🔲️ceilings/🦀️.rs"]
                            pub mod ceilings;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪜️circulation/🦀️.rs"]
                            pub mod circulation;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🛝️ramps/🦀️.rs"]
                            pub mod ramps;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🦴️structure/🦀️.rs"]
                            pub mod structure;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod modify {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod render {
    #[path = "."]
    pub mod plan {
        #[cfg(test)]
        #[path = "../../../../../../🖌️render/🗺️plan/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "../../../../../../🖌️render/🌡️envelope/🦀️.rs"]
    pub mod envelope;
    #[path = "../../../../../../🖌️render/🦴️structure/🦀️.rs"]
    pub mod structure;
}
