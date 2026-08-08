//! ⚡️ Flow artifact — Op facet re-exports `FlowMutation`.
pub use crate::artifacts::flow::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::FlowFixture;
    use protocol::MutationDiff;

    #[test]
    fn set_layout_inverse_restores_base() {
        let base = FlowFixture::default();
        let mutation = FlowMutation::SetLayout { entries: Vec::new() };
        let forward = mutation.diff(&base).apply(&base);
        let restored = mutation.inverse(&base).iter().fold(forward, |projection, inverse| inverse.diff(&projection).apply(&projection));
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
