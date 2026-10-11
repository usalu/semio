//! 🧬️ Sets which generation the generate mode's form and preview are bound to.

use super::{Generation3dConfigPatch, Generation3dSelectedGenerationChange, Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "selected-generation")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSelectedGeneration {
    pub selected_generation_id: Option<String>,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetSelectedGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "selected-generation", kind: "set-selected-generation", record: "SetSelectedGeneration" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfigPatch> {
        protocol::MutationOutcome::new(Generation3dConfigPatch { selected_generation_id: Some(Generation3dSelectedGenerationChange { id: self.selected_generation_id.clone() }), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dConfig) -> Result<Vec<Generation3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { selected_generation_id: base.selected_generation_id.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Selected Generation", "Ausgewählte Erzeugung setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["selectedGenerationId".into()]
    }
}
