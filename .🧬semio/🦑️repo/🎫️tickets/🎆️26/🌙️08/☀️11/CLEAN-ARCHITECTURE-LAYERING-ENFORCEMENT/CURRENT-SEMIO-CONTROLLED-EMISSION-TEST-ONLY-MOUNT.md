# Controlled Semio Emission Tests-only Mount

Nine closed schema cases and actual typed-API native law are registered in the existing whole OS unit suite. Existing original four owned Semio unit law bodies remain byte-identical; only a direct new module registration was appended. The prior controlled borrowing law remains untouched. Actual production owner, Cargo and Value/Async/Trace sources remain unchanged by this mount.

Exact whole target verified by Native: @semio-tech/framework-os-kernel:test-native, existing full --lib --features sync,ureq. Current generation closure must be admitted before compiler execution. No native RED/GREEN or dependency closure is claimed from this four-source test-only mount.

Full originals/absence inverses/authored sources are in generated/value-refusal/semio-emission-test-only-full-authored-1.json; all four immediate post hashes match; both Rust sources are third-party grammar clean. Prior strict Ajv nine / Node Buffer two proof and retained fixture correction are in CURRENT-SEMIO-CONTROLLED-EMISSION-REFUSAL-PREP.md.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧬️schema/⚠️emission/🔣️.json

Original exists: false

```
ABSENT
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧫️fixtures/⚠️emission/🔣️.json

Original exists: false

```
ABSENT
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/⚠️emission/🦀️.rs

Original exists: false

```
ABSENT
```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/🔬️unit/🦀️.rs

Original exists: true

```
use super::*;

#[test]
fn semio_envelope_identity_matches_independent_neutral_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️envelope-identity/🔣️.json")).expect("neutral envelope vectors");
    let expected = &vectors["expected"];
    for case in vectors["cases"].as_array().unwrap() {
        let item = &case["envelope"];
        let envelope = SemioEnvelope {
            plugin: item["plugin"].as_str().unwrap().into(),
            artifact: item["artifact"].as_str().unwrap().into(),
            component: Component::parse(item["component"].as_str().unwrap()).unwrap(),
            version: item["version"].as_u64().unwrap() as u16,
        };
        assert_eq!(envelope.matches_identity(expected["id"].as_str().unwrap(), Component::parse(expected["component"].as_str().unwrap()).unwrap(), expected["version"].as_u64().unwrap() as u16), case["matches"].as_bool().unwrap(), "{}", case["name"]);
    }
}

#[test]
fn text_preamble_round_trip() {
    let env = SemioEnvelope { plugin: "gis".into(), artifact: "gismap".into(), component: Component::Dsl, version: 1 };
    let wrapped = wrap_text(&env, "positions [id:TEXT] { }");
    let (parsed, body) = split_text_preamble(&wrapped).unwrap();
    assert_eq!(parsed, env);
    assert!(body.starts_with("positions"));
}

#[test]
fn binary_header_round_trip() {
    let env = SemioEnvelope { plugin: "gis".into(), artifact: "gismap".into(), component: Component::Pack, version: 1 };
    let inner = b"payload-bytes";
    let wrapped = wrap_binary(&env, inner);
    let (parsed, payload) = unwrap_binary(&wrapped).unwrap();
    assert_eq!(parsed, env);
    assert_eq!(payload, inner);
}

#[test]
fn sniff_text_and_binary() {
    let dsl_env = SemioEnvelope::from_envelope_id("gis.gismap", Component::Dsl, 1).unwrap();
    let text = wrap_text(&dsl_env, "schema=gis.map id=x");
    assert_eq!(sniff(text.as_bytes()).unwrap().component, Component::Dsl);
    let bin = wrap_binary(&SemioEnvelope::from_envelope_id("gis.gismap", Component::Pack, 1).unwrap(), b"x");
    assert_eq!(sniff(&bin).unwrap().component, Component::Pack);
}

```


Finite selected source fence authored after immediate reread: 1624 exact current source rows with full text/inverse/bytes/SHA and zero reread gaps at capture time, in generated/value-refusal/semio-emission-test-only-source-ready-1.json. Actual generation admission remains pending; this is not a compiler-closed/global currentness claim.

Finite selected source fence authored after immediate reread: 2000 exact current source rows with full text/inverse/bytes/SHA and zero reread gaps at capture time, in generated/value-refusal/semio-emission-test-only-source-ready-2.json. High Gen11/check released before this fresh finite capture. The actual selected source bytes are admitted; this is not a compiler-closed/global currentness claim.

Finite selected source fence authored after immediate reread: 2000 exact current source rows with full text/inverse/bytes/SHA and zero reread gaps at capture time, in generated/value-refusal/semio-emission-test-only-source-ready-3.json. High Gen11/check released before this fresh finite capture. The actual selected source bytes are admitted; this is not a compiler-closed/global currentness claim.

Finite selected source fence authored after immediate reread: 2004 exact current source rows with full text/inverse/bytes/SHA and zero reread gaps at capture time, in generated/value-refusal/semio-emission-test-only-source-ready-4.json. High Gen11/check released before this fresh finite capture. The actual selected source bytes are admitted; this is not a compiler-closed/global currentness claim.
