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
                    #[path = "../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "."]
                            pub mod spatial {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏛️spatial/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod data {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧬️data/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod walls {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏰️walls/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod wall_sweeps {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod frame {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏗️frame/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod horizontal {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/⬜️horizontal/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod circulation {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪜️circulation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ramps {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🛝️ramps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod components {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod mep {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🌀️mep/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod grids {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/📏️grids/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod spaces {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏠️spaces/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod zoning {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod curtain {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪟️curtain/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod annotations {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪧️annotations/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ifc4 {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/📚️ifc4/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod projection {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🔬️projection/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "."]
                            pub mod scene {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🧊️gltf/🌳️scene/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "."]
                            pub mod spatial {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📥️import/🏗️ifc/🏛️spatial/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        pub mod cascade_restore;
                        #[path = "."]
                        pub mod split_wall {
                            #[cfg(test)]
                            #[path = "../../../../🦈️split-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod delete_curtain_wall {
                            #[cfg(test)]
                            #[path = "../../../../🦖️delete-curtain-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                    }
                }
            }
        }
    }
}
