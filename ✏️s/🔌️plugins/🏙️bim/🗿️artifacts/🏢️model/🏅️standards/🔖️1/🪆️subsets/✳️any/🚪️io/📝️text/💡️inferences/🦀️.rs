//! 📖️ Inference text grammar: inference values are computed from a snapshot and never authored as DSL text, so this facet declares
//! the grammar only.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak.
pub type ModelInferenceText = String;

pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::text::inferences::plan_linework;
pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::io::text::inferences::stair_runs;
pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::io::text::inferences::quantities;
pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::io::text::inferences::spaces;
pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::io::text::inferences::diagnostics;
pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::io::text::inferences::element_solids;
pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::text::inferences::ramp_runs;
pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::text::inferences::components;
pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::text::inferences::view_linework;
pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::io::text::inferences::schedules;
