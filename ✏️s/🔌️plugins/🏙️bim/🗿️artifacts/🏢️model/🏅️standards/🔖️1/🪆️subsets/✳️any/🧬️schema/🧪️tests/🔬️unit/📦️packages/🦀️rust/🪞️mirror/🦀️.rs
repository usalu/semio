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
                    pub mod binary {
                        #[path = "../../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "../../../../../../🚪️io/📤️export/🧊️gltf/🦀️.rs"]
                        pub mod gltf;
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod model_graph {
                            #[path = "."]
                            pub mod instance {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🕸️model-graph/🧳️instance/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod stair_runs {
                            #[cfg(test)]
                            #[path = "../../../../../💡️inferences/🪜️stair-runs/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod phase_visibility {
                            #[cfg(test)]
                            #[path = "../../../../../💡️inferences/🎭️phase-visibility/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod tests;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod cascade_restore {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🔙️cascade-restore/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod component_rules {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🔌️component-rules/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod create_column {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🏛️create-column/🧪️tests/📌️authored-top-is-stored/🦀️.rs"]
                            pub mod tests_authored_top_is_stored;
                        }
                        #[path = "."]
                        pub mod create_beam {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/➖️create-beam/🧪️tests/📌️authored-top-offset-is-stored/🦀️.rs"]
                            pub mod tests_authored_top_offset_is_stored;
                        }
                        #[path = "."]
                        pub mod set_site {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🗺️set-site/🧪️tests/🪜️raises-the-absolute-levels/🦀️.rs"]
                            pub mod tests_raises_the_absolute_levels;
                        }
                        #[path = "."]
                        pub mod set_building {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🏗️set-building/🧪️tests/🪜️raises-the-absolute-levels/🦀️.rs"]
                            pub mod tests_raises_the_absolute_levels;
                        }
                        #[path = "."]
                        pub mod set_wall_axis {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/〰️set-wall-axis/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod set_wall_base_offset {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🔽️set-wall-base-offset/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod set_wall_type_of {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🥞️set-wall-type-of/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod set_wall_location {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🦚️set-wall-location/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod flip_wall {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🪞️flip-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                    }
                }
            }
        }
    }
}
