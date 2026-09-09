use super::{WriterMainWindowConfig, WriterMainWindowConfigMutation};
use crate::WriterCamera;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: WriterCamera,
}

impl protocol::MutationKind<WriterMainWindowConfig, WriterMainWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &WriterMainWindowConfig) -> protocol::MutationOutcome<WriterMainWindowConfig> {
        let mut next = base.clone();
        next.camera.clone_from(&self.camera);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &WriterMainWindowConfig) -> Vec<WriterMainWindowConfigMutation> {
        vec![Self { camera: base.camera.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Writer Window Camera".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
