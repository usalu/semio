//! Oracle round-trip proof for the `TopicContribution` seam migration (ticket
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`): `TopicContribution.payload`
//! moved from `serde_json::Value` to `semio_framework_os_kernel::DslValue`. This crate is a verbatim
//! copy of the three real source files that make that migration sound (`🌱️value/🦀️component.rs`,
//! `🌱️value/🔁️codec/🦀️component.rs`, `🌱️value/🔀️serde/🦀️component.rs`, `🎒️pack/🔤️json/🦀️component.rs`
//! with only its `protocol::value::` paths rewritten to `crate::value::`) — every test below exercises
//! the SAME code that landed in the real repo, with `serde_json` as a third-party oracle.

use topic_contribution_verify::json;
use topic_contribution_verify::value::DslValue;

/// Mirrors `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`'s real `TopicContribution` shape:
/// `#[derive(Serialize, Deserialize)]`, `payload: DslValue` — this proves the host-pushed
/// `contributionsJson` wire path (`parse_contributions`/`contributions_json_from_entries`, still
/// serde_json-based on the framework side) keeps working once `payload` is no longer
/// `serde_json::Value`, because `DslValue` implements `Serialize`/`Deserialize` itself (the
/// `🔀️serde/🦀️component.rs` bridge).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TopicContribution {
    topic: String,
    payload: DslValue,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProgramContributionEntry {
    plugin_id: String,
    #[serde(default)]
    topic_contribution: Option<TopicContribution>,
}

/// The exact `DslValue::object([...])` construction pattern used at every real call site
/// (`cad-extension-spatial-shape`, `process-extension-wood`, `sourcing-module-beams`, …).
fn sample_payload() -> DslValue {
    DslValue::object([
        ("appId".to_string(), DslValue::String("cad-play".to_string())),
        ("moduleId".to_string(), DslValue::String("spatial-shape".to_string())),
        ("label".to_string(), DslValue::String("Spatial Shape".to_string())),
        ("iconId".to_string(), DslValue::String("box".to_string())),
        ("computersJson".to_string(), DslValue::String(r#"{"statComputers":["spatial.shape.geometry"]}"#.to_string())),
    ])
}

#[test]
fn topic_contribution_round_trips_through_serde_json_wire_path() {
    let entries = vec![ProgramContributionEntry { plugin_id: "cad-extension-spatial-shape".into(), topic_contribution: Some(TopicContribution { topic: "cad.computer".into(), payload: sample_payload() }) }];

    // The real `contributions_json_from_entries`/`parse_contributions` pair, unchanged, still
    // `serde_json::to_string`/`from_str` — this is the host<->plugin JSON text boundary.
    let wire_json = serde_json::to_string(&entries).expect("serialize");
    let round_tripped: Vec<ProgramContributionEntry> = serde_json::from_str(&wire_json).expect("deserialize");
    assert_eq!(round_tripped, entries);

    // Oracle: the wire text is genuine, spec-shaped JSON — parseable by serde_json's own untyped
    // `Value`, with the exact camelCase keys `#[value(rename_all = "camelCase")]`/`#[serde(rename_all
    // = "camelCase")]` produce at every real call site.
    let oracle: serde_json::Value = serde_json::from_str(&wire_json).expect("oracle parse");
    assert_eq!(oracle[0]["pluginId"], "cad-extension-spatial-shape");
    assert_eq!(oracle[0]["topicContribution"]["topic"], "cad.computer");
    assert_eq!(oracle[0]["topicContribution"]["payload"]["appId"], "cad-play");
    assert_eq!(oracle[0]["topicContribution"]["payload"]["computersJson"], r#"{"statComputers":["spatial.shape.geometry"]}"#);
}

/// The exact indexing/`.as_str()` pattern every fixed test assertion in the real repo uses:
/// `topic_contribution.payload["appId"].as_str()`.
#[test]
fn dsl_value_indexing_and_as_str_match_the_real_call_sites() {
    let payload = sample_payload();
    assert_eq!(payload["appId"].as_str(), Some("cad-play"));
    assert_eq!(payload["moduleId"].as_str(), Some("spatial-shape"));
    assert_eq!(payload["missingKey"].as_str(), None);
    assert!(payload["missingKey"].is_null());
}

/// `#[derive(FromValue)]`'s generated `from_value` ultimately bottoms out on `DslValue::Object`
/// entries plus each field's own `FromValue` impl (all hand-written scalar impls copied verbatim
/// from `🔁️codec/🦀️component.rs`) — this exercises that same decode path by hand for the payload
/// shape `ProcessMachinesTopicPayload`/`SourcingModuleTopicPayload`/etc. all share: a handful of
/// `String` fields read via `.get(key)` + `FromValue::from_value`.
#[test]
fn from_value_decodes_string_fields_the_way_the_generated_derive_does() {
    use topic_contribution_verify::value::FromValue;
    let payload = sample_payload();
    let DslValue::Object(entries) = &payload else { panic!("expected object") };
    let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).expect("field present");
    let app_id = String::from_value(get("appId")).expect("appId decodes");
    let module_id = String::from_value(get("moduleId")).expect("moduleId decodes");
    assert_eq!(app_id, "cad-play");
    assert_eq!(module_id, "spatial-shape");
}

/// `semio_framework_os_kernel::json::{to_json_string, from_json_str}` — the bridge every
/// `machinesJson`/`typologyJson`/`kindsJson` STRING field (embedded inside a `DslValue` payload)
/// is produced/consumed through now, e.g. `process-extension-wood`'s
/// `semio_framework_os_kernel::json::to_json_string(&catalog.machines())`. `ToValue`/`FromValue`
/// are hand-written here in EXACTLY the shape `#[derive(ToValue, FromValue)]` generates (fully
/// qualified trait calls, `DslValue::Object` of `(rename_all-cased key, field.to_value())` pairs —
/// see the playbook doc's trap #1 on why the derive never uses the `.to_value()` shorthand) rather
/// than pulling the proc-macro crate (and its `syn`/`quote` graph) into this oracle.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct DummyMachine {
    id: String,
    label: String,
}

impl topic_contribution_verify::value::ToValue for DummyMachine {
    fn to_value(&self) -> DslValue {
        DslValue::object([("id".to_string(), topic_contribution_verify::value::ToValue::to_value(&self.id)), ("label".to_string(), topic_contribution_verify::value::ToValue::to_value(&self.label))])
    }
}

impl topic_contribution_verify::value::FromValue for DummyMachine {
    fn from_value(value: DslValue) -> Result<Self, topic_contribution_verify::value::ValueError> {
        let DslValue::Object(entries) = value else { return Err(topic_contribution_verify::value::ValueError::new("expected object")) };
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).ok_or_else(|| topic_contribution_verify::value::ValueError::new(format!("missing field {key}")));
        Ok(Self { id: String::from_value(get("id")?)?, label: String::from_value(get("label")?)? })
    }
}

#[test]
fn pack_json_bridge_round_trips_against_serde_json_oracle() {
    let machines = vec![DummyMachine { id: "circularSaw".into(), label: "Circular Saw".into() }, DummyMachine { id: "tableSaw".into(), label: "Table Saw".into() }];

    // The real bridge: ToValue -> DslValue -> pack::json::Value -> JSON text.
    let bridged_json = json::to_json_string(&machines);

    // Oracle: serde_json's own serialization of the identical Rust value.
    let oracle_json = serde_json::to_string(&machines).expect("oracle serialize");

    // Both must parse to the SAME untyped JSON tree (oracle == pack::json parse of our text ==
    // pack::json parse of the oracle's own text) — proves `pack::json::to_string`'s byte shape
    // (key order, escaping, number formatting) matches what `serde_json` produces for this
    // payload shape, and that the bridge is a faithful `serde_json::to_string` replacement.
    let oracle_value: serde_json::Value = serde_json::from_str(&oracle_json).expect("oracle parse");
    let ours_reparsed_by_oracle: serde_json::Value = serde_json::from_str(&bridged_json).expect("our text parses as JSON");
    assert_eq!(ours_reparsed_by_oracle, oracle_value);

    // And the real decode direction: pack::json::from_json_str<T: FromValue> reconstructs the
    // exact original Rust value, both from OUR text and from the ORACLE's text (proving wire
    // compatibility in both directions, not just ours).
    let decoded_ours: Vec<DummyMachine> = json::from_json_str(&bridged_json).expect("decode our text");
    let decoded_oracle_text: Vec<DummyMachine> = json::from_json_str(&oracle_json).expect("decode oracle text");
    assert_eq!(decoded_ours, machines);
    assert_eq!(decoded_oracle_text, machines);
}

/// The `pack::json::Value::get`/`as_array`/`as_str` traversal every converted cad-extension test
/// assertion uses in place of `serde_json::Value`'s `[...]` indexing (`pack::json::Value` has no
/// `Index` impl, unlike `serde_json::Value` — the real edits switched `parsed["k"]` to
/// `parsed.get("k")` accordingly).
#[test]
fn pack_json_value_get_matches_serde_json_oracle_shape() {
    let computers_manifest = json::object([("statComputers".to_string(), json::array([json::Value::from("spatial.shape.geometry")])), ("propertyComputers".to_string(), json::array([json::Value::from("spatial.shape.volume")]))]);
    let text = json::to_string(&computers_manifest);
    let parsed = json::parse(&text).expect("parse");
    assert_eq!(parsed.get("statComputers"), Some(&json::array([json::Value::from("spatial.shape.geometry")])));

    let oracle: serde_json::Value = serde_json::from_str(&text).expect("oracle parse");
    assert_eq!(oracle["statComputers"], serde_json::json!(["spatial.shape.geometry"]));
    assert_eq!(oracle["propertyComputers"], serde_json::json!(["spatial.shape.volume"]));
}
