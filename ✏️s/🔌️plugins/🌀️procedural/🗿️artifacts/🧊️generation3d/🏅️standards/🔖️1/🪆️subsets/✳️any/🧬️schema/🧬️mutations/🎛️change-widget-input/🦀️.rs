//! 🎛️ `change-widget-input` payload — the ABSOLUTE typed literal one unconnected operator input of the generator graph
//! (or a text source's text) is set to: an inspector field committed on blur, or a parameter a mesh edit inserts its
//! operator with. The intent IS the value, so editing it in history replays exactly that value on any base.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_label_number, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️WidgetInputValue
/// 📏️ The longest text input, in characters (the schema's `maxLength`).
pub const CHANGE_WIDGET_INPUT_MAXIMUM_TEXT: usize = 1_048_576;
/// 📐️ The longest input name, in characters (the schema's `maxLength`).
pub const CHANGE_WIDGET_INPUT_MAXIMUM_CHANNEL: usize = 256;

/// 🔣️ One typed input literal — on the wire `{type, value}`, a point or a vector as `[x, y, z]`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "type", content = "value", rename_all = "camelCase")]
pub enum WidgetInputValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Point([f64; 3]),
    Vector([f64; 3]),
}

impl WidgetInputValue {
    /// 🏷️ The literal schema an operator param of this type declares (`$schema`).
    pub fn schema(&self) -> &'static str {
        match self {
            Self::Number(_) => "number",
            Self::Text(_) => "text",
            Self::Boolean(_) => "boolean",
            Self::Point(_) => "point",
            Self::Vector(_) => "vector",
        }
    }

    /// 🧮️ Whether the value meets its schema bounds: every number finite, a text of at most 1 MiB characters.
    pub fn admissible(&self) -> bool {
        match self {
            Self::Number(value) => value.is_finite(),
            Self::Point(axes) | Self::Vector(axes) => axes.iter().all(|value| value.is_finite()),
            Self::Text(text) => text.chars().count() <= CHANGE_WIDGET_INPUT_MAXIMUM_TEXT,
            Self::Boolean(_) => true,
        }
    }

    /// 🧩️ The typed literal an operator param holds: `{"$schema", value}`, or `{"$schema", x, y, z}` for a point or a
    /// vector.
    pub fn literal(&self) -> dsl::DslValue {
        let schema = ("$schema".to_string(), dsl::DslValue::String(self.schema().into()));
        let scalar = |value| dsl::DslValue::Object(vec![schema.clone(), ("value".to_string(), value)]);
        match self {
            Self::Number(value) => scalar(dsl::DslValue::float(*value)),
            Self::Text(value) => scalar(dsl::DslValue::String(value.clone())),
            Self::Boolean(value) => scalar(dsl::DslValue::Bool(*value)),
            Self::Point(axes) | Self::Vector(axes) => dsl::DslValue::Object(std::iter::once(schema.clone()).chain(["x", "y", "z"].iter().zip(axes).map(|(axis, value)| (axis.to_string(), dsl::DslValue::float(*value)))).collect()),
        }
    }

    /// 🔎️ The typed input a param literal states, or `None` for a literal of another shape.
    pub fn of_literal(literal: &dsl::DslValue) -> Option<Self> {
        let number = |key: &str| literal.get(key).and_then(dsl::DslValue::as_f64);
        let axes = || Some([number("x")?, number("y")?, number("z")?]);
        Some(match literal.get("$schema").and_then(dsl::DslValue::as_str)? {
            "number" => Self::Number(number("value")?),
            "text" => Self::Text(literal.get("value")?.as_str()?.to_string()),
            "boolean" => Self::Boolean(literal.get("value")?.as_bool()?),
            "point" => Self::Point(axes()?),
            "vector" => Self::Vector(axes()?),
            _ => return None,
        })
    }

    /// 🖊️ The value as a history label prints it, English and German.
    fn label(&self) -> (String, String) {
        match self {
            Self::Number(value) => generation3d_label_number(*value),
            Self::Text(value) => (format!("\"{value}\""), format!("\"{value}\"")),
            Self::Boolean(value) => (if *value { "on" } else { "off" }.to_string(), if *value { "ein" } else { "aus" }.to_string()),
            Self::Point(axes) | Self::Vector(axes) => {
                let [(x_en, x_de), (y_en, y_de), (z_en, z_de)] = axes.map(generation3d_label_number);
                (format!("({x_en}, {y_en}, {z_en})"), format!("({x_de}; {y_de}; {z_de})"))
            }
        }
    }
}
//#endregion 🔖️WidgetInputValue

//#region 🔖️ChangeWidgetInput
/// 🎛️ Sets input `channel` of operator `id` to the typed literal `input` (a text source's `text` to its text).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeWidgetInput {
    pub id: String,
    pub channel: String,
    #[value(flatten)]
    pub input: WidgetInputValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_widget_input(id: impl Into<String>, channel: impl Into<String>, input: WidgetInputValue) -> Generation3dMutation {
    Generation3dMutation::ChangeWidgetInput(ChangeWidgetInput { id: id.into(), channel: channel.into(), input })
}

/// 🚧️ Why an input cannot land on its widget — each one is `mutation.target-mismatch`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMismatch {
    Wired,
    NoSuchInput,
    Untyped,
    OtherType(&'static str),
}

impl InputMismatch {
    /// 🗒️ The reason as an outcome message words it.
    pub fn reason(&self) -> String {
        match self {
            Self::Wired => "is driven by a wire".into(),
            Self::NoSuchInput => "does not exist on this widget".into(),
            Self::Untyped => "holds no typed literal".into(),
            Self::OtherType(schema) => format!("holds a {schema} literal"),
        }
    }
}

impl ChangeWidgetInput {
    /// ⚖️ Whether the payload meets its schema's hard bounds: a non-empty operator id, a non-empty input name of at most
    /// 256 characters and an admissible value.
    pub fn admissible(&self) -> bool {
        !self.id.is_empty() && !self.channel.is_empty() && self.channel.chars().count() <= CHANGE_WIDGET_INPUT_MAXIMUM_CHANNEL && self.input.admissible()
    }

    /// 🛬️ How this input lands on `widget`, the base widget it addresses (`wired` when a wire drives the input): the
    /// widget with the input set — an operator's `channel` param replaced by the typed literal, a text source's text by a
    /// text input on `text` — or `None` when the input already holds this value. The diff and the retained replay both
    /// read it, so they never disagree.
    pub fn landing(&self, widget: &semio_framework_artifact_flow_flow::Widget, wired: bool) -> Result<Option<semio_framework_artifact_flow_flow::Widget>, InputMismatch> {
        use semio_framework_artifact_flow_flow::Widget;
        if wired {
            return Err(InputMismatch::Wired);
        }
        let current = match widget {
            Widget::Neuron { params, .. } => match params.get(&self.channel) {
                None => None,
                Some(value) => Some(WidgetInputValue::of_literal(&dsl::ToValue::to_value(value)).ok_or(InputMismatch::Untyped)?),
            },
            Widget::InputNote { text, .. } if self.channel == "text" => Some(WidgetInputValue::Text(text.clone())),
            _ => return Err(InputMismatch::NoSuchInput),
        };
        match current {
            Some(current) if current.schema() != self.input.schema() => Err(InputMismatch::OtherType(current.schema())),
            Some(current) if current == self.input => Ok(None),
            _ => match (widget, &self.input) {
                (Widget::InputNote { id, .. }, WidgetInputValue::Text(text)) => Ok(Some(Widget::InputNote { id: id.clone(), text: text.clone() })),
                (Widget::InputNote { .. }, _) => Err(InputMismatch::OtherType("text")),
                _ => crate::standards::v1::subsets::any::schema::mutations::generation3d_with_params(widget, vec![(self.channel.as_str(), self.input.literal())]).map(Some).ok_or(InputMismatch::Untyped),
            },
        }
    }
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeWidgetInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "widget-input", kind: "change-widget-input", record: "ChangedWidgetInput" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = self.input.label();
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set input \"{}\" of \"{}\" to {en}", self.channel, self.id), &format!("Eingang \"{}\" von \"{}\" auf {de} setzen", self.channel, self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeWidgetInput
