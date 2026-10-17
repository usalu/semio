//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod a {
                #[path = "../🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod x {
                #[path = "../🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod a {
                #[path = "../🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod e {
                #[path = "../🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod h {
                #[path = "../🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod ua {
                #[path = "../🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod vt {
                #[path = "../🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod x {
                #[path = "../🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🔮️oracles/🦀️.rs"]
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

    pub(crate) fn text(relative: &str) -> String {
        let path = path(relative);
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
    }
}
