//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod baseline {
                #[path = "../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
