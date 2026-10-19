//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                
                
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[cfg(test)]
    #[path = "../../../🪆️subsets/✳️any/📚️examples/🧰️checks/🦀️.rs"]
    pub mod checks;
    #[path = "."]
    pub mod house {
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/📚️examples/🏡️house/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod office {
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/📚️examples/🏢️office/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod demo {
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        
        #[path = "."]
        pub mod entities {
            #[path = "."]
            pub mod energy {
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🧩️entities/🌡️energy/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod mep {
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🧩️entities/🌀️mep/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod components {
                #[cfg(any(test, feature = "testkit"))]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🧩️entities/🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod unit_tests;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/⌨️completeness/🦀️.rs"]
        pub mod completeness_tests;
        
        
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod select_findings {
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️select-findings/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}

