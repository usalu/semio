//! 🧱️ Compliance core: quantities, clause identity, check results, national annex selection and design situations; the layer below the norm family, incremental evaluation and design cases that `semio_s_plugin_norm_compliance` re-exports.

extern crate semio_framework_value_derive as value_derive;

use semio_framework_dispatch_macros::dyn_enum;
use std::fmt;

#[path = "../📐️quantities-clauses-and-text/🦀️.rs"]
mod quantities_clauses_and_text;
pub use quantities_clauses_and_text::*;

#[path = "../✅️check-results/🦀️.rs"]
mod check_results;
pub use check_results::*;

#[path = "../📜️annex-and-tables/🦀️.rs"]
mod annex_and_tables;
pub use annex_and_tables::*;

#[path = "../🏗️design-situation-and-shared/🦀️.rs"]
mod design_situation_and_shared;
pub use design_situation_and_shared::*;

/// 🐍️ Repo-resolved Python interpreter helper for third-party oracle tests.
#[cfg(any(test, feature = "compliance-testing"))]
#[path = "../🧪️tests/🔬️python-oracle/🦀️.rs"]
pub mod python_oracle;
