//! 📋️ Forms editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/📝️blueprint/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view
//! state in `🦀️config.rs`, shared document compute in the artifact's `🧬️schema`, this app's own IO
//! surface in the `🔖️Io` region below. This file is a routing table:
//! `handle` → `FormsCommand::dispatch`, `render` → body-key → node, plus the app-level `🔖️Contributions`
//! region — host-declared plugin contributions (`config.contributions_json`) are consumed by THREE
//! taxonomy nodes (the blueprint builder's palette, the try wizard, and the inspection panel), and every
//! helper in that region takes `&FormsConfig` (an app-only view-state type) or an app-level
//! `ProgramContributionEntry`, so per the DocumentHelpers placement rule they stay here rather than in the
//! artifact's `🧬️schema`.

pub use semio_s_artifact_forms_forms_questions::editor::forms::questions;

use crate::editor::forms::commands::{
    add_question, add_question_option, add_step, add_vector_field, drop_question_kind, export_snapshot, move_question, move_step, next_step, patch_question_options, patch_questions, patch_step, patch_vector_field, previous_step, remove_question,
    remove_question_option, remove_step, remove_vector_field, reset_try, set_active_example, set_contributions, set_spec_json, set_try_value, set_try_value_step, set_try_values, submit, update_form,
};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::modes::{blueprint, fill, responses};
use crate::editor::forms::commands::{export_responses, discard_response};
use crate::editor::forms::modes::blueprint::windows::{builder, try_wizard as try_window};
use crate::editor::forms::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::forms::terminology::{forms_play_labels, FormsLabels};
use crate::editor::forms::support::{self, FORMS_INTERACTION_FIELDS, FORMS_INTERACTION_GRANULARITY_FIELD, FORMS_INTERACTION_GRANULARITY_SECTION};
use crate::op::FormMutation;
use crate::{forms_steps, FormQuestion, FormsSnapshot, FORMS_DOCUMENT_SCHEMA, FORM_BUILTIN_KINDS};
#[cfg(test)] use semio_framework_pack_json::{object};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::IconName;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants
pub use builder::FORMS_PLAY_BODY_BLUEPRINT;
pub use catalogue_panel::FORMS_PLAY_BODY_CATALOGUE;
pub use document_panel::FORMS_PLAY_BODY_ARTIFACT;
pub use inspection_panel::FORMS_PLAY_BODY_INSPECTION;
pub use try_window::FORMS_PLAY_BODY_TRY;

//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🌳️ `fields` domain topology from the document's own step/question nesting — step ids are the SAME
/// row-id-prefixed ids the document panel tree renders (`forms_play_step_tree_id`), question ids are
/// raw question ids (matching both the document panel tree's item ids and every question-editing
/// command's own id vocabulary), so `validate_state` prunes deleted steps/questions and range/
/// transitive selection walk the real document structure.
fn forms_fields_topology(spec: &FormsSnapshot) -> DomainTopology {
    let mut ordered = Vec::new();
    for step in forms_steps(spec) {
        let step_id = crate::schema::forms_play_step_tree_id(&step.id);
        ordered.push(TopologyNode { id: step_id.clone(), granularity: FORMS_INTERACTION_GRANULARITY_SECTION.into(), parent: None });
        for question in step.blocks {
            ordered.push(TopologyNode { id: question.id, granularity: FORMS_INTERACTION_GRANULARITY_FIELD.into(), parent: Some(step_id.clone()) });
        }
    }
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction


pub use semio_framework::ProgramContributionEntry;

//#region 🔖️ActionBridge

/// 🌉️ `{action, args}` → `FormsCommand`. Keys arrive camelCase from the shells (`exampleId`,
/// `windowKindId`), the block-list host emits its own generic verbs (`addBlock`/`removeBlock`/
/// `moveBlock` over `blockId`/`toStepId`), and a `Trigger::Change` control's live value is merged
/// under `value` by `merge_ui_values` — the `*_json` string fields the payloads carry are printed from
/// that value here when no explicit `valueJson`/`valuesJson` text was sent.
#[path = "🌉️args-bridge/🦀️.rs"]
mod args_bridge;

//#endregion 🔖️ActionBridge

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `FormsPlayApp::Command` — the SOLE dispatch surface for forms' own behavior, assembled from the
    /// `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`, the
    /// camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — they are genuinely different vocabularies, and
    /// appending is safe, reordering is a wire-format break.**
    pub enum FormsCommand for FormsSnapshot, FormMutation, FormsConfig, FormsConfigMutation {
        "setTryValue" as "try-value" => set_try_value::SetTryValue,
        "setTryValues" as "try-values" => set_try_values::SetTryValues,
        "resetTry" as "reset-try" => reset_try::ResetTry,
        "previousStep" as "previous-step" => previous_step::PreviousStep,
        "nextStep" as "next-step" => next_step::NextStep,
        "submit" as "submit" => submit::Submit,
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "addStep" as "add-step" => add_step::AddStep,
        "patchStep" as "patch-step" => patch_step::PatchStep,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "moveStep" as "move-step" => move_step::MoveStep,
        "updateForm" as "update-form" => update_form::UpdateForm,
        "addBlock" as "add-question" => add_question::AddQuestion,
        "removeBlock" as "remove-question" => remove_question::RemoveQuestion,
        "patchQuestions" as "patch-questions" => patch_questions::PatchQuestions,
        // 🩹️ `patchQuestionOptions`..`removeVectorField` (rows 18-23) come BEFORE `moveQuestion`/
        // `dropQuestionKind` (rows 24-25) here, even though all 5 live in the same `🎮️commands/*` files —
        // this row order is the pre-migration `forms_protocol::FormsCommand`'s exact binary variant
        // ordinal and must not be re-sorted by file grouping (see the doc comment above: reordering is a
        // wire-format break no round-trip test catches, since every row still round-trips fine on its own).
        "patchQuestionOptions" as "patch-question-options" => patch_question_options::PatchQuestionOptions,
        "addQuestionOption" as "add-question-option" => add_question_option::AddQuestionOption,
        "removeQuestionOption" as "remove-question-option" => remove_question_option::RemoveQuestionOption,
        "patchVectorField" as "patch-vector-field" => patch_vector_field::PatchVectorField,
        "addVectorField" as "add-vector-field" => add_vector_field::AddVectorField,
        "removeVectorField" as "remove-vector-field" => remove_vector_field::RemoveVectorField,
        "moveBlock" as "move-question" => move_question::MoveQuestion,
        "dropQuestionKind" as "drop-question-kind" => drop_question_kind::DropQuestionKind,
        "setSpecJson" as "spec-json" => set_spec_json::SetSpecJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "exportSnapshot" as "export-snapshot" => export_snapshot::ExportSnapshot,
        "setTryValueStep" as "try-value-step" => set_try_value_step::SetTryValueStep,
        "exportResponses" as "export-responses" => export_responses::ExportResponses,
        "discardResponse" as "discard-response" => discard_response::DiscardResponse,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.

//#endregion 🔖️Commands

//#region 🔖️Io
/// 🔌️ Forms' typed media I/O surface (`AppDefinition.io`) — the implicit `document:in`/`document:out`
/// pair (keyed by the `forms.form` document schema) plus the WORKFLOWS-END-TO-END-TYPED-PORTS
/// `dictionary:out` port: the form's currently-configured default field values (see
/// `crate::standards::v1::subsets::any::io::text::snapshot::initial_try_values`), re-exported as a typed `form.dictionary` intrinsic
/// object keyed by question id — the layout app's `fields:in` counterpart. Relocated from the deleted
/// artifact `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is the app's
/// own IO surface, not artifact behaviour.
pub fn forms_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: FORMS_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "dictionary:out".into(),
            label: "Dictionary".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("form.dictionary".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "form.dictionary".into(), name: "Form".into(), dimension: "data".into(), component_kind: "forms".into() },
    }
}
//#endregion 🔖️Io

//#region 🧵️RetainedCommands

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub use retained_commands::*;

//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use store_preparation::*;

//#endregion 📬️StorePreparation

//#region 🔖️FormsPlayApp

#[path = "🕹️play-app/🦀️.rs"]
mod play_app;
pub use play_app::*;

//#endregion 🔖️FormsPlayApp

//#region 🔖️Manifest

#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use manifest::*;

//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit_tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️field-transactions/🦀️.rs"]
mod field_transaction_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts


//#endregion 🪢️TaxonomyMounts
