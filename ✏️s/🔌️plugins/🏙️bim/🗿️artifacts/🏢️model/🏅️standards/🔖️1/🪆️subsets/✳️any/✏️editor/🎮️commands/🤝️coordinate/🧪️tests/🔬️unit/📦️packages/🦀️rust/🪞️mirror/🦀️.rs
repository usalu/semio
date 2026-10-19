//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.


#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        
        
        
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod coordinate {
                #[cfg(test)]
                #[path = "../../../🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_classification {
                #[cfg(test)]
                #[path = "../../../../../../🗂️set-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod remove_classification {
                #[cfg(test)]
                #[path = "../../../../../../🗄️remove-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod apply_template {
                #[cfg(test)]
                #[path = "../../../../../../🧰️apply-template/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod conditions {
                #[cfg(test)]
                #[path = "../../../../../../🌡️conditions/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_template {
                #[cfg(test)]
                #[path = "../../../../../../🧮️edit-template/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod edit_classification {
                #[cfg(test)]
                #[path = "../../../../../../📚️edit-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod search_classification {
                #[cfg(test)]
                #[path = "../../../../../../🔍️search-classification/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod cursor_keys {
                #[cfg(test)]
                #[path = "../../../../../../🧭️cursor-keys/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod place_elements {
                #[cfg(test)]
                #[path = "../../../../../../📍️place-elements/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod place_grid_columns {
                #[cfg(test)]
                #[path = "../../../../../../🏛️place-grid-columns/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod engagement_input {
                #[cfg(test)]
                #[path = "../../../../../../⌨️engagement-input/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod browse_families {
                #[cfg(test)]
                #[path = "../../../../../../🪑️browse-families/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_override {
                #[cfg(test)]
                #[path = "../../../../../../🎚️set-override/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod gesture_keys {
                #[cfg(test)]
                #[path = "../../../../../../🔑️gesture-keys/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod engagement_submit {
                #[cfg(test)]
                #[path = "../../../../../../📨️engagement-submit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
