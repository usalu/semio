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

/// 📥️ Run-time readers of the committed test fixtures, with paths relative to the artifact folder: the oracle tests read them here instead of embedding them with `include_bytes!` or `include_str!`, so the test closure stays inside the source gate.
/// @see <https://doc.rust-lang.org/std/macro.include_bytes.html>
#[cfg(test)]
pub(crate) mod runtime_fixture {
    fn path(relative: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(relative)
    }

    pub(crate) fn bytes(relative: &str) -> Vec<u8> {
        let path = path(relative);
        std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
    }
}
