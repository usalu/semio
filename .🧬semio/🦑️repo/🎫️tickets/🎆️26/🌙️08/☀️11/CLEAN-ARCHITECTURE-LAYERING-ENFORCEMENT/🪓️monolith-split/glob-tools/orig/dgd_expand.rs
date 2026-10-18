//! 🧩️ Expansion library of the DSL derives: the implementation owner `../🦀️.rs` compiled as an ordinary library, so its units are testable; the proc-macro entry points live in `../📦️packages/🦀️rust/🦀️.rs`.

extern crate proc_macro;

use semio_framework_native_build_compiler_resources as compiler_resources;

#[path = "../🦀️.rs"]
mod component;
pub use component::{expand_derive_composite_mutation, expand_derive_mutations, expand_diff_binary, expand_diff_text, expand_dsl_document, expand_mutation_leaf};

/// 🪝️ Test hook of the companion crate `semio-framework-os-kernel-dsl-derive-tests`: the private expansion units, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod component {
        pub use crate::component::*;
    }
}
