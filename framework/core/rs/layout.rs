//! 📐 Window layouts, panel tab constants, and engagement rails.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandDescriptor {
    pub controller_id: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<String>,
}
//#endregion 🔖Command

//#region 🔖PanelTabConstants
pub const FRAMEWORK_PANEL_TAB_HIERARCHY_ID: &str = "framework.panel.hierarchy";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ID: &str = "framework.panel.catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_ID: &str = "framework.panel.inspection";
pub const FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL: &str = "Hierarchy";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL: &str = "Catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL: &str = "Inspection";
pub const FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID: &str = "framework.panel.hierarchy";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID: &str = "framework.panel.catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID: &str = "framework.panel.inspection";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ID: &str = "framework.panel.parameters";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL: &str = "Parameters";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID: &str = "framework.panel.parameters";
//#endregion 🔖PanelTabConstants

//#region 🔖WindowLayout
fn kind_window() -> String {
    "window".into()
}

fn kind_stack() -> String {
    "stack".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayoutWindowNode {
    #[serde(default = "kind_window")]
    pub kind: String,
    pub window_kind_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayoutStackNode {
    #[serde(default = "kind_stack")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "activeId")]
    pub active_window_kind_id: Option<String>,
    pub children: Vec<WindowLayoutWindowNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayoutAxisNode {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    pub children: Vec<WindowLayoutChild>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WindowLayoutChild {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WindowLayoutRoot {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayout {
    pub root: WindowLayoutRoot,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedLayout {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    pub layout: WindowLayout,
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_path: Option<Vec<String>>,
}

pub fn create_window_layout(
    window_kind_id: impl Into<String>,
    title: Option<String>,
    instance_id: Option<String>,
    template_id: Option<String>,
) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode {
        kind: kind_window(),
        window_kind_id: window_kind_id.into(),
        title,
        instance_id,
        template_id,
    }
}

pub fn create_stack_layout(window_kind_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: kind_stack(),
            size: None,
            active_window_kind_id: None,
            children: window_kind_ids
                .iter()
                .enumerate()
                .map(|(index, id)| {
                    create_window_layout(
                        id.clone(),
                        titles.and_then(|rows| rows.get(index).cloned()),
                        None,
                        None,
                    )
                })
                .collect(),
        }),
    }
}

pub fn create_default_layout(
    window_ids: &[String],
    direction: &str,
    sizes: Option<&[f64]>,
    titles: Option<&[String]>,
) -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: direction.into(),
            size: None,
            children: window_ids
                .iter()
                .enumerate()
                .map(|(index, id)| {
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: kind_stack(),
                        size: sizes.and_then(|rows| rows.get(index).copied()),
                        active_window_kind_id: None,
                        children: vec![create_window_layout(
                            id.clone(),
                            titles
                                .and_then(|rows| rows.get(index).cloned())
                                .or_else(|| Some(id.clone())),
                            None,
                            None,
                        )],
                    })
                })
                .collect(),
        }),
    }
}

pub fn create_tab_stack_layout(window_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
    create_stack_layout(window_ids, titles)
}

pub fn create_named_layout(
    id: impl Into<String>,
    label: impl Into<String>,
    layout: WindowLayout,
    origin: impl Into<String>,
    icon_id: Option<String>,
    group_path: Option<Vec<String>>,
) -> NamedLayout {
    NamedLayout {
        id: id.into(),
        label: label.into(),
        icon_id,
        layout,
        origin: origin.into(),
        group_path,
    }
}

pub fn merge_named_layouts(base: &[NamedLayout], extension: &[NamedLayout]) -> Vec<NamedLayout> {
    let mut merged: HashMap<String, NamedLayout> = HashMap::new();
    for entry in base {
        merged.insert(entry.id.clone(), entry.clone());
    }
    for entry in extension {
        merged.insert(entry.id.clone(), entry.clone());
    }
    merged.into_values().collect()
}
//#endregion 🔖WindowLayout

//#region 🔖WindowMeasure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WindowMeasure {
    Select {
        id: String,
        label: Option<String>,
        value: String,
        items: Vec<MeasureSelectItem>,
        on_change: CommandDescriptor,
    },
    Slider {
        id: String,
        label: Option<String>,
        value: f64,
        min: f64,
        max: f64,
        step: Option<f64>,
        on_change: CommandDescriptor,
    },
    Toggle {
        id: String,
        icon_id: String,
        label: Option<String>,
        pressed: bool,
        text: Option<String>,
        on_change: CommandDescriptor,
    },
    Group {
        id: String,
        label: String,
        default_open: Option<bool>,
        children: Vec<WindowMeasure>,
    },
}
//#endregion 🔖WindowMeasure

//#region 🔖WindowEngagement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementOption {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_repeat_last: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_abort: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementStatus {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementPossible {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementRingOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementToggleGroupOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagementSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WindowEngagementControl {
    Slider {
        id: Option<String>,
        label: Option<String>,
        value: f64,
        min: f64,
        max: f64,
        step: Option<f64>,
        unit: Option<String>,
        disabled: Option<bool>,
        on_change: Option<CommandDescriptor>,
        on_commit: Option<CommandDescriptor>,
    },
    Stepper {
        id: Option<String>,
        label: Option<String>,
        value: f64,
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        unit: Option<String>,
        disabled: Option<bool>,
        on_change: Option<CommandDescriptor>,
        on_commit: Option<CommandDescriptor>,
    },
    Ring {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        options: Vec<WindowEngagementRingOption>,
        disabled: Option<bool>,
        on_select: Option<CommandDescriptor>,
    },
    ToggleGroup {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        options: Vec<WindowEngagementToggleGroupOption>,
        disabled: Option<bool>,
        on_select: Option<CommandDescriptor>,
    },
    Select {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        placeholder: Option<String>,
        items: Vec<WindowEngagementSelectItem>,
        disabled: Option<bool>,
        on_change: Option<CommandDescriptor>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEngagement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<WindowEngagementOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<WindowEngagementInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<WindowEngagementControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<WindowEngagementControl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<WindowEngagementStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_engagements: Option<Vec<WindowEngagementPossible>>,
}
//#endregion 🔖WindowEngagement
