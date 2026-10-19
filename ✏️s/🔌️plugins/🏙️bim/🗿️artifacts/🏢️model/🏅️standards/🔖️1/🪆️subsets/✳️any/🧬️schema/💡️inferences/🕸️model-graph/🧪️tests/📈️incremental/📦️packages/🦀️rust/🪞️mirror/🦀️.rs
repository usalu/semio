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
                    pub mod export {
                        #[path = "."]
                        pub mod sheets {
                            #[path = "../../../../../../../../🚪️io/📤️export/📄️sheets/✒️ink/🦀️.rs"]
                            pub mod ink;
                            #[path = "../../../../../../../../🚪️io/📤️export/📄️sheets/📊️tables/🦀️.rs"]
                            pub mod tables;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "."]
                            pub mod path {
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🎨️svg/✒️path/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod style {
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🎨️svg/🎚️style/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../../../../../../../🚪️io/📤️export/🎨️svg/🖍️drawing/🦀️.rs"]
                            pub mod drawing;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod storey_levels {
                            #[cfg(test)]
                            #[path = "../../../../../../🏢️storey-levels/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod model_graph {
                            #[path = "."]
                            pub mod dirty {
                                #[cfg(test)]
                                #[path = "../../../../../🎯️dirty/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod session {
                                #[cfg(test)]
                                #[path = "../../../../../📡️session/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod incremental_tests;
                        }
                        #[path = "."]
                        pub mod wall_layout {
                            #[cfg(test)]
                            #[path = "../../../../../../🧱️wall-layout/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod attach {
                                #[cfg(test)]
                                #[path = "../../../../../../🧱️wall-layout/🧲️attach/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod curtain_layout {
                            #[cfg(test)]
                            #[path = "../../../../../../🪞️curtain-layout/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod opening_frames {
                            #[cfg(test)]
                            #[path = "../../../../../../🪟️opening-frames/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod sheet_layout {
                            #[path = "."]
                            pub mod windows {
                                #[cfg(test)]
                                #[path = "../../../../../../📄️sheet-layout/🖼️windows/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod title_block {
                                #[cfg(test)]
                                #[path = "../../../../../../📄️sheet-layout/🧾️title-block/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        
                    }
                }
            }
        }
    }
}
