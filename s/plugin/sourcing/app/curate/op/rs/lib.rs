//! ⚡ Sourcing curate app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use sourcing::CurateDocument;

//#region 🔖Operations
/// 🛒 Curate document operation: currently always a wholesale swap — every action recomputes the
/// full document and this carries it, with a true inverse restoring the exact prior document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SourcingOperation {
    SetDocument {
        #[dsl(block)]
        document: CurateDocument,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcingDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<CurateDocument>,
}

impl OperationDiff<CurateDocument> for SourcingDiff {
    fn apply(&self, projection: &CurateDocument) -> CurateDocument {
        self.document.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
        }
    }
}

impl Operation<CurateDocument> for SourcingOperation {
    type Diff = SourcingDiff;

    fn diff(&self, _projection: &CurateDocument) -> Self::Diff {
        match self {
            SourcingOperation::SetDocument { document } => SourcingDiff { document: Some(document.clone()) },
        }
    }

    fn backwards(&self, projection: &CurateDocument) -> Vec<Self> {
        match self {
            SourcingOperation::SetDocument { .. } => vec![SourcingOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖Operations

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🌱 Mirrors `sourcing_engine`'s private test-only helper — a small, self-contained fixture
    /// assembly (not business logic), duplicated here rather than shared, per the constitutional
    /// split's "op does not depend on engine in production" boundary (only as a dev-dependency).
    fn sample_document() -> CurateDocument {
        CurateDocument { stock: sourcing_engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_text_binary_equivalence(&SourcingOperation::SetDocument { document: sample_document() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingOperation::SetDocument { document: CurateDocument::default() });
    }
}
//#endregion 🧪Tests
