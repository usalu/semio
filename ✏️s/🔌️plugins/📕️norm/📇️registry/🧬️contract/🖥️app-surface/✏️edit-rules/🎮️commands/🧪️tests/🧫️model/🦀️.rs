//! 🧫️ A typed configuration boundary using the framework's real sparse diff and cold admission.

use semio_framework_value::{FromValue, ToValue};
use semio_os_kernel::{ColdMutation, MutationOutcome};
use super::super::NormEdit;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub struct Configuration { pub a: f64, pub b: f64, pub count: u64 }

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub struct Document { pub configuration: Configuration }

semio_os_kernel::sparse_record_diff! {
    record: Document,
    diff: DocumentDiff,
    fields: { configuration: Configuration }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum Operation { Replace(Configuration), Warning(Configuration), Error, Fatal }

thread_local! { static RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
pub fn reset_retired() { RETIRED.with(|count| count.set(0)); }
pub fn retired() -> usize { RETIRED.with(std::cell::Cell::get) }

const fn descriptor(kind: &'static str, name: &'static str, variant: &'static str, outcomes: &'static [semio_os_kernel::MutationOutcomeClass]) -> semio_os_kernel::MutationLeafDescriptor {
    use semio_os_kernel::*;
    MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🎮️commands/🧪️tests/🧫️model",
        semantic_kind: kind, display_name: name, emoji: "🧫️", aggregate_variant: variant,
        payload_schema: "🦀️.rs#Operation", text_opcode: None, binary_tag: None,
        invertibility: MutationInvertibility::ExplicitMutation,
        diff_participation: MutationDiffParticipation::ApplyOnly, outcome_classes: outcomes,
        composition: MutationComposition::Atomic, required_language_surfaces: &[MutationLanguageSurface::Rust],
    }
}

impl semio_os_kernel::Mutation<Document> for Operation {
    type Diff = DocumentDiff;
    const DESCRIPTORS: &'static [semio_os_kernel::MutationLeafDescriptor] = &[
        descriptor("replace-configuration", "Replace Configuration", "Replace", &[semio_os_kernel::MutationOutcomeClass::Applied]),
        descriptor("warn-configuration", "Warn Configuration", "Warning", &[semio_os_kernel::MutationOutcomeClass::Applied]),
        descriptor("refuse-configuration", "Refuse Configuration", "Error", &[semio_os_kernel::MutationOutcomeClass::Rejected]),
        descriptor("fail-configuration", "Fail Configuration", "Fatal", &[semio_os_kernel::MutationOutcomeClass::Rejected]),
    ];
    fn descriptor(&self) -> &'static semio_os_kernel::MutationLeafDescriptor {
        &Self::DESCRIPTORS[match self { Self::Replace(_) => 0, Self::Warning(_) => 1, Self::Error => 2, Self::Fatal => 3 }]
    }
    fn retire_cold(self) { RETIRED.with(|count| count.set(count.get() + 1)); }
}

impl ColdMutation<Document> for Operation {
    type ReferenceDiff = DocumentDiff;

    fn diff(&self, _: &Document) -> MutationOutcome<DocumentDiff> {
        match self {
            Self::Replace(configuration) => MutationOutcome::new(DocumentDiff { configuration: Some(configuration.clone()) }),
            Self::Warning(configuration) => MutationOutcome::new(DocumentDiff { configuration: Some(configuration.clone()) }).warning("test.warning", "Configuration admitted with a warning"),
            Self::Error => MutationOutcome::error("test.error", "Configuration refused", ["configuration"]),
            Self::Fatal => MutationOutcome::fatal("test.fatal", "Configuration failed", ["configuration"]),
        }
    }

    fn inverse(&self, base: &Document) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Replace(base.configuration.clone())])
    }
}

pub fn document() -> Document { Document { configuration: Configuration { a: 1.0, b: 2.0, count: 1 } } }

pub fn resolve(document: &Document, edit: &NormEdit) -> Result<Vec<Operation>, String> {
    let NormEdit::SetField { path, value } = edit else { return Err("Only field writes belong to this fixture".into()) };
    let mut next = document.configuration.clone();
    match path.as_str() {
        "configuration" => next = Configuration::from_value(value.clone()).map_err(|error| error.to_string())?,
        "configuration.a" => next.a = value.as_f64().ok_or("Expected a number")?,
        "configuration.b" => next.b = value.as_f64().ok_or("Expected a number")?,
        "configuration.count" => next.count = u64::from_value(value.clone()).map_err(|error| error.to_string())?,
        _ => return Err(format!("Unknown field '{path}'")),
    }
    Ok(vec![Operation::Replace(next)])
}

pub fn replay(document: &Document, operations: &[Operation]) -> Document {
    let mut scratch = semio_os_kernel::os_store::ledger::ColdReplayProjection::<Document, Operation>::new(document.clone());
    for (index, operation) in operations.iter().enumerate() {
        let (admitted, messages) = semio_os_kernel::os_store::replay::fold_operation_into(scratch.projection_mut(), operation, u32::try_from(index).expect("operation index"));
        assert!(admitted, "{messages:?}");
    }
    (*scratch).clone()
}
