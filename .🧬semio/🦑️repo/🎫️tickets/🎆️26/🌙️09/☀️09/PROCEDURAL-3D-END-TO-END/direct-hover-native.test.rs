use semio_framework_value::DslValue;
use semio_ui_contract_values::*;
type Fault = String;
fn plugin_sdk_fault(value: String) -> Fault { value }
mod protocol {
    #[derive(serde::Deserialize)]
    pub struct InteractionTarget { pub id: String, pub granularity: String }
}

fn immediate<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    match future.as_mut().poll(&mut std::task::Context::from_waker(waker)) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("hover argument decoding must be immediate"),
    }
}

#[test]
fn exact_action_addresses_fit_inline_and_preserve_pointer_clears() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(HOVER_FIXTURE)).unwrap();
    let first = &fixture["cases"][0];
    let id = first["args"]["targetId"].as_str().unwrap();
    let granularity = first["args"]["granularityId"].as_str().unwrap();
    let old_json = first["expected"].to_string();
    assert!(UiText::try_from_str(&old_json).is_none());
    let _arena = scope::UiArenaFixture::new();
    let binding = serde_json::to_value(graph_hover_args(id, granularity).unwrap()).unwrap();
    assert_eq!(binding["channel"], fixture["channel"]);
    assert_eq!(binding["domainId"], fixture["domain"]);
    assert_eq!(binding["targetId"], id);
    assert_eq!(binding["granularityId"], granularity);
    assert!(binding.get("targets").is_none());
    for case in fixture["cases"].as_array().unwrap() {
        let raw = case["args"].as_object().unwrap();
        let mut arguments = DslValue::object(raw.iter().map(|(key, value)| (key.clone(), DslValue::String(value.as_str().unwrap().to_owned()))));
        if case["name"].as_str().unwrap().contains("leave") {
            let authored = DslValue::object([("targetId".into(), DslValue::String(id.into())), ("granularityId".into(), DslValue::String(granularity.into()))]);
            let leave = DslValue::object([("targets".into(), DslValue::String("[]".into()))]);
            let merged = intent_payload(Some(&authored), &leave, "value");
            for key in ["targetId", "granularityId", "targets"] {
                assert_eq!(merged.get(key), arguments.get(key));
            }
            arguments = merged;
        }
        let decoded = immediate(parse_interaction_hover_targets(Some(&arguments), "interactionHover"));
        if case["fault"].as_bool().unwrap_or(false) {
            assert!(decoded.is_err(), "{}", case["name"]);
        } else {
            let targets = decoded.unwrap().iter().map(|target| serde_json::json!({"id": target.id, "granularity": target.granularity})).collect::<Vec<_>>();
            assert_eq!(serde_json::json!(targets), case["expected"], "{}", case["name"]);
        }
    }
}
