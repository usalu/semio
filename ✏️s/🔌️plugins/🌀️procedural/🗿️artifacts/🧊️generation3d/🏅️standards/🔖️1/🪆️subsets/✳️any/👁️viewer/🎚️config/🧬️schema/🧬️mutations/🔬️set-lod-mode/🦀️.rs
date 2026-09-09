//! 🔬️ Sets the tessellation level of detail (`""`/`coarse`/`fine`) the read-only preview meshes at.

use super::{Generation3dViewConfig, Generation3dViewConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "lod-mode")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<Generation3dViewConfig, Generation3dViewConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &Generation3dViewConfig) -> protocol::MutationOutcome<Generation3dViewConfig> {
        let mut next = base.clone();
        next.lod_mode.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dViewConfig) -> Vec<Generation3dViewConfigMutation> {
        vec![Self { value: base.lod_mode.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Lod Mode".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["lodMode".into()]
    }
}
