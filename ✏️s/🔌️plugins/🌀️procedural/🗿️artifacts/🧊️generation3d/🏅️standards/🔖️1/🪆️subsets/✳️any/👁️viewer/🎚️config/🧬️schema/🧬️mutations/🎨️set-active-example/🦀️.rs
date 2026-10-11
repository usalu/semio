//! 🎨️ Names what this read-only surface is looking at.
//!
//! 📚️ A viewer opens a document, it never rewrites one: the picked example lives HERE, on the
//! surface's own config lane, and `Generation3dViewer`'s viewed-document resolution reads it. That
//! is the whole difference from the sibling surface's `setActiveExample`, which replaces the
//! artifact's fixture through the document lane (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
//!
//! 🕳️ `None` and `Some("")` are DIFFERENT states, which is the whole reason the leaf carries an
//! option: nothing picked yet shows the opened document, while the picker's own `No example` row
//! shows no example at all.

use super::{Generation3dViewConfigPatch, Generation3dActiveExampleChange, Generation3dViewConfig, Generation3dViewConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "active-example")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetActiveExample {
    pub value: Option<String>,
}

impl protocol::MutationKind<Generation3dViewConfig, Generation3dViewConfigMutation> for SetActiveExample {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "active-example", kind: "set-active-example", record: "SetActiveExample" };

    fn diff(&self, base: &Generation3dViewConfig) -> protocol::MutationOutcome<Generation3dViewConfigPatch> {
        protocol::MutationOutcome::new(Generation3dViewConfigPatch { active_example_id: Some(Generation3dActiveExampleChange { id: self.value.clone() }), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dViewConfig) -> Result<Vec<Generation3dViewConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.active_example_id.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Active Example", "Aktives Beispiel setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["activeExampleId".into()]
    }
}
