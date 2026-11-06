//! 🪺️ Typed inspector drafts reuse mutation input schemas and retain raw entered values until native admission.
use semio_framework::{ActionArgDef, ArgSchema, mutation_input_defs, registered_input_schema_document};
use semio_framework::action_staging::*;
use semio_plugin::app::time_travel::{time_travel_coerce, time_travel_default_value, time_travel_switch_variant, time_travel_variant_selector};
use semio_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;

pub const NORM_INSPECTOR_FORM_BODY: &str = "norm.inspector.form";

/// 🎯️ A prepared draft commits through the original existing tool using its declared typed boundary.
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub enum NormInspectorFormTarget {
    Insert { path: String, index: usize },
    Replace { path: String },
}

/// 🧬️ An owned schema-derived plan; source preparation and publication retain their host work ownership.
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct NormInspectorFormPlan {
    pub target: NormInspectorFormTarget,
    pub input: ActionArgDef,
    pub seed: DslValue,
    pub draft_key: String,
}

fn localized(label: &LocalizedLabel) -> StagedFormLabel {
    StagedFormLabel { en: label.resolve(Terminology::Native, Locale::En).to_string(), de: label.resolve(Terminology::Native, Locale::De).to_string() }
}
fn active<'a>(fields: &'a [ActionArgDef], value: Option<&DslValue>) -> Vec<&'a ActionArgDef> {
    let Some(selector) = time_travel_variant_selector(fields) else { return fields.iter().collect() };
    let selected = value.and_then(|value| value.get(&selector.key())).and_then(DslValue::as_str);
    fields.iter().filter(|field| field.group.is_none() || selected.is_some() && field.group.as_deref() == selected).collect()
}
/// 🌱️ Draft seeds choose one declared variant and never combine required fields from incompatible branches.
pub fn norm_inspector_seed(schema: &ArgSchema) -> DslValue {
    match schema {
        ArgSchema::Object { fields } => {
            let variant = time_travel_variant_selector(fields);
            let selected = variant.and_then(|selector| selector.default.as_ref().and_then(DslValue::as_str).map(str::to_string).or_else(|| match &selector.schema { ArgSchema::String { options, .. } => options.first().map(|option| option.value.clone()), _ => None }));
            DslValue::Object(fields.iter().filter(|input| input.required && (variant.is_none() || input.group.is_none() || input.group.as_deref() == selected.as_deref())).map(|input| (input.key(), input.default.clone().unwrap_or_else(|| norm_inspector_seed(&input.schema)))).collect())
        },
        ArgSchema::Array { items, min_items, .. } => DslValue::Array((0..min_items.unwrap_or(0)).map(|_| norm_inspector_seed(items)).collect()),
        _ => time_travel_default_value(schema),
    }
}

fn field(input: &ActionArgDef, path: Vec<String>, value: Option<&DslValue>) -> StagedFormField {
    let label = localized(&input.label);
    let description = input.description.as_ref().map(localized);
    match &input.schema {
        ArgSchema::Object { fields } => StagedFormField::Record {
            path: path.clone(), label, optional: !input.required,
            create: value.filter(|value| value.as_object().is_some()).is_none().then(|| input.default.clone().unwrap_or_else(|| norm_inspector_seed(&input.schema))),
            fields: active(fields, value).into_iter().map(|input| { let mut child = path.clone(); child.push(input.key()); field(input, child, value.and_then(|value| value.get(&input.key()))) }).collect(),
        },
        ArgSchema::Array { items, .. } => {
            let mut item = input.clone(); item.schema = *items.clone(); item.required = true; item.default = None;
            StagedFormField::Array { path: path.clone(), label: label.clone(),
                rows: value.and_then(DslValue::as_array).unwrap_or_default().iter().enumerate().map(|(index, value)| { let mut path = path.clone(); path.push(index.to_string()); field(&item, path, Some(value)) }).collect(),
                add: vec![StagedFormChoice { id: "append".into(), label: StagedFormLabel { en: "Add item".into(), de: "Eintrag hinzufügen".into() }, value: norm_inspector_seed(items) }],
            }
        },
        ArgSchema::String { options, .. } if !options.is_empty() => StagedFormField::Choice { path, label, required: input.required, provenance: StagedFormProvenance::Declaration, choices: options.iter().map(|option| StagedFormChoice { id: option.value.clone(), label: localized(&option.label), value: DslValue::String(option.value.clone()) }).collect(), resets: Vec::new(), description },
        ArgSchema::Boolean => StagedFormField::Choice { path, label, required: input.required, provenance: StagedFormProvenance::Declaration, choices: [(false, "No", "Nein"), (true, "Yes", "Ja")].into_iter().map(|(value, en, de)| StagedFormChoice { id: value.to_string(), label: StagedFormLabel { en: en.into(), de: de.into() }, value: DslValue::Bool(value) }).collect(), resets: Vec::new(), description },
        _ => StagedFormField::Text { path, label, required: input.required, format: StagedFormTextFormat::Text, pattern: None, description },
    }
}
fn coerce(input: &ActionArgDef, value: &DslValue) -> Option<DslValue> {
    if input.nullable && value.is_null() { return Some(DslValue::Null); }
    match (&input.schema, value) {
        (ArgSchema::Object { fields }, DslValue::Object(entries)) => {
            let allowed = active(fields, Some(value));
            let mut output = Vec::new();
            for (key, value) in entries {
                let input = allowed.iter().find(|input| input.key() == *key)?;
                output.push((key.clone(), coerce(input, value)?));
            }
            if allowed.iter().any(|input| input.required && !entries.iter().any(|(key, _)| *key == input.key())) { return None; }
            Some(DslValue::Object(output))
        },
        (ArgSchema::Array { items, .. }, DslValue::Array(values)) => {
            let mut item = input.clone(); item.schema = *items.clone();
            values.iter().map(|value| coerce(&item, value)).collect::<Option<Vec<_>>>().map(DslValue::Array)
        },
        (ArgSchema::Object { .. } | ArgSchema::Array { .. }, _) => None,
        _ => time_travel_coerce(input, false, value),
    }
}
impl NormInspectorFormPlan {
    /// ➕️ Select the declared item field from its concrete insertion mutation payload schema.
    pub fn insertion(schema: &str, item: &str, path: String, index: usize) -> Result<Self, String> {
        let inputs = mutation_input_defs(schema, &registered_input_schema_document).map_err(|error| error.to_string())?;
        let input = inputs.into_iter().find(|input| input.key() == item).ok_or_else(|| format!("missing insertion item schema: {item}"))?;
        let seed = input.default.clone().unwrap_or_else(|| norm_inspector_seed(&input.schema));
        Ok(Self { target: NormInspectorFormTarget::Insert { path, index }, input, seed, draft_key: "value".into() })
    }

    /// 🔀️ Replace a complete structured boundary, dropping old variant fields through the history editor's typed switch.
    pub fn variant(input: ActionArgDef, value: &DslValue, selector_key: &str, variant: &str, path: String) -> Result<Self, String> {
        let ArgSchema::Object { fields } = &input.schema else { return Err("variant boundary is not an object".into()) };
        let selector = time_travel_variant_selector(fields).filter(|selector| selector.key() == selector_key).ok_or_else(|| "variant selector has no declared input vocabulary".to_string())?;
        let ArgSchema::String { options, .. } = &selector.schema else { return Err("variant selector is not a choice".into()) };
        if !options.iter().any(|option| option.value == variant) { return Err("unknown variant".into()); }
        let seed = time_travel_switch_variant(fields, selector, value, variant);
        Ok(Self { target: NormInspectorFormTarget::Replace { path }, input, seed, draft_key: "replacement".into() })
    }

    /// 🧾️ An absent draft offers explicit creation from its seed; a refused present draft stays resident unchanged.
    pub fn response(&self, request: &StagedFormRequest) -> StagedFormResponse {
        let value = request.args.get(&self.draft_key);
        let mut input = self.input.clone(); input.required = true;
        let mut root = field(&input, vec![self.draft_key.clone()], value);
        if let StagedFormField::Record { create, .. } = &mut root { if value.is_none() { *create = Some(self.seed.clone()); } }
        StagedFormResponse { token: request.token.clone(), fields: vec![root], diagnostics: Vec::new(), ready: false }
    }

    /// 🚦️ Readiness carries only the exact native-admitted command payload; refusal preserves the request.
    pub fn status(&self, request: &StagedFormRequest, admit: impl FnOnce(&DslValue) -> Result<(), String>) -> StagedFormStatus {
        let (commit_args, diagnostics) = match self.admitted_args(request, admit) {
            Ok(args) => (Some(args), Vec::new()),
            Err(_) => (None, vec![StagedFormDiagnostic { path: vec![self.draft_key.clone()], code: "norm.inspector.admission".into(), label: StagedFormLabel { en: "Complete the required values accepted by this standard.".into(), de: "Erforderliche, von dieser Norm zugelassene Werte vervollständigen.".into() } }]),
        };
        StagedFormStatus { token: request.token.clone(), ready: commit_args.is_some(), commit_args, diagnostic_offset: "0".into(), diagnostic_total: diagnostics.len().to_string(), diagnostics }
    }

    /// 🛡️ Shape coercion is only preparation; the supplied native domain authority must admit the existing command arguments.
    pub fn admitted_args(&self, request: &StagedFormRequest, admit: impl FnOnce(&DslValue) -> Result<(), String>) -> Result<DslValue, String> {
        let draft = request.args.get(&self.draft_key).ok_or_else(|| "required draft is absent".to_string())?;
        let value = coerce(&self.input, draft).ok_or_else(|| "draft does not match the selected input shape".to_string())?;
        let mut args = match &self.target {
            NormInspectorFormTarget::Insert { path, index } => vec![("path".into(), DslValue::String(path.clone())), ("index".into(), DslValue::json_number(*index as f64))],
            NormInspectorFormTarget::Replace { path } => vec![("path".into(), DslValue::String(path.clone()))],
        };
        args.push(("value".into(), value));
        let args = DslValue::Object(args); admit(&args)?; Ok(args)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


mod body;
pub use body::*;

#[path="paid/🦀️.rs"]
mod paid;
pub use paid::*;

#[cfg(test)]
#[global_allocator]
static INSPECTOR_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
