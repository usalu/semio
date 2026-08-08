//! 📦️ Architect program artifact — the operation-diff carrier (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::program::mutations::{apply_program_mutation, ProgramMutation};
use crate::artifacts::program::Program;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

/// @emoji 📦️ Ordered list of program mutations materializing a document diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDiff {
    pub mutations: Vec<ProgramMutation>,
}

impl MutationDiff<Program> for ProgramDiff {
    fn apply(&self, projection: &Program) -> Program {
        let mut next = projection.clone();
        for operation in &self.mutations {
            apply_program_mutation(&mut next, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.mutations.extend(other.mutations);
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::EntityId;
    use crate::artifacts::program::sample_plugin;
    use protocol::CollectionMutation;

    #[test]
    fn a_diff_applies_its_operations_in_order() {
        let program = sample_plugin();
        let element_id = program.elements[0].header.id.clone();
        let diff = ProgramDiff { mutations: vec![ProgramMutation::Elements(CollectionMutation::Remove { id: element_id.clone() })] };
        let next = diff.apply(&program);
        assert!(!next.elements.iter().any(|row| row.header.id == element_id));
    }

    #[test]
    fn absorb_concatenates_operation_lists() {
        let mut left = ProgramDiff { mutations: vec![ProgramMutation::ClearAdjacency { id: EntityId("a".into()) }] };
        left.absorb(ProgramDiff { mutations: vec![ProgramMutation::ClearAdjacency { id: EntityId("b".into()) }] });
        assert_eq!(left.mutations.len(), 2);
    }
}
//#endregion 🧪️Tests
