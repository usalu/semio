//! 🖍️ The marks a tool shows over a window while it works: how a mark is drawn, what it is, the mark itself and the preview of a tool, shared by the gestures and by the windows that paint them.

use crate::editor::bim::gestures::plane::P;
use value_derive::{FromValue, ToValue};

//#region 🔖️Marks
/// 🖌️ How a mark is drawn.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum Style {
    Ghost,
    Guide,
    Snap,
    Handle,
    Warning,
    Selection,
}

/// 🔷️ What a mark is.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum Shape {
    Path,
    Dot,
    Label,
    Snap,
}

/// 🫧️ One mark over a window: a path (flat `x, y` pairs), a dot, a text label at the first point or a snap marker named by its kind in `text`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Mark {
    pub shape: Shape,
    pub style: Style,
    pub closed: bool,
    pub points: Vec<f64>,
    pub text: String,
}

impl Mark {
    pub fn path(points: &[P], closed: bool, style: Style) -> Self {
        Self { shape: Shape::Path, style, closed, points: points.iter().flatten().copied().collect(), text: String::new() }
    }

    pub fn dot(at: P, style: Style) -> Self {
        Self { shape: Shape::Dot, style, closed: false, points: at.to_vec(), text: String::new() }
    }

    pub fn label(at: P, text: impl Into<String>) -> Self {
        Self { shape: Shape::Label, style: Style::Guide, closed: false, points: at.to_vec(), text: text.into() }
    }

    pub fn corners(&self) -> Vec<P> {
        self.points.chunks_exact(2).map(|pair| [pair[0], pair[1]]).collect()
    }
}

/// 🫧️ The marks a tool shows over its window.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Preview {
    pub marks: Vec<Mark>,
}

impl Preview {
    pub fn of(marks: Vec<Mark>) -> Self {
        Self { marks }
    }

    pub fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }

    /// 🔤️ The preview as the text a window transient carries (empty for no marks).
    pub fn to_text(&self) -> String {
        if self.marks.is_empty() { String::new() } else { semio_framework_pack_json::to_json_string(self) }
    }

    /// 🔤️ The preview a window transient carries; text that does not decode is no preview.
    pub fn from_text(text: &str) -> Self {
        if text.is_empty() {
            return Self::default();
        }
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()
    }
}

//#endregion 🔖️Marks
