use super::super::compute::dependency;
use super::super::{kinds, plan, ModelInferenceSession};
use super::*;
use crate::{ModelDiff, ModelInference};
use protocol::{DiffRegions, Inference};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::BTreeMap;

macro_rules! fixtures {
    ($($path:literal),+ $(,)?) => {
        [$(($path, include_str!(concat!("../../../../../../🧫️fixtures/", $path, "/📸️snapshot/🔣️.json")))),+]
    };
}

const SOUNDNESS: [(&str, &str); 9] = fixtures![
    "🏗️ifc/🏠️house",
    "🏗️ifc/🔲️ceilings",
    "💡️inferences/⚠️diagnostics/💥️defects",
    "💡️inferences/🎨️finishes/🏡️rooms",
    "💡️inferences/🏘️zones/🏡️zoning",
    "💡️inferences/🛝️ramp-runs/🏞️ramps",
    "💡️inferences/🪜️stair-runs/🪜️flights",
    "💡️inferences/🪟️opening-frames/🏡️placed",
    "💡️inferences/🪧️annotation-layout/🏠️room",
];

const EQUALITY: [(&str, &str); 3] = fixtures!["🏗️ifc/🏠️house", "🏗️ifc/🔲️ceilings", "💡️inferences/🪧️annotation-layout/🏠️room"];

fn snapshot_of(value: &Value) -> Option<ModelSnapshot> {
    from_json_str(&value.to_string(), JsonMemberPolicy::Reject).ok()
}

fn perturb(value: &mut Value, strings: bool) {
    match value {
        Value::Number(number) => {
            let next = match (number.as_i64(), number.as_f64()) {
                (Some(whole), _) => Some(Value::from(whole + 1)),
                (None, Some(real)) => Some(Value::from(real + 1.0)),
                _ => None,
            };
            if let Some(next) = next {
                *value = next;
            }
        }
        Value::Bool(flag) => *flag = !*flag,
        Value::String(text) if strings => text.push('~'),
        Value::Array(items) => items.iter_mut().for_each(|item| perturb(item, strings)),
        Value::Object(members) => members.values_mut().for_each(|member| perturb(member, strings)),
        _ => {}
    }
}

fn edited(base: &Value, collection: &str, edit: impl FnOnce(&mut serde_json::Map<String, Value>)) -> Option<ModelSnapshot> {
    let mut json = base.clone();
    edit(json[collection].as_object_mut().expect("a collection of rows"));
    snapshot_of(&json)
}

fn variants(base: &Value) -> Vec<(String, ModelSnapshot)> {
    let mut found = Vec::new();
    for (collection, rows) in base.as_object().into_iter().flatten().filter(|(name, _)| name.as_str() != "project") {
        for id in rows.as_object().into_iter().flat_map(|rows| rows.keys()) {
            let deleted = edited(base, collection, |rows| {
                rows.remove(id);
            });
            let created = edited(base, collection, |rows| {
                rows.insert(format!("{id}~new"), rows[id].clone());
            });
            let perturbed = [true, false].into_iter().find_map(|strings| edited(base, collection, |rows| rows.get_mut(id).into_iter().for_each(|row| perturb(row, strings))));
            for (what, variant) in [("delete", deleted), ("create", created), ("perturb", perturbed)] {
                found.extend(variant.map(|variant| (format!("{what} {collection}/{id}"), variant)));
            }
        }
    }
    found
}

fn dependencies(snapshot: &ModelSnapshot) -> BTreeMap<ModelNode, DslValue> {
    plan::build(snapshot, kinds::closure(kinds::ALL)).into_iter().map(|step| (step.key.clone(), dependency(snapshot, &step.key))).collect()
}

fn json_of(snapshot: &ModelSnapshot) -> Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(snapshot)).expect("a snapshot prints as JSON")
}

fn entry(tag: &str, row: &Value) -> Value {
    let mut members = row.as_object().cloned().unwrap_or_default();
    members.insert("entry".into(), Value::from(tag));
    Value::Object(members)
}

fn between(base: &ModelSnapshot, other: &ModelSnapshot) -> ModelDiff {
    let (before, after) = (json_of(base), json_of(other));
    let mut diff = serde_json::Map::new();
    for (collection, rows) in after.as_object().into_iter().flatten().filter(|(name, _)| name.as_str() != "project") {
        let (old, new) = (before[collection].as_object().cloned().unwrap_or_default(), rows.as_object().cloned().unwrap_or_default());
        let mut delta = serde_json::Map::new();
        old.keys().filter(|id| !new.contains_key(*id)).for_each(|id| {
            delta.insert(id.clone(), serde_json::json!({ "entry": "deleted" }));
        });
        for (id, row) in &new {
            match old.get(id) {
                None => delta.insert(id.clone(), entry("created", row)),
                Some(known) if known != row => delta.insert(id.clone(), entry("replaced", row)),
                Some(_) => None,
            };
        }
        if !delta.is_empty() {
            diff.insert(collection.clone(), Value::Object(delta));
        }
    }
    from_json_str(&Value::Object(diff).to_string(), JsonMemberPolicy::Reject).expect("the delta decodes")
}

#[test]
fn a_node_whose_dependency_a_diff_changed_is_always_reported_touched() {
    let mut checked = 0;
    for (name, text) in SOUNDNESS {
        let json: Value = serde_json::from_str(text).expect("the fixture is JSON");
        let base = snapshot_of(&json).expect("the fixture decodes");
        let before = dependencies(&base);
        for (what, variant) in variants(&json) {
            let diff = between(&base, &variant);
            let touches = diff.touches();
            for step in plan::build(&variant, kinds::closure(kinds::ALL)) {
                let Some(old) = before.get(&step.key) else { continue };
                checked += 1;
                if !touched(&variant, &step.key, &touches) {
                    assert_eq!(&dependency(&variant, &step.key), old, "{name}: {what}: the dependency of {:?} changed but the diff regions {:?} do not reach it", step.key, touches.paths);
                }
            }
        }
    }
    assert!(checked > 1_000, "the law was exercised on {checked} (node, edit) pairs");
}

#[test]
fn an_incremental_update_equals_a_fresh_inference_after_every_row_edit_and_back() {
    for (name, text) in EQUALITY {
        let json: Value = serde_json::from_str(text).expect("the fixture is JSON");
        let base = snapshot_of(&json).expect("the fixture decodes");
        let fresh_base = ModelInference::infer(&base).expect("infers");
        let mut session = ModelInferenceSession::new();
        session.update(&base, &ModelDiff::default());
        assert_eq!(session.inference(), &fresh_base, "{name}: the first update");
        let mut served = 0;
        for (what, variant) in variants(&json) {
            let Ok(fresh) = ModelInference::infer(&variant) else { continue };
            assert_eq!(session.update(&variant, &between(&base, &variant)), &fresh, "{name}: {what}");
            served += session.report().carried;
            assert_eq!(session.update(&base, &between(&variant, &base)), &fresh_base, "{name}: {what}, undone");
        }
        assert!(served > 0, "{name}: the updates carried nodes instead of walking them");
    }
}
