//! 🧰 Declarative per-mode toolbar tool trees.

use crate::layout::CommandDescriptor;
use serde::{Deserialize, Serialize};

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
        on_press: CommandDescriptor,
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
        on_change: CommandDescriptor,
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
        children: Vec<ToolNode>,
    },
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
    on_press: CommandDescriptor,
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
        on_press,
    }
}

pub fn tool_toggle(
    id: impl Into<String>,
    icon_id: impl Into<String>,
    label: impl Into<String>,
    pressed: bool,
    on_change: CommandDescriptor,
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
        children,
    }
}
