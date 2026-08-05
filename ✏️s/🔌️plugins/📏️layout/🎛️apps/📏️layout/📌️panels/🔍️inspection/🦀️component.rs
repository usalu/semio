//! 🔍️ Layout play app panel — the inspector: field editors for the current selection (page or frame).

use crate::apps::layout::config::LayoutConfig;
use crate::apps::layout::layout_action;
use crate::apps::layout::terminology::LayoutLabels;
use crate::artifacts::layout::engine::rgba_to_text;
use crate::artifacts::layout::{Frame, LayoutDocument, LAYOUT_FIXTURE_SCHEMA};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence,
    UiSectionNode, UiSelectItem, UiSelectNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_INSPECTION: &str = "layout.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(LAYOUT_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Helpers
fn story_full_content(doc: &LayoutDocument, story_id: &str) -> String {
    doc.stories.iter().find(|story| story.id == story_id).map(|story| story.content.clone()).unwrap_or_default()
}

fn link_path(doc: &LayoutDocument, link_id: &str) -> String {
    doc.links.iter().find(|link| link.id == link_id).map(|link| link.path.clone()).unwrap_or_default()
}
//#endregion 🔖️Helpers

//#region 🔖️Render
pub fn render(doc: &LayoutDocument, config: &LayoutConfig, labels: &LayoutLabels) -> UiNode {
    if config.selected_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "layout-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![
                ui_text(Label::data(format!("{}: {}", labels.schema.as_str(), LAYOUT_FIXTURE_SCHEMA))),
                ui_text(Label::data(format!("{}: {}", labels.name.as_str(), doc.name))),
                ui_text(Label::data(format!("{}: {}", labels.pages.as_str(), doc.pages.len()))),
                ui_text(Label::data(format!("{}: {}", labels.active_page.as_str(), config.active_page_id))),
            ],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let selected_id = &config.selected_ids[0];
    if let Some(page) = doc.pages.iter().find(|page| page.id == *selected_id) {
        let mut fields = vec![
            ui_inspector_readonly_field("layout-play-inspector.page-id", labels.id, page.id.clone()),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "layout-play-inspector.page-name".into(),
                label: labels.name.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "layout-play-inspector.page-name.input".into(),
                    input_kind: "text".into(),
                    value: page.name.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: layout_action("patchPage", Some(json!({ "pageId": page.id, "field": "name" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ];
        for (field, label, value) in [
            ("width", labels.width, page.width),
            ("height", labels.height, page.height),
            ("marginTop", labels.margin_top, page.margins.top),
            ("marginRight", labels.margin_right, page.margins.right),
            ("marginBottom", labels.margin_bottom, page.margins.bottom),
            ("marginLeft", labels.margin_left, page.margins.left),
            ("columnsGutter", labels.gutter, page.columns.gutter),
        ] {
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("layout-play-inspector.page-{field}"),
                label: label.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: format!("layout-play-inspector.page-{field}.input"),
                    input_kind: "number".into(),
                    value: format!("{value}"),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: layout_action("patchPage", Some(json!({ "pageId": page.id, "field": field }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
        }
        fields.push(UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "layout-play-inspector.page-columnsCount".into(),
            label: labels.columns.into(),
            child: Box::new(UiNode::Input(UiInputNode {
                presence: UiPresence::default(),
                id: "layout-play-inspector.page-columnsCount.input".into(),
                input_kind: "number".into(),
                value: format!("{}", page.columns.count),
                placeholder: None,
                commit: Some("blur".into()),
                on_change: layout_action("patchPage", Some(json!({ "pageId": page.id, "field": "columnsCount" }))),
                min: None,
                max: None,
                step: None,
                accept: None,
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            menu: None,
        }));
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "layout-play-inspector.page".into(), label: labels.group_page.into(), default_open: Some(true), fields }]);
    }
    for page in &doc.pages {
        if let Some(frame) = page.frames.iter().find(|frame| frame.id() == selected_id) {
            let bounds = frame.bounds();
            let frame_id = frame.id().to_string();
            let page_id = page.id.clone();
            let mut fields = vec![
                ui_inspector_readonly_field("layout-play-inspector.frame-id", labels.id, frame_id.clone()),
                ui_inspector_readonly_field("layout-play-inspector.frame-kind", labels.kind, frame.kind_str().to_string()),
                ui_inspector_readonly_field("layout-play-inspector.frame-page", labels.page, page.name.clone()),
            ];
            for (field, label, value) in [("x", labels.x, bounds.x), ("y", labels.y, bounds.y), ("width", labels.width, bounds.width), ("height", labels.height, bounds.height)] {
                fields.push(UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
                    id: format!("layout-play-inspector.frame-{field}"),
                    label: label.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
                        presence: UiPresence::default(),
                        id: format!("layout-play-inspector.frame-{field}.input"),
                        input_kind: "number".into(),
                        value: format!("{}", value as i64),
                        placeholder: None,
                        commit: Some("blur".into()),
                        on_change: layout_action("patchFrame", Some(json!({ "frameId": frame_id, "pageId": page_id, "field": field }))),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }));
            }
            match frame {
                Frame::Rect { fill, stroke, .. } => {
                    for (field, label, value) in [("fill", labels.fill, fill), ("stroke", labels.stroke, stroke)] {
                        fields.push(UiNode::Field(UiFieldNode {
                            presence: UiPresence::default(),
                            id: format!("layout-play-inspector.frame-{field}"),
                            label: label.into(),
                            child: Box::new(UiNode::Input(UiInputNode {
                                presence: UiPresence::default(),
                                id: format!("layout-play-inspector.frame-{field}.input"),
                                input_kind: "text".into(),
                                value: rgba_to_text(value),
                                placeholder: Some(Label::data("r, g, b, a")),
                                commit: Some("blur".into()),
                                on_change: layout_action("patchFrame", Some(json!({ "frameId": frame_id, "pageId": page_id, "field": field }))),
                                min: None,
                                max: None,
                                step: None,
                                accept: None,
                                menu: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                            menu: None,
                        }));
                    }
                }
                Frame::Text { story_id, wrap_mode, columns, .. } => {
                    fields.push(UiNode::Field(UiFieldNode {
                        presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-story".into(),
                        label: labels.story.into(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            presence: UiPresence::default(),
                            id: "layout-play-inspector.frame-story.input".into(),
                            input_kind: "text".into(),
                            value: story_full_content(doc, story_id),
                            placeholder: None,
                            commit: Some("blur".into()),
                            on_change: layout_action("patchFrame", Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "storyContent" }))),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-wrapMode".into(),
                        label: labels.wrap_mode.into(),
                        child: Box::new(UiNode::Select(UiSelectNode {
                            presence: UiPresence::default(),
                            id: "layout-play-inspector.frame-wrapMode.select".into(),
                            value: wrap_mode.clone(),
                            items: vec![
                                UiSelectItem { value: "none".into(), label: labels.wrap_none.into() },
                                UiSelectItem { value: "box".into(), label: labels.wrap_box.into() },
                                UiSelectItem { value: "contour".into(), label: labels.wrap_contour.into() },
                            ],
                            placeholder: None,
                            on_change: layout_action("patchFrame", Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "wrapMode" }))),
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-columns".into(),
                        label: labels.columns.into(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            presence: UiPresence::default(),
                            id: "layout-play-inspector.frame-columns.input".into(),
                            input_kind: "number".into(),
                            value: format!("{columns}"),
                            placeholder: None,
                            commit: Some("blur".into()),
                            on_change: layout_action("patchFrame", Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "columns" }))),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                }
                Frame::Image { link_id, .. } => {
                    fields.push(UiNode::Field(UiFieldNode {
                        presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-linkPath".into(),
                        label: labels.link_path.into(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            presence: UiPresence::default(),
                            id: "layout-play-inspector.frame-linkPath.input".into(),
                            input_kind: "text".into(),
                            value: link_path(doc, link_id),
                            placeholder: None,
                            commit: Some("blur".into()),
                            on_change: layout_action("patchFrame", Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "linkPath" }))),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                }
            }
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "layout-play-inspector.frame".into(), label: labels.group_frame.into(), default_open: Some(true), fields }]);
        }
    }
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "layout-play-inspector.missing".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![ui_text(labels.selection_not_found)],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::commands::view::set_selection;
    use crate::apps::layout::testkit::{dispatch, layout_app, render as render_body};
    use crate::apps::layout::LayoutCommand;

    #[test]
    fn set_selection_reflects_in_inspector() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::SetSelection(set_selection::SetSelection { ids: vec!["frame-text-1".into()] }));
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_INSPECTION);
        assert!(json.contains("frame-text-1"));
    }

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_INSPECTION));
    }
}
//#endregion 🧪️Tests
