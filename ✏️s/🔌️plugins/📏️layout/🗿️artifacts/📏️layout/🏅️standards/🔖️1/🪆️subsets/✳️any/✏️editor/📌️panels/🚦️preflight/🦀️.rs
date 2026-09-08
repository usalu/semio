//! 🚦️ Layout play app panel — preflight: overset text, missing assets, and other publish blockers.
//! `run_layout_preflight` has two real consumers (this panel's tree AND
//! `🎮️commands/🐚️export::export_package`'s zip manifest) but stays here rather than moving to the
//! artifact engine: it takes `&LayoutLabels`, an app-owned terminology type, and artifacts must never
//! depend on apps.

use crate::{Frame, LayoutSnapshot};
use crate::editor::layout::terminology::{layout_labels, preflight_msg, LayoutLabels};
use crate::editor::layout::{layout_action, ui_value_map, ui_value_text};
use semio_framework_plugin::{tree_item_desc, tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, UiValue};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_PREFLIGHT: &str = "layout.play.preflight";
pub const LAYOUT_PLAY_PREFLIGHT_TAB_ID: &str = "layout.panel.preflight";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
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

fn resolve_link_state(link: &crate::ImageLink) -> &str {
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

fn resolve_run_style(doc: &LayoutSnapshot, paragraph_style_id: Option<&str>, character_style_id: Option<&str>) -> (String, f64) {
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
pub fn run_layout_preflight(doc: &LayoutSnapshot, labels: &LayoutLabels) -> Vec<PreflightIssue> {
    let mut issues = Vec::new();
    for page in &doc.pages {
        let resolved = crate::schema::resolve_page(doc, page);
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
    id: impl AsRef<str>,
    label: impl Into<Label>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<(semio_framework_plugin::ActionId, Option<UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let label = label.into();
    let mut item = match action {
        Some(action) => tree_item_with_action(id, label.as_str(), description.clone(), action)?,
        None => tree_item_desc(id, label.as_str(), description.clone())?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        if props.description.is_none() {
            props.description = match description {
                Some(value) => Some(UiText::try_from_string(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout preflight description admission failed"))?),
                None => None,
            };
        }
        props.icon = match icon_id {
            Some(value) => Some(UiText::try_from_string(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout preflight icon admission failed"))?),
            None => None,
        };
    }
    Ok(item)
}

pub fn render(doc: &LayoutSnapshot, labels: &LayoutLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
    PanelTreeBuilder::new("layout-preflight")?.section("layout-preflight.issues", Some(crate::editor::layout::ui_label(labels.preflight.as_str())?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
