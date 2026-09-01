use semio_framework_os_kernel::{DslValue, FromValue, ToValue};
use verify_value_derive::*;

fn dsl_matches_serde(dsl: &DslValue, oracle: &serde_json::Value) {
    assert_eq!(dsl, oracle, "DslValue shape diverges from serde_json oracle");
}

//#region 🔖️PlainUnitEnum
#[test]
fn plain_unit_enum_matches_serde_bare_string() {
    for (variant, wire) in [(SelectionModeTwin::Single, "single"), (SelectionModeTwin::Multiple, "multiple")] {
        let ours = variant.to_value();
        let theirs = serde_json::to_value(variant).expect("serde encode");
        dsl_matches_serde(&ours, &theirs);
        assert_eq!(theirs, serde_json::Value::String(wire.to_string()), "serde itself must be a bare string, not an object");
        let back = SelectionModeTwin::from_value(ours).expect("our decode");
        assert_eq!(back, variant);
        let their_back: SelectionModeTwin = serde_json::from_value(theirs).expect("serde decode");
        assert_eq!(their_back, variant);
    }
}
//#endregion 🔖️PlainUnitEnum

//#region 🔖️EmptyStruct
#[test]
fn empty_struct_matches_serde_empty_object() {
    let value = NoConfigTwin::default();
    let ours = value.to_value();
    let theirs = serde_json::to_value(&value).expect("serde encode");
    dsl_matches_serde(&ours, &theirs);
    assert_eq!(theirs, serde_json::json!({}));
    let back = NoConfigTwin::from_value(ours).expect("our decode");
    assert_eq!(back, value);
}
//#endregion 🔖️EmptyStruct

//#region 🔖️EmptyEnum
#[test]
fn empty_enum_from_value_always_errors_never_panics() {
    assert!(NoConfigMutationTwin::from_value(DslValue::Null).is_err());
    assert!(NoConfigMutationTwin::from_value(DslValue::String("anything".to_string())).is_err());
    assert!(NoConfigMutationTwin::from_value(DslValue::object([])).is_err());
    fn assert_bounds<T: ToValue + FromValue>() {}
    assert_bounds::<NoConfigMutationTwin>();
}
//#endregion 🔖️EmptyEnum

//#region 🔖️OptionDefaultSkip
#[test]
fn option_default_skip_matches_serde_present_and_absent() {
    let with_anchor = DomainSelectionTwin { granularity: "surface".to_string(), ids: vec!["a".to_string(), "b".to_string()], anchor_id: Some("a".to_string()) };
    let without_anchor = DomainSelectionTwin { granularity: "surface".to_string(), ids: vec!["a".to_string()], anchor_id: None };
    for value in [with_anchor, without_anchor] {
        let ours = value.to_value();
        let theirs = serde_json::to_value(&value).expect("serde encode");
        dsl_matches_serde(&ours, &theirs);
        let back = DomainSelectionTwin::from_value(ours).expect("our decode");
        assert_eq!(back, value);
        let their_back: DomainSelectionTwin = serde_json::from_value(theirs).expect("serde decode");
        assert_eq!(their_back, value);
    }
    // absent `anchorId` decodes to `None` via the `default` fallback path, matching serde's
    // `#[serde(default)]` on a field that is missing from the wire object entirely.
    let sparse = DslValue::object([("granularity".to_string(), DslValue::String("s".to_string())), ("ids".to_string(), DslValue::Array(vec![]))]);
    let decoded = DomainSelectionTwin::from_value(sparse).expect("sparse decode");
    assert_eq!(decoded.anchor_id, None);
}
//#endregion 🔖️OptionDefaultSkip

//#region 🔖️Composite
#[test]
fn composite_btreemap_of_composite_matches_serde() {
    let mut state = StateTwin::default();
    state.selection.insert("outline".to_string(), DomainSelectionTwin { granularity: "surface".to_string(), ids: vec!["x".to_string()], anchor_id: None });
    state.active_mode.insert("outline".to_string(), SelectionModeTwin::Multiple);
    let ours = state.to_value();
    let theirs = serde_json::to_value(&state).expect("serde encode");
    dsl_matches_serde(&ours, &theirs);
    let back = StateTwin::from_value(ours).expect("our decode");
    assert_eq!(back, state);
    let their_back: StateTwin = serde_json::from_value(theirs).expect("serde decode");
    assert_eq!(their_back, state);
}
//#endregion 🔖️Composite

//#region 🔖️ExternallyTaggedSingleVariant
#[test]
fn externally_tagged_single_variant_matches_serde_default_shape() {
    let value = ConfigMutationTwin::SetState(DomainSelectionTwin { granularity: "g".to_string(), ids: vec!["1".to_string()], anchor_id: Some("1".to_string()) });
    let ours = value.to_value();
    let theirs = serde_json::to_value(&value).expect("serde encode");
    dsl_matches_serde(&ours, &theirs);
    assert!(theirs.get("setState").is_some(), "serde's own externally-tagged shape must key on the camelCased variant name");
    let back = ConfigMutationTwin::from_value(ours).expect("our decode");
    assert_eq!(back, value);
    let their_back: ConfigMutationTwin = serde_json::from_value(theirs).expect("serde decode");
    assert_eq!(their_back, value);
}
//#endregion 🔖️ExternallyTaggedSingleVariant

//#region 🔖️RandomizedRoundTrip
#[test]
fn randomized_state_round_trips_against_serde_lcg_seeded() {
    let mut state: u64 = 0x9E3779B97F4A7C15;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for case in 0..200u32 {
        let n_entries = (next() % 5) as usize;
        let mut selection = std::collections::BTreeMap::new();
        let mut active_mode = std::collections::BTreeMap::new();
        for i in 0..n_entries {
            let key = format!("domain-{}-{case}", i);
            let has_anchor = next() % 2 == 0;
            selection.insert(
                key.clone(),
                DomainSelectionTwin {
                    granularity: format!("g{}", next() % 7),
                    ids: (0..(next() % 4)).map(|j| format!("id{j}")).collect(),
                    anchor_id: if has_anchor { Some(format!("anchor{}", next() % 3)) } else { None },
                },
            );
            active_mode.insert(key, if next() % 2 == 0 { SelectionModeTwin::Single } else { SelectionModeTwin::Multiple });
        }
        let value = StateTwin { selection, active_mode };
        let ours = value.to_value();
        let theirs = serde_json::to_value(&value).expect("serde encode");
        dsl_matches_serde(&ours, &theirs);
        assert_eq!(StateTwin::from_value(ours).expect("our decode"), value, "case {case}");
    }
}
//#endregion 🔖️RandomizedRoundTrip
