//! ⚡️ GIS artifact — OpText/OpBinary codecs + grammar (wire codecs in 📡️spr).

pub use crate::artifacts::gismap::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
