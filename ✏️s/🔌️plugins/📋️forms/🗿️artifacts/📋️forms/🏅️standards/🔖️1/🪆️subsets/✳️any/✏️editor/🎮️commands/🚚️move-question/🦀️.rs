//! ❓️ ❓️ Forms play app commands command — `move-question`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::schema::{locate_question, forms_play_step_tree_id};
use crate::editor::forms::questions::placement::question_insert_index;
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "move-question")]
pub struct MoveQuestion {
    pub question_id: String,
    pub to_step_id: String,
    pub target_id: Option<String>,
    pub position: String,
    pub index: Option<u64>,
}

pub fn handle(payload: &MoveQuestion, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let source = locate_question(spec, &payload.question_id).ok_or_else(|| Fault::from("forms.question.missing"))?;
    let destination = spec.definition.steps.iter().find(|step| step.id == payload.to_step_id).ok_or_else(|| Fault::from("forms.question.missing-step"))?;
    let target = payload.target_id.clone().unwrap_or_else(|| forms_play_step_tree_id(&payload.to_step_id));
    let resolved_index = match payload.index {
        Some(index) => index.min(destination.blocks.len().saturating_sub(usize::from(source.step_id == payload.to_step_id)) as u64) as usize,
        None => question_insert_index(&spec.definition, &payload.to_step_id, &target, &payload.position, Some(&payload.question_id)).map_err(|error| Fault::from(format!("forms.question.{error}")))?,
    };
    Ok(Emit {
        artifact_mutations: vec![FormMutation::MoveBlockToStep(crate::mutations::move_block_to_step::mutation::MoveBlockToStep {
            step_id: source.step_id,
            block_id: payload.question_id.clone(),
            to_step_id: payload.to_step_id.clone(),
            index: resolved_index,
        })],
        ..Default::default()
    })
}
