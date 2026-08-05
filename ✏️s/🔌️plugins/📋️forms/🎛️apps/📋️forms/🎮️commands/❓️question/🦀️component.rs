//! ❓️ Forms play app commands — question lifecycle (add / remove / patch / move / drop-from-catalogue).

use crate::apps::forms::config::{FormsConfig, FormsConfigOperation};
use crate::apps::forms::{parse_value_json, reset_try_config_operations};
use crate::artifacts::forms::engine::{create_form_id, dsl_to_value, forms_play_step_tree_id, locate_question, update_block_operation, value_to_dsl};
use crate::artifacts::forms::{op::FormOperation, FormQuestion, FormSpec, FormVectorField};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️Shell
/// 🌱️ A blank question of the given `kind`/`id` — every field defaulted to `None`.
pub fn question_shell(id: String, label: String, kind: String) -> FormQuestion {
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
pub fn default_question_for_kind(kind: &str, id: String) -> FormQuestion {
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
pub fn patch_question_field(spec: &FormSpec, question_id: &str, field: &str, raw_value: &Value) -> Option<FormOperation> {
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
pub fn patch_building_component_param(spec: &FormSpec, question_id: &str, param_key: &str, raw_value: &Value) -> Option<FormOperation> {
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

/// 🌳️ Resolves a document-tree drop target id (`"step:<id>"` or a question id) back to its owning step.
fn resolve_step_id_from_tree_target(spec: &FormSpec, target_id: &str) -> Option<String> {
    if let Some(step_id) = target_id.strip_prefix("step:") {
        return Some(step_id.to_string());
    }
    locate_question(spec, target_id).map(|location| location.step_id)
}

/// 🌳️ Resolves the insertion index within `step_id` implied by dropping onto `target_id` at
/// `drop_position` (`"before"`/`"after"`/`"inside"`).
fn resolve_question_insert_index(spec: &FormSpec, step_id: &str, target_id: &str, drop_position: &str) -> Option<usize> {
    let step = spec.steps.iter().find(|step| step.id == step_id)?;
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

//#region 🔖️AddQuestion
pub mod add_question {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-question")]
    pub struct AddQuestion {
        pub kind: String,
        pub step_id: Option<String>,
    }

    pub fn handle(payload: &AddQuestion, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let spec = doc.projection;
        let Some(step_id) = payload.step_id.clone().or_else(|| spec.steps.first().map(|step| step.id.clone())) else {
            return Ok(Emit::default());
        };
        let question = default_question_for_kind(&payload.kind, create_form_id("q"));
        let mut config_operations = reset_try_config_operations();
        config_operations.push(FormsConfigOperation::SetSelection { ids: vec![question.id.clone()] });
        Ok(Emit { document_operations: vec![FormOperation::AddBlock { step_id, block: question, index: None }], config_operations, ..Default::default() })
    }
}
//#endregion 🔖️AddQuestion

//#region 🔖️RemoveQuestion
pub mod remove_question {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-question")]
    pub struct RemoveQuestion {
        pub question_id: String,
    }

    pub fn handle(payload: &RemoveQuestion, doc: &DocumentView<'_, FormSpec>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let spec = doc.projection;
        let config = cfg.projection;
        let Some(location) = locate_question(spec, &payload.question_id) else {
            return Ok(Emit::default());
        };
        let mut config_operations = reset_try_config_operations();
        config_operations.push(FormsConfigOperation::SetSelection { ids: config.selected_ids.iter().filter(|id| **id != payload.question_id).cloned().collect() });
        Ok(Emit { document_operations: vec![FormOperation::RemoveBlock { step_id: location.step_id, block_id: payload.question_id.clone() }], config_operations, ..Default::default() })
    }
}
//#endregion 🔖️RemoveQuestion

//#region 🔖️PatchQuestions
pub mod patch_questions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-questions")]
    pub struct PatchQuestions {
        pub question_ids: Vec<String>,
        pub field: String,
        pub value_json: String,
        pub param_key: Option<String>,
    }

    pub fn handle(payload: &PatchQuestions, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let spec = doc.projection;
        let raw_value = parse_value_json(&payload.value_json);
        let operations: Vec<FormOperation> = if payload.field == "param" {
            let param_key = payload.param_key.as_deref().unwrap_or("");
            payload.question_ids.iter().filter_map(|question_id| patch_building_component_param(spec, question_id, param_key, &raw_value)).collect()
        } else {
            payload.question_ids.iter().filter_map(|question_id| patch_question_field(spec, question_id, &payload.field, &raw_value)).collect()
        };
        if operations.is_empty() {
            return Ok(Emit::config(reset_try_config_operations()));
        }
        Ok(Emit { document_operations: operations, config_operations: reset_try_config_operations(), coalesce_key: Some(format!("patch:{}:{}", payload.field, payload.question_ids.join(","))), ..Default::default() })
    }
}
//#endregion 🔖️PatchQuestions

//#region 🔖️MoveQuestion
pub mod move_question {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-question")]
    pub struct MoveQuestion {
        pub question_id: String,
        pub to_step_id: String,
        pub target_id: Option<String>,
        pub position: String,
        pub index: Option<u64>,
    }

    pub fn handle(payload: &MoveQuestion, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let spec = doc.projection;
        let Some(source) = locate_question(spec, &payload.question_id) else {
            return Ok(Emit::default());
        };
        let target_id = payload.target_id.as_deref().unwrap_or(&payload.question_id);
        let resolved_index = payload.index.map(|value| value as usize).unwrap_or_else(|| resolve_question_insert_index(spec, &payload.to_step_id, target_id, &payload.position).unwrap_or(0));
        Ok(Emit {
            document_operations: vec![FormOperation::MoveBlock { block_id: payload.question_id.clone(), from_step_id: source.step_id, to_step_id: payload.to_step_id.clone(), index: resolved_index }],
            config_operations: reset_try_config_operations(),
            ..Default::default()
        })
    }
}
//#endregion 🔖️MoveQuestion

//#region 🔖️DropQuestionKind
pub mod drop_question_kind {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "drop-question-kind")]
    pub struct DropQuestionKind {
        pub kind: String,
        pub target_id: String,
        pub drop_position: String,
    }

    pub fn handle(payload: &DropQuestionKind, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let spec = doc.projection;
        let Some(step_id) = resolve_step_id_from_tree_target(spec, &payload.target_id) else {
            return Ok(Emit::default());
        };
        let index = resolve_question_insert_index(spec, &step_id, &payload.target_id, &payload.drop_position);
        let question = default_question_for_kind(&payload.kind, create_form_id("q"));
        let mut config_operations = reset_try_config_operations();
        config_operations.push(FormsConfigOperation::SetSelection { ids: vec![question.id.clone()] });
        Ok(Emit { document_operations: vec![FormOperation::AddBlock { step_id, block: question, index }], config_operations, ..Default::default() })
    }
}
//#endregion 🔖️DropQuestionKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app};
    use crate::apps::forms::FormsCommand;
    use add_question::AddQuestion;
    use drop_question_kind::DropQuestionKind;
    use move_question::MoveQuestion;
    use patch_questions::PatchQuestions;
    use remove_question::RemoveQuestion;

    #[test]
    fn add_question_action_appends_question() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::AddQuestion(AddQuestion { kind: "text".into(), step_id: None }));
        assert!(crate::artifacts::forms::engine::flatten_questions(&app.projection().expect("projection")).iter().any(|(_, question)| question.kind == "text"));
    }

    #[test]
    fn add_question_undo_redo_round_trip() {
        let mut app = forms_app();
        let before = crate::artifacts::forms::engine::flatten_questions(&app.projection().expect("projection")).len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, FormsCommand::AddQuestion(AddQuestion { kind: "text".into(), step_id: None }), |app| crate::artifacts::forms::engine::flatten_questions(&app.projection().expect("projection")).len(), before, before + 1);
    }

    #[test]
    fn drop_question_kind_inserts_and_selects() {
        let mut app = forms_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        dispatch(&mut app, FormsCommand::DropQuestionKind(DropQuestionKind { kind: "slider".into(), target_id: crate::artifacts::forms::engine::forms_play_step_tree_id(&step_id), drop_position: "inside".into() }));
        let spec = app.projection().expect("projection");
        assert!(spec.steps[0].blocks.iter().any(|question| question.kind == "slider"));
        let blueprint = crate::apps::forms::testkit::render(&mut app, crate::apps::forms::FORMS_PLAY_BODY_BLUEPRINT);
        assert!(blueprint.contains(r#""selectedId":"#));
    }

    #[test]
    fn inspector_patch_updates_required() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::SetActiveExample(crate::apps::forms::commands::import::set_active_example::SetActiveExample { example_id: "default".into() }));
        let name_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        dispatch(&mut app, FormsCommand::PatchQuestions(PatchQuestions { question_ids: vec![name_id], field: "required".into(), value_json: "false".into(), param_key: None }));
        let spec = app.projection().expect("projection");
        assert!(!spec.steps[0].blocks[0].required.unwrap_or(true));
    }

    #[test]
    fn remove_question_clears_it_from_the_selection() {
        let mut app = forms_app();
        let question_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        dispatch(&mut app, FormsCommand::RemoveQuestion(RemoveQuestion { question_id: question_id.clone() }));
        assert!(crate::artifacts::forms::engine::flatten_questions(&app.projection().expect("projection")).iter().all(|(_, question)| question.id != question_id));
    }

    #[test]
    fn move_question_relocates_it_to_the_target_step() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::AddStep(crate::apps::forms::commands::step::add_step::AddStep {}));
        let spec = app.projection().expect("projection");
        let question_id = spec.steps[0].blocks[0].id.clone();
        let target_step_id = spec.steps.last().unwrap().id.clone();
        dispatch(&mut app, FormsCommand::MoveQuestion(MoveQuestion { question_id: question_id.clone(), to_step_id: target_step_id.clone(), target_id: None, position: "inside".into(), index: None }));
        let moved = app.projection().expect("projection");
        assert!(moved.steps.iter().find(|step| step.id == target_step_id).expect("target step").blocks.iter().any(|question| question.id == question_id));
    }
}
//#endregion 🧪️Tests
