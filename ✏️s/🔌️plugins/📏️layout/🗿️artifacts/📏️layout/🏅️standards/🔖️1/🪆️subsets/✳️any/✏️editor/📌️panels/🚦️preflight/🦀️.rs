//! 🚦️ Layout play app panel — preflight: overset text, missing assets, and other publish blockers.
//! `run_layout_preflight` has two real consumers (this panel's tree AND
//! `🎮️commands/🐚️export::export_package`'s zip manifest) but stays here rather than moving to the
//! artifact engine: it takes `&LayoutLabels`, an app-owned terminology type, and artifacts must never
//! depend on apps.

use crate::artifacts::layout::{Frame, LayoutSnapshot};
use crate::editor::layout::terminology::{layout_labels, preflight_msg, LayoutLabels};
use crate::editor::layout::{layout_action, ui_value_map, ui_value_text};
use semio_framework_plugin::{tree_item_desc, tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, UiValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_PREFLIGHT: &str = "layout.play.preflight";
pub const LAYOUT_PLAY_PREFLIGHT_TAB_ID: &str = "layout.panel.preflight";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(LAYOUT_PLAY_PREFLIGHT_TAB_ID.into()), label: LocalizedLabel::native("Preflight", "Preflight"), group: PanelGroup::Workbench, body_key: Some(LAYOUT_PLAY_BODY_PREFLIGHT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Preflight
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct PreflightIssue {
    pub severity: String,
    pub code: String,
    pub message: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
}

async fn resolve_link_state(link: &crate::artifacts::layout::ImageLink) -> &str {
    if let Some(state) = link.state.as_deref() {
        return state;
    }
    if link.path.is_empty() || link.hash == "sha256:missing" {
        return "missing";
    }
    if link.dpi < 150 {
        return "low_resolution";
    }
    "ok"
}

async fn resolve_run_style(doc: &LayoutSnapshot, paragraph_style_id: Option<&str>, character_style_id: Option<&str>) -> (String, f64) {
    let paragraph = paragraph_style_id.and_then(|id| doc.paragraph_styles.iter().find(|style| style.id == id)).or_else(|| doc.paragraph_styles.first());
    let (mut family, mut size) = paragraph.map_or_else(|| ("Layout Sans".into(), 12.0), |style| (style.font_family.clone(), style.font_size));
    if let Some(character_id) = character_style_id {
        if let Some(character) = doc.character_styles.iter().find(|style| style.id == character_id) {
            if let Some(font_family) = &character.font_family {
                family = font_family.clone();
            }
            if let Some(font_size) = character.font_size {
                size = font_size;
            }
        }
    }
    (family, size)
}

/// 🚦️ Runs every preflight check over the document, returning the flat issue list — shared by this
/// panel's tree and the export package's zip manifest.
pub async fn run_layout_preflight(doc: &LayoutSnapshot, labels: &LayoutLabels) -> Vec<PreflightIssue> {
    let mut issues = Vec::new();
    for page in &doc.pages {
        let resolved = crate::artifacts::layout::schema::resolve_page(doc, page);
        for entry in resolved {
            let frame = &entry.frame;
            if !frame.visible() {
                continue;
            }
            let bounds = frame.bounds();
            if bounds.x < 0.0 || bounds.y < 0.0 || bounds.x + bounds.width > page.width || bounds.y + bounds.height > page.height {
                issues.push(PreflightIssue {
                    severity: "warning".into(),
                    code: "object.out_of_bounds".into(),
                    message: preflight_msg(labels.preflight_out_of_bounds, &[frame.id()]),
                    object_id: Some(frame.id().into()),
                    page_id: Some(page.id.clone()),
                });
            }
            match frame {
                Frame::Image { link_id, .. } => {
                    let link = doc.links.iter().find(|entry| entry.id == *link_id);
                    match link.map(resolve_link_state) {
                        Some("missing") | None => issues.push(PreflightIssue {
                            severity: "error".into(),
                            code: "asset.missing".into(),
                            message: preflight_msg(labels.preflight_asset_missing, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        Some("modified") => issues.push(PreflightIssue {
                            severity: "warning".into(),
                            code: "asset.modified".into(),
                            message: preflight_msg(labels.preflight_asset_modified, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        Some("low_resolution") => issues.push(PreflightIssue {
                            severity: "warning".into(),
                            code: "asset.low_resolution".into(),
                            message: preflight_msg(labels.preflight_asset_low_resolution, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        _ => {}
                    }
                    if link.is_some_and(|entry| entry.proxy_data_url.is_none()) && bounds.width > 0.0 && bounds.height > 0.0 {
                        issues.push(PreflightIssue {
                            severity: "info".into(),
                            code: "image.empty_frame".into(),
                            message: preflight_msg(labels.preflight_image_empty_frame, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        });
                    }
                }
                Frame::Text { story_id, thread_next, .. } => {
                    let Some(story) = doc.stories.iter().find(|story| story.id == *story_id) else {
                        issues.push(PreflightIssue {
                            severity: "error".into(),
                            code: "text.missing_story".into(),
                            message: preflight_msg(labels.preflight_text_missing_story, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        });
                        continue;
                    };
                    let styles: Vec<(String, f64)> =
                        if story.style_runs.is_empty() { vec![resolve_run_style(doc, None, None)] } else { story.style_runs.iter().map(|run| resolve_run_style(doc, run.paragraph_style_id.as_deref(), run.character_style_id.as_deref())).collect() };
                    for (family, size) in &styles {
                        if *size < 8.0 {
                            issues.push(PreflightIssue {
                                severity: "warning".into(),
                                code: "text.below_minimum_size".into(),
                                message: preflight_msg(labels.preflight_text_below_minimum_size, &[frame.id()]),
                                object_id: Some(frame.id().into()),
                                page_id: Some(page.id.clone()),
                            });
                        }
                        let known_family = family == "Layout Sans" || doc.paragraph_styles.iter().any(|style| style.font_family == *family);
                        if !known_family {
                            issues.push(PreflightIssue {
                                severity: "error".into(),
                                code: "font.missing".into(),
                                message: preflight_msg(labels.preflight_font_missing, &[family, frame.id()]),
                                object_id: Some(frame.id().into()),
                                page_id: Some(page.id.clone()),
                            });
                        }
                    }
                    if thread_next.is_none() && story.content.len() > 400 {
                        issues.push(PreflightIssue { severity: "error".into(), code: "text.overset".into(), message: preflight_msg(labels.preflight_text_overset, &[frame.id()]), object_id: Some(frame.id().into()), page_id: Some(page.id.clone()) });
                    }
                }
                Frame::Rect { .. } => {}
            }
        }
    }
    if doc.print_target.as_deref() == Some("print") {
        for link in &doc.links {
            if link.color_profile.as_deref() == Some("RGB") {
                issues.push(PreflightIssue { severity: "warning".into(), code: "asset.rgb_in_print".into(), message: preflight_msg(labels.preflight_asset_rgb_in_print, &[&link.id]), object_id: Some(link.id.clone()), page_id: None });
            }
        }
    }
    issues
}
//#endregion 🔖️Preflight

//#region 🔖️Render
fn layout_tree_item(
    id: impl Into<String>,
    label: impl TryInto<Label>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<(semio_framework_plugin::ActionId, Option<UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = match action {
        Some(action) => tree_item_with_action(id, label, description.clone(), action)?,
        None => tree_item_desc(id, label, description.clone())?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        if props.description.is_none() {
            props.description = match description {
                Some(value) => Some(UiText::try_from_string(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout preflight description admission failed"))?),
                None => None,
            };
        }
        props.icon = match icon_id {
            Some(value) => Some(UiText::try_from_string(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout preflight icon admission failed"))?),
            None => None,
        };
    }
    Ok(item)
}

pub async fn render(doc: &LayoutSnapshot, cfg: &crate::editor::layout::config::LayoutConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let labels = layout_labels(cfg);
    let issues = run_layout_preflight(doc, labels);
    let mut items = UiFixedList::default();
    if issues.is_empty() {
        let item = layout_tree_item("layout-preflight.empty", labels.no_issues, None, Some("check-circle".into()), None)?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout preflight empty row admission failed"))?;
    } else {
        for issue in &issues {
            let issue_value = ui_value_map([
                ("severity", ui_value_text(&issue.severity)?),
                ("code", ui_value_text(&issue.code)?),
                ("message", ui_value_text(&issue.message)?),
                (
                    "objectId",
                    match &issue.object_id {
                        Some(value) => ui_value_text(value)?,
                        None => UiValue::Null,
                    },
                ),
                (
                    "pageId",
                    match &issue.page_id {
                        Some(value) => ui_value_text(value)?,
                        None => UiValue::Null,
                    },
                ),
            ])?;
            let args = ui_value_map([("issue", issue_value)])?;
            let item = layout_tree_item(
                format!("layout-preflight.{}.{}", issue.code, issue.object_id.clone().unwrap_or_else(|| issue.message.clone())),
                Label::data(issue.message.clone()),
                Some(format!("{} · {}", issue.severity, issue.code)),
                Some(if issue.severity == "error" { "alert-circle" } else { "alert-triangle" }.into()),
                Some(layout_action("focusPreflightIssue", Some(args))?),
            )?;
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout preflight issue admission failed"))?;
        }
    }
    PanelTreeBuilder::new("layout-preflight")?.section("layout-preflight.issues", Some(labels.preflight.into()), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{layout_app, render as render_body};
    use semio_framework_plugin::AppLabels;

    #[semio_framework_async_macros::async_test]
    async fn preflight_finds_missing_asset() {
        let issues = run_layout_preflight(&crate::artifacts::layout::schema::default_document(), LayoutLabels::labels(semio_framework_plugin::Locale::En, semio_framework_plugin::Terminology::Native));
        assert!(issues.iter().any(|issue| issue.code == "asset.missing"));
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_PREFLIGHT);
        assert!(json.contains("asset.missing") || json.contains("Linked asset missing"));
    }

    #[semio_framework_async_macros::async_test]
    async fn preflight_reports_all_expected_issue_codes() {
        let json = r#"{
            "schema": "layout.layout",
            "name": "Preflight Fixture",
            "grid": {"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},
            "paragraphStyles": [{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],
            "characterStyles": [
                {"id":"character.small","fontFamily":"Layout Sans","fontSize":6},
                {"id":"character.exotic","fontFamily":"Comic Sans","fontSize":10}
            ],
            "stories": [
                {"id":"story-small","content":"Small caption text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.small"}]},
                {"id":"story-exotic","content":"Exotic font text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.exotic"}]},
                {"id":"story-overset","content":"placeholder","styleRuns":[]}
            ],
            "links": [
                {"id":"link-missing","path":"a.png","hash":"sha256:missing","width":100,"height":100,"dpi":300,"state":"missing"},
                {"id":"link-modified","path":"b.png","hash":"sha256:abc","width":100,"height":100,"dpi":300,"state":"modified"},
                {"id":"link-lowres","path":"c.png","hash":"sha256:def","width":100,"height":100,"dpi":72},
                {"id":"link-rgb","path":"d.png","hash":"sha256:ghi","width":100,"height":100,"dpi":300,"colorProfile":"RGB"}
            ],
            "parentPages": [],
            "spreads": [{"id":"spread-1","name":"Spread 1","pageIds":["page-1"]}],
            "pages": [{
                "id":"page-1","name":"Page 1","spreadId":"spread-1","width":200,"height":200,
                "margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},
                "guides":[], "layerIds":["layer-1"],
                "layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-oob","frame-missing","frame-modified","frame-lowres","frame-no-story","frame-small","frame-exotic","frame-overset"]}],
                "frames":[
                    {"id":"frame-oob","layerId":"layer-1","kind":"rect","bounds":{"x":150,"y":150,"w":100,"h":100,"rotation":0},"fill":[0,0,0,1]},
                    {"id":"frame-missing","layerId":"layer-1","kind":"image","bounds":{"x":0,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-missing"},
                    {"id":"frame-modified","layerId":"layer-1","kind":"image","bounds":{"x":20,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-modified"},
                    {"id":"frame-lowres","layerId":"layer-1","kind":"image","bounds":{"x":40,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-lowres"},
                    {"id":"frame-no-story","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":40,"w":50,"h":20,"rotation":0},"storyId":"story-absent","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-small","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":60,"w":50,"h":20,"rotation":0},"storyId":"story-small","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-exotic","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":80,"w":50,"h":20,"rotation":0},"storyId":"story-exotic","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-overset","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":100,"w":50,"h":20,"rotation":0},"storyId":"story-overset","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"}
                ],
                "overrides":[]
            }],
            "printTarget":"print"
        }"#;
        let mut doc: LayoutSnapshot = serde_json::from_str(json).expect("preflight fixture");
        if let Some(story) = doc.stories.iter_mut().find(|story| story.id == "story-overset") {
            story.content = "a".repeat(450);
        }
        let issues = run_layout_preflight(&doc, LayoutLabels::labels(semio_framework_plugin::Locale::En, semio_framework_plugin::Terminology::Native));
        let codes: Vec<&str> = issues.iter().map(|issue| issue.code.as_str()).collect();
        for expected in ["object.out_of_bounds", "asset.missing", "asset.modified", "asset.low_resolution", "image.empty_frame", "text.missing_story", "text.below_minimum_size", "font.missing", "text.overset", "asset.rgb_in_print"] {
            assert!(codes.contains(&expected), "missing preflight code: {expected}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_preflight_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), LAYOUT_PLAY_PREFLIGHT_TAB_ID);
        assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_PREFLIGHT));
    }
}
//#endregion 🧪️Tests
