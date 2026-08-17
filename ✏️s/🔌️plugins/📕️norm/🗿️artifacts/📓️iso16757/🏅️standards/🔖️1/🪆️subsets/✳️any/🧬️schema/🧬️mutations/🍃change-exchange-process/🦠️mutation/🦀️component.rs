//! 🔄️ `change-exchange-process` — sets the ISO 16757 exchange-process stage scalar.

use crate::artifacts::iso16757::{part_5::ExchangeProcess, Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeExchangeProcess {
    pub new_exchange_process: ExchangeProcess,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for ChangeExchangeProcess {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "exchange-process", kind: "change-exchange-process", record: "ChangedExchangeProcess" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change exchange process to {:?}", self.new_exchange_process)
    }
}
//#endregion 🔖️Payload
