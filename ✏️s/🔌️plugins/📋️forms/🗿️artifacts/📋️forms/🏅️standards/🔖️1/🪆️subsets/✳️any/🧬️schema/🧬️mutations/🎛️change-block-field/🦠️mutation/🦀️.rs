//! 🎛️ Forms mutation payload — `change-block-field`, the ABSOLUTE set of ONE field of one question (design §17.1 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): an inspector edit, a choice edit or a vector component scrub IS the value of
//! that field, so a history edit changes exactly that value and replays it on any base. `replace-block` stays for a
//! whole-question replacement by intent (a kind change re-seeds the question).

use crate::{FormExpr, FormMutation, FormQuestion, FormQuestionOption, FormVectorField, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🎛️BlockField
/// 🔣️ One settable question field and its typed value (`None` clears an optional field) — on the wire `{field, value}`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(tag = "field", content = "value", rename_all = "camelCase")]
pub enum BlockField {
    Label(String),
    Description(Option<String>),
    Placeholder(Option<String>),
    Text(Option<String>),
    Unit(Option<String>),
    Schema(Option<String>),
    Src(Option<String>),
    Accept(Option<String>),
    FixtureSlug(Option<String>),
    Required(Option<bool>),
    Min(Option<f64>),
    Max(Option<f64>),
    Step(Option<f64>),
    Default(Option<semio_framework_value::DslValue>),
    Params(Option<semio_framework_value::DslValue>),
    Condition(Option<FormExpr>),
    Options(Option<Vec<FormQuestionOption>>),
    Fields(Option<Vec<FormVectorField>>),
}

impl BlockField {
    /// 🗂️ One empty value per field, in declaration order — the field catalog [`BlockField::changes`] walks.
    fn catalog() -> [Self; 18] {
        [
            Self::Label(String::new()),
            Self::Description(None),
            Self::Placeholder(None),
            Self::Text(None),
            Self::Unit(None),
            Self::Schema(None),
            Self::Src(None),
            Self::Accept(None),
            Self::FixtureSlug(None),
            Self::Required(None),
            Self::Min(None),
            Self::Max(None),
            Self::Step(None),
            Self::Default(None),
            Self::Params(None),
            Self::Condition(None),
            Self::Options(None),
            Self::Fields(None),
        ]
    }

    /// 🏷️ The field's label in English and German, as a history row names it.
    pub fn labels(&self) -> (&'static str, &'static str) {
        match self {
            Self::Label(_) => ("label", "Beschriftung"),
            Self::Description(_) => ("description", "Beschreibung"),
            Self::Placeholder(_) => ("placeholder", "Platzhalter"),
            Self::Text(_) => ("text", "Text"),
            Self::Unit(_) => ("unit", "Einheit"),
            Self::Schema(_) => ("schema", "Schema"),
            Self::Src(_) => ("source", "Quelle"),
            Self::Accept(_) => ("accepted files", "Akzeptierte Dateien"),
            Self::FixtureSlug(_) => ("fixture slug", "Fixture-Kürzel"),
            Self::Required(_) => ("required flag", "Pflichtfeld"),
            Self::Min(_) => ("minimum", "Minimum"),
            Self::Max(_) => ("maximum", "Maximum"),
            Self::Step(_) => ("step", "Schrittweite"),
            Self::Default(_) => ("default", "Standardwert"),
            Self::Params(_) => ("parameters", "Parameter"),
            Self::Condition(_) => ("condition", "Bedingung"),
            Self::Options(_) => ("options", "Optionen"),
            Self::Fields(_) => ("vector fields", "Vektorfelder"),
        }
    }

    /// 🔎️ The same field as `question` holds it now — what an undo restores.
    pub fn read(&self, question: &FormQuestion) -> Self {
        match self {
            Self::Label(_) => Self::Label(question.label.clone()),
            Self::Description(_) => Self::Description(question.description.clone()),
            Self::Placeholder(_) => Self::Placeholder(question.placeholder.clone()),
            Self::Text(_) => Self::Text(question.text.clone()),
            Self::Unit(_) => Self::Unit(question.unit.clone()),
            Self::Schema(_) => Self::Schema(question.schema.clone()),
            Self::Src(_) => Self::Src(question.src.clone()),
            Self::Accept(_) => Self::Accept(question.accept.clone()),
            Self::FixtureSlug(_) => Self::FixtureSlug(question.example_id.clone()),
            Self::Required(_) => Self::Required(question.required),
            Self::Min(_) => Self::Min(question.min),
            Self::Max(_) => Self::Max(question.max),
            Self::Step(_) => Self::Step(question.step),
            Self::Default(_) => Self::Default(question.default.clone()),
            Self::Params(_) => Self::Params(question.params.clone()),
            Self::Condition(_) => Self::Condition(question.condition.clone()),
            Self::Options(_) => Self::Options(question.options.clone()),
            Self::Fields(_) => Self::Fields(question.fields.clone()),
        }
    }

    /// ✏️ Sets this field to this value on `next`.
    pub fn set_on(&self, next: &mut FormQuestion) {
        match self.clone() {
            Self::Label(value) => next.label = value,
            Self::Description(value) => next.description = value,
            Self::Placeholder(value) => next.placeholder = value,
            Self::Text(value) => next.text = value,
            Self::Unit(value) => next.unit = value,
            Self::Schema(value) => next.schema = value,
            Self::Src(value) => next.src = value,
            Self::Accept(value) => next.accept = value,
            Self::FixtureSlug(value) => next.example_id = value,
            Self::Required(value) => next.required = value,
            Self::Min(value) => next.min = value,
            Self::Max(value) => next.max = value,
            Self::Step(value) => next.step = value,
            Self::Default(value) => next.default = value,
            Self::Params(value) => next.params = value,
            Self::Condition(value) => next.condition = value,
            Self::Options(value) => next.options = value,
            Self::Fields(value) => next.fields = value,
        }
    }

    /// 🧮️ Every field `after` holds differently from `before`, in declaration order — the leaves of an edit that keeps the
    /// question's id and kind.
    pub fn changes(before: &FormQuestion, after: &FormQuestion) -> Vec<Self> {
        Self::catalog().iter().map(|field| field.read(after)).filter(|field| field.read(before) != *field).collect()
    }
}
//#endregion 🎛️BlockField

//#region 🎛️ChangeBlockField
/// 🎚️ Sets `change`'s field of the question `block_id` (in whichever step holds it) to `change`'s value.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeBlockField {
    pub block_id: String,
    #[value(flatten)]
    pub change: BlockField,
}

impl MutationKind<FormsSnapshot, FormMutation> for ChangeBlockField {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "block-field", kind: "change-block-field", record: "ChangedBlockField" };

    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_change_block_field(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse_change_block_field(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = self.change.labels();
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change {en} of question \"{}\"", self.block_id), &format!("{de} der Frage \"{}\" ändern", self.block_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.block_id.clone()]
    }
}
//#endregion 🎛️ChangeBlockField
