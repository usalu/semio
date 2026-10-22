//! 🎥️ SetCamera changes only the addressed configuration field.

use crate::editor::equation::modes::edit::windows::graph::config::mutations::{EquationCamera, EquationGraphWindowConfig, EquationGraphWindowConfigDiff};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: EquationCamera,
}

impl<Op> protocol::MutationKind<EquationGraphWindowConfig, Op> for SetCamera
where
    Op: protocol::Mutation<EquationGraphWindowConfig, Diff = EquationGraphWindowConfigDiff> + From<SetCamera>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}

impl<Op> protocol::ColdMutationKind<EquationGraphWindowConfig, Op> for SetCamera
where
    Op: protocol::ColdMutation<EquationGraphWindowConfig, ReferenceDiff = EquationGraphWindowConfigDiff> + From<SetCamera>,
{
    fn diff(&self, base: &EquationGraphWindowConfig) -> protocol::MutationOutcome<EquationGraphWindowConfigDiff> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(EquationGraphWindowConfigDiff::default()).warning("mutation.no-op", "Configuration field is unchanged.");
        }
        protocol::MutationOutcome::new(EquationGraphWindowConfigDiff { camera: Some(self.camera.clone()) })
    }
    fn inverse(&self, base: &EquationGraphWindowConfig) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if base.camera == self.camera {
            Vec::new()
        } else {
            vec![Op::from(SetCamera { camera: base.camera.clone() })]
        }
    
    })())
}
}
