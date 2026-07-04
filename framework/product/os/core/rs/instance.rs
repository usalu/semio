//! 📦 App instance schemas, parameters, and studio bindings.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub const OS_PARAMETER_PORT_PREFIX: &str = "param.";

//#region 🔖Schemas
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsSourceDocument {
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_ref: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsAppInstance {
    pub id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
    pub source_document: OsSourceDocument,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsInstanceState {
    pub id: u32,
    pub app_id: String,
    pub controller_id: String,
    pub document_json: String,
    pub view_state: semio_framework_core::ViewState,
    pub generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsParameterType {
    Numeric,
    Categorical,
    Toggle,
    Text,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsParameterFieldSpec {
    pub field_path: String,
    pub label: String,
    #[serde(rename = "type")]
    pub parameter_type: OsParameterType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsParameterFieldBinding {
    pub parameter_id: String,
    pub instance_id: String,
    pub field_path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OsParameter {
    Numeric {
        id: String,
        name: String,
        value: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
    },
    Categorical {
        id: String,
        name: String,
        value: String,
        options: Vec<String>,
    },
    Toggle {
        id: String,
        name: String,
        value: bool,
    },
    Text {
        id: String,
        name: String,
        value: String,
    },
}
//#endregion 🔖Schemas

//#region 🔖Parameters
static OS_ID: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔 Allocates stable ids for OS studio entities.
pub fn create_os_id(prefix: &str) -> String {
    let n = OS_ID.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

/// @emoji 🎛️ Reads the runtime value from a studio parameter definition.
pub fn os_parameter_value(parameter: &OsParameter) -> Value {
    match parameter {
        OsParameter::Numeric { value, .. } => Value::from(*value),
        OsParameter::Categorical { value, .. } => Value::from(value.clone()),
        OsParameter::Toggle { value, .. } => Value::from(*value),
        OsParameter::Text { value, .. } => Value::from(value.clone()),
    }
}

/// @emoji 🎛️ Returns whether a parameter type can drive a bindable field type.
pub fn os_parameter_types_compatible(left: &OsParameterType, right: &OsParameterType) -> bool {
    left == right
}

/// @emoji 🎛️ Creates a default studio parameter of the given type.
pub fn create_default_os_parameter(
    parameter_type: &OsParameterType,
    name: &str,
    id: Option<&str>,
) -> OsParameter {
    let parameter_id = id
        .map(str::to_string)
        .unwrap_or_else(|| create_os_id("param"));
    match parameter_type {
        OsParameterType::Numeric => OsParameter::Numeric {
            id: parameter_id,
            name: name.into(),
            value: 0.0,
            min: Some(0.0),
            max: Some(100.0),
            step: Some(1.0),
        },
        OsParameterType::Categorical => OsParameter::Categorical {
            id: parameter_id,
            name: name.into(),
            value: "Option A".into(),
            options: vec!["Option A".into(), "Option B".into()],
        },
        OsParameterType::Toggle => OsParameter::Toggle {
            id: parameter_id,
            name: name.into(),
            value: false,
        },
        OsParameterType::Text => OsParameter::Text {
            id: parameter_id,
            name: name.into(),
            value: String::new(),
        },
    }
}

fn clamp_numeric_value(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
    let mut next = value;
    if let Some(min) = min.filter(|v| v.is_finite()) {
        next = next.max(min);
    }
    if let Some(max) = max.filter(|v| v.is_finite()) {
        next = next.min(max);
    }
    if let Some(step) = step.filter(|v| v.is_finite() && *v > 0.0) {
        let anchor = min.filter(|v| v.is_finite()).unwrap_or(0.0);
        next = anchor + ((next - anchor) / step).round() * step;
        if let Some(min) = min.filter(|v| v.is_finite()) {
            next = next.max(min);
        }
        if let Some(max) = max.filter(|v| v.is_finite()) {
            next = next.min(max);
        }
    }
    next
}

/// @emoji 🎛️ Applies a partial patch to a studio parameter, enforcing type constraints.
pub fn patch_os_parameter(parameter: &OsParameter, patch: &Value) -> OsParameter {
    let name = patch
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| parameter_name(parameter));
    let patch_type = patch.get("type").and_then(|v| v.as_str());
    let use_numeric = patch_type == Some("numeric")
        || (patch_type.is_none() && matches!(parameter, OsParameter::Numeric { .. }));
    if use_numeric {
        let current = match parameter {
            OsParameter::Numeric { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Numeric, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Numeric {
            id,
            min: current_min,
            max: current_max,
            step: current_step,
            value: current_value,
            ..
        } = current
        {
            let min = patch
                .get("min")
                .and_then(|v| v.as_f64())
                .or(current_min);
            let max = patch
                .get("max")
                .and_then(|v| v.as_f64())
                .or(current_max);
            let step = patch
                .get("step")
                .and_then(|v| v.as_f64())
                .or(current_step);
            let raw_value = patch
                .get("value")
                .and_then(|v| v.as_f64())
                .unwrap_or(current_value);
            return OsParameter::Numeric {
                id,
                name,
                min,
                max,
                step,
                value: clamp_numeric_value(raw_value, min, max, step),
            };
        }
    }
    let use_categorical = patch_type == Some("categorical")
        || (patch_type.is_none() && matches!(parameter, OsParameter::Categorical { .. }));
    if use_categorical {
        let current = match parameter {
            OsParameter::Categorical { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Categorical, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Categorical {
            id,
            value: current_value,
            options: current_options,
            ..
        } = current
        {
            let options = patch
                .get("options")
                .and_then(|v| v.as_array())
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| entry.as_str().map(str::to_string))
                        .collect::<Vec<_>>()
                })
                .unwrap_or(current_options);
            let unique_options = if options.is_empty() {
                vec!["Option A".into()]
            } else {
                options
            };
            let value = patch
                .get("value")
                .and_then(|v| v.as_str())
                .filter(|v| unique_options.iter().any(|option| option == *v))
                .map(str::to_string)
                .or_else(|| {
                    unique_options
                        .iter()
                        .find(|option| **option == current_value)
                        .cloned()
                })
                .unwrap_or_else(|| unique_options[0].clone());
            return OsParameter::Categorical {
                id,
                name,
                options: unique_options,
                value,
            };
        }
    }
    if patch_type == Some("toggle")
        || (patch_type.is_none() && matches!(parameter, OsParameter::Toggle { .. }))
    {
        let current = match parameter {
            OsParameter::Toggle { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Toggle, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Toggle {
            id,
            value: current_value,
            ..
        } = current
        {
            let value = patch
                .get("value")
                .and_then(|v| v.as_bool())
                .unwrap_or(current_value);
            return OsParameter::Toggle { id, name, value };
        }
    }
    let current = match parameter {
        OsParameter::Text { .. } => parameter.clone(),
        _ => create_default_os_parameter(&OsParameterType::Text, &name, Some(parameter_id(parameter))),
    };
    if let OsParameter::Text {
        id,
        value: current_value,
        ..
    } = current
    {
        let value = patch
            .get("value")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .unwrap_or(current_value);
        return OsParameter::Text { id, name, value };
    }
    parameter.clone()
}

fn parameter_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

fn parameter_name(parameter: &OsParameter) -> String {
    match parameter {
        OsParameter::Numeric { name, .. }
        | OsParameter::Categorical { name, .. }
        | OsParameter::Toggle { name, .. }
        | OsParameter::Text { name, .. } => name.clone(),
    }
}

fn json_pointer_segments(pointer: &str) -> Vec<String> {
    if let Some(rest) = pointer.strip_prefix('/') {
        rest.split('/').filter(|segment| !segment.is_empty()).map(str::to_string).collect()
    } else {
        pointer
            .split('.')
            .filter(|segment| !segment.is_empty())
            .map(str::to_string)
            .collect()
    }
}

/// @emoji 🎛️ Deep-sets a JSON-pointer path on a plain object projection.
pub fn set_json_pointer_value(root: &mut Value, pointer: &str, value: Value) {
    let segments = json_pointer_segments(pointer);
    if segments.is_empty() {
        return;
    }
    let mut current = root;
    for segment in &segments[..segments.len() - 1] {
        if !current.is_object() {
            *current = Value::Object(Default::default());
        }
        let object = current.as_object_mut().expect("object");
        let entry = object
            .entry(segment.clone())
            .or_insert_with(|| Value::Object(Default::default()));
        if !entry.is_object() {
            *entry = Value::Object(Default::default());
        }
        current = entry;
    }
    if let Some(object) = current.as_object_mut() {
        object.insert(segments.last().cloned().unwrap_or_default(), value);
    }
}

/// @emoji 🎛️ Applies bound studio parameter values onto an app projection via JSON pointers.
pub fn apply_parameter_values_to_projection(
    projection: Value,
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    instance_id: &str,
) -> Value {
    let instance_bindings: Vec<_> = bindings
        .iter()
        .filter(|binding| binding.instance_id == instance_id)
        .collect();
    if instance_bindings.is_empty() {
        return projection;
    }
    let mut clone = projection;
    for binding in instance_bindings {
        let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
            continue;
        };
        set_json_pointer_value(&mut clone, &binding.field_path, os_parameter_value(parameter));
    }
    clone
}

trait OsParameterId {
    fn id(&self) -> &str;
}

impl OsParameterId for OsParameter {
    fn id(&self) -> &str {
        parameter_id(self)
    }
}

/// @emoji 🎛️ Resolves bound parameter values for an app instance as a field-path map.
pub fn resolve_parameter_values_for_instance(
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    instance_id: &str,
) -> HashMap<String, Value> {
    let mut values = HashMap::new();
    for binding in bindings
        .iter()
        .filter(|entry| entry.instance_id == instance_id)
    {
        let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
            continue;
        };
        values.insert(binding.field_path.clone(), os_parameter_value(parameter));
    }
    values
}

/// @emoji 🎛️ Builds the media graph input port id for a bound studio parameter.
pub fn parameter_port_id(instance_id: &str, parameter_id: &str) -> String {
    media_port_id_for_spec(instance_id, &format!("{OS_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
}

/// @emoji 🎛️ Returns whether a media port id denotes a studio parameter input channel.
pub fn is_parameter_port_id(port_id: &str) -> bool {
    media_port_spec_id(port_id)
        .map(|spec_id| spec_id.starts_with(OS_PARAMETER_PORT_PREFIX))
        .unwrap_or(false)
}

/// @emoji 🎛️ Extracts the studio parameter id from a parameter input port id.
pub fn parameter_id_from_port_id(port_id: &str) -> Option<String> {
    let spec_id = media_port_spec_id(port_id)?;
    spec_id
        .strip_prefix(OS_PARAMETER_PORT_PREFIX)
        .map(str::to_string)
}

pub fn media_port_id_for_spec(instance_id: &str, spec_id: &str, direction: &str) -> String {
    format!("{instance_id}:{spec_id}:{direction}")
}

pub fn media_port_spec_id(port_id: &str) -> Option<String> {
    let parts: Vec<_> = port_id.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    Some(parts[1..parts.len() - 1].join(":"))
}
//#endregion 🔖Parameters

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patches_numeric_parameter_with_constraints() {
        let parameter = create_default_os_parameter(&OsParameterType::Numeric, "Zoom", None);
        let patched = patch_os_parameter(
            &parameter,
            &serde_json::json!({ "value": 12.0, "max": 10.0 }),
        );
        match patched {
            OsParameter::Numeric { value, .. } => assert_eq!(value, 10.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn applies_json_pointer_parameter_overrides() {
        let projection = serde_json::json!({ "brushSize": 8 });
        let overridden = apply_parameter_values_to_projection(
            projection,
            &[OsParameterFieldBinding {
                parameter_id: "p1".into(),
                instance_id: "i1".into(),
                field_path: "/brushSize".into(),
            }],
            &[OsParameter::Numeric {
                id: "p1".into(),
                name: "Brush".into(),
                value: 42.0,
                min: None,
                max: None,
                step: None,
            }],
            "i1",
        );
        assert_eq!(overridden["brushSize"], 42.0);
    }
}
//#endregion 🧪Tests
