//! 🥅 Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link ActionBus}.

pub mod action_bus {
// #region action_bus
//! 🎯 Action routing between renderer and app controllers.

use serde_json::Value;
use std::collections::HashMap;

pub trait ActionHandler: Send {
    fn id(&self) -> &str;
    fn handle(&mut self, action: &str, args: Option<&Value>) -> Vec<String>;
}

pub struct ActionBus {
    controllers: HashMap<String, Box<dyn ActionHandler>>,
}

impl Default for ActionBus {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionBus {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn ActionHandler>) {
        let id = handler.id().to_string();
        self.controllers.insert(id, handler);
    }

    pub fn unregister(&mut self, controller_id: &str) {
        self.controllers.remove(controller_id);
    }

    pub fn dispatch(&mut self, controller_id: &str, action: &str, args: Option<&Value>) -> Vec<String> {
        self.controllers
            .get_mut(controller_id)
            .map(|handler| handler.handle(action, args))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoHandler {
        id: String,
    }

    impl ActionHandler for EchoHandler {
        fn id(&self) -> &str {
            &self.id
        }

        fn handle(&mut self, action: &str, _args: Option<&Value>) -> Vec<String> {
            vec![format!("{action}:ok")]
        }
    }

    #[test]
    fn dispatches_to_registered_handler() {
        let mut bus = ActionBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        let ops = bus.dispatch("app", "ping", None);
        assert_eq!(ops, vec!["ping:ok"]);
    }
}
// #endregion action_bus
}

pub mod layout {
// #region layout
//! 📐 Window layouts, panel tab constants, and engagement rails.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖Action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDescriptor {
    pub controller_id: String,
    pub action: String,
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
//#endregion 🔖Action

//#region 🔖PanelTabConstants
pub const FRAMEWORK_PANEL_TAB_DOCUMENT_ID: &str = "framework.panel.document";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ID: &str = "framework.panel.catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_ID: &str = "framework.panel.inspection";
pub const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL: &str = "Document";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL: &str = "Catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL: &str = "Inspection";
pub const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID: &str = "framework.panel.document";
pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID: &str = "framework.panel.catalogue";
pub const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID: &str = "framework.panel.inspection";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ID: &str = "framework.panel.parameters";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL: &str = "Parameters";
pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID: &str = "framework.panel.parameters";

/// 🗣️ Resolves a well-known framework panel-tab id to its native English/German label; unknown ids resolve to None so app-specific panel tabs are left untouched.
pub fn framework_panel_tab_label(id: &str, is_de: bool) -> Option<&'static str> {
    match (id, is_de) {
        (FRAMEWORK_PANEL_TAB_DOCUMENT_ID, false) => Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL),
        (FRAMEWORK_PANEL_TAB_DOCUMENT_ID, true) => Some("Dokument"),
        (FRAMEWORK_PANEL_TAB_CATALOGUE_ID, false) => Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL),
        (FRAMEWORK_PANEL_TAB_CATALOGUE_ID, true) => Some("Katalog"),
        (FRAMEWORK_PANEL_TAB_INSPECTION_ID, false) => Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
        (FRAMEWORK_PANEL_TAB_INSPECTION_ID, true) => Some("Inspektion"),
        (FRAMEWORK_PANEL_TAB_PARAMETERS_ID, false) => Some(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL),
        (FRAMEWORK_PANEL_TAB_PARAMETERS_ID, true) => Some("Parameter"),
        _ => None,
    }
}
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

/// 🧭 Collects every `window_kind_id` referenced by a layout tree.
pub fn collect_window_kind_ids_from_layout(layout: &WindowLayout) -> Vec<String> {
    let mut ids = Vec::new();
    collect_window_kind_ids_from_root(&layout.root, &mut ids);
    ids
}

fn collect_window_kind_ids_from_root(root: &WindowLayoutRoot, out: &mut Vec<String>) {
    match root {
        WindowLayoutRoot::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
        WindowLayoutRoot::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
    }
}

fn collect_window_kind_ids_from_children(children: &[WindowLayoutChild], out: &mut Vec<String>) {
    for child in children {
        match child {
            WindowLayoutChild::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
            WindowLayoutChild::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
        }
    }
}

fn collect_window_kind_ids_from_stack(stack: &WindowLayoutStackNode, out: &mut Vec<String>) {
    for window in &stack.children {
        out.push(window.window_kind_id.clone());
    }
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
        on_change: ActionDescriptor,
    },
    Slider {
        id: String,
        label: Option<String>,
        value: f64,
        min: f64,
        max: f64,
        step: Option<f64>,
        on_change: ActionDescriptor,
    },
    Toggle {
        id: String,
        icon_id: String,
        label: Option<String>,
        pressed: bool,
        text: Option<String>,
        on_change: ActionDescriptor,
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
    pub action: Option<ActionDescriptor>,
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
    pub on_change: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_repeat_last: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_abort: Option<ActionDescriptor>,
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
    pub action: Option<ActionDescriptor>,
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
        on_change: Option<ActionDescriptor>,
        on_commit: Option<ActionDescriptor>,
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
        on_change: Option<ActionDescriptor>,
        on_commit: Option<ActionDescriptor>,
    },
    Ring {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        options: Vec<WindowEngagementRingOption>,
        disabled: Option<bool>,
        on_select: Option<ActionDescriptor>,
    },
    ToggleGroup {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        options: Vec<WindowEngagementToggleGroupOption>,
        disabled: Option<bool>,
        on_select: Option<ActionDescriptor>,
    },
    Select {
        id: Option<String>,
        label: Option<String>,
        value: Option<String>,
        placeholder: Option<String>,
        items: Vec<WindowEngagementSelectItem>,
        disabled: Option<bool>,
        on_change: Option<ActionDescriptor>,
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

pub fn default_viewport_engagement() -> WindowEngagement {
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: None,
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "framework.viewport.status".into(),
            text: "Viewport".into(),
        }]),
        possible_engagements: None,
    }
}
//#endregion 🔖WindowEngagement

//#region 🔖WireFormatGoldenTests
/** 🧊 Golden wire-format tests: freeze exact JSON for layout/action/engagement types
before these move into ui_wgpu, so the move can be proven byte-identical. */
#[cfg(test)]
mod layout_wire_format_tests {
    use super::*;

    const GOLDEN_ACTION_DESCRIPTOR_JSON: &str = "[{\"controllerId\":\"ctrl\",\"action\":\"doThing\",\"args\":42},{\"controllerId\":\"ctrl\",\"action\":\"doOther\"},{\"variant\":\"primary\",\"size\":\"md\"}]";

    #[test]
    fn action_descriptor_and_style_spec_serialize_to_golden_json() {
        let values = (
            ActionDescriptor {
                controller_id: "ctrl".into(),
                action: "doThing".into(),
                args: Some(serde_json::json!(42)),
            },
            ActionDescriptor { controller_id: "ctrl".into(), action: "doOther".into(), args: None },
            StyleSpec {
                variant: Some("primary".into()),
                size: Some("md".into()),
                density: None,
            },
        );
        let json = serde_json::to_string(&values).unwrap();
        assert_eq!(json, GOLDEN_ACTION_DESCRIPTOR_JSON);
    }

    const GOLDEN_WINDOW_LAYOUT_JSON: &str = "{\"root\":{\"kind\":\"horizontal\",\"children\":[{\"kind\":\"stack\",\"size\":0.5,\"activeWindowKindId\":\"main\",\"children\":[{\"kind\":\"window\",\"windowKindId\":\"main\",\"title\":\"Main\"}]},{\"kind\":\"vertical\",\"size\":0.5,\"children\":[]}]}}";

    #[test]
    fn window_layout_serializes_to_golden_json() {
        let layout = WindowLayout {
            root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                kind: "horizontal".into(),
                size: None,
                children: vec![
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: "stack".into(),
                        size: Some(0.5),
                        active_window_kind_id: Some("main".into()),
                        children: vec![WindowLayoutWindowNode {
                            kind: "window".into(),
                            window_kind_id: "main".into(),
                            title: Some("Main".into()),
                            instance_id: None,
                            template_id: None,
                        }],
                    }),
                    WindowLayoutChild::Axis(WindowLayoutAxisNode {
                        kind: "vertical".into(),
                        size: Some(0.5),
                        children: vec![],
                    }),
                ],
            }),
        };
        let json = serde_json::to_string(&layout).unwrap();
        assert_eq!(json, GOLDEN_WINDOW_LAYOUT_JSON);
        let roundtripped: WindowLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, layout);
    }

    const GOLDEN_WINDOW_MEASURE_JSON: &str = "[{\"kind\":\"select\",\"id\":\"m1\",\"label\":\"Mode\",\"value\":\"a\",\"items\":[{\"id\":\"a\",\"value\":\"a\",\"label\":\"A\"}],\"on_change\":{\"controllerId\":\"ctrl\",\"action\":\"measureSelect\"}},{\"kind\":\"slider\",\"id\":\"m2\",\"label\":null,\"value\":1.0,\"min\":0.0,\"max\":2.0,\"step\":0.5,\"on_change\":{\"controllerId\":\"ctrl\",\"action\":\"measureSlider\"}},{\"kind\":\"toggle\",\"id\":\"m3\",\"icon_id\":\"icon.grid\",\"label\":null,\"pressed\":true,\"text\":null,\"on_change\":{\"controllerId\":\"ctrl\",\"action\":\"measureToggle\"}},{\"kind\":\"group\",\"id\":\"m4\",\"label\":\"Group\",\"default_open\":true,\"children\":[]}]";

    #[test]
    fn window_measure_serializes_to_golden_json() {
        let measures = vec![
            WindowMeasure::Select {
                id: "m1".into(),
                label: Some("Mode".into()),
                value: "a".into(),
                items: vec![MeasureSelectItem { id: "a".into(), value: "a".into(), label: "A".into() }],
                on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureSelect".into(), args: None },
            },
            WindowMeasure::Slider {
                id: "m2".into(),
                label: None,
                value: 1.0,
                min: 0.0,
                max: 2.0,
                step: Some(0.5),
                on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureSlider".into(), args: None },
            },
            WindowMeasure::Toggle {
                id: "m3".into(),
                icon_id: "icon.grid".into(),
                label: None,
                pressed: true,
                text: None,
                on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureToggle".into(), args: None },
            },
            WindowMeasure::Group {
                id: "m4".into(),
                label: "Group".into(),
                default_open: Some(true),
                children: vec![],
            },
        ];
        let json = serde_json::to_string(&measures).unwrap();
        assert_eq!(json, GOLDEN_WINDOW_MEASURE_JSON);
        let roundtripped: Vec<WindowMeasure> = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, measures);
    }

    const GOLDEN_WINDOW_ENGAGEMENT_JSON: &str = "{\"sessionActive\":true,\"options\":[{\"id\":\"opt1\",\"label\":\"Option\",\"pressed\":false}],\"input\":{\"id\":\"in1\",\"value\":\"v\"},\"control\":{\"kind\":\"slider\",\"id\":\"sl1\",\"label\":null,\"value\":1.0,\"min\":0.0,\"max\":2.0,\"step\":null,\"unit\":null,\"disabled\":null,\"on_change\":null,\"on_commit\":null},\"status\":[{\"id\":\"st1\",\"text\":\"Ready\"}],\"possibleEngagements\":[{\"id\":\"pe1\",\"label\":\"Possible\"}]}";

    #[test]
    fn window_engagement_serializes_to_golden_json() {
        let engagement = WindowEngagement {
            session_active: Some(true),
            options: Some(vec![WindowEngagementOption {
                id: "opt1".into(),
                label: Some("Option".into()),
                icon_id: None,
                pressed: Some(false),
                disabled: None,
                action: None,
            }]),
            input: Some(WindowEngagementInput {
                id: Some("in1".into()),
                value: Some("v".into()),
                placeholder: None,
                disabled: None,
                on_change: None,
                on_submit: None,
                on_repeat_last: None,
                on_abort: None,
            }),
            control: Some(WindowEngagementControl::Slider {
                id: Some("sl1".into()),
                label: None,
                value: 1.0,
                min: 0.0,
                max: 2.0,
                step: None,
                unit: None,
                disabled: None,
                on_change: None,
                on_commit: None,
            }),
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "st1".into(), text: "Ready".into() }]),
            possible_engagements: Some(vec![WindowEngagementPossible {
                id: "pe1".into(),
                label: "Possible".into(),
                detail: None,
                action: None,
            }]),
        };
        let json = serde_json::to_string(&engagement).unwrap();
        assert_eq!(json, GOLDEN_WINDOW_ENGAGEMENT_JSON);
        let roundtripped: WindowEngagement = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, engagement);
    }
}
//#endregion 🔖WireFormatGoldenTests
// #endregion layout
}

pub mod mesh {
// #region mesh
//! 🔺 Shared mesh geometry: primitives, compact JSON, OBJ/GLB interchange.

use serde::{Deserialize, Serialize};

//#region MeshData
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshData {
    #[serde(default)]
    pub positions: Vec<f32>,
    #[serde(default)]
    pub normals: Vec<f32>,
    #[serde(default)]
    pub colors: Vec<f32>,
    #[serde(default)]
    pub indices: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub face_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vertex_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_positions: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_is_seam: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint_texture_base64: Option<String>,
}

impl MeshData {
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn compute_normals(&mut self) {
        let count = self.vertex_count();
        self.normals = vec![0.0; count * 3];
        for tri in self.indices.chunks_exact(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = [self.positions[i0 * 3], self.positions[i0 * 3 + 1], self.positions[i0 * 3 + 2]];
            let p1 = [self.positions[i1 * 3], self.positions[i1 * 3 + 1], self.positions[i1 * 3 + 2]];
            let p2 = [self.positions[i2 * 3], self.positions[i2 * 3 + 1], self.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                e0[1] * e1[2] - e0[2] * e1[1],
                e0[2] * e1[0] - e0[0] * e1[2],
                e0[0] * e1[1] - e0[1] * e1[0],
            ];
            for &idx in tri {
                let i = idx as usize * 3;
                self.normals[i] += n[0];
                self.normals[i + 1] += n[1];
                self.normals[i + 2] += n[2];
            }
        }
        for chunk in self.normals.chunks_exact_mut(3) {
            let len = (chunk[0] * chunk[0] + chunk[1] * chunk[1] + chunk[2] * chunk[2]).sqrt();
            if len > 1e-8 {
                chunk[0] /= len;
                chunk[1] /= len;
                chunk[2] /= len;
            }
        }
    }

    pub fn aabb(&self) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for chunk in self.positions.chunks_exact(3) {
            for axis in 0..3 {
                min[axis] = min[axis].min(chunk[axis]);
                max[axis] = max[axis].max(chunk[axis]);
            }
        }
        (min, max)
    }

    pub fn merge(&mut self, other: &MeshData) {
        let base = self.vertex_count() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.colors.extend_from_slice(&other.colors);
        self.indices
            .extend(other.indices.iter().map(|index| index + base));
    }
}
//#endregion MeshData

//#region Primitives
fn push_triangle(mesh: &mut MeshData, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

pub fn mesh_box(width: f32, height: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        push_triangle(&mut mesh, a, b, c);
        push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_plane(width: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

pub fn mesh_uv_sphere(radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::PI;
        let phi1 = v1 * std::f32::consts::PI;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = sphere_point(radius, phi0, theta0);
            let p10 = sphere_point(radius, phi0, theta1);
            let p01 = sphere_point(radius, phi1, theta0);
            let p11 = sphere_point(radius, phi1, theta1);
            if ring > 0 {
                push_triangle(&mut mesh, p00, p10, p11);
            }
            if ring + 1 < rings {
                push_triangle(&mut mesh, p00, p11, p01);
            }
        }
    }
    mesh.compute_normals();
    mesh
}

fn sphere_point(radius: f32, phi: f32, theta: f32) -> [f32; 3] {
    let sin_phi = phi.sin();
    [
        radius * sin_phi * theta.cos(),
        radius * phi.cos(),
        radius * sin_phi * theta.sin(),
    ]
}

pub fn mesh_ico_sphere(radius: f32, subdivisions: u32) -> MeshData {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        normalize3([-1.0, t, 0.0]),
        normalize3([1.0, t, 0.0]),
        normalize3([-1.0, -t, 0.0]),
        normalize3([1.0, -t, 0.0]),
        normalize3([0.0, -1.0, t]),
        normalize3([0.0, 1.0, t]),
        normalize3([0.0, -1.0, -t]),
        normalize3([0.0, 1.0, -t]),
        normalize3([t, 0.0, -1.0]),
        normalize3([t, 0.0, 1.0]),
        normalize3([-t, 0.0, -1.0]),
        normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces = vec![
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = std::collections::HashMap::new();
        for face in &faces {
            let a = midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[
                [face[0], a, c],
                [face[1], b, a],
                [face[2], c, b],
                [a, b, c],
            ]);
        }
        faces = next;
    }
    let mut mesh = MeshData::default();
    for face in faces {
        let a = scale3(verts[face[0] as usize], radius);
        let b = scale3(verts[face[1] as usize], radius);
        let c = scale3(verts[face[2] as usize], radius);
        push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn midpoint(
    verts: &mut Vec<[f32; 3]>,
    cache: &mut std::collections::HashMap<(u32, u32), u32>,
    a: u32,
    b: u32,
) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = normalize3([
        (verts[a as usize][0] + verts[b as usize][0]) * 0.5,
        (verts[a as usize][1] + verts[b as usize][1]) * 0.5,
        (verts[a as usize][2] + verts[b as usize][2]) * 0.5,
    ]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

pub fn mesh_cylinder(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        push_triangle(&mut mesh, p00, p01, p11);
        push_triangle(&mut mesh, p00, p11, p10);
        push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_cone(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        push_triangle(&mut mesh, apex, p1, p0);
        push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_torus(major_radius: f32, minor_radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::TAU;
        let phi1 = v1 * std::f32::consts::TAU;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = torus_point(major_radius, minor_radius, phi0, theta0);
            let p10 = torus_point(major_radius, minor_radius, phi0, theta1);
            let p01 = torus_point(major_radius, minor_radius, phi1, theta0);
            let p11 = torus_point(major_radius, minor_radius, phi1, theta1);
            push_triangle(&mut mesh, p00, p10, p11);
            push_triangle(&mut mesh, p00, p11, p01);
        }
    }
    mesh.compute_normals();
    mesh
}

fn torus_point(major: f32, minor: f32, phi: f32, theta: f32) -> [f32; 3] {
    let r = major + minor * theta.cos();
    [r * phi.cos(), minor * theta.sin(), r * phi.sin()]
}

pub fn mesh_from_kind(kind: &str) -> MeshData {
    match kind {
        "vortex-marker" => mesh_ico_sphere(0.12, 1),
        "vertex-marker" => mesh_ico_sphere(1.0, 1),
        "sphere" | "uvSphere" => mesh_uv_sphere(0.5, 16, 12),
        "icoSphere" => mesh_ico_sphere(0.5, 1),
        "plane" => mesh_plane(1.0, 1.0),
        "cylinder" => mesh_cylinder(0.5, 1.0, 16),
        "cone" => mesh_cone(0.5, 1.0, 16),
        "torus" => mesh_torus(0.5, 0.15, 16, 12),
        _ => mesh_box(1.0, 1.0, 1.0),
    }
}

/** @emoji 🔩 Builds mesh data from indexed brep tessellation buffers. */
pub fn mesh_from_indexed(positions: &[f32], normals: &[f32], indices: &[u32]) -> MeshData {
    let mut mesh = MeshData {
        positions: positions.to_vec(),
        normals: normals.to_vec(),
        indices: indices.to_vec(),
        ..MeshData::default()
    };
    if mesh.normals.is_empty() && !mesh.positions.is_empty() {
        mesh.compute_normals();
    }
    mesh
}
//#endregion Primitives

//#region Obj
pub fn mesh_to_obj(mesh: &MeshData, object_name: &str) -> String {
    let mut out = format!("o {object_name}\n");
    for chunk in mesh.positions.chunks_exact(3) {
        out.push_str(&format!("v {} {} {}\n", chunk[0], chunk[1], chunk[2]));
    }
    if mesh.normals.len() == mesh.positions.len() {
        for chunk in mesh.normals.chunks_exact(3) {
            out.push_str(&format!("vn {} {} {}\n", chunk[0], chunk[1], chunk[2]));
        }
    }
    for tri in mesh.indices.chunks_exact(3) {
        let a = tri[0] + 1;
        let b = tri[1] + 1;
        let c = tri[2] + 1;
        if mesh.normals.len() == mesh.positions.len() {
            out.push_str(&format!("f {a}//{a} {b}//{b} {c}//{c}\n"));
        } else {
            out.push_str(&format!("f {a} {b} {c}\n"));
        }
    }
    out
}
//#endregion Obj

//#region Glb
pub fn mesh_to_glb(mesh: &MeshData) -> Vec<u8> {
    let positions = f32_slice_to_bytes(&mesh.positions);
    let normals = if mesh.normals.len() == mesh.positions.len() {
        f32_slice_to_bytes(&mesh.normals)
    } else {
        let mut copy = mesh.clone();
        copy.compute_normals();
        f32_slice_to_bytes(&copy.normals)
    };
    let indices = u32_slice_to_bytes(&mesh.indices);
    let bin = [positions.as_slice(), normals.as_slice(), indices.as_slice()].concat();
    let padded_bin = pad_to_4(bin);
    let positions_len = positions.len();
    let normals_len = normals.len();
    let indices_len = indices.len();
    let positions_offset = 0usize;
    let normals_offset = positions_offset + positions_len;
    let indices_offset = normals_offset + normals_len;
    let json = format!(
        r#"{{
  "asset": {{"version": "2.0"}},
  "scene": 0,
  "scenes": [{{"nodes": [0]}}],
  "nodes": [{{"mesh": 0}}],
  "meshes": [{{
    "primitives": [{{
      "attributes": {{"POSITION": 0, "NORMAL": 1}},
      "indices": 2,
      "mode": 4
    }}]
  }}],
  "accessors": [
    {{"bufferView": 0, "componentType": 5126, "count": {}, "type": "VEC3", "min": {}, "max": {}}},
    {{"bufferView": 1, "componentType": 5126, "count": {}, "type": "VEC3"}},
    {{"bufferView": 2, "componentType": 5125, "count": {}, "type": "SCALAR"}}
  ],
  "bufferViews": [
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}}
  ],
  "buffers": [{{"byteLength": {}}}]
}}"#,
        mesh.vertex_count(),
        json_vec3_min(&mesh.positions),
        json_vec3_max(&mesh.positions),
        mesh.vertex_count(),
        mesh.indices.len(),
        positions_offset,
        positions_len,
        normals_offset,
        normals_len,
        indices_offset,
        indices_len,
        padded_bin.len()
    );
    let mut json_bytes = json.into_bytes();
    while json_bytes.len() % 4 != 0 {
        json_bytes.push(b' ');
    }
    let total_len = 12 + 8 + json_bytes.len() + 8 + padded_bin.len();
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&(2u32).to_le_bytes());
    out.extend_from_slice(&(total_len as u32).to_le_bytes());
    out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json_bytes);
    out.extend_from_slice(&(padded_bin.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\x00");
    out.extend_from_slice(&padded_bin);
    out
}

pub fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    if bytes.len() < 12 || &bytes[0..4] != b"glTF" {
        return Err("invalid glb header".into());
    }
    let mut offset = 12usize;
    let mut json = None;
    let mut bin = None;
    while offset + 8 <= bytes.len() {
        let chunk_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        let chunk_type = &bytes[offset + 4..offset + 8];
        offset += 8;
        let end = offset + chunk_len;
        if end > bytes.len() {
            break;
        }
        let chunk = &bytes[offset..end];
        if chunk_type == b"JSON" {
            json = Some(String::from_utf8_lossy(chunk).to_string());
        } else if chunk_type == b"BIN\x00" {
            bin = Some(chunk.to_vec());
        }
        offset = end;
    }
    let json = json.ok_or_else(|| "glb missing json chunk".to_string())?;
    let bin = bin.ok_or_else(|| "glb missing bin chunk".to_string())?;
    let root: serde_json::Value = serde_json::from_str(&json).map_err(|err| err.to_string())?;
    let accessors = root
        .get("accessors")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing accessors".to_string())?;
    let buffer_views = root
        .get("bufferViews")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing bufferViews".to_string())?;
    let meshes = root
        .get("meshes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing meshes".to_string())?;
    let primitive = meshes[0]
        .get("primitives")
        .and_then(|v| v.as_array())
        .and_then(|v| v.first())
        .ok_or_else(|| "glb missing primitive".to_string())?;
    let position_accessor = primitive
        .get("attributes")
        .and_then(|v| v.get("POSITION"))
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "glb missing POSITION".to_string())? as usize;
    let normal_accessor = primitive
        .get("attributes")
        .and_then(|v| v.get("NORMAL"))
        .and_then(|v| v.as_u64());
    let index_accessor = primitive
        .get("indices")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "glb missing indices".to_string())? as usize;
    let positions = read_accessor_f32_vec3(&accessors[position_accessor], &buffer_views, &bin)?;
    let normals = if let Some(index) = normal_accessor {
        read_accessor_f32_vec3(&accessors[index as usize], &buffer_views, &bin)?
    } else {
        Vec::new()
    };
    let indices = read_accessor_u32(&accessors[index_accessor], &buffer_views, &bin)?;
    let mut mesh = MeshData {
        positions,
        normals,
        colors: Vec::new(),
        indices,
        ..Default::default()
    };
    if mesh.normals.is_empty() {
        mesh.compute_normals();
    }
    Ok(mesh)
}

fn read_accessor_f32_vec3(
    accessor: &serde_json::Value,
    buffer_views: &[serde_json::Value],
    bin: &[u8],
) -> Result<Vec<f32>, String> {
    let count = accessor.get("count").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let view_index = accessor.get("bufferView").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let byte_offset = accessor
        .get("byteOffset")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let view = &buffer_views[view_index];
    let view_offset = view.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let start = view_offset + byte_offset;
    let mut out = Vec::with_capacity(count * 3);
    for index in 0..count {
        let base = start + index * 12;
        if base + 12 > bin.len() {
            break;
        }
        for axis in 0..3 {
            let value = f32::from_le_bytes(bin[base + axis * 4..base + axis * 4 + 4].try_into().unwrap());
            out.push(value);
        }
    }
    Ok(out)
}

fn read_accessor_u32(
    accessor: &serde_json::Value,
    buffer_views: &[serde_json::Value],
    bin: &[u8],
) -> Result<Vec<u32>, String> {
    let count = accessor.get("count").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let view_index = accessor.get("bufferView").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let byte_offset = accessor
        .get("byteOffset")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let view = &buffer_views[view_index];
    let view_offset = view.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let start = view_offset + byte_offset;
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let base = start + index * 4;
        if base + 4 > bin.len() {
            break;
        }
        out.push(u32::from_le_bytes(bin[base..base + 4].try_into().unwrap()));
    }
    Ok(out)
}

fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 4 != 0 {
        data.push(0);
    }
    data
}

fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", max[0], max[1], max[2])
}
//#endregion Glb

//#region Dwg
/// 📐 Hand-rolled DWG codec: a self-contained, round-trippable binary interchange format using the AC1015 (R2000) file magic and an R2000-flavored section-locator/CRC/handle container (bit primitives BS/BL/BD/handle refs per https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf). Entity/header field layouts are a semio-defined subset chosen for lossless round-tripping through this codec; byte-exact third-party AutoCAD/ODA interop needs follow-up validation against a real DWG viewer.

//#region DwgTypes
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DwgDrawing {
    pub layers: Vec<DwgLayer>,
    pub entities: Vec<DwgEntity>,
    pub extmin: [f64; 3],
    pub extmax: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct DwgLayer {
    pub name: String,
    pub color: u8,
}

impl Default for DwgLayer {
    fn default() -> Self {
        Self { name: "0".to_string(), color: 7 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DwgColor {
    ByLayer,
    ByBlock,
    Index(u8),
}

impl DwgColor {
    fn to_bs(self) -> u16 {
        match self {
            DwgColor::ByLayer => 256,
            DwgColor::ByBlock => 0,
            DwgColor::Index(index) => index as u16,
        }
    }

    fn from_bs(value: u16) -> Self {
        match value {
            256 => DwgColor::ByLayer,
            0 => DwgColor::ByBlock,
            other => DwgColor::Index(other as u8),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DwgEntity {
    pub layer: usize,
    pub color: DwgColor,
    pub geometry: DwgGeometry,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DwgGeometry {
    Line { start: [f64; 3], end: [f64; 3] },
    Point { at: [f64; 3] },
    Circle { center: [f64; 3], radius: f64, normal: [f64; 3] },
    Arc { center: [f64; 3], radius: f64, start_angle: f64, end_angle: f64, normal: [f64; 3] },
    Ellipse { center: [f64; 3], major_axis: [f64; 3], ratio: f64, start_param: f64, end_param: f64, normal: [f64; 3] },
    LwPolyline { closed: bool, elevation: f64, vertices: Vec<[f64; 2]>, bulges: Vec<f64> },
    Spline { degree: u32, control_points: Vec<[f64; 3]>, knots: Vec<f64>, weights: Vec<f64> },
    Text { at: [f64; 3], height: f64, rotation: f64, content: String },
    Face3d { corners: [[f64; 3]; 4] },
    Polyline3d { closed: bool, vertices: Vec<[f64; 3]> },
    PolyfaceMesh { vertices: Vec<[f64; 3]>, faces: Vec<[i32; 4]> },
}

impl DwgDrawing {
    pub fn ensure_layer(&mut self, name: &str) -> usize {
        if let Some(index) = self.layers.iter().position(|layer| layer.name == name) {
            return index;
        }
        self.layers.push(DwgLayer { name: name.to_string(), color: 7 });
        self.layers.len() - 1
    }

    fn recompute_extents(&mut self) {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        let touch = |p: [f64; 3], min: &mut [f64; 3], max: &mut [f64; 3]| {
            for axis in 0..3 {
                min[axis] = min[axis].min(p[axis]);
                max[axis] = max[axis].max(p[axis]);
            }
        };
        for entity in &self.entities {
            match &entity.geometry {
                DwgGeometry::Line { start, end } => {
                    touch(*start, &mut min, &mut max);
                    touch(*end, &mut min, &mut max);
                }
                DwgGeometry::Point { at } => touch(*at, &mut min, &mut max),
                DwgGeometry::Circle { center, radius, .. } | DwgGeometry::Arc { center, radius, .. } => {
                    touch([center[0] - radius, center[1] - radius, center[2]], &mut min, &mut max);
                    touch([center[0] + radius, center[1] + radius, center[2]], &mut min, &mut max);
                }
                DwgGeometry::Ellipse { center, major_axis, .. } => {
                    let r = (major_axis[0] * major_axis[0] + major_axis[1] * major_axis[1]).sqrt();
                    touch([center[0] - r, center[1] - r, center[2]], &mut min, &mut max);
                    touch([center[0] + r, center[1] + r, center[2]], &mut min, &mut max);
                }
                DwgGeometry::LwPolyline { vertices, elevation, .. } => {
                    for v in vertices {
                        touch([v[0], v[1], *elevation], &mut min, &mut max);
                    }
                }
                DwgGeometry::Spline { control_points, .. } | DwgGeometry::Polyline3d { vertices: control_points, .. } => {
                    for p in control_points {
                        touch(*p, &mut min, &mut max);
                    }
                }
                DwgGeometry::PolyfaceMesh { vertices, .. } => {
                    for p in vertices {
                        touch(*p, &mut min, &mut max);
                    }
                }
                DwgGeometry::Text { at, .. } => touch(*at, &mut min, &mut max),
                DwgGeometry::Face3d { corners } => {
                    for p in corners {
                        touch(*p, &mut min, &mut max);
                    }
                }
            }
        }
        if min[0].is_finite() {
            self.extmin = min;
            self.extmax = max;
        }
    }
}
//#endregion DwgTypes

//#region DwgBits
struct DwgBitWriter {
    bytes: Vec<u8>,
    bit: u8,
}

impl DwgBitWriter {
    fn new() -> Self {
        Self { bytes: Vec::new(), bit: 0 }
    }

    fn write_bit(&mut self, value: bool) {
        if self.bit == 0 {
            self.bytes.push(0);
        }
        if value {
            let last = self.bytes.len() - 1;
            self.bytes[last] |= 1 << (7 - self.bit);
        }
        self.bit = (self.bit + 1) % 8;
    }

    fn write_bits(&mut self, value: u64, count: u8) {
        for i in (0..count).rev() {
            self.write_bit((value >> i) & 1 != 0);
        }
    }

    fn write_b(&mut self, value: bool) {
        self.write_bit(value);
    }

    fn write_bb(&mut self, value: u8) {
        self.write_bits(value as u64, 2);
    }

    fn write_rc(&mut self, value: u8) {
        self.write_bits(value as u64, 8);
    }

    fn write_rs(&mut self, value: u16) {
        self.write_rc((value & 0xFF) as u8);
        self.write_rc((value >> 8) as u8);
    }

    fn write_rl(&mut self, value: u32) {
        self.write_rs((value & 0xFFFF) as u16);
        self.write_rs((value >> 16) as u16);
    }

    fn write_rd(&mut self, value: f64) {
        let bits = value.to_bits();
        self.write_rl((bits & 0xFFFF_FFFF) as u32);
        self.write_rl((bits >> 32) as u32);
    }

    fn write_bs(&mut self, value: u16) {
        match value {
            0 => self.write_bb(2),
            256 => self.write_bb(3),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rs(v);
            }
        }
    }

    fn write_bl(&mut self, value: u32) {
        match value {
            0 => self.write_bb(2),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rl(v);
            }
        }
    }

    fn write_bd(&mut self, value: f64) {
        if value == 0.0 {
            self.write_bb(2);
        } else if value == 1.0 {
            self.write_bb(1);
        } else {
            self.write_bb(0);
            self.write_rd(value);
        }
    }

    fn write_2rd(&mut self, v: [f64; 2]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
    }

    fn write_3bd(&mut self, v: [f64; 3]) {
        self.write_bd(v[0]);
        self.write_bd(v[1]);
        self.write_bd(v[2]);
    }

    fn write_3rd(&mut self, v: [f64; 3]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
        self.write_rd(v[2]);
    }

    fn write_be(&mut self, normal: [f64; 3]) {
        if normal == [0.0, 0.0, 1.0] {
            self.write_b(true);
        } else {
            self.write_b(false);
            self.write_3bd(normal);
        }
    }

    fn write_t(&mut self, text: &str) {
        let bytes = text.as_bytes();
        let len = bytes.len().min(0xFFFF);
        self.write_rs(len as u16);
        for &b in &bytes[..len] {
            self.write_rc(b);
        }
    }

    fn write_ms(&mut self, mut value: u32) {
        loop {
            let mut chunk = (value & 0x7FFF) as u16;
            value >>= 15;
            if value != 0 {
                chunk |= 0x8000;
                self.write_rs(chunk);
            } else {
                self.write_rs(chunk);
                break;
            }
        }
    }

    fn write_handle(&mut self, code: u8, handle: u64) {
        let mut bytes = Vec::new();
        let mut v = handle;
        while v != 0 {
            bytes.insert(0, (v & 0xFF) as u8);
            v >>= 8;
        }
        self.write_rc((code << 4) | bytes.len() as u8);
        for b in bytes {
            self.write_rc(b);
        }
    }

    fn pad_to_byte(&mut self) {
        while self.bit != 0 {
            self.write_bit(false);
        }
    }

    fn bit_len(&self) -> usize {
        self.bytes.len() * 8 - if self.bit == 0 { 0 } else { 8 - self.bit as usize }
    }
}

struct DwgBitReader<'a> {
    bytes: &'a [u8],
    byte_pos: usize,
    bit: u8,
}

impl<'a> DwgBitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, byte_pos: 0, bit: 0 }
    }

    fn read_bit(&mut self) -> Result<bool, String> {
        if self.byte_pos >= self.bytes.len() {
            return Err("dwg bitstream underflow".to_string());
        }
        let value = (self.bytes[self.byte_pos] >> (7 - self.bit)) & 1 != 0;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.byte_pos += 1;
        }
        Ok(value)
    }

    fn read_bits(&mut self, count: u8) -> Result<u64, String> {
        let mut value = 0u64;
        for _ in 0..count {
            value = (value << 1) | self.read_bit()? as u64;
        }
        Ok(value)
    }

    fn read_b(&mut self) -> Result<bool, String> {
        self.read_bit()
    }

    fn read_bb(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(2)? as u8)
    }

    fn read_rc(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(8)? as u8)
    }

    fn read_rs(&mut self) -> Result<u16, String> {
        let lo = self.read_rc()? as u16;
        let hi = self.read_rc()? as u16;
        Ok(lo | (hi << 8))
    }

    fn read_rl(&mut self) -> Result<u32, String> {
        let lo = self.read_rs()? as u32;
        let hi = self.read_rs()? as u32;
        Ok(lo | (hi << 16))
    }

    fn read_rd(&mut self) -> Result<f64, String> {
        let lo = self.read_rl()? as u64;
        let hi = self.read_rl()? as u64;
        Ok(f64::from_bits(lo | (hi << 32)))
    }

    fn read_bs(&mut self) -> Result<u16, String> {
        match self.read_bb()? {
            0 => self.read_rs(),
            1 => Ok(self.read_rc()? as u16),
            2 => Ok(0),
            _ => Ok(256),
        }
    }

    fn read_bl(&mut self) -> Result<u32, String> {
        match self.read_bb()? {
            0 => self.read_rl(),
            1 => Ok(self.read_rc()? as u32),
            2 => Ok(0),
            _ => Err("invalid BL flag".to_string()),
        }
    }

    fn read_bd(&mut self) -> Result<f64, String> {
        match self.read_bb()? {
            0 => self.read_rd(),
            1 => Ok(1.0),
            2 => Ok(0.0),
            _ => Err("invalid BD flag".to_string()),
        }
    }

    fn read_2rd(&mut self) -> Result<[f64; 2], String> {
        Ok([self.read_rd()?, self.read_rd()?])
    }

    fn read_3bd(&mut self) -> Result<[f64; 3], String> {
        Ok([self.read_bd()?, self.read_bd()?, self.read_bd()?])
    }

    fn read_be(&mut self) -> Result<[f64; 3], String> {
        if self.read_b()? {
            Ok([0.0, 0.0, 1.0])
        } else {
            self.read_3bd()
        }
    }

    fn read_t(&mut self) -> Result<String, String> {
        let len = self.read_rs()? as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.read_rc()?);
        }
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    fn read_ms(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let chunk = self.read_rs()?;
            value |= ((chunk & 0x7FFF) as u32) << shift;
            shift += 15;
            if chunk & 0x8000 == 0 {
                break;
            }
        }
        Ok(value)
    }

    fn read_handle(&mut self) -> Result<(u8, u64), String> {
        let head = self.read_rc()?;
        let code = head >> 4;
        let len = head & 0x0F;
        let mut value = 0u64;
        for _ in 0..len {
            value = (value << 8) | self.read_rc()? as u64;
        }
        Ok((code, value))
    }

    fn pad_to_byte(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.byte_pos += 1;
        }
    }
}

fn dwg_crc16(seed: u16, data: &[u8]) -> u16 {
    let mut crc = seed;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}
//#endregion DwgBits

//#region DwgObjects
const DWG_TYPE_LAYER: u16 = 51;
const DWG_TYPE_LINE: u16 = 19;
const DWG_TYPE_POINT: u16 = 27;
const DWG_TYPE_CIRCLE: u16 = 18;
const DWG_TYPE_ARC: u16 = 17;
const DWG_TYPE_ELLIPSE: u16 = 35;
const DWG_TYPE_LWPOLYLINE: u16 = 77;
const DWG_TYPE_SPLINE: u16 = 36;
const DWG_TYPE_TEXT: u16 = 1;
const DWG_TYPE_FACE3D: u16 = 28;
const DWG_TYPE_POLYLINE3D: u16 = 16;
const DWG_TYPE_POLYLINE_PFACE: u16 = 29;

const HANDLE_MODEL_SPACE: u64 = 0x10;
const HANDLE_LAYER_BASE: u64 = 0x20;
const HANDLE_ENTITY_BASE: u64 = 0x1000;

fn dwg_write_object(out: &mut Vec<u8>, object_type: u16, handle: u64, body: &mut DwgBitWriter, handles: &mut DwgBitWriter) {
    let bitsize = body.bit_len() as u32;
    body.pad_to_byte();
    handles.pad_to_byte();

    let mut framed = DwgBitWriter::new();
    framed.write_bs(object_type);
    framed.write_rl(bitsize);
    framed.write_handle(0, handle);
    framed.pad_to_byte();
    for byte in &body.bytes {
        framed.bytes.push(*byte);
    }
    for byte in &handles.bytes {
        framed.bytes.push(*byte);
    }

    let payload = framed.bytes;
    let mut sized = DwgBitWriter::new();
    sized.write_ms(payload.len() as u32);
    sized.pad_to_byte();

    out.extend_from_slice(&sized.bytes);
    out.extend_from_slice(&payload);
    let crc = dwg_crc16(0xC0C1, &payload);
    out.extend_from_slice(&crc.to_le_bytes());
}

fn dwg_encode_entity_common(body: &mut DwgBitWriter, handles: &mut DwgBitWriter, layer_handle: u64, color: DwgColor) {
    body.write_bb(0);
    body.write_bl(0);
    body.write_b(true);
    body.write_bs(color.to_bs());
    body.write_bd(1.0);
    body.write_bb(0);
    body.write_bb(0);
    body.write_bs(0);
    body.write_rc(29);

    handles.write_handle(3, HANDLE_MODEL_SPACE);
    handles.write_handle(5, layer_handle);
}

fn dwg_decode_entity_common(reader: &mut DwgBitReader) -> Result<DwgColor, String> {
    let _entmode = reader.read_bb()?;
    let _numreactors = reader.read_bl()?;
    let _nolinks = reader.read_b()?;
    let color = DwgColor::from_bs(reader.read_bs()?);
    let _ltype_scale = reader.read_bd()?;
    let _ltype_flags = reader.read_bb()?;
    let _plotstyle_flags = reader.read_bb()?;
    let _invisibility = reader.read_bs()?;
    let _lineweight = reader.read_rc()?;
    Ok(color)
}

fn dwg_decode_entity_handles(reader: &mut DwgBitReader) -> Result<u64, String> {
    reader.pad_to_byte();
    let (_owner_code, _owner) = reader.read_handle()?;
    let (_layer_code, layer_handle) = reader.read_handle()?;
    Ok(layer_handle)
}

fn dwg_encode_entity(objects_bytes: &mut Vec<u8>, object_map: &mut Vec<(u64, usize)>, next_handle: &mut u64, layer_handle: u64, entity: &DwgEntity) {
    let handle = *next_handle;
    *next_handle += 1;
    let mut body = DwgBitWriter::new();
    let mut handles = DwgBitWriter::new();
    dwg_encode_entity_common(&mut body, &mut handles, layer_handle, entity.color);

    let object_type = match &entity.geometry {
        DwgGeometry::Line { start, end } => {
            body.write_3bd(*start);
            body.write_3bd(*end);
            DWG_TYPE_LINE
        }
        DwgGeometry::Point { at } => {
            body.write_3bd(*at);
            DWG_TYPE_POINT
        }
        DwgGeometry::Circle { center, radius, normal } => {
            body.write_3bd(*center);
            body.write_bd(*radius);
            body.write_be(*normal);
            DWG_TYPE_CIRCLE
        }
        DwgGeometry::Arc { center, radius, start_angle, end_angle, normal } => {
            body.write_3bd(*center);
            body.write_bd(*radius);
            body.write_bd(*start_angle);
            body.write_bd(*end_angle);
            body.write_be(*normal);
            DWG_TYPE_ARC
        }
        DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal } => {
            body.write_3bd(*center);
            body.write_3bd(*major_axis);
            body.write_be(*normal);
            body.write_bd(*ratio);
            body.write_bd(*start_param);
            body.write_bd(*end_param);
            DWG_TYPE_ELLIPSE
        }
        DwgGeometry::Text { at, height, rotation, content } => {
            body.write_3bd(*at);
            body.write_bd(*height);
            body.write_bd(*rotation);
            body.write_t(content);
            DWG_TYPE_TEXT
        }
        DwgGeometry::Face3d { corners } => {
            for corner in corners {
                body.write_3bd(*corner);
            }
            DWG_TYPE_FACE3D
        }
        DwgGeometry::LwPolyline { closed, elevation, vertices, bulges } => {
            body.write_b(*closed);
            body.write_bd(*elevation);
            body.write_bl(vertices.len() as u32);
            for (i, v) in vertices.iter().enumerate() {
                body.write_2rd(*v);
                body.write_bd(bulges.get(i).copied().unwrap_or(0.0));
            }
            DWG_TYPE_LWPOLYLINE
        }
        DwgGeometry::Spline { degree, control_points, knots, weights } => {
            body.write_bl(*degree);
            body.write_bl(control_points.len() as u32);
            for p in control_points {
                body.write_3bd(*p);
            }
            body.write_bl(knots.len() as u32);
            for k in knots {
                body.write_rd(*k);
            }
            body.write_bl(weights.len() as u32);
            for w in weights {
                body.write_rd(*w);
            }
            DWG_TYPE_SPLINE
        }
        DwgGeometry::Polyline3d { closed, vertices } => {
            body.write_b(*closed);
            body.write_bl(vertices.len() as u32);
            for v in vertices {
                body.write_3bd(*v);
            }
            DWG_TYPE_POLYLINE3D
        }
        DwgGeometry::PolyfaceMesh { vertices, faces } => {
            body.write_bl(vertices.len() as u32);
            for v in vertices {
                body.write_3bd(*v);
            }
            body.write_bl(faces.len() as u32);
            for face in faces {
                for idx in face {
                    body.write_bl(idx.unsigned_abs());
                    body.write_b(*idx < 0);
                }
            }
            DWG_TYPE_POLYLINE_PFACE
        }
    };

    let offset = objects_bytes.len();
    dwg_write_object(objects_bytes, object_type, handle, &mut body, &mut handles);
    object_map.push((handle, offset));
}

fn dwg_decode_entity(object_type: u16, reader: &mut DwgBitReader) -> Result<Option<(u64, DwgColor, DwgGeometry)>, String> {
    match object_type {
        DWG_TYPE_LINE => {
            let color = dwg_decode_entity_common(reader)?;
            let start = reader.read_3bd()?;
            let end = reader.read_3bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Line { start, end })))
        }
        DWG_TYPE_POINT => {
            let color = dwg_decode_entity_common(reader)?;
            let at = reader.read_3bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Point { at })))
        }
        DWG_TYPE_CIRCLE => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let radius = reader.read_bd()?;
            let normal = reader.read_be()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Circle { center, radius, normal })))
        }
        DWG_TYPE_ARC => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let radius = reader.read_bd()?;
            let start_angle = reader.read_bd()?;
            let end_angle = reader.read_bd()?;
            let normal = reader.read_be()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Arc { center, radius, start_angle, end_angle, normal })))
        }
        DWG_TYPE_ELLIPSE => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let major_axis = reader.read_3bd()?;
            let normal = reader.read_be()?;
            let ratio = reader.read_bd()?;
            let start_param = reader.read_bd()?;
            let end_param = reader.read_bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal })))
        }
        DWG_TYPE_TEXT => {
            let color = dwg_decode_entity_common(reader)?;
            let at = reader.read_3bd()?;
            let height = reader.read_bd()?;
            let rotation = reader.read_bd()?;
            let content = reader.read_t()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Text { at, height, rotation, content })))
        }
        DWG_TYPE_FACE3D => {
            let color = dwg_decode_entity_common(reader)?;
            let corners = [reader.read_3bd()?, reader.read_3bd()?, reader.read_3bd()?, reader.read_3bd()?];
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Face3d { corners })))
        }
        DWG_TYPE_LWPOLYLINE => {
            let color = dwg_decode_entity_common(reader)?;
            let closed = reader.read_b()?;
            let elevation = reader.read_bd()?;
            let count = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(count);
            let mut bulges = Vec::with_capacity(count);
            for _ in 0..count {
                vertices.push(reader.read_2rd()?);
                bulges.push(reader.read_bd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::LwPolyline { closed, elevation, vertices, bulges })))
        }
        DWG_TYPE_SPLINE => {
            let color = dwg_decode_entity_common(reader)?;
            let degree = reader.read_bl()?;
            let cp_count = reader.read_bl()? as usize;
            let mut control_points = Vec::with_capacity(cp_count);
            for _ in 0..cp_count {
                control_points.push(reader.read_3bd()?);
            }
            let knot_count = reader.read_bl()? as usize;
            let mut knots = Vec::with_capacity(knot_count);
            for _ in 0..knot_count {
                knots.push(reader.read_rd()?);
            }
            let weight_count = reader.read_bl()? as usize;
            let mut weights = Vec::with_capacity(weight_count);
            for _ in 0..weight_count {
                weights.push(reader.read_rd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Spline { degree, control_points, knots, weights })))
        }
        DWG_TYPE_POLYLINE3D => {
            let color = dwg_decode_entity_common(reader)?;
            let closed = reader.read_b()?;
            let count = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(count);
            for _ in 0..count {
                vertices.push(reader.read_3bd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Polyline3d { closed, vertices })))
        }
        DWG_TYPE_POLYLINE_PFACE => {
            let color = dwg_decode_entity_common(reader)?;
            let vcount = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(vcount);
            for _ in 0..vcount {
                vertices.push(reader.read_3bd()?);
            }
            let fcount = reader.read_bl()? as usize;
            let mut faces = Vec::with_capacity(fcount);
            for _ in 0..fcount {
                let mut face = [0i32; 4];
                for slot in face.iter_mut() {
                    let magnitude = reader.read_bl()? as i32;
                    let negative = reader.read_b()?;
                    *slot = if negative { -magnitude } else { magnitude };
                }
                faces.push(face);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::PolyfaceMesh { vertices, faces })))
        }
        _ => Ok(None),
    }
}
//#endregion DwgObjects

//#region DwgWrite
const DWG_FILE_HEADER_LEN: usize = 55;
const DWG_SENTINEL_HEADER_VARS_BEGIN: [u8; 16] = [0xCF, 0x7B, 0x1F, 0x23, 0xFD, 0xDE, 0x38, 0xA9, 0x5F, 0x7C, 0x68, 0xB8, 0x4E, 0x6D, 0x33, 0x5F];
const DWG_SENTINEL_HEADER_VARS_END: [u8; 16] = [0x30, 0x84, 0xE0, 0xDC, 0x02, 0x21, 0xC7, 0x56, 0xA0, 0x83, 0x97, 0x47, 0xB1, 0x92, 0xCC, 0xA0];
const DWG_SENTINEL_CLASSES_BEGIN: [u8; 16] = [0x8D, 0xA1, 0xC4, 0xB8, 0xC4, 0xA9, 0xF8, 0xC5, 0xC0, 0xDC, 0xF4, 0x5F, 0xE7, 0xCF, 0xB6, 0x8A];
const DWG_SENTINEL_CLASSES_END: [u8; 16] = [0x72, 0x5E, 0x3B, 0x47, 0x3B, 0x56, 0x07, 0x3A, 0x3F, 0x23, 0x0B, 0xA0, 0x18, 0x30, 0x49, 0x75];
const DWG_SENTINEL_FILE_HEADER_END: [u8; 16] = [0x95, 0xA0, 0x4E, 0x28, 0x99, 0x82, 0x1A, 0xE5, 0x5E, 0x41, 0xE0, 0x5F, 0x9D, 0x3A, 0x4D, 0x00];

/// 📐 Serializes a drawing to a semio DWG (AC1015-flavored) byte stream.
pub fn dwg_to_bytes(drawing: &DwgDrawing) -> Result<Vec<u8>, String> {
    let mut drawing = drawing.clone();
    if drawing.layers.is_empty() {
        drawing.layers.push(DwgLayer::default());
    }
    drawing.recompute_extents();

    let layer_handles: Vec<u64> = (0..drawing.layers.len()).map(|i| HANDLE_LAYER_BASE + i as u64).collect();
    let mut objects_bytes = Vec::new();
    let mut object_map: Vec<(u64, usize)> = Vec::new();

    for (i, layer) in drawing.layers.iter().enumerate() {
        let handle = layer_handles[i];
        let mut body = DwgBitWriter::new();
        body.write_t(&layer.name);
        body.write_rc(layer.color);
        let mut handles = DwgBitWriter::new();
        let offset = objects_bytes.len();
        dwg_write_object(&mut objects_bytes, DWG_TYPE_LAYER, handle, &mut body, &mut handles);
        object_map.push((handle, offset));
    }

    let mut next_handle = HANDLE_ENTITY_BASE;
    for entity in &drawing.entities {
        let layer_handle = layer_handles.get(entity.layer).copied().unwrap_or(layer_handles[0]);
        dwg_encode_entity(&mut objects_bytes, &mut object_map, &mut next_handle, layer_handle, entity);
    }

    let mut header_body = DwgBitWriter::new();
    header_body.write_3rd(drawing.extmin);
    header_body.write_3rd(drawing.extmax);
    header_body.write_handle(0, next_handle);
    header_body.pad_to_byte();
    let header_payload = header_body.bytes;
    let header_crc = dwg_crc16(0xC0C1, &header_payload);

    let mut header_section = Vec::new();
    header_section.extend_from_slice(&DWG_SENTINEL_HEADER_VARS_BEGIN);
    header_section.extend_from_slice(&(header_payload.len() as u32).to_le_bytes());
    header_section.extend_from_slice(&header_payload);
    header_section.extend_from_slice(&header_crc.to_le_bytes());
    header_section.extend_from_slice(&DWG_SENTINEL_HEADER_VARS_END);

    let mut classes_section = Vec::new();
    classes_section.extend_from_slice(&DWG_SENTINEL_CLASSES_BEGIN);
    classes_section.extend_from_slice(&0u32.to_le_bytes());
    classes_section.extend_from_slice(&dwg_crc16(0xC0C1, &[]).to_le_bytes());
    classes_section.extend_from_slice(&DWG_SENTINEL_CLASSES_END);

    let header_vars_offset = DWG_FILE_HEADER_LEN;
    let classes_offset = header_vars_offset + header_section.len();
    let objects_offset = classes_offset + classes_section.len();
    let object_map_offset = objects_offset + objects_bytes.len();

    let mut map_section = Vec::new();
    map_section.extend_from_slice(&(object_map.len() as u32).to_le_bytes());
    for (handle, local_offset) in &object_map {
        map_section.extend_from_slice(&handle.to_le_bytes());
        map_section.extend_from_slice(&((objects_offset + local_offset) as u64).to_le_bytes());
    }
    let map_crc = dwg_crc16(0xC0C1, &map_section);
    map_section.extend_from_slice(&map_crc.to_le_bytes());

    let mut file_header = Vec::new();
    file_header.extend_from_slice(b"AC1015");
    file_header.extend_from_slice(&3u32.to_le_bytes());
    let locators: [(u8, u32, u32); 3] = [
        (0, header_vars_offset as u32, header_section.len() as u32),
        (1, classes_offset as u32, classes_section.len() as u32),
        (2, object_map_offset as u32, map_section.len() as u32),
    ];
    for (num, seeker, size) in locators {
        file_header.push(num);
        file_header.extend_from_slice(&seeker.to_le_bytes());
        file_header.extend_from_slice(&size.to_le_bytes());
    }
    let locator_crc = dwg_crc16(0, &file_header) ^ 0x8461;
    file_header.extend_from_slice(&locator_crc.to_le_bytes());
    file_header.extend_from_slice(&DWG_SENTINEL_FILE_HEADER_END);
    debug_assert_eq!(file_header.len(), DWG_FILE_HEADER_LEN);

    let mut out = Vec::with_capacity(object_map_offset + map_section.len());
    out.extend_from_slice(&file_header);
    out.extend_from_slice(&header_section);
    out.extend_from_slice(&classes_section);
    out.extend_from_slice(&objects_bytes);
    out.extend_from_slice(&map_section);
    Ok(out)
}
//#endregion DwgWrite

//#region DwgRead
/// 📐 Parses a semio DWG (AC1015-flavored) byte stream, tolerating and skipping unrecognized or malformed objects.
pub fn dwg_from_bytes(bytes: &[u8]) -> Result<DwgDrawing, String> {
    if bytes.len() < 6 || &bytes[0..6] != b"AC1015" {
        let found = String::from_utf8_lossy(bytes.get(0..6).unwrap_or(b"??????")).to_string();
        return Err(format!("unsupported dwg version '{found}': only AC1015 (R2000) is supported"));
    }
    if bytes.len() < DWG_FILE_HEADER_LEN {
        return Err("dwg file header truncated".to_string());
    }
    let section_count = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
    let mut cursor = 10usize;
    let mut locators: Vec<(u8, usize, usize)> = Vec::new();
    for _ in 0..section_count.min(16) {
        if cursor + 9 > bytes.len() {
            return Err("dwg section locator truncated".to_string());
        }
        let num = bytes[cursor];
        let seeker = u32::from_le_bytes(bytes[cursor + 1..cursor + 5].try_into().unwrap()) as usize;
        let size = u32::from_le_bytes(bytes[cursor + 5..cursor + 9].try_into().unwrap()) as usize;
        locators.push((num, seeker, size));
        cursor += 9;
    }

    let (_, map_offset, map_size) = *locators
        .iter()
        .find(|(num, _, _)| *num == 2)
        .ok_or_else(|| "dwg missing object map locator".to_string())?;
    if map_offset + map_size > bytes.len() || map_size < 4 {
        return Err("dwg object map out of bounds".to_string());
    }
    let map_bytes = &bytes[map_offset..map_offset + map_size];
    let count = u32::from_le_bytes(map_bytes[0..4].try_into().unwrap()) as usize;
    let mut entries = Vec::with_capacity(count);
    let mut pos = 4usize;
    for _ in 0..count {
        if pos + 16 > map_bytes.len() {
            break;
        }
        let handle = u64::from_le_bytes(map_bytes[pos..pos + 8].try_into().unwrap());
        let address = u64::from_le_bytes(map_bytes[pos + 8..pos + 16].try_into().unwrap()) as usize;
        entries.push((handle, address));
        pos += 16;
    }

    let mut layers: Vec<DwgLayer> = Vec::new();
    let mut layer_handle_index: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut pending_entities: Vec<(u64, DwgColor, DwgGeometry)> = Vec::new();

    for (handle, address) in &entries {
        if *address >= bytes.len() {
            continue;
        }
        let mut sizer = DwgBitReader::new(&bytes[*address..]);
        let payload_len = match sizer.read_ms() {
            Ok(v) => v as usize,
            Err(_) => continue,
        };
        sizer.pad_to_byte();
        let payload_start = address + sizer.byte_pos;
        if payload_start + payload_len > bytes.len() {
            continue;
        }
        let payload = &bytes[payload_start..payload_start + payload_len];
        let mut reader = DwgBitReader::new(payload);
        let object_type = match reader.read_bs() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let _bitsize = match reader.read_rl() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if reader.read_handle().is_err() {
            continue;
        }
        reader.pad_to_byte();

        if object_type == DWG_TYPE_LAYER {
            if let (Ok(name), Ok(color)) = (reader.read_t(), reader.read_rc()) {
                layer_handle_index.insert(*handle, layers.len());
                layers.push(DwgLayer { name, color });
            }
            continue;
        }

        if let Ok(Some((layer_handle, color, geometry))) = dwg_decode_entity(object_type, &mut reader) {
            pending_entities.push((layer_handle, color, geometry));
        }
    }

    if layers.is_empty() {
        layers.push(DwgLayer::default());
    }

    let entities = pending_entities
        .into_iter()
        .map(|(layer_handle, color, geometry)| DwgEntity {
            layer: layer_handle_index.get(&layer_handle).copied().unwrap_or(0),
            color,
            geometry,
        })
        .collect();

    let mut drawing = DwgDrawing { layers, entities, extmin: [0.0; 3], extmax: [0.0; 3] };
    drawing.recompute_extents();
    Ok(drawing)
}
//#endregion DwgRead

//#region DwgBridges
/// 🔺 Wraps mesh data as a single polyface-mesh drawing.
pub fn mesh_to_dwg_drawing(mesh: &MeshData) -> DwgDrawing {
    let vertices: Vec<[f64; 3]> = mesh.positions.chunks_exact(3).map(|c| [c[0] as f64, c[1] as f64, c[2] as f64]).collect();
    let faces: Vec<[i32; 4]> = mesh
        .indices
        .chunks_exact(3)
        .map(|tri| [tri[0] as i32 + 1, tri[1] as i32 + 1, tri[2] as i32 + 1, tri[2] as i32 + 1])
        .collect();
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices, faces } });
    drawing.recompute_extents();
    drawing
}

/// 🔺 Collects polyface-mesh and 3dface entities into mesh data.
pub fn dwg_drawing_to_mesh(drawing: &DwgDrawing) -> MeshData {
    let mut mesh = MeshData::default();
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::PolyfaceMesh { vertices, faces } => {
                let base = mesh.vertex_count() as u32;
                for v in vertices {
                    mesh.positions.extend_from_slice(&[v[0] as f32, v[1] as f32, v[2] as f32]);
                }
                for face in faces {
                    let idx: Vec<u32> = face.iter().map(|i| (i.unsigned_abs().saturating_sub(1)) + base).collect();
                    if face[2] == face[3] {
                        mesh.indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                    } else {
                        mesh.indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                        mesh.indices.extend_from_slice(&[idx[0], idx[2], idx[3]]);
                    }
                }
            }
            DwgGeometry::Face3d { corners } => {
                let base = mesh.vertex_count() as u32;
                for c in corners {
                    mesh.positions.extend_from_slice(&[c[0] as f32, c[1] as f32, c[2] as f32]);
                }
                mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
                if corners[3] != corners[2] {
                    mesh.indices.extend_from_slice(&[base, base + 2, base + 3]);
                }
            }
            _ => {}
        }
    }
    mesh.compute_normals();
    mesh
}

/// ✏️ Path segment mirror of the 2d kernel's PathSegment (kernel/2d/engine/rs/lib.rs), kept dependency-free.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DwgPathSegment {
    Move { to: [f64; 2] },
    Line { to: [f64; 2] },
    Quad { ctrl: [f64; 2], to: [f64; 2] },
    Cubic { ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2] },
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: [f64; 2] },
    Close,
}

fn arc_bulge(from: [f64; 2], to: [f64; 2], radius: f64, sweep: bool) -> f64 {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    if chord < 1e-9 || radius < 1e-9 {
        return 0.0;
    }
    let included_angle = 2.0 * (chord * 0.5 / radius).clamp(-1.0, 1.0).asin();
    let bulge = (included_angle / 4.0).tan();
    if sweep {
        bulge
    } else {
        -bulge
    }
}

fn bulge_to_segment(from: [f64; 2], to: [f64; 2], bulge: f64) -> DwgPathSegment {
    if bulge.abs() < 1e-9 {
        return DwgPathSegment::Line { to };
    }
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    let included_angle = 4.0 * bulge.atan();
    let denom = (2.0 * (included_angle / 2.0).sin()).abs();
    let radius = if denom > 1e-9 { chord / denom } else { 0.0 };
    DwgPathSegment::Arc { rx: radius, ry: radius, rotation: 0.0, large_arc: included_angle.abs() > std::f64::consts::PI, sweep: bulge > 0.0, to }
}

/// ✏️ Converts flattened path segments to dwg entities: line/close runs to lwpolylines with bulge arcs, curves to splines.
pub fn paths_to_dwg_drawing(paths: &[Vec<DwgPathSegment>]) -> DwgDrawing {
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    for path in paths {
        let mut vertices: Vec<[f64; 2]> = Vec::new();
        let mut bulges: Vec<f64> = Vec::new();
        let mut closed = false;
        let mut cursor = [0.0, 0.0];
        let mut start = [0.0, 0.0];
        for segment in path {
            match segment {
                DwgPathSegment::Move { to } => {
                    if !vertices.is_empty() {
                        drawing.entities.push(DwgEntity {
                            layer,
                            color: DwgColor::ByLayer,
                            geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices: vertices.clone(), bulges: bulges.clone() },
                        });
                        vertices.clear();
                        bulges.clear();
                        closed = false;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                    start = *to;
                }
                DwgPathSegment::Line { to } => {
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Quad { ctrl, to } => {
                    let c1 = [cursor[0] + 2.0 / 3.0 * (ctrl[0] - cursor[0]), cursor[1] + 2.0 / 3.0 * (ctrl[1] - cursor[1])];
                    let c2 = [to[0] + 2.0 / 3.0 * (ctrl[0] - to[0]), to[1] + 2.0 / 3.0 * (ctrl[1] - to[1])];
                    let spline_points = [cursor, c1, c2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline {
                            degree: 3,
                            control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(),
                            knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                            weights: vec![1.0; 4],
                        },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Cubic { ctrl1, ctrl2, to } => {
                    let spline_points = [cursor, *ctrl1, *ctrl2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline {
                            degree: 3,
                            control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(),
                            knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                            weights: vec![1.0; 4],
                        },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Arc { rx, sweep, to, .. } => {
                    let bulge = arc_bulge(cursor, *to, *rx, *sweep);
                    if let Some(last) = bulges.last_mut() {
                        *last = bulge;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Close => {
                    closed = true;
                    cursor = start;
                }
            }
        }
        if !vertices.is_empty() {
            drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices, bulges } });
        }
    }
    drawing.recompute_extents();
    drawing
}

/// ✏️ Converts drawing entities back to path segments, one path per entity.
pub fn dwg_drawing_to_paths(drawing: &DwgDrawing) -> Vec<Vec<DwgPathSegment>> {
    let mut paths = Vec::new();
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::LwPolyline { closed, vertices, bulges, .. } => {
                if vertices.is_empty() {
                    continue;
                }
                let mut segments = vec![DwgPathSegment::Move { to: vertices[0] }];
                for i in 1..vertices.len() {
                    let from = vertices[i - 1];
                    let to = vertices[i];
                    let bulge = bulges.get(i - 1).copied().unwrap_or(0.0);
                    segments.push(bulge_to_segment(from, to, bulge));
                }
                if *closed && vertices.len() > 1 {
                    let bulge = bulges.last().copied().unwrap_or(0.0);
                    segments.push(bulge_to_segment(vertices[vertices.len() - 1], vertices[0], bulge));
                    segments.push(DwgPathSegment::Close);
                }
                paths.push(segments);
            }
            DwgGeometry::Spline { degree, control_points, .. } if *degree == 3 && control_points.len() == 4 => {
                paths.push(vec![
                    DwgPathSegment::Move { to: [control_points[0][0], control_points[0][1]] },
                    DwgPathSegment::Cubic {
                        ctrl1: [control_points[1][0], control_points[1][1]],
                        ctrl2: [control_points[2][0], control_points[2][1]],
                        to: [control_points[3][0], control_points[3][1]],
                    },
                ]);
            }
            DwgGeometry::Circle { center, radius, .. } => {
                paths.push(vec![
                    DwgPathSegment::Move { to: [center[0] + radius, center[1]] },
                    DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: true, to: [center[0] - radius, center[1]] },
                    DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: true, to: [center[0] + radius, center[1]] },
                    DwgPathSegment::Close,
                ]);
            }
            _ => {}
        }
    }
    paths
}
//#endregion DwgBridges
//#endregion Dwg

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_has_triangles() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        assert_eq!(mesh.triangle_count(), 12);
        assert_eq!(mesh.normals.len(), mesh.positions.len());
    }

    #[test]
    fn obj_contains_faces() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        let obj = mesh_to_obj(&mesh, "box");
        assert!(obj.contains("o box"));
        assert!(obj.contains("f "));
    }

    #[test]
    fn glb_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6);
        let glb = mesh_to_glb(&mesh);
        let decoded = mesh_from_glb(&glb).expect("decode glb");
        assert_eq!(decoded.vertex_count(), mesh.vertex_count());
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    #[test]
    fn primitive_kinds() {
        assert!(mesh_from_kind("sphere").vertex_count() > 0);
        assert!(mesh_from_kind("box").vertex_count() > 0);
    }

    #[test]
    fn dwg_bit_primitives_round_trip_at_unaligned_offsets() {
        let mut writer = DwgBitWriter::new();
        writer.write_bit(true);
        writer.write_bit(false);
        writer.write_bit(true);
        writer.write_bs(0);
        writer.write_bs(256);
        writer.write_bs(42);
        writer.write_bs(12345);
        writer.write_bl(0);
        writer.write_bl(200);
        writer.write_bl(70000);
        writer.write_bd(0.0);
        writer.write_bd(1.0);
        writer.write_bd(3.14159);
        writer.write_ms(70000);
        writer.write_handle(5, 0x1234);
        writer.write_t("héllo");
        writer.pad_to_byte();

        let mut reader = DwgBitReader::new(&writer.bytes);
        assert!(reader.read_bit().unwrap());
        assert!(!reader.read_bit().unwrap());
        assert!(reader.read_bit().unwrap());
        assert_eq!(reader.read_bs().unwrap(), 0);
        assert_eq!(reader.read_bs().unwrap(), 256);
        assert_eq!(reader.read_bs().unwrap(), 42);
        assert_eq!(reader.read_bs().unwrap(), 12345);
        assert_eq!(reader.read_bl().unwrap(), 0);
        assert_eq!(reader.read_bl().unwrap(), 200);
        assert_eq!(reader.read_bl().unwrap(), 70000);
        assert_eq!(reader.read_bd().unwrap(), 0.0);
        assert_eq!(reader.read_bd().unwrap(), 1.0);
        assert_eq!(reader.read_bd().unwrap(), 3.14159);
        assert_eq!(reader.read_ms().unwrap(), 70000);
        assert_eq!(reader.read_handle().unwrap(), (5, 0x1234));
        assert_eq!(reader.read_t().unwrap(), "héllo");
    }

    #[test]
    fn dwg_crc16_matches_seed_on_empty_input() {
        assert_eq!(dwg_crc16(0xC0C1, &[]), 0xC0C1);
        assert_ne!(dwg_crc16(0xC0C1, &[1, 2, 3]), 0xC0C1);
    }

    #[test]
    fn dwg_writer_produces_a_structurally_valid_container() {
        let bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode empty drawing");
        assert_eq!(&bytes[0..6], b"AC1015");
        let section_count = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        assert_eq!(section_count, 3);
        assert_eq!(&bytes[DWG_FILE_HEADER_LEN - 16..DWG_FILE_HEADER_LEN], &DWG_SENTINEL_FILE_HEADER_END);
    }

    #[test]
    fn dwg_full_entity_set_round_trips() {
        let mut drawing = DwgDrawing::default();
        let layer_a = drawing.ensure_layer("outline");
        let layer_b = drawing.ensure_layer("solids");
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::Index(3), geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [10.0, 5.0, 0.0] } });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 3.0] } });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByBlock, geometry: DwgGeometry::Circle { center: [0.0, 0.0, 0.0], radius: 5.0, normal: [0.0, 0.0, 1.0] } });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::Index(1),
            geometry: DwgGeometry::Arc { center: [0.0, 0.0, 0.0], radius: 3.0, start_angle: 0.0, end_angle: 1.57, normal: [0.0, 0.0, 1.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::Index(2),
            geometry: DwgGeometry::Ellipse { center: [1.0, 1.0, 0.0], major_axis: [4.0, 0.0, 0.0], ratio: 0.5, start_param: 0.0, end_param: 6.28, normal: [0.0, 0.0, 1.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], bulges: vec![0.0, 0.5, 0.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Spline {
                degree: 3,
                control_points: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [3.0, 2.0, 0.0], [4.0, 0.0, 0.0]],
                knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                weights: vec![1.0; 4],
            },
        });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [0.0, 0.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".to_string() } });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Face3d { corners: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Polyline3d { closed: false, vertices: vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [1.0, 0.0, 5.0]] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] },
        });

        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded = dwg_from_bytes(&bytes).expect("decode");

        assert_eq!(decoded.entities.len(), drawing.entities.len());
        assert_eq!(decoded.layers.len(), drawing.layers.len());
        for (original, round_tripped) in drawing.entities.iter().zip(decoded.entities.iter()) {
            assert_eq!(original.geometry, round_tripped.geometry);
            assert_eq!(original.color, round_tripped.color);
            assert_eq!(drawing.layers[original.layer].name, decoded.layers[round_tripped.layer].name);
        }
    }

    #[test]
    fn dwg_mesh_bridge_round_trips_triangle_count_and_positions() {
        let mesh = mesh_box(2.0, 2.0, 2.0);
        let drawing = mesh_to_dwg_drawing(&mesh);
        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded_drawing = dwg_from_bytes(&bytes).expect("decode");
        let decoded_mesh = dwg_drawing_to_mesh(&decoded_drawing);
        assert_eq!(decoded_mesh.triangle_count(), mesh.triangle_count());
        assert_eq!(decoded_mesh.vertex_count(), mesh.vertex_count());
    }

    #[test]
    fn dwg_path_bridge_round_trips_cubic_control_points_exactly() {
        let paths = vec![vec![
            DwgPathSegment::Move { to: [0.0, 0.0] },
            DwgPathSegment::Line { to: [5.0, 0.0] },
            DwgPathSegment::Cubic { ctrl1: [6.0, 1.0], ctrl2: [7.0, 3.0], to: [5.0, 4.0] },
            DwgPathSegment::Close,
        ]];
        let drawing = paths_to_dwg_drawing(&paths);
        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded = dwg_from_bytes(&bytes).expect("decode");
        let round_tripped_paths = dwg_drawing_to_paths(&decoded);

        let cubic_found = round_tripped_paths.iter().flatten().any(|segment| {
            matches!(segment, DwgPathSegment::Cubic { ctrl1, ctrl2, to }
                if (ctrl1[0] - 6.0).abs() < 1e-9 && (ctrl2[1] - 3.0).abs() < 1e-9 && (to[1] - 4.0).abs() < 1e-9)
        });
        assert!(cubic_found, "expected the exact cubic control points to survive the dwg round trip");

        let line_found = round_tripped_paths.iter().flatten().any(|segment| matches!(segment, DwgPathSegment::Line { to } if (to[0] - 5.0).abs() < 1e-9));
        assert!(line_found, "expected the polyline segment to survive the dwg round trip");
    }

    #[test]
    fn dwg_rejects_unsupported_version() {
        let mut bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode");
        bytes[0..6].copy_from_slice(b"AC1018");
        let err = dwg_from_bytes(&bytes).expect_err("should reject non-R2000 version");
        assert!(err.contains("AC1018"));
    }

    #[test]
    fn dwg_reader_skips_unknown_object_types_without_failing() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 1.0, 1.0] } });
        let mut bytes = dwg_to_bytes(&drawing).expect("encode");

        let mut bogus_body = DwgBitWriter::new();
        bogus_body.write_rc(0xFF);
        let mut bogus_handles = DwgBitWriter::new();
        let bogus_offset = bytes.len();
        dwg_write_object(&mut bytes, 900, 0x9999, &mut bogus_body, &mut bogus_handles);

        let map_locator_pos = 10 + 2 * 9;
        let map_offset = u32::from_le_bytes(bytes[map_locator_pos + 1..map_locator_pos + 5].try_into().unwrap());
        let map_size = u32::from_le_bytes(bytes[map_locator_pos + 5..map_locator_pos + 9].try_into().unwrap());
        let mut new_entry = Vec::new();
        new_entry.extend_from_slice(&0x9999u64.to_le_bytes());
        new_entry.extend_from_slice(&(bogus_offset as u64).to_le_bytes());
        let insert_at = map_offset as usize + 4;
        for (i, b) in new_entry.iter().enumerate() {
            bytes.insert(insert_at + i, *b);
        }
        let new_count = u32::from_le_bytes(bytes[map_offset as usize..map_offset as usize + 4].try_into().unwrap()) + 1;
        bytes[map_offset as usize..map_offset as usize + 4].copy_from_slice(&new_count.to_le_bytes());
        let new_size = map_size + new_entry.len() as u32;
        bytes[map_locator_pos + 5..map_locator_pos + 9].copy_from_slice(&new_size.to_le_bytes());

        let decoded = dwg_from_bytes(&bytes).expect("reader should tolerate the unknown object type");
        assert_eq!(decoded.entities.len(), 1);
    }
}
// #endregion mesh
}

pub mod platform {
// #region platform
//! 🖥️ Root shell: apps, URI chrome, panel toggles, and shared action bus.

use crate::action_bus::ActionBus;
use crate::ui::AppDefinition;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PanelVisibility {
    pub left_side_panel: bool,
    pub right_side_panel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct PlatformSpec {
    pub id: String,
    pub name: String,
    pub default_active_app_id: Option<String>,
    pub initial_panel_visibility: Option<PanelVisibility>,
}

pub struct Platform {
    pub action_bus: ActionBus,
    pub apps: Vec<AppDefinition>,
    pub active_app_id: String,
    pub generation: u64,
    pub chrome_generation: u64,
    pub uri: String,
    pub panel_visibility: PanelVisibility,
    pub id: String,
    pub name: String,
    generation_counter: AtomicU64,
    chrome_generation_counter: AtomicU64,
}

impl Platform {
    pub fn new(spec: Option<PlatformSpec>) -> Self {
        let spec = spec.unwrap_or_default();
        let panel_visibility = spec.initial_panel_visibility.clone().unwrap_or_default();
        Self {
            action_bus: ActionBus::new(),
            apps: Vec::new(),
            active_app_id: spec.default_active_app_id.clone().unwrap_or_default(),
            generation: 0,
            chrome_generation: 0,
            uri: "/".into(),
            panel_visibility,
            id: spec.id,
            name: spec.name,
            generation_counter: AtomicU64::new(0),
            chrome_generation_counter: AtomicU64::new(0),
        }
    }

    pub fn add_app(&mut self, app: AppDefinition) {
        if self.active_app_id.is_empty() {
            self.active_app_id = app.id.clone();
        }
        self.apps.push(app);
        self.notify();
    }

    pub fn get_active_app(&self) -> Option<&AppDefinition> {
        self.apps
            .iter()
            .find(|app| app.id == self.active_app_id)
            .or_else(|| self.apps.first())
    }

    pub fn set_active_app_id(&mut self, id: String) {
        if self.active_app_id == id {
            return;
        }
        self.active_app_id = id;
        self.notify_chrome();
    }

    pub fn set_panel_visibility(&mut self, next: PanelVisibility) {
        if self.panel_visibility == next {
            return;
        }
        self.panel_visibility = next;
        self.notify_chrome();
    }

    pub fn notify(&mut self) {
        self.generation = self.generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }

    pub fn notify_chrome(&mut self) {
        self.chrome_generation = self.chrome_generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{ModeDefinition, WindowKindDefinition};

    #[test]
    fn adds_first_app_as_active() {
        let mut platform = Platform::new(None);
        platform.add_app(AppDefinition {
            id: "draw-play".into(),
            label: "Draw".into(),
            document: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: vec![ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                tools: Vec::new(),
                layout_id: None,
            }],
            default_mode_id: Some("edit".into()),
            window_kinds: vec![WindowKindDefinition {
                id: "composite".into(),
                label: "Canvas".into(),
                body_key: "composite".into(),
                surface_kind: crate::SurfaceKind::Canvas2d,
                icon_id: None,
                measures: Vec::new(),
                engagement: None,
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }],
            panel_tabs: vec![],
            keybindings: vec![],
            actions: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
        });
        assert_eq!(platform.active_app_id, "draw-play");
    }
}
// #endregion platform
}

pub mod tools {
// #region tools
//! 🧰 Declarative per-mode toolbar tool trees.

use crate::layout::ActionDescriptor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ToolCategory {
    Selection,
    Tools,
    Actions,
    History,
    Sync,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ToolNode {
    Separator {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
    },
    Button {
        id: String,
        icon_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category: Option<ToolCategory>,
        on_press: ActionDescriptor,
    },
    Toggle {
        id: String,
        icon_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pressed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category: Option<ToolCategory>,
        on_change: ActionDescriptor,
    },
    Collection {
        id: String,
        icon_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        order: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category: Option<ToolCategory>,
        children: Vec<ToolNode>,
    },
}

impl ToolNode {
    pub fn category(&self) -> ToolCategory {
        match self {
            ToolNode::Separator { .. } => ToolCategory::Tools,
            ToolNode::Button { category, .. } => category.unwrap_or(ToolCategory::Actions),
            ToolNode::Toggle { category, .. } => category.unwrap_or(ToolCategory::Tools),
            ToolNode::Collection { category, .. } => category.unwrap_or(ToolCategory::Tools),
        }
    }

    pub fn with_category(mut self, category: ToolCategory) -> Self {
        match &mut self {
            ToolNode::Button { category: slot, .. }
            | ToolNode::Toggle { category: slot, .. }
            | ToolNode::Collection { category: slot, .. } => *slot = Some(category),
            ToolNode::Separator { .. } => {}
        }
        self
    }
}

pub fn tool_separator(id: impl Into<String>) -> ToolNode {
    ToolNode::Separator {
        id: id.into(),
        order: None,
        disabled: None,
    }
}

pub fn tool_button(
    id: impl Into<String>,
    icon_id: impl Into<String>,
    label: impl Into<String>,
    on_press: ActionDescriptor,
) -> ToolNode {
    let label = label.into();
    ToolNode::Button {
        id: id.into(),
        icon_id: icon_id.into(),
        label: Some(label.clone()),
        text: None,
        title: Some(label),
        order: None,
        disabled: None,
        category: None,
        on_press,
    }
}

pub fn tool_toggle(
    id: impl Into<String>,
    icon_id: impl Into<String>,
    label: impl Into<String>,
    pressed: bool,
    on_change: ActionDescriptor,
) -> ToolNode {
    let label = label.into();
    ToolNode::Toggle {
        id: id.into(),
        icon_id: icon_id.into(),
        label: Some(label.clone()),
        text: None,
        title: Some(label),
        order: None,
        pressed: Some(pressed),
        disabled: None,
        category: None,
        on_change,
    }
}

pub fn tool_collection(
    id: impl Into<String>,
    icon_id: impl Into<String>,
    label: impl Into<String>,
    children: Vec<ToolNode>,
) -> ToolNode {
    let label = label.into();
    ToolNode::Collection {
        id: id.into(),
        icon_id: icon_id.into(),
        label: Some(label.clone()),
        text: None,
        title: Some(label),
        order: None,
        disabled: None,
        category: None,
        children,
    }
}

//#region 🔖WireFormatGoldenTests
/** 🧊 Golden wire-format tests: freeze exact JSON for ToolNode before it moves into ui_wgpu. */
#[cfg(test)]
mod tool_node_wire_format_tests {
    use super::*;
    use crate::layout::ActionDescriptor;

    const GOLDEN_TOOL_NODE_JSON: &str = "[{\"kind\":\"separator\",\"id\":\"sep1\",\"order\":1},{\"kind\":\"button\",\"id\":\"btn1\",\"iconId\":\"icon.tool\",\"label\":\"Tool\",\"title\":\"Tool\",\"category\":\"history\",\"onPress\":{\"controllerId\":\"ctrl\",\"action\":\"runTool\"}},{\"kind\":\"toggle\",\"id\":\"tog1\",\"iconId\":\"icon.toggle\",\"label\":\"Toggle\",\"title\":\"Toggle\",\"pressed\":true,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"toggleTool\"}},{\"kind\":\"collection\",\"id\":\"col1\",\"iconId\":\"icon.group\",\"label\":\"Group\",\"title\":\"Group\",\"children\":[{\"kind\":\"separator\",\"id\":\"sep2\"}]}]";

    #[test]
    fn tool_node_serializes_to_golden_json() {
        let nodes = vec![
            ToolNode::Separator { id: "sep1".into(), order: Some(1), disabled: None },
            tool_button(
                "btn1",
                "icon.tool",
                "Tool",
                ActionDescriptor { controller_id: "ctrl".into(), action: "runTool".into(), args: None },
            )
            .with_category(ToolCategory::History),
            tool_toggle(
                "tog1",
                "icon.toggle",
                "Toggle",
                true,
                ActionDescriptor { controller_id: "ctrl".into(), action: "toggleTool".into(), args: None },
            ),
            tool_collection("col1", "icon.group", "Group", vec![tool_separator("sep2")]),
        ];
        let json = serde_json::to_string(&nodes).unwrap();
        assert_eq!(json, GOLDEN_TOOL_NODE_JSON);
        let roundtripped: Vec<ToolNode> = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, nodes);
    }
}
//#endregion 🔖WireFormatGoldenTests
// #endregion tools
}

pub mod ui {
// #region ui
//! 🧩 Declarative UI graph types shared by kernel, plugins, and renderers.

use crate::layout::NamedLayout;
use crate::layout::WindowEngagement;
use crate::layout::WindowLayout;
use crate::layout::WindowMeasure;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖Action
pub use crate::layout::{ActionDescriptor, StyleSpec};
//#endregion 🔖Action

//#region 🔖Primitives
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiStackNode {
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_action: Option<ActionDescriptor>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTextNode {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasize: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiButtonNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub icon_id: String,
    pub label: String,
    pub action: ActionDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSeparatorNode {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiImageNode {
    pub id: String,
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiInputNode {
    pub id: String,
    pub input_kind: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    pub on_change: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSelectItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSelectNode {
    pub id: String,
    pub value: String,
    pub items: Vec<UiSelectItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub on_change: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiToggleNode {
    pub id: String,
    pub icon_id: String,
    pub pressed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub on_change: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiVec3Node {
    pub id: String,
    pub value: Option<[f64; 3]>,
    pub on_change: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKeyValueEntry {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKeyValueNode {
    pub entries: Vec<UiKeyValueEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSliderNode {
    pub id: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub on_change: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiNumberStepperNode {
    pub id: String,
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    pub on_absolute: ActionDescriptor,
    pub on_delta: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiRingNode {
    pub id: String,
    pub orb_id: String,
    pub t: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub on_change: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiIconSelectNode {
    pub id: String,
    pub value: String,
    pub uniform: bool,
    pub classifier_kind: String,
    pub on_change: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiControlNode {
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    Vec3(UiVec3Node),
    Button(UiButtonNode),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiFieldNode {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub child: Box<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSectionNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeItemAction {
    pub icon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub action: ActionDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_on_hover: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeItemNode {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "icon")]
    pub icon_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "expanded")]
    pub default_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_action: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhover_action: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<UiTreeItemAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UiTreeItemNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<UiControlNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

impl UiTreeItemNode {
    /** @emoji 🌳 Builds a tree item with optional extensions unset. */
    pub fn base(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeSectionNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub items: Vec<UiTreeItemNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeNode {
    pub sections: Vec<UiTreeSectionNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_change: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_action: Option<ActionDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiInspectorFieldGroup {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub fields: Vec<UiNode>,
}

pub const UI_INSPECTOR_MIXED_PLACEHOLDER: &str = "Mixed";
//#endregion 🔖Primitives

//#region 🔖InspectorHelpers
pub fn ui_inspector_all_equal<T: PartialEq>(values: &[T]) -> bool {
    if values.len() <= 1 {
        return true;
    }
    values.windows(2).all(|pair| pair[0] == pair[1])
}

pub struct UiInspectorMixedText {
    pub value: String,
    pub placeholder: Option<String>,
}

pub fn ui_inspector_mixed_text(values: &[String]) -> UiInspectorMixedText {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedText {
        value: if uniform {
            values.first().cloned().unwrap_or_default()
        } else {
            String::new()
        },
        placeholder: if uniform {
            None
        } else {
            Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
        },
    }
}

pub struct UiInspectorMixedNumber {
    pub value: f64,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_number(values: &[f64]) -> UiInspectorMixedNumber {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedNumber {
        value: if uniform {
            *values.first().unwrap_or(&0.0)
        } else {
            f64::NAN
        },
        uniform,
    }
}

pub fn ui_inspector_mixed_select(values: &[String]) -> UiInspectorMixedText {
    ui_inspector_mixed_text(values)
}

pub struct UiInspectorMixedToggle {
    pub pressed: bool,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_toggle(values: &[bool]) -> UiInspectorMixedToggle {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedToggle {
        pressed: uniform && values.first().copied().unwrap_or(false),
        uniform,
    }
}

pub fn ui_inspector_mixed_slider(values: &[f64]) -> UiInspectorMixedNumber {
    ui_inspector_mixed_number(values)
}

pub struct UiInspectorMixedVec3 {
    pub value: Option<[f64; 3]>,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_vec3(values: &[[f64; 3]]) -> UiInspectorMixedVec3 {
    let serialized: Vec<String> = values
        .iter()
        .map(|row| serde_json::to_string(row).unwrap_or_default())
        .collect();
    let uniform = ui_inspector_all_equal(&serialized);
    UiInspectorMixedVec3 {
        value: if uniform { values.first().copied() } else { None },
        uniform,
    }
}

pub fn ui_inspector_readonly_field(
    id: impl Into<String>,
    label: impl Into<String>,
    value: impl Into<String>,
) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id,
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: None,
            on_change: ActionDescriptor {
                controller_id: String::new(),
                action: String::new(),
                args: None,
            },
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

pub fn ui_inspector_groups_to_tree(groups: &[UiInspectorFieldGroup]) -> UiNode {
    let sections: Vec<UiSectionNode> = groups
        .iter()
        .filter(|group| !group.fields.is_empty())
        .map(|group| UiSectionNode {
            id: group.id.clone(),
            label: Some(group.label.clone()),
            default_open: Some(group.default_open.unwrap_or(true)),
            children: group.fields.clone(),
        })
        .collect();
    ui_declarative_sections_to_tree(&sections)
}

pub fn ui_declarative_sections_to_tree(sections: &[UiSectionNode]) -> UiNode {
    let tree_sections: Vec<UiTreeSectionNode> = sections
        .iter()
        .map(|section| UiTreeSectionNode {
            id: section.id.clone(),
            label: section.label.clone(),
            default_open: Some(section.default_open.unwrap_or(true)),
            items: section
                .children
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    ui_declarative_child_to_tree_item(child, format!("{}.{}", section.id, index))
                })
                .collect(),
        })
        .collect();
    UiNode::Tree(if tree_sections.is_empty() {
        UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "empty".into(),
                label: None,
                default_open: None,
                items: vec![UiTreeItemNode {
                    id: "empty".into(),
                    label: "—".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    action: None,
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                }],
            }],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        }
    } else {
        UiTreeNode {
            sections: tree_sections,
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        }
    })
}

fn ui_declarative_child_to_tree_item(node: &UiNode, fallback_id: String) -> UiTreeItemNode {
    match node {
        UiNode::Text(text) => UiTreeItemNode {
            id: format!("{}.text", fallback_id),
            label: text.value.clone(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        UiNode::Field(field) => {
            let description = if let UiNode::Input(input) = field.child.as_ref() {
                input
                    .placeholder
                    .clone()
                    .or_else(|| if input.value.is_empty() { None } else { Some(input.value.clone()) })
            } else {
                None
            };
            UiTreeItemNode {
                id: field.id.clone(),
                label: field.label.clone(),
                description,
                icon_id: None,
                selected: None,
                default_open: None,
                action: None,
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: ui_node_to_control(&field.child),
                is_hidden: None,
            }
        }
        UiNode::Button(button) => UiTreeItemNode {
            id: button.id.clone().unwrap_or(fallback_id),
            label: button.label.clone(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: Some(UiControlNode::Button(button.clone())),
            is_hidden: None,
        },
        UiNode::Input(input) => tree_control_item(input.id.clone(), UiControlNode::Input(input.clone())),
        UiNode::Select(select) => tree_control_item(select.id.clone(), UiControlNode::Select(select.clone())),
        UiNode::Toggle(toggle) => tree_control_item(toggle.id.clone(), UiControlNode::Toggle(toggle.clone())),
        UiNode::Vec3(vec3) => tree_control_item(vec3.id.clone(), UiControlNode::Vec3(vec3.clone())),
        UiNode::KeyValue(key_value) => tree_control_item(fallback_id, UiControlNode::KeyValue(key_value.clone())),
        UiNode::Slider(slider) => tree_control_item(slider.id.clone(), UiControlNode::Slider(slider.clone())),
        UiNode::NumberStepper(stepper) => {
            tree_control_item(stepper.id.clone(), UiControlNode::NumberStepper(stepper.clone()))
        }
        UiNode::Ring(ring) => tree_control_item(ring.id.clone(), UiControlNode::Ring(ring.clone())),
        UiNode::IconSelect(icon_select) => {
            tree_control_item(icon_select.id.clone(), UiControlNode::IconSelect(icon_select.clone()))
        }
        UiNode::Separator(_) => UiTreeItemNode {
            id: format!("{}.sep", fallback_id),
            label: "—".into(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        other => UiTreeItemNode {
            id: fallback_id,
            label: format!("{other:?}"),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
    }
}

fn tree_control_item(id: String, control: UiControlNode) -> UiTreeItemNode {
    UiTreeItemNode {
        id,
        label: String::new(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: Some(control),
        is_hidden: None,
    }
}
//#endregion 🔖InspectorHelpers

//#region 🔖ComponentScenes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceKind {
    #[serde(rename = "canvas-2d")]
    Canvas2d,
    #[serde(rename = "world-3d")]
    World3d,
    #[serde(rename = "node-graph")]
    NodeGraph,
    #[serde(rename = "text-editor")]
    TextEditor,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "raster")]
    Raster,
    #[serde(rename = "virtualFileSystem")]
    VirtualFileSystem,
    #[serde(rename = "gis2d-map")]
    GisMap,
    #[serde(rename = "puzzle2d-board")]
    Puzzle2dBoard,
    #[serde(rename = "icon-render")]
    IconRender,
    #[serde(rename = "note-canvas")]
    NoteCanvas,
    #[serde(rename = "vcs-history")]
    VcsHistory,
}

impl SurfaceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canvas2d => "canvas-2d",
            Self::World3d => "world-3d",
            Self::NodeGraph => "node-graph",
            Self::TextEditor => "text-editor",
            Self::Table => "table",
            Self::Raster => "raster",
            Self::VirtualFileSystem => "virtualFileSystem",
            Self::GisMap => "gis2d-map",
            Self::Puzzle2dBoard => "puzzle2d-board",
            Self::IconRender => "icon-render",
            Self::NoteCanvas => "note-canvas",
            Self::VcsHistory => "vcs-history",
        }
    }

    pub fn is_viewport(self) -> bool {
        matches!(
            self,
            Self::World3d | Self::NodeGraph | Self::Canvas2d | Self::Puzzle2dBoard | Self::NoteCanvas
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dScene {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub layers_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World3dScene {
    pub camera_json: String,
    #[serde(default = "world3d_default_meshes_json")]
    pub meshes_json: String,
    pub instances_json: String,
    #[serde(default = "world3d_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vortices_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attractions_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_volumes_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engagement_preview_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunking_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_menu_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fit_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldMeshLodEntry {
    pub lod: f64,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldLodRecord {
    #[serde(default = "default_true")]
    pub automatic: bool,
    #[serde(default = "default_manual_lod")]
    pub manual: f64,
    #[serde(default = "default_distance_reference")]
    pub distance_reference: f64,
    #[serde(default)]
    pub depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "default_true")]
    pub show_grid: bool,
    #[serde(default)]
    pub grid_datum: Option<[f64; 3]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldChunkingRecord {
    pub chunk_size: f64,
    pub max_distance: f64,
}

fn default_manual_lod() -> f64 {
    100.0
}

fn default_distance_reference() -> f64 {
    100.0
}

fn default_grid_factor() -> f64 {
    10.0
}

fn default_true() -> bool {
    true
}

pub fn world3d_default_lod_json() -> String {
    serde_json::json!({
        "automatic": true,
        "manual": 100.0,
        "distanceReference": 100.0,
        "depthVariable": false,
        "gridFactor": 10.0,
        "gridSnapEnabled": false,
        "showGrid": true,
        "gridDatum": [0.0, 0.0, 0.0],
    })
    .to_string()
}

pub fn world3d_chunking_json(chunk_size: f64, max_distance: f64) -> String {
    serde_json::json!({
        "chunkSize": chunk_size,
        "maxDistance": max_distance,
    })
    .to_string()
}

pub fn world3d_default_selection_json() -> String {
    r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into()
}

pub fn world3d_default_meshes_json() -> String {
    "[]".into()
}

pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
    serde_json::json!({
        "position": position,
        "target": target,
        "up": [0.0, 0.0, 1.0],
        "fov": fov,
    })
    .to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphScene {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operators_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_menu_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_items_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_off_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computing_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixture_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_peers_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEditorScene {
    pub buffer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlays_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholders_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_carets_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_spans_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newline_gates_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableScene {
    pub columns_json: String,
    pub rows_json: String,
}

/** @emoji 🖼️ Raster scene: WASM `RasterSession` sync channels for the composite/navigator windows, see raster/rs/lib.rs. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterScene {
    pub document_sync_json: String,
    pub assets_json: String,
    pub camera_json: String,
    pub selection_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_tool: String,
    pub brush_size: f64,
    pub brush_opacity: f64,
    pub view_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_viewport_json: Option<String>,
}

/** @emoji 🖼️ Icon-render scene: client-side render request for a shot preview, see https://threejs.org/docs/#examples/en/renderers/SVGRenderer. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconRenderScene {
    pub request_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualFileSystemScene {
    pub schema_json: String,
    pub rows_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_row_ids_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_row_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_drop_enabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapScene {
    pub map_fixture_json: String,
    pub camera_json: String,
    #[serde(default = "gis_map_default_render_mode")]
    pub render_mode: String,
    #[serde(default = "gis_map_default_vector_style")]
    pub vector_style: String,
    #[serde(default = "gis_map_default_lod_mode")]
    pub lod_mode: String,
    #[serde(default = "gis_map_default_tile_url_template")]
    pub tile_url_template: String,
    #[serde(default = "gis_map_default_vector_tile_url_template")]
    pub vector_tile_url_template: String,
    #[serde(default = "gis_map_default_layer_visibility_json")]
    pub layer_visibility_json: String,
    #[serde(default = "gis_map_default_layer_stroke_scale_json")]
    pub layer_stroke_scale_json: String,
    #[serde(default = "gis_map_default_selection_json")]
    pub selection_json: String,
    #[serde(default = "gis_map_default_hover_json")]
    pub hover_json: String,
    #[serde(default = "gis_map_default_selection_method")]
    pub selection_method: String,
    #[serde(default = "gis_map_default_selection_mode")]
    pub selection_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_menu_json: Option<String>,
}

pub fn gis_map_default_render_mode() -> String {
    "combined".into()
}

pub fn gis_map_default_vector_style() -> String {
    "colored".into()
}

pub fn gis_map_default_lod_mode() -> String {
    "automatic".into()
}

pub fn gis_map_default_tile_url_template() -> String {
    "/osm/{z}/{x}/{y}.png".into()
}

pub fn gis_map_default_vector_tile_url_template() -> String {
    "/vt/{z}/{x}/{y}.pbf".into()
}

pub fn gis_map_default_layer_visibility_json() -> String {
    r#"{"raster":true,"water":true,"land":true,"roads":true,"buildings":true,"borders":true,"labels":true,"positions":true,"positionLabels":true,"routes":true,"regions":true}"#.into()
}

pub fn gis_map_default_layer_stroke_scale_json() -> String {
    r#"{"raster":1,"water":1,"land":1,"roads":1,"buildings":1,"borders":1,"labels":1,"positions":1,"positionLabels":1,"routes":1,"regions":1}"#.into()
}

pub fn gis_map_default_selection_json() -> String {
    r#"{"positions":[],"routes":[]}"#.into()
}

pub fn gis_map_default_hover_json() -> String {
    "null".into()
}

pub fn gis_map_default_selection_method() -> String {
    "rectangle".into()
}

pub fn gis_map_default_selection_mode() -> String {
    "default".into()
}

impl GisMapScene {
    /** @emoji 🗺️ Builds a GIS map scene with optional extensions unset. */
    pub fn base(map_fixture_json: String, camera_json: String) -> Self {
        Self {
            map_fixture_json,
            camera_json,
            render_mode: gis_map_default_render_mode(),
            vector_style: gis_map_default_vector_style(),
            lod_mode: gis_map_default_lod_mode(),
            tile_url_template: gis_map_default_tile_url_template(),
            vector_tile_url_template: gis_map_default_vector_tile_url_template(),
            layer_visibility_json: gis_map_default_layer_visibility_json(),
            layer_stroke_scale_json: gis_map_default_layer_stroke_scale_json(),
            selection_json: gis_map_default_selection_json(),
            hover_json: gis_map_default_hover_json(),
            selection_method: gis_map_default_selection_method(),
            selection_mode: gis_map_default_selection_mode(),
            context_menu_json: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dBoardScene {
    pub fixture_json: String,
    pub camera_json: String,
    #[serde(default = "puzzle2d_board_default_kind_catalogs_json")]
    pub kind_catalogs_json: String,
    #[serde(default = "puzzle2d_board_default_selection_json")]
    pub selection_json: String,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_tool: Option<String>,
    #[serde(default = "puzzle2d_board_default_selection_method")]
    pub selection_method: String,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "puzzle2d_board_default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default)]
    pub suggestion_offset: f64,
    #[serde(default = "puzzle2d_board_default_brush_kind_weights_json")]
    pub brush_kind_weights_json: String,
    #[serde(default = "puzzle2d_board_default_kind_compatibility_json")]
    pub kind_compatibility_json: String,
    #[serde(default = "puzzle2d_board_default_lod_mode")]
    pub lod_mode: String,
}

pub fn puzzle2d_board_default_kind_catalogs_json() -> String {
    "{}".into()
}

pub fn puzzle2d_board_default_selection_json() -> String {
    "[]".into()
}

pub fn puzzle2d_board_default_selection_method() -> String {
    "rectangle".into()
}

pub fn puzzle2d_board_default_grid_factor() -> f64 {
    1.0
}

pub fn puzzle2d_board_default_brush_kind_weights_json() -> String {
    "{}".into()
}

pub fn puzzle2d_board_default_kind_compatibility_json() -> String {
    "[]".into()
}

pub fn puzzle2d_board_default_lod_mode() -> String {
    "automatic".into()
}

impl Puzzle2dBoardScene {
    /** @emoji 🧩 Builds a puzzle 2D board scene with optional extensions unset. */
    pub fn base(fixture_json: String, camera_json: String, interactive: bool) -> Self {
        Self {
            fixture_json,
            camera_json,
            kind_catalogs_json: puzzle2d_board_default_kind_catalogs_json(),
            selection_json: puzzle2d_board_default_selection_json(),
            interactive,
            hovered_id: None,
            active_tool: None,
            selection_method: puzzle2d_board_default_selection_method(),
            grid_snap_enabled: false,
            grid_factor: puzzle2d_board_default_grid_factor(),
            suggestion_offset: 0.0,
            brush_kind_weights_json: puzzle2d_board_default_brush_kind_weights_json(),
            kind_compatibility_json: puzzle2d_board_default_kind_compatibility_json(),
            lod_mode: puzzle2d_board_default_lod_mode(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCanvasScene {
    pub document_json: String,
    #[serde(default = "note_canvas_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_tool: String,
    pub view_mode: String,
    #[serde(default)]
    pub interactive: bool,
}

pub fn note_canvas_default_selection_json() -> String {
    "[]".into()
}

impl NoteCanvasScene {
    /** @emoji 📝 Builds a note canvas scene with the default empty selection. */
    pub fn base(document_json: String, active_tool: String, view_mode: String, interactive: bool) -> Self {
        Self {
            document_json,
            selection_json: note_canvas_default_selection_json(),
            hovered_id: None,
            active_tool,
            view_mode,
            interactive,
        }
    }
}

/** @emoji 🗄️ A checkpoint ancestor-graph history view. `columns_json` is a `HistoryColumn[]` array
 * (see `vcs::HistoryColumn`), newest checkpoint first. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsHistoryScene {
    pub columns_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiExternalSlotNode {
    pub plugin_id: String,
    pub app_id: String,
    pub body_key: String,
    pub params_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiComponentSceneNode {
    pub surface_id: String,
    pub controller_id: String,
    pub component_kind: SurfaceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pane_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_2d: Option<Canvas2dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_3d: Option<World3dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_graph: Option<NodeGraphScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_editor: Option<TextEditorScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster: Option<RasterScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_file_system: Option<VirtualFileSystemScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gis_map: Option<GisMapScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puzzle2d_board: Option<Puzzle2dBoardScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_render: Option<IconRenderScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_canvas: Option<NoteCanvasScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_history: Option<VcsHistoryScene>,
}
//#endregion 🔖ComponentScenes

//#region 🔖UiNode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiNode {
    Stack(UiStackNode),
    Text(UiTextNode),
    Button(UiButtonNode),
    Separator(UiSeparatorNode),
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    Vec3(UiVec3Node),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
    Field(UiFieldNode),
    Section(UiSectionNode),
    Tree(UiTreeNode),
    Image(UiImageNode),
    ComponentScene(UiComponentSceneNode),
    ExternalSlot(UiExternalSlotNode),
}

impl NodeGraphScene {
    /** @emoji 🕸️ Builds a node-graph scene with optional extensions unset. */
    pub fn base(nodes_json: String, edges_json: String, viewport_json: String) -> Self {
        Self {
            nodes_json,
            edges_json,
            viewport_json,
            editable: None,
            operators_json: None,
            context_menu_json: None,
            find_items_json: None,
            selection_json: None,
            hover_json: None,
            preview_off_json: None,
            lod_json: None,
            catalogue_json: None,
            controls_json: None,
            clusters_json: None,
            computing_json: None,
            capabilities_json: None,
            fixture_json: None,
            presence_peers_json: None,
        }
    }
}

impl TextEditorScene {
    /** @emoji ✍️ Builds a text-editor scene with optional extensions unset. */
    pub fn base(buffer: String, language: Option<String>, selection_json: Option<String>) -> Self {
        Self {
            buffer,
            language,
            selection_json,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: None,
            camera_json: None,
            hover_json: None,
            newline_gates_json: None,
            rename_json: None,
        }
    }
}

//#region 🔖SceneActions
/** @emoji 🎮 Renderer-to-plugin action names for node-graph surfaces. */
pub mod node_graph_actions {
    pub const SELECT: &str = "nodeGraphSelect";
    pub const HOVER: &str = "nodeGraphHover";
    pub const EDIT: &str = "nodeGraphEdit";
    pub const VIEWPORT: &str = "nodeGraphViewport";
    pub const SPOTLIGHT_COMMIT: &str = "spotlightCommit";
}

/** @emoji ✍️ Renderer-to-plugin action names for text-editor surfaces. */
pub mod text_editor_actions {
    pub const EDIT: &str = "textEdit";
    pub const SELECT: &str = "textSelect";
    pub const HOVER: &str = "textHover";
    pub const REQUEST_COMPLETIONS: &str = "requestCompletions";
    pub const COMMIT_RENAME: &str = "commitRename";
    pub const FORMAT_DOCUMENT: &str = "formatDocument";
}

/** @emoji 🗺️ Renderer-to-plugin action names for GIS map surfaces. */
pub mod puzzle2d_board_actions {
    pub const APPLY_BOARD_EVENTS: &str = "applyBoardEvents";
}

/** @emoji 📝 Renderer-to-plugin action names for note canvas surfaces. */
pub mod note_canvas_actions {
    pub const APPLY_NOTE_EVENTS: &str = "applyNoteEvents";
}

pub mod gis_map_actions {
    pub const SET_CAMERA: &str = "setCamera";
    pub const SET_FEATURE_SELECTION: &str = "setFeatureSelection";
    pub const SET_HOVER: &str = "setHover";
    pub const SET_SELECTION_METHOD: &str = "setSelectionMethod";
    pub const SET_SELECTION_MODE: &str = "setSelectionMode";
    pub const CLEAR_SELECTION: &str = "clearSelection";
    pub const SELECT_ALL: &str = "selectAll";
    pub const DESELECT: &str = "deselect";
    pub const FOCUS_FEATURE: &str = "focusFeature";
    pub const OPEN_SOURCE: &str = "openSource";
    pub const SET_LAYER_STROKE_SCALE: &str = "setLayerStrokeScale";
    pub const FIT_WORLD: &str = "fitWorld";
}
//#endregion 🔖SceneActions

pub fn ui_stack_vertical(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("standard".into()),
        padding: None,
        id: None,
        selected: None,
        activate: None,
        children,
        drop_action: None,
    })
}

/** @emoji 🖼️ Builds an image node rendering a source URL or path. */
pub fn ui_image(id: impl Into<String>, src: impl Into<String>, alt: Option<String>) -> UiNode {
    UiNode::Image(UiImageNode {
        id: id.into(),
        src: src.into(),
        alt,
    })
}

/** @emoji 🎛 Extracts the control payload of a {@link UiNode} when it is a control variant. */
pub fn ui_node_to_control(node: &UiNode) -> Option<UiControlNode> {
    match node {
        UiNode::Input(input) => Some(UiControlNode::Input(input.clone())),
        UiNode::Select(select) => Some(UiControlNode::Select(select.clone())),
        UiNode::Toggle(toggle) => Some(UiControlNode::Toggle(toggle.clone())),
        UiNode::Vec3(vec3) => Some(UiControlNode::Vec3(vec3.clone())),
        UiNode::Button(button) => Some(UiControlNode::Button(button.clone())),
        UiNode::KeyValue(key_value) => Some(UiControlNode::KeyValue(key_value.clone())),
        UiNode::Slider(slider) => Some(UiControlNode::Slider(slider.clone())),
        UiNode::NumberStepper(stepper) => Some(UiControlNode::NumberStepper(stepper.clone())),
        UiNode::Ring(ring) => Some(UiControlNode::Ring(ring.clone())),
        UiNode::IconSelect(icon) => Some(UiControlNode::IconSelect(icon.clone())),
        _ => None,
    }
}

/** @emoji 🎛 Wraps a {@link UiControlNode} back into its matching {@link UiNode} control variant (inverse of {@link ui_node_to_control}). */
pub fn ui_control_to_node(control: UiControlNode) -> UiNode {
    match control {
        UiControlNode::Input(input) => UiNode::Input(input),
        UiControlNode::Select(select) => UiNode::Select(select),
        UiControlNode::Toggle(toggle) => UiNode::Toggle(toggle),
        UiControlNode::Vec3(vec3) => UiNode::Vec3(vec3),
        UiControlNode::Button(button) => UiNode::Button(button),
        UiControlNode::KeyValue(key_value) => UiNode::KeyValue(key_value),
        UiControlNode::Slider(slider) => UiNode::Slider(slider),
        UiControlNode::NumberStepper(stepper) => UiNode::NumberStepper(stepper),
        UiControlNode::Ring(ring) => UiNode::Ring(ring),
        UiControlNode::IconSelect(icon) => UiNode::IconSelect(icon),
    }
}

impl Default for UiNode {
    fn default() -> Self {
        ui_stack_vertical(vec![])
    }
}

pub fn ui_text(value: impl Into<String>) -> UiNode {
    UiNode::Text(UiTextNode {
        value: value.into(),
        emphasize: None,
        data_attributes: None,
    })
}

/** @emoji 🔌 Renders a contributing plugin body inline at this tree position. */
pub fn ui_external_slot(
    plugin_id: impl Into<String>,
    app_id: impl Into<String>,
    body_key: impl Into<String>,
    params_json: impl Into<String>,
) -> UiNode {
    UiNode::ExternalSlot(UiExternalSlotNode {
        plugin_id: plugin_id.into(),
        app_id: app_id.into(),
        body_key: body_key.into(),
        params_json: params_json.into(),
    })
}

fn component_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    component_kind: SurfaceKind,
    pane_id: Option<String>,
    binding_id: Option<String>,
    canvas_2d: Option<Canvas2dScene>,
    world_3d: Option<World3dScene>,
    node_graph: Option<NodeGraphScene>,
    text_editor: Option<TextEditorScene>,
    table: Option<TableScene>,
    raster: Option<RasterScene>,
    virtual_file_system: Option<VirtualFileSystemScene>,
    gis_map: Option<GisMapScene>,
    puzzle2d_board: Option<Puzzle2dBoardScene>,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind,
        pane_id,
        binding_id,
        canvas_2d,
        world_3d,
        node_graph,
        text_editor,
        table,
        raster,
        virtual_file_system,
        gis_map,
        puzzle2d_board,
        icon_render: None,
        note_canvas: None,
        vcs_history: None,
    })
}

pub fn build_canvas_2d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: Canvas2dScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::Canvas2d,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_world_3d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: World3dScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::World3d,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_node_graph_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: NodeGraphScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::NodeGraph,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_text_editor_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TextEditorScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::TextEditor,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
    )
}

//#region 🔖TextIdentifierOccurrences
/// 🔎 Expands an offset in `text` to the bounds of the identifier (`[A-Za-z0-9_]+`) it falls in, if any.
pub fn text_identifier_bounds_at(text: &str, offset: usize) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let is_ident = |byte: u8| (byte as char).is_ascii_alphanumeric() || byte == b'_';
    let mut index = offset.min(bytes.len());
    while index > 0 && is_ident(bytes[index - 1]) {
        index -= 1;
    }
    let start = index;
    while index < bytes.len() && is_ident(bytes[index]) {
        index += 1;
    }
    if start == index {
        None
    } else {
        Some((start, index))
    }
}

/// 🔎 JSON `{selection, hover}` occurrence ranges for the identifier under `cursor`, for editor cross-highlighting.
pub fn text_identifier_occurrences_json(text: &str, cursor: usize) -> Option<String> {
    let (start, end) = text_identifier_bounds_at(text, cursor)?;
    let needle = &text[start..end];
    if needle.is_empty() {
        return None;
    }
    let mut ranges = Vec::new();
    let mut scan = 0usize;
    while let Some(found) = text[scan..].find(needle) {
        let at = scan + found;
        let next_end = at + needle.len();
        if text_identifier_bounds_at(text, at) == Some((at, next_end)) {
            ranges.push(serde_json::json!({ "start": at, "end": next_end }));
        }
        scan = at + needle.len();
    }
    let ranges_json = serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into());
    Some(serde_json::json!({ "selection": ranges_json, "hover": ranges_json }).to_string())
}
//#endregion 🔖TextIdentifierOccurrences

pub fn build_table_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TableScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::Table,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
    )
}

pub fn build_raster_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: RasterScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::Raster,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
    )
}

pub fn build_virtual_file_system_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: VirtualFileSystemScene,
    pane_id: Option<String>,
    binding_id: Option<String>,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::VirtualFileSystem,
        pane_id,
        binding_id,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
    )
}

pub fn build_gis_map_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: GisMapScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::GisMap,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
    )
}

pub fn build_puzzle2d_board_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: Puzzle2dBoardScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        SurfaceKind::Puzzle2dBoard,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
    )
}

pub fn build_icon_render_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: IconRenderScene,
) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(
        surface_id,
        controller_id,
        SurfaceKind::IconRender,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ) else {
        unreachable!()
    };
    UiNode::ComponentScene(UiComponentSceneNode {
        icon_render: Some(scene),
        ..node
    })
}

pub fn build_note_canvas_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: NoteCanvasScene,
) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(
        surface_id,
        controller_id,
        SurfaceKind::NoteCanvas,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ) else {
        unreachable!()
    };
    UiNode::ComponentScene(UiComponentSceneNode {
        note_canvas: Some(scene),
        ..node
    })
}

pub fn build_vcs_history_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: VcsHistoryScene,
) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(
        surface_id,
        controller_id,
        SurfaceKind::VcsHistory,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ) else {
        unreachable!()
    };
    UiNode::ComponentScene(UiComponentSceneNode {
        vcs_history: Some(scene),
        ..node
    })
}
//#endregion 🔖UiNode

//#region 🔖Manifest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub action: ActionDescriptor,
}

/// @emoji 🗂️ Classifies a declared action by how it interacts with VCS history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionKind {
    /// Mutates the document — dispatched as VCS operations with a true inverse, recorded in history.
    Operation,
    /// Ephemeral view state (camera, selection, hover, active tool) — not recorded in history.
    View,
    /// Framework-provided undo/redo/checkpoint/alternative — auto-injected, never app-declared.
    History,
    /// Shell-only effect (navigate, export, spawn) — no document mutation.
    Shell,
}

/// @emoji 📇 Declares one action an app can receive via `ActionDescriptor.action`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinition {
    pub id: String,
    pub label: String,
    pub kind: ActionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_schema: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    #[serde(default)]
    pub in_palette: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl ActionDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: ActionKind) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind,
            icon_id: None,
            args_schema: None,
            keys: None,
            in_palette: true,
            category: None,
        }
    }
}

/// @emoji 🕹️ The six framework-owned History actions, auto-injected into every `AppDefinition`.
pub fn history_action_definitions() -> Vec<ActionDefinition> {
    vec![
        ActionDefinition {
            keys: Some("mod+z".into()),
            ..ActionDefinition::new("undo", "Undo", ActionKind::History)
        },
        ActionDefinition {
            keys: Some("mod+shift+z".into()),
            ..ActionDefinition::new("redo", "Redo", ActionKind::History)
        },
        ActionDefinition::new("commitCheckpoint", "Commit Checkpoint", ActionKind::History),
        ActionDefinition::new("createAlternative", "Create Alternative", ActionKind::History),
        ActionDefinition::new("switchAlternative", "Switch Alternative", ActionKind::History),
        ActionDefinition::new("checkoutCheckpoint", "Checkout Checkpoint", ActionKind::History),
    ]
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<crate::tools::ToolNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDefinition {
    pub id: String,
    pub label: String,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub measures: Vec<WindowMeasure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement: Option<WindowEngagement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_projection_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_event_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PanelGroup {
    Workbench,
    Details,
    Display,
    Settings,
}

impl PanelGroup {
    pub fn side(&self) -> &'static str {
        match self {
            PanelGroup::Workbench | PanelGroup::Display => "left",
            PanelGroup::Details | PanelGroup::Settings => "right",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "workbench",
            PanelGroup::Details => "details",
            PanelGroup::Display => "display",
            PanelGroup::Settings => "settings",
        }
    }
}

/// 🌳 A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it). Exactly one of the two is set; `group` is only meaningful on root (non-nested) entries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelTabDefinition {
    pub id: String,
    pub label: String,
    pub group: PanelGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_key: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PanelTabDefinition>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDefinition {
    pub id: String,
    pub label: String,
    pub document: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    pub controller_id: String,
    pub modes: Vec<ModeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode_id: Option<String>,
    pub window_kinds: Vec<WindowKindDefinition>,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_layout: Option<WindowLayout>,
    /// 🗣️ Terminology ids this app declares beyond the implicit "native" default.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terminologies: Vec<String>,
}

/// 🧭 Resolves the dock layout a mode should present.
pub fn resolve_layout_for_mode(app: &AppDefinition, mode_id: &str) -> Option<WindowLayout> {
    let mode = app.modes.iter().find(|mode| mode.id == mode_id)?;
    if let Some(layout_id) = &mode.layout_id {
        if let Some(named) = app.named_layouts.iter().find(|entry| entry.id == *layout_id) {
            return Some(named.layout.clone());
        }
    }
    app.default_layout.clone()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDefinition {
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub document: Vec<String>,
    pub yields: String,
}

/// 🪜 Formats a canonical app document for chrome.
pub fn app_document_label(document: &[String]) -> String {
    document.join(" · ")
}

/// 🗂️ Formats a window tab within its canonical app document.
pub fn app_window_document_label(app: &AppDefinition, window_label: &str) -> String {
    let mut document = app.document.clone();
    let normalized_window = window_label.trim().to_lowercase();
    let normalized_app = app.label.trim().to_lowercase();
    if !normalized_window.is_empty()
        && normalized_window != normalized_app
        && document.last().is_none_or(|segment| segment.to_lowercase() != normalized_window)
    {
        document.push(normalized_window);
    }
    app_document_label(&document)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    pub label: String,
    pub document_json: String,
    pub app_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Contribution {
    FormsQuestionKind {
        app_id: String,
        question_kind: String,
        label: String,
        icon_id: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        default_value_json: String,
        params_body_key: String,
        preview_body_key: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub programs: Vec<ProgramDefinition>,
    pub examples: Vec<ExampleDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributions: Vec<Contribution>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_mode_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_window_kind_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributions_json: Option<String>,
    /// 🗣️ Active UI locale (e.g. "en", "de"); plugins resolve their own label set from this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 🗣️ Active terminology id ("native" default, or an app-declared alternative term set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology: Option<String>,
}

/// 🗣️ Locale/terminology-aware label patch for an already-instantiated app's manifest, resolved fresh per `ViewState`
/// (unlike `AppDefinition`, which is assembled once at plugin-load time and cannot itself react to locale changes).
/// The shell merges this over the static `AppDefinition` labels by id; ids absent from a map keep their static English label.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppLabelsOverlay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_label: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub window_kind_labels: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub panel_tab_labels: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mode_labels: std::collections::HashMap<String, String>,
}

impl AppLabelsOverlay {
    /// 🗣️ Starts an overlay pre-populated with the well-known framework panel-tab labels (Document/Catalogue/Inspection/Parameters) for every panel tab id supplied; apps then extend it with their own window-kind/mode labels.
    pub fn with_framework_panel_tabs(panel_tab_ids: impl IntoIterator<Item = impl Into<String>>, is_de: bool) -> Self {
        let mut overlay = Self::default();
        for id in panel_tab_ids {
            let id = id.into();
            if let Some(label) = crate::layout::framework_panel_tab_label(&id, is_de) {
                overlay.panel_tab_labels.insert(id, label.into());
            }
        }
        overlay
    }
}

//#region 🔖Kernel
pub mod kernel {
//! 🧠 Local-first action kernel contracts: actions, operations, capabilities, window I/O.

use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖Identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssetHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityToken(pub u128);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PluginInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActionInvocationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AppInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActorId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaId(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentVersion(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowKindId(pub String);
//#endregion 🔖Identifiers

//#region 🔖HybridLogicalTimestamp
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridLogicalTimestamp {
    pub actor: u64,
    pub physical_ms: u64,
    pub logical: u64,
}

impl HybridLogicalTimestamp {
    pub fn new(actor: u64, physical_ms: u64) -> Self {
        Self {
            actor,
            physical_ms,
            logical: 0,
        }
    }

    pub fn tick(&mut self, physical_ms: u64) {
        if physical_ms > self.physical_ms {
            self.physical_ms = physical_ms;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
    }

    pub fn merge(&mut self, other: &Self) {
        if other.physical_ms > self.physical_ms {
            self.physical_ms = other.physical_ms;
            self.logical = other.logical;
        } else if other.physical_ms == self.physical_ms && other.logical > self.logical {
            self.logical = other.logical;
        }
        self.logical = self.logical.saturating_add(1);
    }

    pub fn cmp_key(&self) -> (u64, u64) {
        (self.physical_ms, self.logical)
    }
}
//#endregion 🔖HybridLogicalTimestamp

//#region 🔖Capability
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Rights {
    Read,
    Write,
    Invoke,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceKind {
    Document,
    Projection,
    Window,
    Asset,
    Network,
    Backbone,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    Instance,
    App,
    Plugin,
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityRequirement {
    pub resource: ResourceKind,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub subject: PluginInstanceId,
    pub resource: ResourceId,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityGrant {
    pub token: CapabilityToken,
    pub capability: Capability,
}
//#endregion 🔖Capability

//#region 🔖Action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDef {
    pub id: ActionId,
    pub input_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_capabilities: Vec<CapabilityRequirement>,
    pub deterministic: bool,
    pub produces_operations: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub id: ActionInvocationId,
    pub app: AppInstanceId,
    pub action: ActionId,
    pub input: Value,
    pub actor: ActorId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causal_context: Vec<OperationId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub level: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HostEffect {
    OpenWindow { kind: WindowKindId, params: Value },
    CloseWindow { window: WindowHandle },
    Notify { message: String },
    RequestSync,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppEvent {
    pub kind: String,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDiff {
    pub schema_id: SchemaId,
    pub payload: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UndoPolicy {
    ExactBaseOnly,
    TransformAgainstConcurrent,
    SemanticUndo,
    CompensatingAction,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseOperation {
    pub target_operation: OperationId,
    pub inverse_diff: DocumentDiff,
    pub base_version: DocumentVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<OperationId>,
    pub undo_policy: UndoPolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelOperation {
    pub id: OperationId,
    pub document: DocumentHandle,
    pub base_version: DocumentVersion,
    pub action_id: ActionInvocationId,
    pub diff: DocumentDiff,
    pub inverse: InverseOperation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<OperationId>,
    pub author: ActorId,
    pub timestamp: HybridLogicalTimestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoGroup {
    pub action_id: ActionInvocationId,
    pub operations: Vec<OperationId>,
    pub inverse_operations: Vec<InverseOperation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub output: Value,
    pub operations: Vec<KernelOperation>,
    pub inverse_group: UndoGroup,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_effects: Vec<HostEffect>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<AppEvent>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionContext {
    pub invocation: ActionInvocation,
    pub document_projection: Value,
    pub view_state: super::ViewState,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub granted_capabilities: Vec<CapabilityGrant>,
}
//#endregion 🔖Action

//#region 🔖Sync
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PayloadHash(pub String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpEnvelope {
    pub id: OperationId,
    pub actor: ActorId,
    pub document: DocumentId,
    pub schema_version: SchemaVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deps: Vec<OperationId>,
    pub payload_hash: PayloadHash,
    pub diff: DocumentDiff,
    pub inverse: InverseOperation,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OpDagError {
    #[error("duplicate operation id: {0}")]
    Duplicate(String),
}

/// @emoji 🕸️ Causal DAG of exchanged {@link OpEnvelope}s: buffers envelopes until their deps are applied.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpDag {
    envelopes: std::collections::HashMap<String, OpEnvelope>,
    applied: std::collections::HashSet<String>,
    applied_order: Vec<String>,
    drained: usize,
    pending: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InsertResult {
    Applied,
    Pending,
    AlreadyApplied,
}

impl OpDag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, envelope: OpEnvelope) -> Result<InsertResult, OpDagError> {
        let id = envelope.id.0.clone();
        if self.applied.contains(&id) {
            return Ok(InsertResult::AlreadyApplied);
        }
        if self.envelopes.contains_key(&id) {
            return Err(OpDagError::Duplicate(id));
        }
        for dependency in &envelope.deps {
            if !self.applied.contains(&dependency.0) && !self.envelopes.contains_key(&dependency.0) {
                self.envelopes.insert(id.clone(), envelope);
                if !self.pending.contains(&id) {
                    self.pending.push(id);
                }
                return Ok(InsertResult::Pending);
            }
        }
        self.envelopes.insert(id.clone(), envelope);
        self.mark_applied(&id);
        self.drain_ready();
        Ok(InsertResult::Applied)
    }

    pub fn ready(&self) -> Vec<&OpEnvelope> {
        self.pending
            .iter()
            .filter_map(|id| self.envelopes.get(id))
            .filter(|envelope| {
                envelope
                    .deps
                    .iter()
                    .all(|dependency| self.applied.contains(&dependency.0))
            })
            .collect()
    }

    pub fn applied_ids(&self) -> Vec<String> {
        self.applied.iter().cloned().collect()
    }

    /// @emoji 🧺 Drains envelopes applied since the last drain, in causal application order.
    pub fn drain_applied_envelopes(&mut self) -> Vec<OpEnvelope> {
        let fresh: Vec<String> = self.applied_order[self.drained..].to_vec();
        self.drained = self.applied_order.len();
        fresh
            .iter()
            .filter_map(|id| self.envelopes.get(id).cloned())
            .collect()
    }

    fn mark_applied(&mut self, id: &str) {
        self.applied.insert(id.to_string());
        self.applied_order.push(id.to_string());
        self.pending.retain(|pending| pending != id);
    }

    fn drain_ready(&mut self) {
        loop {
            let ready: Vec<String> = self
                .pending
                .iter()
                .filter(|id| {
                    self.envelopes
                        .get(*id)
                        .is_some_and(|envelope| {
                            envelope
                                .deps
                                .iter()
                                .all(|dependency| self.applied.contains(&dependency.0))
                        })
                })
                .cloned()
                .collect();
            if ready.is_empty() {
                break;
            }
            for id in ready {
                self.mark_applied(&id);
            }
        }
    }
}

#[cfg(test)]
mod op_dag_tests {
    use super::*;

    fn sample_envelope(id: &str, deps: Vec<&str>) -> OpEnvelope {
        OpEnvelope {
            id: OperationId(id.into()),
            actor: ActorId("actor-1".into()),
            document: DocumentId("document-1".into()),
            schema_version: SchemaVersion("test.v1".into()),
            deps: deps.into_iter().map(|dep| OperationId(dep.into())).collect(),
            payload_hash: PayloadHash("hash".into()),
            diff: DocumentDiff {
                schema_id: SchemaId("diff.v1".into()),
                payload: serde_json::json!({"value": id}),
            },
            inverse: InverseOperation {
                target_operation: OperationId(id.into()),
                inverse_diff: DocumentDiff {
                    schema_id: SchemaId("diff.v1".into()),
                    payload: serde_json::json!({}),
                },
                base_version: DocumentVersion(0),
                dependencies: Vec::new(),
                undo_policy: UndoPolicy::ExactBaseOnly,
            },
        }
    }

    #[test]
    fn inserts_pending_until_dependencies_arrive() {
        let mut dag = OpDag::new();
        assert!(matches!(
            dag.insert(sample_envelope("op-2", vec!["op-1"])),
            Ok(InsertResult::Pending)
        ));
        assert!(matches!(
            dag.insert(sample_envelope("op-1", vec![])),
            Ok(InsertResult::Applied)
        ));
        assert_eq!(dag.applied_ids().len(), 2);
    }

    #[test]
    fn drains_applied_envelopes_in_causal_order() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("op-2", vec!["op-1"])).unwrap();
        dag.insert(sample_envelope("op-1", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(
            drained.iter().map(|envelope| envelope.id.0.clone()).collect::<Vec<_>>(),
            vec!["op-1".to_string(), "op-2".to_string()]
        );
        assert!(dag.drain_applied_envelopes().is_empty(), "second drain yields nothing new");
        dag.insert(sample_envelope("op-3", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].id.0, "op-3");
    }
}
//#endregion 🔖Sync

//#region 🔖Window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appearance {
    pub mode: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEvent {
    pub kind: String,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionRequest {
    pub invocation: ActionInvocation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDef {
    pub id: WindowKindId,
    pub params_schema: SchemaId,
    pub document_projection_schema: SchemaId,
    pub input_event_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<CapabilityRequirement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowInput {
    pub window: WindowHandle,
    pub params: Value,
    pub document_projection: Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<WindowEvent>,
    pub size: PhysicalSize,
    pub scale_factor: f64,
    pub appearance: Appearance,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOutput {
    pub ui: super::UiNode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionRequest>,
}
//#endregion 🔖Window

//#region 🔖MergeStrategy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DocumentKind {
    PlainRecord,
    OrderedSequence,
    TextSequence,
    TombstonedGraph,
    ContentAddressedBlob,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeStrategyKind {
    LwwRegister,
    OrderedSequence,
    TextSequence,
    TombstonedGraphSet,
    ContentAddressedBlob,
}

impl DocumentKind {
    pub fn merge_strategy(&self) -> MergeStrategyKind {
        match self {
            DocumentKind::PlainRecord => MergeStrategyKind::LwwRegister,
            DocumentKind::OrderedSequence => MergeStrategyKind::OrderedSequence,
            DocumentKind::TextSequence => MergeStrategyKind::TextSequence,
            DocumentKind::TombstonedGraph => MergeStrategyKind::TombstonedGraphSet,
            DocumentKind::ContentAddressedBlob => MergeStrategyKind::ContentAddressedBlob,
        }
    }
}
//#endregion 🔖MergeStrategy
}
//#endregion 🔖Kernel

#[cfg(test)]
mod app_document_tests {
    use super::app_document_label;

    #[test]
    fn formats_app_document_for_chrome() {
        assert_eq!(
            app_document_label(&["semio".into(), "puzzle".into(), "3d".into()]),
            "semio · puzzle · 3d"
        );
    }
}
//#endregion 🔖Manifest

//#region 🔖WireFormatGoldenTests
/** 🧊 Golden wire-format tests: freeze exact JSON for every UiNode/scene/SurfaceKind
before these types move into ui_wgpu, so the move can be proven byte-identical. */
#[cfg(test)]
mod ui_node_wire_format_tests {
    use super::*;

    fn act(action: &str) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: "ctrl".into(),
            action: action.into(),
            args: None,
        }
    }

    fn sample_tree() -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: Some("md".into()),
            padding: None,
            id: Some("root".into()),
            selected: None,
            activate: None,
            drop_action: None,
            children: vec![
                UiNode::Text(UiTextNode {
                    value: "Hello".into(),
                    emphasize: Some(true),
                    data_attributes: None,
                }),
                UiNode::Button(UiButtonNode {
                    id: Some("btn1".into()),
                    icon_id: "icon.save".into(),
                    label: "Save".into(),
                    action: act("save"),
                    style: None,
                    disabled: Some(false),
                }),
                UiNode::Separator(UiSeparatorNode {}),
                UiNode::Input(UiInputNode {
                    id: "inp1".into(),
                    input_kind: "text".into(),
                    value: "abc".into(),
                    placeholder: Some("type...".into()),
                    commit: None,
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    on_change: act("setValue"),
                }),
                UiNode::Select(UiSelectNode {
                    id: "sel1".into(),
                    value: "a".into(),
                    items: vec![
                        UiSelectItem { value: "a".into(), label: "A".into() },
                        UiSelectItem { value: "b".into(), label: "B".into() },
                    ],
                    placeholder: None,
                    on_change: act("selectChange"),
                }),
                UiNode::Toggle(UiToggleNode {
                    id: "tog1".into(),
                    icon_id: "icon.bold".into(),
                    pressed: true,
                    text: None,
                    on_change: act("toggle"),
                }),
                UiNode::Vec3(UiVec3Node {
                    id: "vec1".into(),
                    value: Some([1.0, 2.0, 3.0]),
                    on_change: act("vecChange"),
                }),
                UiNode::KeyValue(UiKeyValueNode {
                    entries: vec![UiKeyValueEntry { label: "K".into(), value: "V".into() }],
                }),
                UiNode::Slider(UiSliderNode {
                    id: "sl1".into(),
                    value: 0.5,
                    min: 0.0,
                    max: 1.0,
                    step: 0.1,
                    unit: Some("%".into()),
                    on_change: act("sliderChange"),
                }),
                UiNode::NumberStepper(UiNumberStepperNode {
                    id: "num1".into(),
                    value: 2.0,
                    step: 1.0,
                    uniform: true,
                    on_absolute: act("setAbs"),
                    on_delta: act("setDelta"),
                }),
                UiNode::Ring(UiRingNode {
                    id: "ring1".into(),
                    orb_id: "orb1".into(),
                    t: 0.25,
                    disabled: None,
                    on_change: act("ringChange"),
                }),
                UiNode::IconSelect(UiIconSelectNode {
                    id: "icn1".into(),
                    value: "star".into(),
                    uniform: true,
                    classifier_kind: "icon".into(),
                    on_change: act("iconChange"),
                }),
                UiNode::Field(UiFieldNode {
                    id: "field1".into(),
                    label: "Field".into(),
                    description: Some("desc".into()),
                    required: Some(true),
                    error: None,
                    child: Box::new(UiNode::Text(UiTextNode {
                        value: "child".into(),
                        emphasize: None,
                        data_attributes: None,
                    })),
                }),
                UiNode::Section(UiSectionNode {
                    id: "sec1".into(),
                    label: Some("Section".into()),
                    default_open: Some(true),
                    children: vec![],
                }),
                UiNode::Tree(UiTreeNode {
                    sections: vec![UiTreeSectionNode {
                        id: "treesec1".into(),
                        label: Some("Items".into()),
                        default_open: Some(true),
                        items: vec![UiTreeItemNode::base("item1", "Item 1")],
                    }],
                    selected_ids: Some(vec!["item1".into()]),
                    highlighted_ids: None,
                    selection_change: None,
                    drop_action: None,
                }),
                UiNode::Image(UiImageNode {
                    id: "img1".into(),
                    src: "icon.png".into(),
                    alt: Some("alt text".into()),
                }),
                UiNode::ComponentScene(UiComponentSceneNode {
                    surface_id: "surf1".into(),
                    controller_id: "ctrl".into(),
                    component_kind: SurfaceKind::World3d,
                    pane_id: None,
                    binding_id: None,
                    canvas_2d: None,
                    world_3d: Some(World3dScene {
                        camera_json: "{}".into(),
                        meshes_json: "[]".into(),
                        instances_json: "[]".into(),
                        selection_json: "{}".into(),
                        vortices_json: None,
                        attractions_json: None,
                        target_volumes_json: None,
                        references_json: None,
                        brush_preview_json: None,
                        interaction_json: None,
                        engagement_preview_json: None,
                        lod_json: None,
                        chunking_json: None,
                        context_menu_json: None,
                        environment_json: None,
                        frame_json: None,
                        fit_json: None,
                    }),
                    node_graph: None,
                    text_editor: None,
                    table: None,
                    raster: None,
                    virtual_file_system: None,
                    gis_map: None,
                    puzzle2d_board: None,
                    icon_render: None,
                    note_canvas: None,
                    vcs_history: None,
                }),
                UiNode::ExternalSlot(UiExternalSlotNode {
                    plugin_id: "plugin1".into(),
                    app_id: "app1".into(),
                    body_key: "body1".into(),
                    params_json: "{}".into(),
                }),
            ],
        })
    }

    const GOLDEN_UI_NODE_TREE_JSON: &str = "{\"type\":\"stack\",\"direction\":\"vertical\",\"gap\":\"md\",\"id\":\"root\",\"children\":[{\"type\":\"text\",\"value\":\"Hello\",\"emphasize\":true},{\"type\":\"button\",\"id\":\"btn1\",\"iconId\":\"icon.save\",\"label\":\"Save\",\"action\":{\"controllerId\":\"ctrl\",\"action\":\"save\"},\"disabled\":false},{\"type\":\"separator\"},{\"type\":\"input\",\"id\":\"inp1\",\"inputKind\":\"text\",\"value\":\"abc\",\"placeholder\":\"type...\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"setValue\"}},{\"type\":\"select\",\"id\":\"sel1\",\"value\":\"a\",\"items\":[{\"value\":\"a\",\"label\":\"A\"},{\"value\":\"b\",\"label\":\"B\"}],\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"selectChange\"}},{\"type\":\"toggle\",\"id\":\"tog1\",\"iconId\":\"icon.bold\",\"pressed\":true,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"toggle\"}},{\"type\":\"vec3\",\"id\":\"vec1\",\"value\":[1.0,2.0,3.0],\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"vecChange\"}},{\"type\":\"keyValue\",\"entries\":[{\"label\":\"K\",\"value\":\"V\"}]},{\"type\":\"slider\",\"id\":\"sl1\",\"value\":0.5,\"min\":0.0,\"max\":1.0,\"step\":0.1,\"unit\":\"%\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"sliderChange\"}},{\"type\":\"numberStepper\",\"id\":\"num1\",\"value\":2.0,\"step\":1.0,\"uniform\":true,\"onAbsolute\":{\"controllerId\":\"ctrl\",\"action\":\"setAbs\"},\"onDelta\":{\"controllerId\":\"ctrl\",\"action\":\"setDelta\"}},{\"type\":\"ring\",\"id\":\"ring1\",\"orbId\":\"orb1\",\"t\":0.25,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"ringChange\"}},{\"type\":\"iconSelect\",\"id\":\"icn1\",\"value\":\"star\",\"uniform\":true,\"classifierKind\":\"icon\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"iconChange\"}},{\"type\":\"field\",\"id\":\"field1\",\"label\":\"Field\",\"description\":\"desc\",\"required\":true,\"child\":{\"type\":\"text\",\"value\":\"child\"}},{\"type\":\"section\",\"id\":\"sec1\",\"label\":\"Section\",\"defaultOpen\":true,\"children\":[]},{\"type\":\"tree\",\"sections\":[{\"id\":\"treesec1\",\"label\":\"Items\",\"defaultOpen\":true,\"items\":[{\"id\":\"item1\",\"label\":\"Item 1\"}]}],\"selectedIds\":[\"item1\"]},{\"type\":\"image\",\"id\":\"img1\",\"src\":\"icon.png\",\"alt\":\"alt text\"},{\"type\":\"componentScene\",\"surfaceId\":\"surf1\",\"controllerId\":\"ctrl\",\"componentKind\":\"world-3d\",\"world3d\":{\"cameraJson\":\"{}\",\"meshesJson\":\"[]\",\"instancesJson\":\"[]\",\"selectionJson\":\"{}\"}},{\"type\":\"externalSlot\",\"pluginId\":\"plugin1\",\"appId\":\"app1\",\"bodyKey\":\"body1\",\"paramsJson\":\"{}\"}]}";

    #[test]
    fn ui_node_tree_serializes_to_golden_json() {
        let node = sample_tree();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json, GOLDEN_UI_NODE_TREE_JSON,
            "UiNode wire format drifted \u{2014} lock this in before moving the type into ui_wgpu"
        );
        let roundtripped: UiNode = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, node);
    }

    const GOLDEN_SURFACE_KIND_JSON: &str = "[\"canvas-2d\",\"world-3d\",\"node-graph\",\"text-editor\",\"table\",\"raster\",\"virtualFileSystem\",\"gis2d-map\",\"puzzle2d-board\",\"icon-render\",\"note-canvas\",\"vcs-history\"]";

    #[test]
    fn surface_kind_serializes_to_golden_json() {
        let kinds = vec![
            SurfaceKind::Canvas2d,
            SurfaceKind::World3d,
            SurfaceKind::NodeGraph,
            SurfaceKind::TextEditor,
            SurfaceKind::Table,
            SurfaceKind::Raster,
            SurfaceKind::VirtualFileSystem,
            SurfaceKind::GisMap,
            SurfaceKind::Puzzle2dBoard,
            SurfaceKind::IconRender,
            SurfaceKind::NoteCanvas,
            SurfaceKind::VcsHistory,
        ];
        let json = serde_json::to_string(&kinds).unwrap();
        assert_eq!(json, GOLDEN_SURFACE_KIND_JSON);
        let roundtripped: Vec<SurfaceKind> = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, kinds);
    }

    const GOLDEN_SCENES_JSON: &str = "[{\"cameraX\":1.0,\"cameraY\":2.0,\"zoom\":1.5,\"layersJson\":\"[]\"},{\"columnsJson\":\"[]\",\"rowsJson\":\"[]\"},{\"documentSyncJson\":\"{}\",\"assetsJson\":\"[]\",\"cameraJson\":\"{}\",\"selectionJson\":\"[]\",\"hoveredId\":\"h1\",\"activeTool\":\"brush\",\"brushSize\":4.0,\"brushOpacity\":1.0,\"viewMode\":\"composite\"},{\"requestJson\":\"{}\"},{\"schemaJson\":\"{}\",\"rowsJson\":\"[]\",\"emptyMessage\":\"Empty\",\"dragDropEnabled\":true},{\"mapFixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"renderMode\":\"combined\",\"vectorStyle\":\"colored\",\"lodMode\":\"automatic\",\"tileUrlTemplate\":\"/osm/{z}/{x}/{y}.png\",\"vectorTileUrlTemplate\":\"/vt/{z}/{x}/{y}.pbf\",\"layerVisibilityJson\":\"{\\\"raster\\\":true,\\\"water\\\":true,\\\"land\\\":true,\\\"roads\\\":true,\\\"buildings\\\":true,\\\"borders\\\":true,\\\"labels\\\":true,\\\"positions\\\":true,\\\"positionLabels\\\":true,\\\"routes\\\":true,\\\"regions\\\":true}\",\"layerStrokeScaleJson\":\"{\\\"raster\\\":1,\\\"water\\\":1,\\\"land\\\":1,\\\"roads\\\":1,\\\"buildings\\\":1,\\\"borders\\\":1,\\\"labels\\\":1,\\\"positions\\\":1,\\\"positionLabels\\\":1,\\\"routes\\\":1,\\\"regions\\\":1}\",\"selectionJson\":\"{\\\"positions\\\":[],\\\"routes\\\":[]}\",\"hoverJson\":\"null\",\"selectionMethod\":\"rectangle\",\"selectionMode\":\"default\"},{\"fixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"kindCatalogsJson\":\"{}\",\"selectionJson\":\"[]\",\"interactive\":true,\"selectionMethod\":\"rectangle\",\"gridSnapEnabled\":false,\"gridFactor\":1.0,\"suggestionOffset\":0.0,\"brushKindWeightsJson\":\"{}\",\"kindCompatibilityJson\":\"[]\",\"lodMode\":\"automatic\"},{\"documentJson\":\"{}\",\"selectionJson\":\"[]\",\"activeTool\":\"select\",\"viewMode\":\"edit\",\"interactive\":true},{\"columnsJson\":\"[]\"}]";

    #[test]
    fn scene_records_serialize_to_golden_json() {
        let scenes = (
            Canvas2dScene { camera_x: 1.0, camera_y: 2.0, zoom: 1.5, layers_json: "[]".into() },
            TableScene { columns_json: "[]".into(), rows_json: "[]".into() },
            RasterScene {
                document_sync_json: "{}".into(),
                assets_json: "[]".into(),
                camera_json: "{}".into(),
                selection_json: "[]".into(),
                hovered_id: Some("h1".into()),
                active_tool: "brush".into(),
                brush_size: 4.0,
                brush_opacity: 1.0,
                view_mode: "composite".into(),
                composite_viewport_json: None,
            },
            IconRenderScene { request_json: "{}".into(), footer: None, frame_json: None },
            VirtualFileSystemScene {
                schema_json: "{}".into(),
                rows_json: "[]".into(),
                selected_row_ids_json: None,
                hovered_row_id: None,
                empty_message: Some("Empty".into()),
                drag_drop_enabled: Some(true),
            },
            GisMapScene::base("{}".into(), "{}".into()),
            Puzzle2dBoardScene::base("{}".into(), "{}".into(), true),
            NoteCanvasScene::base("{}".into(), "select".into(), "edit".into(), true),
            VcsHistoryScene { columns_json: "[]".into() },
        );
        let json = serde_json::to_string(&scenes).unwrap();
        assert_eq!(json, GOLDEN_SCENES_JSON);
    }
}
//#endregion 🔖WireFormatGoldenTests
// #endregion ui
}


pub use action_bus::{ActionBus, ActionHandler};
pub use layout::{
    collect_window_kind_ids_from_layout, create_default_layout, create_named_layout, create_stack_layout,
    create_tab_stack_layout, create_window_layout, merge_named_layouts, ActionDescriptor, NamedLayout,
    StyleSpec, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption,
    WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, default_viewport_engagement,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
    FRAMEWORK_PANEL_TAB_PARAMETERS_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
    framework_panel_tab_label,
};
pub use mesh::{
    mesh_box, mesh_cone, mesh_cylinder, mesh_from_glb, mesh_from_indexed, mesh_from_kind, mesh_ico_sphere,
    mesh_plane, mesh_to_glb, mesh_to_obj, mesh_torus, mesh_uv_sphere, MeshData,
    dwg_drawing_to_mesh, dwg_drawing_to_paths, dwg_from_bytes, dwg_to_bytes, mesh_to_dwg_drawing, paths_to_dwg_drawing,
    DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgLayer, DwgPathSegment,
};
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use tools::{tool_button, tool_collection, tool_separator, tool_toggle, ToolCategory, ToolNode};
pub use ui::*;
pub use ui::kernel::{
    ActorId, AppEvent, AppInstanceId, AssetHandle, Capability, CapabilityGrant, CapabilityRequirement,
    CapabilityToken, ActionContext, ActionDef, ActionId, ActionInvocation, ActionInvocationId,
    ActionRequest, ActionResult, Diagnostic, HostEffect, HybridLogicalTimestamp, InverseOperation,
    InsertResult, KernelOperation, MergeStrategyKind, DocumentDiff, DocumentHandle, DocumentId, DocumentKind,
    DocumentVersion, OpDag, OpDagError, OpEnvelope, OperationId, PayloadHash, PhysicalSize, PluginInstanceId,
    ResourceId, ResourceKind, Appearance, Rights, SchemaId, SchemaVersion, Scope, UndoGroup, UndoPolicy,
    WindowEvent, WindowHandle, WindowInput, WindowKindDef, WindowKindId, WindowOutput,
};
