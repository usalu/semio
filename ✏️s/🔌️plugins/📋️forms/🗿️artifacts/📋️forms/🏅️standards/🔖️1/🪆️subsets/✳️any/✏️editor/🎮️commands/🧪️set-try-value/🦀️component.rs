//! 🧪️ 🧪️ Forms play app commands command — `set-try-value`.

use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::{parse_value_json, try_values_json_text, try_values_map};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖️Values
/// ✏️ Patches one JSON-object field of a try value keyed by `key` (used by the vector-field-parameter
/// try-value shape, e.g. a building-component question's `height`/`radius`/`sides` params).
async fn patch_try_object_field(values: &mut Map<String, Value>, key: &str, field: &str, raw: &Value) {
    let mut object = values.get(key).cloned().unwrap_or_else(|| json!({}));
    if let Some(map) = object.as_object_mut() {
        map.insert(field.into(), raw.clone());
        values.insert(key.into(), object);
    }
}

/// ✏️ Patches one numeric index of a try value keyed by `key` (used by the vector question kind's
/// per-component try value).
async fn patch_try_vector_field(values: &mut Map<String, Value>, key: &str, index: usize, raw: &Value) {
    let mut array = values.get(key).and_then(|value| value.as_array().cloned()).unwrap_or_default();
    while array.len() <= index {
        array.push(json!(0.0));
    }
    array[index] = raw.clone();
    values.insert(key.into(), Value::Array(array));
}
//#endregion 🔖️Values

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "try-value")]
pub struct SetTryValue {
    pub key: String,
    pub value_json: Option<String>,
    pub option_value: Option<String>,
    pub vector_index: Option<u64>,
    pub param_key: Option<String>,
}

pub async fn handle(payload: &SetTryValue, _doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let mut values = try_values_map(config);
    if let Some(option_value) = &payload.option_value {
        let mut selected = values.get(payload.key.as_str()).and_then(|value| value.as_array().cloned()).unwrap_or_default();
        let pressed = payload.value_json.as_deref().map(parse_value_json).and_then(|value| value.as_bool()).unwrap_or(false);
        if pressed {
            if !selected.iter().any(|entry| entry.as_str() == Some(option_value.as_str())) {
                selected.push(json!(option_value));
            }
        } else {
            selected.retain(|entry| entry.as_str() != Some(option_value.as_str()));
        }
        values.insert(payload.key.clone(), Value::Array(selected));
    } else if let Some(index) = payload.vector_index {
        if let Some(raw) = payload.value_json.as_deref().map(parse_value_json) {
            patch_try_vector_field(&mut values, &payload.key, index as usize, &raw);
        }
    } else if let Some(param_key) = &payload.param_key {
        if let Some(raw) = payload.value_json.as_deref().map(parse_value_json) {
            patch_try_object_field(&mut values, &payload.key, param_key, &raw);
        }
    } else if let Some(raw) = payload.value_json.as_deref().map(parse_value_json) {
        values.insert(payload.key.clone(), raw);
    }
    Ok(Emit::config(vec![FormsConfigMutation::SetTryValues { json: try_values_json_text(&values) }]))
}
