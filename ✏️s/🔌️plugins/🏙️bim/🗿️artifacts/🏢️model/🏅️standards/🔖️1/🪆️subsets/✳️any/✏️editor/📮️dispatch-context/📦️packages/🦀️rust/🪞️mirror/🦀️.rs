//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.



#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod gestures {
            #[path = "../../../../🧵️gestures/🧩️area/🦀️.rs"]
            pub mod area;
            #[path = "../../../../🧵️gestures/🧭️session/🦀️.rs"]
            pub mod session;
            #[path = "../../../../🧵️gestures/🧲️snap/🦀️.rs"]
            pub mod snap;
        }
        #[path = "."]
        pub mod dispatch_context {
            #[path = "../../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod chrome {
            #[path = "../../../../🎛️chrome/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod transient {
            #[path = "../../../../🫧️transient/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod plan {
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🗺️plan/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod world {
                        #[path = "."]
                        pub mod envelope {
                            #[cfg(test)]
                            #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧊️world/🌡️envelope/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧊️world/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod section {
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📐️section/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod schedule {
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧮️schedule/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod family {
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧬️family/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod sheet {
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📄️sheet/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod options {
                #[path = "../../../../📌️panels/🧭️options/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod outliner {
                #[path = "../../../../📌️panels/🌳️outliner/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod library {
                #[path = "../../../../📌️panels/🛍️library/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod diagnostics {
                #[path = "../../../../📌️panels/🚨️diagnostics/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
