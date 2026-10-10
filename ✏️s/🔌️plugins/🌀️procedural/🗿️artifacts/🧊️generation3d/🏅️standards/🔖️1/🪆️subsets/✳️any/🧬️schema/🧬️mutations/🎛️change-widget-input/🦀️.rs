//! 🎛️ `change-widget-input` payload — the ABSOLUTE typed literal one unconnected operator input of the generator graph
//! (or a text source's text) is set to: an inspector field committed on blur, or a parameter a mesh edit inserts its
//! operator with. The intent IS the value, so editing it in history replays exactly that value on any base.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_label_number,Generation3dMutation};

use crate::Generation3dSnapshot;

//#region 🔖️WidgetInputValue
/// 📏️ The longest text input, in characters (the schema's `maxLength`).
pub const CHANGE_WIDGET_INPUT_MAXIMUM_TEXT: usize = 16_777_216;
/// 📐️ The longest input name, in characters (the schema's `maxLength`).
pub const CHANGE_WIDGET_INPUT_MAXIMUM_CHANNEL: usize = 256;
/// ✂️ The most characters of a text input a history label prints before it elides the rest with `…`.
pub const CHANGE_WIDGET_INPUT_LABEL_TEXT: usize = 32;

/// 🔣️ One typed input literal — on the wire `{type, value}`, a point or a vector as `[x, y, z]`.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetInputValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Point([f64; 3]),
    Vector([f64; 3]),
    Plane { origin: [f64; 3], normal: [f64; 3] },
    NumberList(Vec<f64>),
    TextList(Vec<String>),
    BooleanList(Vec<bool>),
    PointList(Vec<[f64; 3]>),
    VectorList(Vec<[f64; 3]>),
}

impl WidgetInputValue {
    /// 🏷️ The literal schema an operator param of this type declares (`$schema`).
    pub fn schema(&self) -> &'static str {
        match self {
            Self::Number(_) | Self::NumberList(_) => "number",
            Self::Text(_) | Self::TextList(_) => "text",
            Self::Boolean(_) | Self::BooleanList(_) => "boolean",
            Self::Point(_) | Self::PointList(_) => "point",
            Self::Vector(_) | Self::VectorList(_) => "vector",
            Self::Plane { .. } => "plane",
        }
    }

    /// 📃️ Ordered scalar items of a homogeneous collection, or no collection for a scalar.
    pub fn items(&self) -> Option<Vec<Self>> {
        Some(match self {
            Self::NumberList(items) => items.iter().copied().map(Self::Number).collect(),
            Self::TextList(items) => items.iter().cloned().map(Self::Text).collect(),
            Self::BooleanList(items) => items.iter().copied().map(Self::Boolean).collect(),
            Self::PointList(items) => items.iter().copied().map(Self::Point).collect(),
            Self::VectorList(items) => items.iter().copied().map(Self::Vector).collect(),
            _ => return None,
        })
    }

    /// 🧩️ One homogeneous list of the declared element type; nested collections are refused.
    pub fn collection(schema: &str, items: Vec<Self>) -> Option<Self> {
        if items.len() > 1024 || items.iter().any(|item| item.schema() != schema || item.items().is_some() || !item.admissible()) { return None; }
        Some(match schema {
            "number" => Self::NumberList(items.into_iter().filter_map(|item| if let Self::Number(value) = item { Some(value) } else { None }).collect()),
            "text" => Self::TextList(items.into_iter().filter_map(|item| if let Self::Text(value) = item { Some(value) } else { None }).collect()),
            "boolean" => Self::BooleanList(items.into_iter().filter_map(|item| if let Self::Boolean(value) = item { Some(value) } else { None }).collect()),
            "point" => Self::PointList(items.into_iter().filter_map(|item| if let Self::Point(value) = item { Some(value) } else { None }).collect()),
            "vector" => Self::VectorList(items.into_iter().filter_map(|item| if let Self::Vector(value) = item { Some(value) } else { None }).collect()),
            _ => return None,
        })
    }

    /// 🧮️ Whether the value meets its schema bounds: every number finite, a text of at most 16 MiB characters. These are
    /// the only hard bounds there are: an operator input declares a type, a cardinality and a default but no numeric range
    /// (`ChannelSpec`), and a pure fold never reads the kind descriptor anyway (design §20.9) — an operator refuses an
    /// out-of-domain value when it evaluates, as the node's answer, never as a refused edit.
    pub fn admissible(&self) -> bool {
        match self {
            Self::Number(value) => value.is_finite(),
            Self::Point(axes) | Self::Vector(axes) => axes.iter().all(|value| value.is_finite()),
            Self::Plane { origin, normal } => origin.iter().chain(normal).all(|value| value.is_finite()),
            Self::Text(text) => text.chars().count() <= CHANGE_WIDGET_INPUT_MAXIMUM_TEXT,
            Self::Boolean(_) => true,
            _ => self.items().is_some_and(|items| items.len() <= 1024 && items.iter().all(Self::admissible)),
        }
    }

    /// 🧩️ The typed literal an operator param holds: `{"$schema", value}`, or `{"$schema", x, y, z}` for a point or a
    /// vector.
    pub fn literal(&self) -> semio_framework_value::DslValue {
        if let Some(items) = self.items() {
            return semio_framework_value::DslValue::Object(std::iter::once(("$schema".into(), semio_framework_value::DslValue::String("list".into()))).chain(items.iter().enumerate().map(|(index, item)| (index.to_string(), item.literal()))).collect());
        }
        let schema = ("$schema".to_string(), semio_framework_value::DslValue::String(self.schema().into()));
        let scalar = |value| semio_framework_value::DslValue::Object(vec![schema.clone(), ("value".to_string(), value)]);
        match self {
            Self::Number(value) => scalar(semio_framework_value::DslValue::float(*value)),
            Self::Text(value) => scalar(semio_framework_value::DslValue::String(value.clone())),
            Self::Boolean(value) => scalar(semio_framework_value::DslValue::Bool(*value)),
            Self::Point(axes) | Self::Vector(axes) => semio_framework_value::DslValue::Object(std::iter::once(schema.clone()).chain(["x", "y", "z"].iter().zip(axes).map(|(axis, value)| (axis.to_string(), semio_framework_value::DslValue::float(*value)))).collect()),
            Self::Plane { origin, normal } => semio_framework_value::DslValue::Object(vec![schema.clone(), ("origin".to_string(), Self::Point(*origin).literal()), ("normal".to_string(), Self::Vector(*normal).literal())]),
            _ => unreachable!(),
        }
    }

    /// 🔎️ The typed input a param literal states, or `None` for a literal of another shape.
    pub fn of_literal(literal: &semio_framework_value::DslValue) -> Option<Self> {
        if literal.get("$schema").and_then(semio_framework_value::DslValue::as_str) == Some("list") {
            return Self::of_literal_as(literal, literal.get("0")?.get("$schema")?.as_str()?);
        }
        let number = |key: &str| literal.get(key).and_then(semio_framework_value::DslValue::as_f64);
        let axes = || Some([number("x")?, number("y")?, number("z")?]);
        Some(match literal.get("$schema").and_then(semio_framework_value::DslValue::as_str)? {
            "number" => Self::Number(number("value")?),
            "text" => Self::Text(literal.get("value")?.as_str()?.to_string()),
            "boolean" => Self::Boolean(literal.get("value")?.as_bool()?),
            "point" => Self::Point(axes()?),
            "vector" => Self::Vector(axes()?),
            "plane" => match (Self::of_literal(literal.get("origin")?)?, Self::of_literal(literal.get("normal")?)?) {
                (Self::Point(origin), Self::Vector(normal)) => Self::Plane { origin, normal },
                _ => return None,
            },
            _ => return None,
        })
    }

    /// 🏷️ Reads an existing neural list dictionary using its declared element type, including an empty list.
    pub fn of_literal_as(literal: &semio_framework_value::DslValue, schema: &str) -> Option<Self> {
        if literal.get("$schema").and_then(semio_framework_value::DslValue::as_str) != Some("list") { return Self::of_literal(literal); }
        let fields = literal.as_object()?;
        if fields.iter().any(|(key, _)| key != "$schema" && key.parse::<usize>().ok().is_none_or(|index| index.to_string() != *key)) { return None; }
        let count = fields.len().checked_sub(1)?;
        let items = (0..count).map(|index| Self::of_literal(literal.get(&index.to_string())?)).collect::<Option<Vec<_>>>()?;
        Self::collection(schema, items)
    }

    /// 🖊️ The value as a history label prints it, English and German.
    fn label(&self) -> (String, String) {
        match self {
            Self::Number(value) => generation3d_label_number(*value),
            Self::Text(value) => {
                let shown: String = value.chars().take(CHANGE_WIDGET_INPUT_LABEL_TEXT).collect();
                let quoted = if shown.len() < value.len() { format!("\"{shown}…\"") } else { format!("\"{shown}\"") };
                (quoted.clone(), quoted)
            }
            Self::Boolean(value) => (if *value { "on" } else { "off" }.to_string(), if *value { "ein" } else { "aus" }.to_string()),
            Self::Point(axes) | Self::Vector(axes) => {
                let [(x_en, x_de), (y_en, y_de), (z_en, z_de)] = axes.map(generation3d_label_number);
                (format!("({x_en}, {y_en}, {z_en})"), format!("({x_de}; {y_de}; {z_de})"))
            }
            Self::Plane { origin, normal } => {
                let [(ox_en, ox_de), (oy_en, oy_de), (oz_en, oz_de)] = origin.map(generation3d_label_number);
                let [(nx_en, nx_de), (ny_en, ny_de), (nz_en, nz_de)] = normal.map(generation3d_label_number);
                (format!("plane at ({ox_en}, {oy_en}, {oz_en}) facing ({nx_en}, {ny_en}, {nz_en})"), format!("Ebene bei ({ox_de}; {oy_de}; {oz_de}) mit Normale ({nx_de}; {ny_de}; {nz_de})"))
            }
            _ => { let count = self.items().map_or(0, |items| items.len()); (format!("{count} {} items", self.schema()), format!("{count} Listeneinträge")) }
        }
    }
}
//#endregion 🔖️WidgetInputValue

//#region 🔖️ChangeWidgetInput
/// 🎛️ Sets input `channel` of operator `id` to the typed literal `input` (a text source's `text` to its text).
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeWidgetInput {
    pub id: String,
    pub channel: String,
    pub input: WidgetInputValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_widget_input(id: impl Into<String>, channel: impl Into<String>, input: WidgetInputValue) -> Generation3dMutation {
    Generation3dMutation::ChangeWidgetInput(ChangeWidgetInput { id: id.into(), channel: channel.into(), input })
}

/// 🚧️ Why an input cannot land on its widget: an input the record does not hold is `mutation.target-missing`, every other
/// refusal `mutation.target-mismatch`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputRefusal {
    UnknownChannel,
    Wired,
    Untyped,
    OtherType(&'static str),
    PortContract,
}

impl InputRefusal {
    /// 🏷️ The outcome code of the refusal (9-code vocabulary).
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnknownChannel => "mutation.target-missing",
            _ => "mutation.target-mismatch",
        }
    }

    /// 🗒️ The reason as an outcome message words it.
    pub fn reason(&self) -> String {
        match self {
            Self::UnknownChannel => "does not exist on this widget".into(),
            Self::Wired => "is driven by a wire".into(),
            Self::Untyped => "holds no typed literal".into(),
            Self::OtherType(schema) => format!("holds a {schema} literal"),
            Self::PortContract => "does not hold a value of this shape (scalar vs. list, or another item type)".into(),
        }
    }
}

impl ChangeWidgetInput {
    /// ⚖️ Whether the payload meets its schema's hard bounds: a non-empty operator id, a non-empty input name of at most
    /// 256 characters and an admissible value.
    pub fn admissible(&self) -> bool {
        !self.id.is_empty() && !self.channel.is_empty() && self.channel.chars().count() <= CHANGE_WIDGET_INPUT_MAXIMUM_CHANNEL && self.input.admissible()
    }

    /// 🛬️ How this input lands on `widget`, the base widget it addresses (`wired` when a wire drives the input), decided
    /// from the record ALONE (design §20.9): an operator's inputs are the keys of its `params` (an inserted operator records
    /// every declared input's default literal), a text source's only input is `text`. The answer is the widget with the
    /// input set to the typed literal, or `None` when the input already holds this value; a scalar never lands on a list
    /// input nor a list on a scalar one. The diff and the retained replay both read it, so they never disagree, and the same
    /// history folds identically on a replica without the operator's extension.
    pub fn landing(&self, widget: &semio_framework_artifact_flow_flow::Widget, wired: bool) -> Result<Option<semio_framework_artifact_flow_flow::Widget>, InputRefusal> {
        use semio_framework_artifact_flow_flow::Widget;
        let stored = match widget {
            Widget::Neuron { params, .. } => semio_framework_value::ToValue::to_value(params.get(&self.channel).ok_or(InputRefusal::UnknownChannel)?),
            Widget::InputNote { text, .. } if self.channel == "text" => WidgetInputValue::Text(text.clone()).literal(),
            _ => return Err(InputRefusal::UnknownChannel),
        };
        if wired {
            return Err(InputRefusal::Wired);
        }
        let stored_list = stored.get("$schema").and_then(semio_framework_value::DslValue::as_str) == Some("list");
        let stored_item = stored.get("0").and_then(|item| item.get("$schema")).and_then(semio_framework_value::DslValue::as_str);
        if stored_list != self.input.items().is_some() || stored_item.is_some_and(|schema| schema != self.input.schema()) {
            return Err(InputRefusal::PortContract);
        }
        let current = WidgetInputValue::of_literal_as(&stored, self.input.schema()).ok_or(InputRefusal::Untyped)?;
        if current.schema() != self.input.schema() {
            return Err(InputRefusal::OtherType(current.schema()));
        }
        if current == self.input {
            return Ok(None);
        }
        match (widget, &self.input) {
            (Widget::InputNote { id, .. }, WidgetInputValue::Text(text)) => Ok(Some(Widget::InputNote { id: id.clone(), text: text.clone() })),
            (Widget::InputNote { .. }, _) => Err(InputRefusal::OtherType("text")),
            _ => crate::standards::v1::subsets::any::schema::mutations::generation3d_with_params(widget, vec![(self.channel.as_str(), self.input.literal())]).map(Some).ok_or(InputRefusal::Untyped),
        }
    }
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeWidgetInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "widget-input", kind: "change-widget-input", record: "ChangedWidgetInput" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
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
