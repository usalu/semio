//! 🔬️ Sets the tessellation level of detail (`""`/`coarse`/`fine`) the editor previews mesh at.

use super::{Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "lod-mode")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        let mut next = base.clone();
        next.lod_mode.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Generation3dConfigMutation> {
        vec![Self { value: base.lod_mode.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Lod Mode".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["lodMode".into()]
    }
}
