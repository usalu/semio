//! `upsert-silo-shell` — upsert a `SiloShell` by id into `silo_shells`.

use crate::{SiloShell, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateSiloShellInputs {
    pub silo_shell: SiloShell,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateSiloShellInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "siloShell", kind: "update-silo-shell-inputs", record: "UpdatedSiloShell" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert silo shell {}", self.silo_shell.id),
            &format!("Siloschale setzen {}", self.silo_shell.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.silo_shell.id.clone()]
    }
}
//#endregion 🔖️Payload
