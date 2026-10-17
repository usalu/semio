//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod baseline {
                #[path = "../🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
