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
                    
                    #[path = "../../../../🚪️io/📥️import/🦀️.rs"]
                    pub mod import;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧪️tests/🔭️schema4/🦀️.rs"]
                            pub mod schema4_tests;
                            #[path = "."]
                            pub mod energy {
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/📤️export/🏗️ifc/🔥️energy/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod entities {
            #[path = "../../../../✏️editor/🧩️entities/🧱️row-types/🦀️.rs"]
            pub mod row_types;
            #[path = "../../../../✏️editor/🧩️entities/🕰️phasing/🦀️.rs"]
            pub mod phasing;
            #[path = "../../../../✏️editor/🧩️entities/🏗️frame/🦀️.rs"]
            pub mod frame;
            #[path = "../../../../✏️editor/🧩️entities/➕️create-rows/🦀️.rs"]
            pub mod create_rows;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/✏️edit/🦀️.rs"]
                        pub mod edit;
                        #[path = "."]
                        pub mod vocabulary {
                            #[cfg(test)]
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/🏷️vocabulary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod plan {
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/👁️view/🪟️windows/🗺️plan/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod world {
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/👁️view/🪟️windows/🧊️world/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}

