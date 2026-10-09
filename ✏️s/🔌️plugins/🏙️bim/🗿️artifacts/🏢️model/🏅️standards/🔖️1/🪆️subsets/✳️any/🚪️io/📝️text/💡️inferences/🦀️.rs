//! 📖️ Inference text grammar: inference values are computed from a snapshot and never authored as DSL text, so this facet declares
//! the grammar only.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak.
pub type ModelInferenceText = String;

#[path = "🗺️plan-linework/🦀️.rs"]
pub mod plan_linework;
#[path = "🪜️stair-runs/🦀️.rs"]
pub mod stair_runs;
#[path = "🧮️quantities/🦀️.rs"]
pub mod quantities;
#[path = "🏠️spaces/🦀️.rs"]
pub mod spaces;
#[path = "⚠️diagnostics/🦀️.rs"]
pub mod diagnostics;
#[path = "🧊️element-solids/🦀️.rs"]
pub mod element_solids;
#[path = "🛝️ramp-runs/🦀️.rs"]
pub mod ramp_runs;
#[path = "🖼️view-linework/🦀️.rs"]
pub mod view_linework;
#[path = "📋️schedules/🦀️.rs"]
pub mod schedules;
