//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.


#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod gestures {
            #[path = "../../../../../🧵️gestures/🏗️frame/🦀️.rs"]
            pub mod frame;
            #[path = "../../../../../🧵️gestures/🫧️overlay/🦀️.rs"]
            pub mod overlay;
            #[path = "../../../../../🧵️gestures/🎯️select/🦀️.rs"]
            pub mod select;
            #[path = "../../../../../🧵️gestures/🔪️split/🦀️.rs"]
            pub mod split;
            #[path = "../../../../../🧵️gestures/👯️duplicate/🦀️.rs"]
            pub mod duplicate;
            #[path = "../../../../../🧵️gestures/✂️reshape/🦀️.rs"]
            pub mod reshape;
            #[path = "../../../../../🧵️gestures/🪑️place/🦀️.rs"]
            pub mod place;
            #[path = "."]
            pub mod viewports {
                #[cfg(test)]
                #[path = "../../../../../🧵️gestures/🖼️viewports/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        
        
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod move_storey {
                #[path = "../../../🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod split_wall {
                #[path = "../../../../✂️split-wall/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_property {
                #[path = "../../../../🧾️set-property/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod remove_property {
                #[path = "../../../../🗃️remove-property/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod select_findings {
                #[path = "../../../../🎯️select-findings/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod coordinate {
                #[path = "../../../../🤝️coordinate/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_classification {
                #[path = "../../../../🗂️set-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod remove_classification {
                #[path = "../../../../🗄️remove-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod apply_template {
                #[path = "../../../../🧰️apply-template/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod conditions {
                #[path = "../../../../🌡️conditions/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_template {
                #[path = "../../../../🧮️edit-template/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_classification {
                #[path = "../../../../📚️edit-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod search_classification {
                #[path = "../../../../🔍️search-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod place_elements {
                #[path = "../../../../📍️place-elements/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod place_grid_columns {
                #[path = "../../../../🏛️place-grid-columns/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod browse_families {
                #[path = "../../../../🪑️browse-families/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_override {
                #[path = "../../../../🎚️set-override/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
