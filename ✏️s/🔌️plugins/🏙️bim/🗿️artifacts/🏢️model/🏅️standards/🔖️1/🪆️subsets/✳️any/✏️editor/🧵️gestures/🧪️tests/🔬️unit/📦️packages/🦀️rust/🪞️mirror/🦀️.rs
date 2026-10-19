//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.


#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod gestures {
            #[path = "."]
            pub mod annotate {
                #[cfg(test)]
                #[path = "../../../../../📏️annotate/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod area {
                #[cfg(test)]
                #[path = "../../../../../🧩️area/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod chain {
                #[cfg(test)]
                #[path = "../../../../../🧱️chain/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod frame {
                #[cfg(test)]
                #[path = "../../../../../🏗️frame/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod opening {
                #[cfg(test)]
                #[path = "../../../../../🪟️opening/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod point {
                #[cfg(test)]
                #[path = "../../../../../📍️point/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod select {
                #[cfg(test)]
                #[path = "../../../../../🎯️select/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod session {
                #[cfg(test)]
                #[path = "../../../../../🧭️session/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod snap {
                #[cfg(test)]
                #[path = "../../../../../🧲️snap/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod sweep {
                #[cfg(test)]
                #[path = "../../../../../🧷️sweep/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod transform {
                #[cfg(test)]
                #[path = "../../../../../🚚️transform/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod duplicate {
                #[cfg(test)]
                #[path = "../../../../../👯️duplicate/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod reshape {
                #[cfg(test)]
                #[path = "../../../../../✂️reshape/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod typed {
                #[cfg(test)]
                #[path = "../../../../../⌨️typed/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod place {
                #[cfg(test)]
                #[path = "../../../../../🪑️place/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod route {
                #[cfg(test)]
                #[path = "../../../../../🌀️route/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[cfg(test)]
            #[path = "../../../🦀️.rs"]
            pub mod tests;
            #[cfg(test)]
            #[path = "../../../../🧷️app/🦀️.rs"]
            pub mod app_tests;
        }
        #[path = "."]
        pub mod chrome {
            #[cfg(test)]
            #[path = "../../../../../../🎛️chrome/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod properties {
                #[path = "."]
                pub mod overrides {
                    #[cfg(test)]
                    #[path = "../../../../../../📌️panels/🔍️properties/🎚️overrides/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod remove_property {
                #[cfg(test)]
                #[path = "../../../../../../🎮️commands/🗃️remove-property/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
