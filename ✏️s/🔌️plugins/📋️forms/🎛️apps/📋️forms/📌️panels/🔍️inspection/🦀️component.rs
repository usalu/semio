//! 🔍️ Forms play app panel — the inspector: per-question fields for the current blueprint selection.

use crate::apps::forms::config::FormsConfig;
use crate::apps::forms::terminology::FormsLabels;
use crate::apps::forms::{catalogue_kinds, forms_action, parse_contributions, render_extension_question, ProgramContributionEntry};
use crate::artifacts::forms::schema::{dsl_f64_value, dsl_string_value, is_extension_question_kind, locate_question};
use crate::artifacts::forms::{FormQuestion, FormsSnapshot};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_text, ActionDescriptor, Label, LocalizedLabel, PanelGroup,
    PanelTabDefinition, PanelTabKind, UiButtonNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiNumberStepperNode, UiPresence, UiSectionNode, UiSelectItem, UiSelectNode, UiToggleNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::json;

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_INSPECTION: &str = "forms.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(FORMS_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn inspector_patch(question_ids: &[String], field: &str) -> ActionDescriptor {
    forms_action("patchQuestions", Some(json!({ "questionIds": question_ids, "field": field })))
}

fn inspector_text_field(question_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: None,
            on_change: inspector_patch(question_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn inspector_number_field(question_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
            id: format!("{field_id}.stepper"),
            value: mixed.value,
            step: 0.1,
            uniform: mixed.uniform,
            on_absolute: inspector_patch(question_ids, field),
            on_delta: inspector_patch(question_ids, field),
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn question_kind_editor_fields(question: &FormQuestion, question_ids: &[String], contributions: &[ProgramContributionEntry], id_prefix: &str, labels: &FormsLabels) -> Vec<UiNode> {
    let fid = |suffix: &str| format!("{id_prefix}.{suffix}");
    let mut fields = Vec::new();
    fields.push(inspector_text_field(question_ids, &fid("description"), labels.description, &[question.description.clone().unwrap_or_default()], "description"));
    match question.kind.as_str() {
        "text" | "longText" => {
            fields.push(inspector_text_field(question_ids, &fid("placeholder"), labels.placeholder, &[question.placeholder.clone().unwrap_or_default()], "placeholder"));
            fields.push(inspector_text_field(question_ids, &fid("default"), labels.default, &[question.default.as_ref().map(dsl_string_value).unwrap_or_default()], "default"));
        }
        "number" | "slider" => {
            fields.push(inspector_number_field(question_ids, &fid("min"), labels.min, &[question.min.unwrap_or(0.0)], "min"));
            fields.push(inspector_number_field(question_ids, &fid("max"), labels.max, &[question.max.unwrap_or(100.0)], "max"));
            fields.push(inspector_number_field(question_ids, &fid("step"), labels.step_field, &[question.step.unwrap_or(1.0)], "step"));
            fields.push(inspector_number_field(question_ids, &fid("default"), labels.default, &[question.default.as_ref().map_or(0.0, dsl_f64_value)], "default"));
            if question.kind == "slider" {
                fields.push(inspector_text_field(question_ids, &fid("unit"), labels.unit, &[question.unit.clone().unwrap_or_default()], "unit"));
            }
        }
        "boolean" => {
            let pressed = question.default.as_ref().and_then(|default| crate::artifacts::forms::schema::dsl_to_value(default).as_bool()).unwrap_or(false);
            fields.push(UiNode::Field(UiFieldNode {
                id: fid("default"),
                label: labels.default.into(),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: fid("default.toggle"),
                    icon_id: "check".into(),
                    text: Some(if pressed { labels.yes.into() } else { labels.no.into() }),
                    on_change: inspector_patch(question_ids, "default"),
                    presence: UiPresence::selected(pressed),
                    menu: None,
                })),
                menu: None,
            }));
        }
        "single" | "multi" => {
            if let Some(options) = &question.options {
                for option in options {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("option.{}", option.value)),
                        label: Label::data(format!("{} {}", labels.option.as_str(), option.value)),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: fid(&format!("option.{}.input", option.value)),
                            input_kind: "text".into(),
                            value: option.label.clone(),
                            placeholder: None,
                            commit: None,
                            on_change: forms_action("patchQuestionOptions", Some(json!({ "questionIds": question_ids, "optionValue": option.value, "field": "label" }))),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(fid(&format!("option.{}.remove", option.value))),
                        icon_id: "trash-2".into(),
                        label: labels.remove_option.into(),
                        action: forms_action("removeQuestionOption", Some(json!({ "questionId": question.id, "optionValue": option.value }))),
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some(fid("option.add")),
                icon_id: "plus".into(),
                label: labels.add_option.into(),
                action: forms_action("addQuestionOption", Some(json!({ "questionId": question.id, "label": "New option" }))),
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        "date" | "color" => {
            fields.push(inspector_text_field(question_ids, &fid("default"), labels.default, &[question.default.as_ref().map(dsl_string_value).unwrap_or_default()], "default"));
        }
        "vector" => {
            fields.push(inspector_text_field(question_ids, &fid("schema"), labels.schema, &[question.schema.clone().unwrap_or_default()], "schema"));
            fields.push(inspector_number_field(question_ids, &fid("step"), labels.step_field, &[question.step.unwrap_or(0.1)], "step"));
            if let Some(vector_fields) = &question.fields {
                for field in vector_fields {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("vector.{}.label", field.key)),
                        label: Label::data(format!("{} {}", field.key, labels.vector_field_label_suffix.as_str())),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: fid(&format!("vector.{}.label.input", field.key)),
                            input_kind: "text".into(),
                            value: field.label.clone().unwrap_or_else(|| field.key.clone()),
                            placeholder: None,
                            commit: None,
                            on_change: forms_action("patchVectorField", Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "label" }))),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("vector.{}.value", field.key)),
                        label: Label::data(format!("{} {}", field.key, labels.vector_field_value_suffix.as_str())),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
                            id: fid(&format!("vector.{}.value.stepper", field.key)),
                            value: field.value.unwrap_or(0.0),
                            step: question.step.unwrap_or(0.1),
                            uniform: true,
                            on_absolute: forms_action("patchVectorField", Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "value" }))),
                            on_delta: forms_action("patchVectorField", Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "value" }))),
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(fid(&format!("vector.{}.remove", field.key))),
                        icon_id: "trash-2".into(),
                        label: Label::data(format!("{} {}", labels.remove.as_str(), field.key)),
                        action: forms_action("removeVectorField", Some(json!({ "questionId": question.id, "fieldKey": field.key }))),
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some(fid("vector.add")),
                icon_id: "plus".into(),
                label: labels.add_vector_field.into(),
                action: forms_action("addVectorField", Some(json!({ "questionId": question.id, "fieldKey": "field" }))),
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        "note" => {
            fields.push(inspector_text_field(question_ids, &fid("text"), labels.text, &[question.text.clone().unwrap_or_default()], "text"));
        }
        "image" => {
            fields.push(inspector_text_field(question_ids, &fid("src"), labels.src, &[question.src.clone().unwrap_or_default()], "src"));
        }
        "file" => {
            fields.push(inspector_text_field(question_ids, &fid("accept"), labels.accept, &[question.accept.clone().unwrap_or_default()], "accept"));
        }
        kind if is_extension_question_kind(kind) => {
            let values = serde_json::Map::new();
            fields.push(render_extension_question(question, &values, contributions, "blueprint", true));
            if let Some(slug) = &question.fixture_slug {
                fields.push(ui_inspector_readonly_field(fid("fixtureSlug"), labels.fixture_slug, slug));
            }
        }
        _ => {}
    }
    fields
}

pub fn render(spec: &FormsSnapshot, config: &FormsConfig, term_labels: &FormsLabels) -> UiNode {
    let contributions = parse_contributions(config);
    let questions: Vec<FormQuestion> = config.selected_ids.iter().filter_map(|id| locate_question(spec, id).map(|location| location.question)).collect();
    if questions.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "forms-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![
                ui_text(Label::data(format!("Schema: {}", crate::artifacts::forms::FORMS_DOCUMENT_SCHEMA))),
                ui_text(Label::data(format!("Steps: {}", spec.steps.len()))),
                ui_text(Label::data(format!("Questions: {}", crate::artifacts::forms::schema::flatten_questions(spec).len()))),
            ],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let question_ids: Vec<String> = questions.iter().map(|question| question.id.clone()).collect();
    let labels: Vec<String> = questions.iter().map(|question| question.label.clone()).collect();
    let kinds: Vec<String> = questions.iter().map(|question| question.kind.clone()).collect();
    let required: Vec<bool> = questions.iter().map(|question| question.required.unwrap_or(false)).collect();
    let kind_mixed = ui_inspector_mixed_text(&kinds);
    let required_mixed = ui_inspector_mixed_toggle(&required);
    let kind_items: Vec<UiSelectItem> = catalogue_kinds(&contributions, term_labels).into_iter().map(|(kind, label, _)| UiSelectItem { value: kind, label: Label::data(label) }).collect();
    let mut base_fields = vec![
        inspector_text_field(&question_ids, "forms-play-inspector.label", term_labels.label, &labels, "label"),
        UiNode::Field(UiFieldNode {
            id: "forms-play-inspector.kind".into(),
            label: term_labels.kind.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                id: "forms-play-inspector.kind.select".into(),
                value: kind_mixed.value,
                placeholder: kind_mixed.placeholder.map(Label::data),
                items: kind_items,
                on_change: inspector_patch(&question_ids, "kind"),
                presence: UiPresence::default(),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        ui_inspector_readonly_field("forms-play-inspector.id", term_labels.id, if question_ids.len() == 1 { question_ids[0].clone() } else { format!("{} {}", question_ids.len(), term_labels.selected.as_str()) }),
        UiNode::Field(UiFieldNode {
            id: "forms-play-inspector.required".into(),
            label: term_labels.required.into(),
            child: Box::new(UiNode::Toggle(UiToggleNode {
                id: "forms-play-inspector.required.toggle".into(),
                icon_id: "check".into(),
                text: if required_mixed.uniform { Some(if required_mixed.pressed { term_labels.yes.into() } else { term_labels.no.into() }) } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
                on_change: inspector_patch(&question_ids, "required"),
                presence: UiPresence::selected(required_mixed.uniform && required_mixed.pressed),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
    ];
    if questions.len() == 1 {
        base_fields.extend(question_kind_editor_fields(&questions[0], &question_ids, &contributions, "forms-play-inspector", term_labels));
    }
    let groups = vec![UiInspectorFieldGroup { presence: UiPresence::default(), id: "forms-play-inspector.base".into(), label: term_labels.question.into(), default_open: None, fields: base_fields }];
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{building_component_contributions, building_component_question, dispatch, forms_app, render as render_body};
    use crate::apps::forms::commands::selection::set_selection::SetSelection;
    use crate::apps::forms::FORMS_PLAY_BODY_INSPECTION as BODY_INSPECTION;
    use crate::apps::forms::FormsCommand;

    #[test]
    fn kind_editor_fields_are_editable_when_unset() {
        let question = crate::apps::forms::commands::question::question_shell("q-num".into(), "Amount".into(), "number".into());
        let fields = question_kind_editor_fields(&question, &["q-num".into()], &[], "forms-blueprint.q-num", crate::apps::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&fields).unwrap();
        assert!(json.contains("forms-blueprint.q-num.min"));
        assert!(json.contains("forms-blueprint.q-num.max"));
        assert!(json.contains("forms-blueprint.q-num.default"));
        assert!(json.contains("forms-blueprint.q-num.description"));
    }

    #[test]
    fn extension_question_kind_editor_shows_the_fixture_slug_readonly_field() {
        let node = question_kind_editor_fields(&building_component_question(), &["geometry".into()], &building_component_contributions(), "forms-play-inspector", crate::apps::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("fixtureSlug"));
    }

    #[test]
    fn empty_selection_shows_the_document_summary() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_INSPECTION);
        assert!(json.contains("forms-play-inspector.empty"));
    }

    #[test]
    fn a_single_selected_question_exposes_its_kind_editor_fields() {
        let mut app = forms_app();
        let first_question_id = app.snapshot().expect("projection").steps[0].blocks[0].id.clone();
        dispatch(&mut app, FormsCommand::SetSelection(SetSelection { ids: vec![first_question_id] }));
        let json = render_body(&mut app, BODY_INSPECTION);
        assert!(json.contains("forms-play-inspector.label"));
    }
}
//#endregion 🧪️Tests
