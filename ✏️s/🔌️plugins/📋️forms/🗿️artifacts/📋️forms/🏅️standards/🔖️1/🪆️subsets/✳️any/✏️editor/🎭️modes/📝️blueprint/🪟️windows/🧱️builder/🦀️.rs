//! 🧱️ Forms play app — the blueprint window: the drag/drop playbook builder authoring the form.

use crate::editor::forms::config::FormsConfig;
use crate::FormsSnapshot;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;

//#region 🔖️Constants
pub const FORMS_PLAY_WINDOW_BLUEPRINT: &str = "forms-blueprint";
pub const FORMS_PLAY_BODY_BLUEPRINT: &str = "forms.play.blueprint";
const FORMS_PLAY_SURFACE_BLUEPRINT: &str = "forms.play.blueprint";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: FORMS_PLAY_WINDOW_BLUEPRINT.into(),
        label: LocalizedLabel::native("Design", "Entwurf"),
        body_key: FORMS_PLAY_BODY_BLUEPRINT.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "clipboard-list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_forms_app`'s `.window_kind_interactions(..)` call.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ Projects the framework-owned selection into the blueprint's selected card.
pub fn render(spec: &FormsSnapshot, config: &FormsConfig, view: &semio_framework_plugin::ViewModel, selected: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let contributions = crate::editor::forms::parse_contributions(config);
    let palette = crate::editor::forms::catalogue_kinds(&contributions, view).into_iter()
        .map(|(kind,label,icon)| semio_framework_plugin::BlockListPaletteEntry {block_kind:kind,label,icon_id:icon.as_str().to_owned()}).collect();
    let steps = spec.definition.steps.iter().map(|step| semio_framework_plugin::BlockListStep {
        id:step.id.clone(),title:step.title.clone(),description:step.description.clone(),
        target:Some(semio_framework_plugin::BlockListSelectionTarget {granularity:crate::editor::forms::FORMS_INTERACTION_GRANULARITY_SECTION.into(),id:crate::schema::forms_play_step_tree_id(&step.id)}),
        blocks:step.blocks.iter().map(|question| semio_framework_plugin::BlockListBlock {
            id:question.id.clone(),label:question.label.clone(),kind:question.kind.clone(),description:question.description.clone(),
            target:Some(semio_framework_plugin::BlockListSelectionTarget {granularity:crate::editor::forms::FORMS_INTERACTION_GRANULARITY_FIELD.into(),id:question.id.clone()}),
        }).collect(),
    }).collect();
    let scene = semio_framework_plugin::BlockListScene {
        steps,palette,
        selected_id: selected.map(|id| id.strip_prefix("step:").unwrap_or(id).into()),
        dragging_id: None,
        domain_id: Some(crate::editor::forms::FORMS_INTERACTION_FIELDS.into()),
    };
    semio_framework_plugin::scene_surface(FORMS_PLAY_SURFACE_BLUEPRINT, semio_framework_ui_contract::SurfaceKind::BlockList, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
