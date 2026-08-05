//! 📦️ Architect program artifact — the operation-diff carrier (constitutional: diff).

use crate::artifacts::program::op::{apply_plugin_operation, ProgramOperation};
use crate::artifacts::program::Program;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

/// @emoji 📦️ Ordered list of program operations materializing a document diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDiff {
    pub operations: Vec<ProgramOperation>,
}

impl OperationDiff<Program> for ProgramDiff {
    fn apply(&self, projection: &Program) -> Program {
        let mut next = projection.clone();
        for operation in &self.operations {
            apply_plugin_operation(&mut next, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.operations.extend(other.operations);
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::EntityId;
    use crate::artifacts::program::sample_plugin;
    use protocol::CollectionOperation;

    #[test]
    fn a_diff_applies_its_operations_in_order() {
        let program = sample_plugin();
        let element_id = program.elements[0].header.id.clone();
        let diff = ProgramDiff { operations: vec![ProgramOperation::Elements(CollectionOperation::Remove { id: element_id.clone() })] };
        let next = diff.apply(&program);
        assert!(!next.elements.iter().any(|row| row.header.id == element_id));
    }

    #[test]
    fn absorb_concatenates_operation_lists() {
        let mut left = ProgramDiff { operations: vec![ProgramOperation::ClearAdjacency { id: EntityId("a".into()) }] };
        left.absorb(ProgramDiff { operations: vec![ProgramOperation::ClearAdjacency { id: EntityId("b".into()) }] });
        assert_eq!(left.operations.len(), 2);
    }
}
//#endregion 🧪️Tests
