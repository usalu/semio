//! 🏭️ Product artifact envelopes, OS diff codecs, and source-authorized mutation derives.

use proc_macro::TokenStream;
use quote::quote;
use std::{collections::{BTreeMap, HashSet}, fs, path::{Component, Path, PathBuf}};
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[path = "🔖️mutation-source-authority/🦀️.rs"]
mod source_authority;
pub use source_authority::*;

#[path = "🧾️mutation-leaf-descriptor/🦀️.rs"]
mod mutation_leaf_descriptor;
pub use mutation_leaf_descriptor::*;

#[path = "🪪️mutation-leaf-attrs-and-schema-index/🦀️.rs"]
mod mutation_leaf_attrs_and_schema_index;
pub use mutation_leaf_attrs_and_schema_index::*;

#[path = "🏗️document-diff-and-composite-expansion/🦀️.rs"]
mod document_diff_and_composite_expansion;
pub use document_diff_and_composite_expansion::*;

#[path = "🔁️mutations-expansion/🦀️.rs"]
mod mutations_expansion;
pub use mutations_expansion::*;
