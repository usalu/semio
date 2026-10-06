//! 🧪️ Shared contributed recipe corpus exercises borrowed admission and all cancellation frontiers.
use super::*;
use serde_json::Value;
struct Control { cancel_at: Option<usize>, budget: usize }
impl ProviderOperationControl for Control {
    fn checkpoint(&mut self, completed: usize) -> Result<(), ProviderRefusal> {
        if self.cancel_at == Some(completed) { return Err(ProviderRefusal::Cancelled); }
        if completed >= self.budget { return Err(ProviderRefusal::Budget); }
        Ok(())
    }
}
fn strings(value: Option<&Value>) -> Option<Vec<&str>> { value?.as_array()?.iter().map(Value::as_str).collect() }
fn check(value: &Value, control: &mut Control) -> (Result<(), ProviderRefusal>, usize, usize) {
    let mut admission = ProviderAdmission::default();
    let mut received = 0;
    let mut receive = |target: ProviderTarget<'_>| { assert!(!target.laws.is_empty()); received += 1; };
    let Some(object) = value.as_object() else { let result = admission.admit(None, control, &mut receive); return (result, admission.completed(), received); };
    let keys = object.keys().map(String::as_str).collect::<Vec<_>>();
    let raw = object.get("law-targets").and_then(Value::as_array);
    let target_keys = raw.map(|targets| targets.iter().map(|target| target.as_object().map(|target| target.keys().map(String::as_str).collect::<Vec<_>>()).unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
    let laws = raw.map(|targets| targets.iter().map(|target| strings(target.get("laws"))).collect::<Vec<_>>()).unwrap_or_default();
    let features = raw.map(|targets| targets.iter().map(|target| strings(target.get("features"))).collect::<Vec<_>>()).unwrap_or_default();
    let targets = raw.map(|targets| targets.iter().enumerate().map(|(index,target)| ProviderTargetInput { keys: &target_keys[index], kind: target.get("target-kind").and_then(Value::as_str), name: target.get("target-name").and_then(Value::as_str), laws: laws[index].as_deref(), features: features[index].as_deref(), default_features: target.get("default-features").and_then(Value::as_bool) }).collect::<Vec<_>>());
    let input = ProviderInput { keys: &keys, schema_version: object.get("schema-version").and_then(Value::as_u64), targets: targets.as_deref() };
    let result = admission.admit(Some(&input), control, &mut receive);
    (result, admission.completed(), received)
}
#[test]
fn shared_recipe_outcomes() {
    let corpus: Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️providers/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let (result, completed, received) = check(&row["value"], &mut Control { cancel_at: None, budget: 65536 });
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        if result.is_ok() { assert_eq!(received, row["value"]["law-targets"].as_array().unwrap().len()); }
        assert!(completed > 0);
    }
}
#[test]
fn every_callback_refuses_cancelled_or_exhausted_work() {
    let corpus: Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️providers/🔣️.json")).unwrap();
    let row = corpus["cases"].as_array().unwrap().iter().find(|row| row["id"] == "owned-lib-features").unwrap();
    let (_, total, _) = check(&row["value"], &mut Control { cancel_at: None, budget: 65536 });
    for frontier in 0..total {
        let (result, completed, received) = check(&row["value"], &mut Control { cancel_at: Some(frontier), budget: 65536 });
        assert_eq!(result, Err(ProviderRefusal::Cancelled)); assert_eq!(completed, frontier); assert_eq!(received, 0);
        let (result, completed, _) = check(&row["value"], &mut Control { cancel_at: None, budget: frontier });
        assert_eq!(result, Err(ProviderRefusal::Budget)); assert_eq!(completed, frontier);
    }
}
