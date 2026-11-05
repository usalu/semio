//! 🧱️ App-surface foundation: ids and chrome labels, value paths, field metadata, report-tree helpers, document editors and the store ownership macros every higher app-surface crate builds on. Lowest layer of the split app-surface crates; the facade `semio-s-plugin-norm-app-surface` re-exports it.

#![allow(async_fn_in_trait, unused_imports)]

include!("../📋️prelude/🦀️.rs");

#[path = "../🛡️store-ownership-macros/🦀️.rs"]
mod store_ownership_macros;

#[path = "../🏷️ids-chrome-and-labels/🦀️.rs"]
mod ids_chrome_and_labels;
pub use ids_chrome_and_labels::*;

#[path = "../🧭️value-path/🦀️.rs"]
mod value_path;
pub use value_path::*;

#[path = "../🔖️field-meta/🦀️.rs"]
mod field_meta;
pub use field_meta::*;

#[path = "../🌳️report-tree/🦀️.rs"]
mod report_tree;
pub use report_tree::*;

#[path = "../📝️document-editors/🦀️.rs"]
mod document_editors;
pub use document_editors::*;

#[path = "../🌳️report-tree/✅️feedback/🦀️.rs"]
mod field_feedback;
pub use field_feedback::*;
