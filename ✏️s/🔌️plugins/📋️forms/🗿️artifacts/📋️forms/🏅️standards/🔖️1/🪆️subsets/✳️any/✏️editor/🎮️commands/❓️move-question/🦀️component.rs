//! ❓️ ❓️ Forms play app commands command — `move-question`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use crate::artifacts::forms::schema::{locate_question, update_block_operation, value_to_dsl};
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormQuestion, FormsSnapshot, FormVectorField};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️Shell
/// 🌱️ A blank question of the given `kind`/`id` — every field defaulted to `None`.
pub async fn question_shell(id: String, label: String, kind: String) -> FormQuestion {
    FormQuestion {
        id,
        label,
        kind,
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

/// 🌱️ A freshly created question, seeded with sensible per-kind defaults — shared by `addQuestion` and
/// `dropQuestionKind`.
pub async fn default_question_for_kind(kind: &str, id: String) -> FormQuestion {
    match kind {
        "text" => {
            let mut question = question_shell(id, "Text".into(), "text".into());
            question.placeholder = Some("Enter text".into());
            question
        }
        "longText" => {
            let mut question = question_shell(id, "Long Text".into(), "longText".into());
            question.placeholder = Some("Enter long text".into());
            question
        }
        "number" => {
            let mut question = question_shell(id, "Number".into(), "number".into());
            question.default = Some(value_to_dsl(&json!(0)));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "slider" => {
            let mut question = question_shell(id, "Slider".into(), "slider".into());
            question.default = Some(value_to_dsl(&json!(50)));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "boolean" => {
            let mut question = question_shell(id, "Boolean".into(), "boolean".into());
            question.default = Some(value_to_dsl(&json!(false)));
            question
        }
        "single" | "multi" => {
            let mut question = question_shell(id, if kind == "single" { "Single Select" } else { "Multi Select" }.into(), kind.into());
            question.default = if kind == "multi" { Some(value_to_dsl(&json!([]))) } else { None };
            question.options = Some(vec![crate::artifacts::forms::FormQuestionOption { value: "a".into(), label: "Option A".into() }, crate::artifacts::forms::FormQuestionOption { value: "b".into(), label: "Option B".into() }]);
            question
        }
        "note" => {
            let mut question = question_shell(id, "Note".into(), "note".into());
            question.text = Some("Informational note".into());
            question
        }
        "date" => {
            let mut question = question_shell(id, "Date".into(), "date".into());
            question.default = Some(value_to_dsl(&json!("2026-01-01")));
            question
        }
        "color" => {
            let mut question = question_shell(id, "Color".into(), "color".into());
            question.default = Some(value_to_dsl(&json!("#336699")));
            question
        }
        "image" => question_shell(id, "Image".into(), "image".into()),
        "file" => {
            let mut question = question_shell(id, "File".into(), "file".into());
            question.accept = Some(".pdf".into());
            question
        }
        "vector" => {
            let mut question = question_shell(id, "Vector".into(), "vector".into());
            question.schema = Some("vec3".into());
            question.step = Some(0.1);
            question.fields = Some(vec![
                FormVectorField { key: "x".into(), label: Some("X".into()), value: Some(0.0) },
                FormVectorField { key: "y".into(), label: Some("Y".into()), value: Some(0.0) },
                FormVectorField { key: "z".into(), label: Some("Z".into()), value: Some(0.0) },
            ]);
            question
        }
        "buildingComponent" => {
            let mut question = question_shell(id, "Building Component".into(), "buildingComponent".into());
            question.fixture_slug = Some("hexagonal-mushroom-column".into());
            question.params = Some(value_to_dsl(&json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })));
            question
        }
        _ => question_shell(id, kind.into(), kind.into()),
    }
}

/// ✏️ Patches one scalar field of a question by name — every field `PatchQuestions` can address except
/// `"param"` (routed to [`patch_building_component_param`] instead, since it targets a nested params map).
pub async fn patch_question_field(spec: &FormsSnapshot, question_id: &str, field: &str, raw_value: &Value) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| match field {
        "label" => question.label = raw_value.as_str().unwrap_or("").to_string(),
        "kind" => question.kind = raw_value.as_str().unwrap_or("text").to_string(),
        "description" => question.description = raw_value.as_str().map(str::to_string),
        "placeholder" => question.placeholder = raw_value.as_str().map(str::to_string),
        "required" => question.required = Some(raw_value.as_bool().unwrap_or(false)),
        "text" => question.text = raw_value.as_str().map(str::to_string),
        "default" => question.default = Some(value_to_dsl(raw_value)),
        "min" => question.min = raw_value.as_f64(),
        "max" => question.max = raw_value.as_f64(),
        "step" => question.step = raw_value.as_f64(),
        "unit" => question.unit = raw_value.as_str().map(str::to_string),
        "schema" => question.schema = raw_value.as_str().map(str::to_string),
        "src" => question.src = raw_value.as_str().map(str::to_string),
        "accept" => question.accept = raw_value.as_str().map(str::to_string),
        "fixtureSlug" => question.fixture_slug = raw_value.as_str().map(str::to_string),
        _ => {}
    })
}

/// ✏️ Patches one key of a `buildingComponent` question's nested params object.
pub async fn patch_building_component_param(spec: &FormsSnapshot, question_id: &str, param_key: &str, raw_value: &Value) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut params = question.params.take().unwrap_or(dsl::DslValue::Object(vec![]));
        if let dsl::DslValue::Object(entries) = &mut params {
            let value = value_to_dsl(raw_value);
            if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == param_key) {
                *slot = value;
            } else {
                entries.push((param_key.to_string(), value));
            }
        }
        question.params = Some(params);
    })
}

/// 🌳️ Resolves the insertion index within `step_id` implied by dropping onto `target_id` at
/// `drop_position` (`"before"`/`"after"`/`"inside"`).
async fn resolve_question_insert_index(spec: &FormsSnapshot, step_id: &str, target_id: &str, drop_position: &str) -> Option<usize> {
    let steps = forms_steps(spec);
    let step = steps.iter().find(|step| step.id == step_id)?;
    if target_id.starts_with("step:") {
        return Some(if drop_position == "before" { 0 } else { step.blocks.len() });
    }
    let target_index = step.blocks.iter().position(|question| question.id == target_id)?;
    Some(match drop_position {
        "before" => target_index,
        "after" => target_index + 1,
        _ => step.blocks.len(),
    })
}
//#endregion 🔖️Shell






#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-question")]
pub struct MoveQuestion {
    pub question_id: String,
    pub to_step_id: String,
    pub target_id: Option<String>,
    pub position: String,
    pub index: Option<u64>,
}

pub async fn handle(payload: &MoveQuestion, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let Some(source) = locate_question(spec, &payload.question_id) else {
        return Ok(Emit::default());
    };
    let target_id = payload.target_id.as_deref().unwrap_or(&payload.question_id);
    let resolved_index = payload.index.map_or_else(|| resolve_question_insert_index(spec, &payload.to_step_id, target_id, &payload.position).unwrap_or(0), |value| value as usize);
    Ok(Emit {
        artifact_mutations: vec![FormMutation::MoveBlockToStep(crate::artifacts::forms::mutations::move_block_to_step::mutation::MoveBlockToStep {
            step_id: source.step_id,
            block_id: payload.question_id.clone(),
            to_step_id: payload.to_step_id.clone(),
            index: resolved_index,
        })],
        config_mutations: reset_try_config_mutations(),
        ..Default::default()
    })
}
