//! 📐 Window layouts and measure rails.

use crate::ui::CommandDescriptor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabStackLayout {
    pub kind: String,
    pub tabs: Vec<TabSpec>,
    pub active_tab_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabSpec {
    pub id: String,
    pub label: String,
    pub body_key: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayout {
    pub window_kind_id: String,
    pub layout: TabStackLayout,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedLayout {
    pub id: String,
    pub label: String,
    pub windows: Vec<WindowLayout>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

pub fn create_tab_stack_layout(tabs: Vec<TabSpec>, active_tab_id: impl Into<String>) -> TabStackLayout {
    TabStackLayout {
        kind: "tabStack".into(),
        tabs,
        active_tab_id: active_tab_id.into(),
    }
}
