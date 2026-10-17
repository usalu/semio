//! 🎬️ The bundled "reuse map" document of the gismap artifact, handcrafted in the `.gismap` DSL. It lives in its own crate so that the example and the text snapshot facet share one embedded copy instead of each logic crate carrying 243 KB.

/// 🗺️ The complete document text.
pub const DEMO_DSL: &str = include_str!("../../🗣️.dsl.semio");
