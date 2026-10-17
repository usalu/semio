//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ecma_376 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod strict {
                #[path = "../🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod transitional {
                #[path = "../🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
