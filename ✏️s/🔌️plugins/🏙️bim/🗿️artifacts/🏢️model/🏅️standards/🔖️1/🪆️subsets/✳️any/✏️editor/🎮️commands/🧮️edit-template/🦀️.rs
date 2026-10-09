//! 🧮️ `editTemplate`: one edit of the property definitions of a property set template, from the properties panel: add a definition (`Name` or `Name:kind`), remove or move one, or set one of its fields (name, kind, unit,
//! description, required, default, minimum, maximum, allowed values). The edit is a pure function of the definition list ([`apply`]) and becomes one `set-property-template` mutation that decides whether the result stands;
//! changing the kind of a definition clears its default, allowed values and range, which belong to the old kind.

use crate::editor::bim::entities::property_value;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::mutations::set_property_template::SetPropertyTemplate;
use crate::{ModelMutation, ModelSnapshot, PropertyDef, PropertyKind, PropertyTemplatePatch};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-template")]
pub struct EditTemplate {
    pub id: String,
    pub op: String,
    pub index: String,
    pub field: String,
    pub value: String,
}

/// 🏷️ The kind a name denotes (`text`, `real`, `length`, …).
pub fn parse_kind(text: &str) -> Option<PropertyKind> {
    PropertyKind::ALL.into_iter().find(|kind| kind.name().eq_ignore_ascii_case(text.trim()))
}

fn optional(text: &str) -> Option<String> {
    Some(text.trim().to_string()).filter(|text| !text.is_empty())
}

fn bound(text: &str) -> Result<Option<f64>, &'static str> {
    match text.trim() {
        "" => Ok(None),
        word => word.parse::<f64>().ok().filter(|number| number.is_finite()).map(Some).ok_or("bim.template.value-invalid"),
    }
}

fn set_field(definition: &mut PropertyDef, field: &str, value: &str) -> Result<(), &'static str> {
    match field {
        "name" => definition.name = value.trim().to_string(),
        "kind" => {
            let kind = parse_kind(value).ok_or("bim.template.value-invalid")?;
            if kind != definition.kind {
                definition.kind = kind;
                definition.default_value = None;
                definition.allowed = Vec::new();
                definition.minimum = None;
                definition.maximum = None;
            }
        }
        "unit" => definition.unit = optional(value),
        "description" => definition.description = optional(value),
        "required" => definition.required = value.trim().parse().map_err(|_| "bim.template.value-invalid")?,
        "default_value" => definition.default_value = optional(value).map(|text| property_value(&text, Some(definition.kind.name())).ok_or("bim.template.value-invalid")).transpose()?,
        "minimum" => definition.minimum = bound(value)?,
        "maximum" => definition.maximum = bound(value)?,
        "allowed" => definition.allowed = value.split(';').map(str::trim).filter(|part| !part.is_empty()).map(|part| property_value(part, Some(definition.kind.name())).ok_or("bim.template.value-invalid")).collect::<Result<_, _>>()?,
        _ => return Err("bim.template.edit-invalid"),
    }
    Ok(())
}

/// 🧮️ The definition list after one edit; the error is the fault code of an edit that makes no sense (an unknown operation, field or position, a value that is no number or no value of the kind).
pub fn apply(definitions: &[PropertyDef], edit: &EditTemplate) -> Result<Vec<PropertyDef>, &'static str> {
    let mut list = definitions.to_vec();
    let at = || edit.index.trim().parse::<usize>().ok().filter(|index| *index < definitions.len()).ok_or("bim.template.edit-invalid");
    match edit.op.as_str() {
        "add" => {
            let (name, kind) = edit.value.split_once(':').map_or((edit.value.as_str(), "text"), |(name, kind)| (name, kind));
            let kind = parse_kind(kind).ok_or("bim.template.value-invalid")?;
            if name.trim().is_empty() {
                return Err("bim.template.edit-invalid");
            }
            list.push(PropertyDef { name: name.trim().to_string(), kind, unit: None, description: None, required: false, default_value: None, allowed: Vec::new(), minimum: None, maximum: None });
        }
        "remove" => {
            list.remove(at()?);
        }
        "up" | "down" => {
            let index = at()?;
            let target = if edit.op == "up" { index.checked_sub(1) } else { Some(index + 1).filter(|next| *next < list.len()) };
            list.swap(index, target.ok_or("bim.template.edit-invalid")?);
        }
        "set" => {
            let index = at()?;
            set_field(&mut list[index], edit.field.trim(), &edit.value)?;
        }
        _ => return Err("bim.template.edit-invalid"),
    }
    Ok(list)
}

pub fn handle(payload: &EditTemplate, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let template = doc.snapshot.property_templates.get(&payload.id).ok_or_else(|| fault("bim.template.target-missing", format!("the property template '{}' does not exist", payload.id)))?;
    let properties = apply(&template.properties, payload).map_err(|code| fault(code, format!("the template '{}' cannot take the edit '{} {}'", template.name, payload.op, payload.field)))?;
    Ok(Emit::mutations(vec![ModelMutation::SetPropertyTemplate(SetPropertyTemplate::from_patch(payload.id.clone(), PropertyTemplatePatch { properties: Some(properties), ..Default::default() }))]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
