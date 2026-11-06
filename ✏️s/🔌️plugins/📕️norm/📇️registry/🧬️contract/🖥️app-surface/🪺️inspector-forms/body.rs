//! 🪟️ Owned schema controls lend ordinary native staged-body views and exact resident edit bindings.
use super::*;
use semio_framework_plugin_decl_staged_form::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_plugin::app::time_travel::{time_travel_pointer_get, time_travel_variant_selector};

#[derive(Clone, semio_framework_value::RetireOwned)]
struct Choice { key: String, label: String }
#[derive(semio_framework_value::RetireOwned)]
struct Choices(Vec<Choice>);
impl StagedScalarChoiceSource for Choices {
    fn len(&self) -> usize { self.0.len() }
    fn choice(&self, index: usize) -> Option<StagedScalarChoice<'_>> { self.0.get(index).map(|choice| StagedScalarChoice { key: &choice.key, label: &choice.label }) }
}
#[derive(semio_framework_value::RetireOwned)]
enum Kind { Text(String), Choice { value: String, choices: Choices, path: Vec<String> }, Button }
#[derive(semio_framework_value::RetireOwned)]
enum Binding { Action(StagedFormBinding), Page(StagedFormPageBinding) }
#[derive(semio_framework_value::RetireOwned)]
struct Control { label: String, description: Option<String>, required: bool, kind: Kind, binding: Binding, address: u64 }

/// 🪟️ This immutable page is the only source borrowed by the common paid native body cursor.
#[derive(semio_framework_value::RetireOwned)]
pub struct NormInspectorBodySource {
    pub status: StagedFormStatus,
    controller: String,
    locale: Locale,
    controls: Vec<Control>,
}
fn label(label: &StagedFormLabel, locale: Locale) -> String { match locale { Locale::En => label.en.clone(), Locale::De => label.de.clone() } }
fn value_at<'a>(args: &'a DslValue, path: &[String]) -> Option<&'a DslValue> {
    let mut cursor = args;
    for part in path { cursor = match cursor { DslValue::Object(_) => cursor.get(part)?, DslValue::Array(values) => values.get(part.parse::<usize>().ok()?)?, _ => return None }; }
    Some(cursor)
}
fn text(value: Option<&DslValue>) -> String {
    match value { Some(DslValue::String(value)) => value.clone(), Some(DslValue::Bool(value)) => value.to_string(), Some(DslValue::Number(value)) => value.as_f64().to_string(), _ => String::new() }
}
fn action(token: &StagedFormToken, edit: StagedFormBindingEdit, layout: StagedFormLayout) -> Binding {
    Binding::Action(StagedFormBinding { token: token.clone(), layout, edits: vec![edit] })
}
fn fixed(token: &StagedFormToken, edit: StagedFormEdit) -> Binding { action(token, StagedFormBindingEdit::Fixed { edit }, StagedFormLayout::Replace) }
fn variant_options(plan: &NormInspectorFormPlan, args: &DslValue, path: &[String], choices: &[StagedFormChoice]) -> Option<(Vec<String>, Vec<StagedFormBindingOption>)> {
    if path.len() < 2 || path[0] != plan.draft_key { return None; }
    let mut schema = &plan.input.schema;
    let mut current = args.get(&plan.draft_key)?;
    for part in &path[1..path.len() - 1] {
        match schema {
            ArgSchema::Object { fields } => {
                schema = &active(fields, Some(current)).into_iter().find(|input| input.key() == *part)?.schema;
                current = current.get(part)?;
            },
            ArgSchema::Array { items, .. } => {
                let index = part.parse::<usize>().ok()?;
                if index.to_string() != *part { return None; }
                schema = items;
                current = current.as_array()?.get(index)?;
            },
            _ => return None,
        }
    }
    let ArgSchema::Object { fields } = schema else { return None };
    let leaf = path.last()?;
    let selector = time_travel_variant_selector(fields).filter(|selector| selector.key() == *leaf)?;
    let parent = path[..path.len() - 1].to_vec();
    let options = choices.iter().map(|choice| {
        let variant = choice.value.as_str()?;
        Some(StagedFormBindingOption { key: choice.id.clone(), value: time_travel_switch_variant(fields, selector, current, variant) })
    }).collect::<Option<Vec<_>>>()?;
    Some((parent, options))
}
fn push_control(controls: &mut Vec<Control>, label: String, description: Option<String>, required: bool, kind: Kind, binding: Binding) {
    controls.push(Control { label, description, required, kind, binding, address: controls.len() as u64 });
}
fn collect(plan: &NormInspectorFormPlan, request: &StagedFormRequest, fields: &[StagedFormField], controls: &mut Vec<Control>) {
    let locale = request.locale;
    let token = &request.token;
    for field in fields {
        match field {
            StagedFormField::Text { path, label: title, required, description, .. } => push_control(controls, label(title, locale), description.as_ref().map(|text| label(text, locale)), *required, Kind::Text(text(value_at(&request.args, path))), action(token, StagedFormBindingEdit::Text { path: path.clone() }, StagedFormLayout::Preserve)),
            StagedFormField::Choice { path, label: title, required, choices, description, .. } => {
                let options = choices.iter().map(|choice| StagedFormBindingOption { key: choice.id.clone(), value: choice.value.clone() }).collect();
                let (target, options) = variant_options(plan, &request.args, path, choices).unwrap_or_else(|| (path.clone(), options));
                let selected = choices.iter().find(|choice| Some(&choice.value) == value_at(&request.args, path)).map(|choice| choice.id.clone()).unwrap_or_default();
                push_control(controls, label(title, locale), description.as_ref().map(|text| label(text, locale)), *required, Kind::Choice { path: path.clone(), value: selected, choices: Choices(choices.iter().map(|choice| Choice { key: choice.id.clone(), label: label(&choice.label, locale) }).collect()) }, action(token, StagedFormBindingEdit::Choice { kind: StagedFormChoiceEditKind::Set, path: target, options }, StagedFormLayout::Replace));
            },
            StagedFormField::Record { path, label: title, fields, create, optional } => {
                if value_at(&request.args, path).and_then(DslValue::as_object).is_none() {
                    if let Some(value) = create { push_control(controls, format!("{} — {}", label(title, locale), match locale { Locale::En => "Create draft", Locale::De => "Entwurf anlegen" }), None, !*optional, Kind::Button, fixed(token, StagedFormEdit::Set { path: path.clone(), value: value.clone() })); }
                } else {
                    collect(plan, request, fields, controls);
                    if *optional { controls.push(Control { label: format!("{} — {}", label(title, locale), match locale { Locale::En => "Clear", Locale::De => "Leeren" }), description: None, required: false, kind: Kind::Button, binding: fixed(token, StagedFormEdit::Remove { path: path.clone() }), address: controls.len() as u64 }); }
                }
            },
            StagedFormField::Array { path, label: title, rows, add } => {
                collect(plan, request, rows, controls);
                for (index, _) in rows.iter().enumerate() {
                    let mut row = path.clone(); row.push(index.to_string());
                    controls.push(Control { label: format!("{} {} — {}", label(title, locale), index + 1, match locale { Locale::En => "Remove", Locale::De => "Entfernen" }), description: None, required: false, kind: Kind::Button, binding: fixed(token, StagedFormEdit::Remove { path: row }), address: controls.len() as u64 });
                }
                for choice in add {
                    let edit = if value_at(&request.args, path).is_none() { StagedFormEdit::Set { path: path.clone(), value: DslValue::Array(vec![choice.value.clone()]) } } else { StagedFormEdit::Append { path: path.clone(), value: choice.value.clone() } };
                    controls.push(Control { label: format!("{} — {}", label(title, locale), label(&choice.label, locale)), description: None, required: false, kind: Kind::Button, binding: fixed(token, edit), address: controls.len() as u64 });
                }
            },
            StagedFormField::Constant { .. } => {},
        }
    }
}
impl NormInspectorBodySource {
    /// 🧾️ The scheduled preparation owner supplies the already-native-admitted status and resident request.
    pub fn prepare(plan: &NormInspectorFormPlan, request: &StagedFormRequest, status: StagedFormStatus, controller: &str) -> Result<Self, ValueError> {
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "staging.page");
        let response = plan.response(request);
        let mut controls = Vec::new();
        collect(plan, request, &response.fields, &mut controls);
        let offset = request.page.as_ref().map_or(Some(0), |page| page.field_offset.parse::<usize>().ok()).ok_or_else(invalid)?;
        if offset > controls.len() || offset == controls.len() && offset != 0 { return Err(invalid()); }
        let total = controls.len();
        let large = |control: &Control| matches!(&control.kind, Kind::Choice { choices, .. } if choices.0.len() > 32);
        let first_large = controls.iter().enumerate().skip(offset).take(STAGED_FORM_BODY_LIMITS.content).find(|(_, control)| large(control)).map(|(index, _)| index);
        let end = match first_large { Some(index) if index == offset => offset + 1, Some(index) => index, None => (offset + STAGED_FORM_BODY_LIMITS.content).min(total) };
        let mut page = controls.drain(offset..end).collect::<Vec<_>>();
        let mut previous = (offset > 0).then(|| (offset.saturating_sub(STAGED_FORM_BODY_LIMITS.content), None));
        let mut next = (end < total).then_some((end, None));
        if let Some(control) = page.first_mut().filter(|control| large(control)) {
            let Kind::Choice { value, choices, path } = &mut control.kind else { unreachable!() };
            let selected = request.page.as_ref().and_then(|page| page.choice.as_ref()).filter(|choice| choice.path == *path);
            let choice_offset = selected.map_or(Some(0), |choice| choice.offset.parse::<usize>().ok()).ok_or_else(invalid)?;
            if choice_offset % 31 != 0 || choice_offset >= choices.0.len() { return Err(invalid()); }
            let count = choices.0.len();
            let current = choices.0.iter().find(|choice| choice.key == *value).cloned();
            let mut visible = choices.0.drain(choice_offset..(choice_offset + 31).min(count)).collect::<Vec<_>>();
            if let Some(current) = current.filter(|current| !visible.iter().any(|choice| choice.key == current.key)) { visible.insert(0, current); }
            choices.0 = visible;
            let Binding::Action(binding) = &mut control.binding else { return Err(invalid()) };
            let Some(StagedFormBindingEdit::Choice { options, .. }) = binding.edits.first_mut() else { return Err(invalid()) };
            options.retain(|option| choices.0.iter().any(|choice| choice.key == option.key));
            if choice_offset > 0 { previous = Some((offset, Some(StagedFormChoicePage { path: path.clone(), offset: (choice_offset - 31).to_string() }))); }
            if choice_offset + 31 < count { next = Some((offset, Some(StagedFormChoicePage { path: path.clone(), offset: (choice_offset + 31).to_string() }))); }
        }
        for (target, en, de) in [(previous, "Previous values", "Vorherige Werte"), (next, "Next values", "Weitere Werte")] {
            if let Some((field_offset, choice)) = target { page.push(Control { label: match request.locale { Locale::En => en, Locale::De => de }.into(), description: None, required: false, kind: Kind::Button, binding: Binding::Page(StagedFormPageBinding { token: request.token.clone(), page: StagedFormPage { field_offset: field_offset.to_string(), diagnostic_offset: "0".into(), choice } }), address: u64::MAX - page.len() as u64 - 1 }); }
        }
        Ok(Self { status, controller: controller.into(), locale: request.locale, controls: page })
    }
}
impl StagedBodySource for NormInspectorBodySource {
    fn identity(&self) -> usize { self as *const _ as usize }
    fn title(&self) -> &str { match self.locale { Locale::En => "Complete item values", Locale::De => "Eintragswerte vervollständigen" } }
    fn status(&self) -> &StagedFormStatus { &self.status }
    fn controls(&self) -> usize { self.controls.len() }
    fn control(&self, index: usize) -> Result<StagedBodyControl<'_>, ValueError> {
        let control = self.controls.get(index).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "staging.control"))?;
        let kind = match &control.kind {
            Kind::Text(value) => StagedScalarKind::Text { value },
            Kind::Choice { value, choices, .. } => StagedScalarKind::Choice { value, choices },
            Kind::Button => StagedScalarKind::Button { page: matches!(control.binding, Binding::Page(_)) },
        };
        Ok(StagedBodyControl { scalar: StagedScalarControl { owner: self.identity(), address: control.address, controller: &self.controller, label: &control.label, description: control.description.as_deref(), disabled: false, kind }, required: control.required, error: None, binding: Some(match &control.binding { Binding::Action(binding) => StagedBodyBinding::Action(binding), Binding::Page(binding) => StagedBodyBinding::Page(binding) }) })
    }
}
