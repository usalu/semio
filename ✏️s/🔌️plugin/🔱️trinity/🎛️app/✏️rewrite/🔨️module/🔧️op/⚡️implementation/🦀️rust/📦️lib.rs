//! ⚡️ Trinity Rewrite app — operation enum + laws (constitutional: op).

use rewrite::{RewriteRuleState, TrinityRewriteError, REWRITE_RULE_SCHEMA};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use store::{create_document_envelope, DocumentCommand, DocumentEnvelope, DocumentStore};

//#region 🔖️Types
/// 🔁️ Whole-state snapshot diff: the rule document is one small unit, so history stores full pre/post states rather than field-level patches.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewriteRuleDiff {
    pub next: Option<RewriteRuleState>,
}

impl OperationDiff<RewriteRuleState> for RewriteRuleDiff {
    fn apply(&self, projection: &RewriteRuleState) -> RewriteRuleState {
        self.next.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.next.is_some() {
            self.next = other.next;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RewriteRuleOperation {
    SetState { state: RewriteRuleState },
}

impl Operation<RewriteRuleState> for RewriteRuleOperation {
    type Diff = RewriteRuleDiff;

    fn diff(&self, _projection: &RewriteRuleState) -> Self::Diff {
        match self {
            RewriteRuleOperation::SetState { state } => RewriteRuleDiff { next: Some(state.clone()) },
        }
    }

    fn backwards(&self, projection: &RewriteRuleState) -> Vec<Self> {
        vec![RewriteRuleOperation::SetState { state: projection.clone() }]
    }
}

pub type RewriteRuleEnvelope = DocumentEnvelope<RewriteRuleState, RewriteRuleOperation>;
pub type RewriteRuleStore = DocumentStore<RewriteRuleState, RewriteRuleOperation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewriteRuleState) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}

pub fn dispatch_rewrite_rule_state(store: &mut RewriteRuleStore, state: RewriteRuleState) -> Result<(), TrinityRewriteError> {
    let current = store.projection()?;
    if current == state {
        return Ok(());
    }
    store.dispatch(DocumentCommand::Apply { operations: vec![RewriteRuleOperation::SetState { state }], description: None }).map_err(TrinityRewriteError::from)
}
//#endregion 🔖️Types
