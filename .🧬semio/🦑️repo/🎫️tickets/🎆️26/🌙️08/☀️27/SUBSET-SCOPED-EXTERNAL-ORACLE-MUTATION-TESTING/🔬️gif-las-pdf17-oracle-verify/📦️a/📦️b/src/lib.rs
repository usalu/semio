//! 🔬️ Scratch harness: links the REAL, already-committed oracle modules for `gif@89a/any`,
//! `las@1.0/any` and `pdf@1.7/any` by path, exactly as the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` crate does, so their existing `#[cfg(test)]`
//! suites can be compiled and run without the unrelated pdf@1.4 breakage blocking the whole shared
//! crate. See this crate's `Cargo.toml` header for why it exists; not a permanent part of the repo.
//!
//! Nested two extra directories deep (`📦️a/📦️b`) purely so `pdf_1_7_any`'s own test module's
//! `env!("CARGO_MANIFEST_DIR")`-relative fixture path resolves to a symlink kept INSIDE this ticket
//! folder (`../../../🗿️artifacts/…`) instead of escaping it — no other reason for the extra nesting.

#[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs"]
pub mod document;

#[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🖼️raster/🦀️component.rs"]
pub mod raster;

#[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
pub mod gif_89a_any;

#[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs"]
pub mod las_1_0_any;

#[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧪️oracle/🦀️component.rs"]
pub mod pdf_1_7_any;
