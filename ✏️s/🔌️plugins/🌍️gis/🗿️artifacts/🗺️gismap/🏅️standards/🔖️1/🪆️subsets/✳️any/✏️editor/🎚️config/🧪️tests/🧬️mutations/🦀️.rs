//! 🧪️ Direct configuration identity, sparse composition and exact map-entry inverse laws.

use super::*;
use protocol::{Mutation, MutationDiff, MutationLeaf, OpBinary, OpText};

//#region 🧪️DirectContracts
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧬️schema/🧬️mutations/🧪️tests/🎚️gis2d-config-direct/🔣️vectors.json")).expect("configuration law fixture")
}

pub(crate) fn assert_leaf<T>(sample: usize, wrap: fn(T) -> Gis2dConfigMutation, descriptor: &str)
where T: MutationLeaf + serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug {
    let fixture = fixture();
    let envelope = &fixture["valid"][sample]["payload"];
    let mut payload = envelope.clone();
    payload.as_object_mut().unwrap().remove("operation");
    let value: T = serde_json::from_value(payload.clone()).expect("leaf payload");
    assert_eq!(serde_json::to_value(&value).unwrap(), payload);
    assert_eq!(serde_json::to_value(T::DESCRIPTOR).unwrap(), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
    assert_eq!(T::PROVENANCE.owner, T::DESCRIPTOR.owner);
    assert_eq!(T::PROVENANCE.source_path, format!("{}/🦀️.rs", T::DESCRIPTOR.owner));
    assert_eq!(T::PROVENANCE.descriptor_path, format!("{}/🔣️.json", T::DESCRIPTOR.owner));
    payload["unknown"] = serde_json::json!(true);
    assert!(serde_json::from_value::<T>(payload).is_err());
    let operation = wrap(value);
    assert_eq!(operation.descriptor(), &T::DESCRIPTOR);
    assert_eq!(serde_json::to_value(&operation).unwrap(), *envelope);
    assert_eq!(serde_json::from_value::<Gis2dConfigMutation>(envelope.clone()).unwrap(), operation);
    assert_eq!(operation.print_op().split_whitespace().next(), T::DESCRIPTOR.text_opcode);
    assert_eq!(Gis2dConfigMutation::parse_op(&operation.print_op()).unwrap(), operation);
    let bytes = operation.encode_op().expect("binary leaf payload");
    assert_eq!(bytes[0], dsl::variants_binary::OP_BINARY_FORMAT);
    assert_eq!(Some(u32::from(bytes[1])), T::DESCRIPTOR.binary_tag);
    assert_eq!(Gis2dConfigMutation::decode_op(&bytes).unwrap(), operation);
    let before = populated();
    assert_eq!(undo(&before, &operation), before);
}

#[test]
fn neutral_envelopes_share_json_text_binary_and_inverse_contracts() {
    let fixture = fixture();
    assert_eq!(Gis2dConfigMutation::DESCRIPTORS.len(), 7);
    for row in fixture["valid"].as_array().unwrap() {
        let operation: Gis2dConfigMutation = serde_json::from_value(row["payload"].clone()).unwrap();
        assert_eq!(serde_json::to_value(&operation).unwrap(), row["payload"]);
        assert_eq!(Gis2dConfigMutation::parse_op(&operation.print_op()).unwrap(), operation);
        assert_eq!(Gis2dConfigMutation::decode_op(&operation.encode_op().unwrap()).unwrap(), operation);
        let before = populated();
        assert_eq!(undo(&before, &operation), before);
    }
    for row in fixture["invalid"].as_array().unwrap() {
        assert!(serde_json::from_value::<Gis2dConfigMutation>(row["payload"].clone()).is_err(), "{}", row["name"]);
    }
    assert!(Gis2dConfigMutation::parse_op("camera {}").is_err());
    assert!(Gis2dConfigMutation::parse_op("unknown").is_err());
    assert!(Gis2dConfigMutation::decode_op(&[0, 0]).is_err());
    assert!(Gis2dConfigMutation::decode_op(&[1, 7]).is_err());
}

#[test]
fn neutral_state_cases_match_stored_and_replayed_inverse_order() {
    for row in fixture()["stateCases"].as_array().unwrap() {
        let before: Gis2dConfig = serde_json::from_value(row["before"].clone()).unwrap();
        let operations = row.get("operations").cloned().unwrap_or_else(|| serde_json::json!([row["operation"]]));
        let mut after = before.clone();
        let mut inverses = Vec::new();
        for value in operations.as_array().unwrap() {
            let operation: Gis2dConfigMutation = serde_json::from_value(value.clone()).unwrap();
            inverses.extend(operation.inverse(&after));
            let outcome = operation.diff(&after);
            if row["expected"]["outcome"] == "warning" { assert_eq!(outcome.worst_level(), Some(dsl::Severity::Warning)); }
            after = outcome.diff().apply(&after).unwrap();
        }
        if row["expected"]["afterEqualsBefore"] == true { assert_eq!(after, before); }
        if let Some(expected) = row.get("after") {
            let actual = serde_json::to_value(&after).unwrap();
            for (key, value) in expected.as_object().unwrap() { assert_eq!(&actual[key], value); }
        }
        if let Some(expected) = row.get("inverseStoredOrder") { assert_eq!(serde_json::to_value(&inverses).unwrap(), *expected); }
        inverses.reverse();
        if let Some(expected) = row.get("inverseReplayOrder") { assert_eq!(serde_json::to_value(&inverses).unwrap(), *expected); }
        for operation in inverses { after = apply(&after, &operation); }
        assert_eq!(after, before);
    }
}
//#endregion 🧪️DirectContracts

//#region 🧪️Identity
fn populated() -> Gis2dConfig {
    let mut value = Gis2dConfig::default();
    value.camera_json = r#"{"x":8,"y":9,"zoom":4}"#.into();
    value.render_mode = "vector".into();
    value.vector_style = "figureGround".into();
    value.locale = "de-DE".into();
    value.layer_visibility.insert("water".into(), false);
    value.layer_stroke_scale.insert("roads".into(), 2.0);
    value
}

fn apply(base: &Gis2dConfig, operation: &Gis2dConfigMutation) -> Gis2dConfig {
    operation.diff(base).diff().apply(base).expect("valid configuration diff")
}

fn undo(base: &Gis2dConfig, operation: &Gis2dConfigMutation) -> Gis2dConfig {
    operation.inverse(base).into_iter().rev().fold(apply(base, operation), |state, inverse| apply(&state, &inverse))
}

#[test]
fn no_op_preserves_every_populated_field() {
    let base = populated();
    let operation = Gis2dConfigMutation::SetRenderMode(SetRenderMode { value: base.render_mode.clone() });
    let outcome = operation.diff(&base);
    assert_eq!(outcome.diff(), &Gis2dConfigDiff::default());
    assert_eq!(outcome.worst_level(), Some(dsl::Severity::Warning));
    assert_eq!(outcome.diff().apply(&base).unwrap(), base);
}
//#endregion 🧪️Identity

//#region 🧪️Inverse
#[test]
fn visibility_inverse_distinguishes_absence_and_explicit_default() {
    for previous in [None, Some(false), Some(true)] {
        let mut base = populated();
        base.layer_visibility.remove("water");
        if let Some(value) = previous { base.layer_visibility.insert("water".into(), value); }
        for visible in [None, Some(false), Some(true)] {
            let operation = Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible });
            assert_eq!(undo(&base, &operation), base);
            let after = apply(&base, &operation);
            assert_eq!(after.layer_visibility.get("water").copied(), visible);
        }
    }
}

#[test]
fn stroke_inverse_distinguishes_absence_and_explicit_default() {
    for previous in [None, Some(1.0), Some(2.0)] {
        let mut base = populated();
        base.layer_stroke_scale.remove("roads");
        if let Some(value) = previous { base.layer_stroke_scale.insert("roads".into(), value); }
        for value in [None, Some(1.0), Some(2.0)] {
            let operation = Gis2dConfigMutation::SetLayerStrokeScale(SetLayerStrokeScale { layer_id: "roads".into(), value });
            assert_eq!(undo(&base, &operation), base);
            assert_eq!(apply(&base, &operation).layer_stroke_scale.get("roads").copied(), value);
        }
    }
}
//#endregion 🧪️Inverse

//#region 🧪️Composition
#[test]
fn independent_sparse_writes_compose_and_serde_retains_removal() {
    let base = populated();
    let camera = Gis2dConfigMutation::SetCamera(SetCamera { camera_json: "{}".into() });
    let locale = Gis2dConfigMutation::SetLocale(SetLocale { value: "en-GB".into() });
    let clear = Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible: None });
    let mut combined = camera.diff(&base).into_parts().0;
    combined.absorb(locale.diff(&base).into_parts().0);
    combined.absorb(clear.diff(&base).into_parts().0);
    let decoded = serde_json::from_value::<Gis2dConfigDiff>(serde_json::to_value(&combined).unwrap()).unwrap();
    let actual = decoded.apply(&base).unwrap();
    assert_eq!(actual, apply(&apply(&apply(&base, &camera), &locale), &clear));
    assert_eq!(actual.camera_json, "{}");
    assert_eq!(actual.locale, "en-GB");
    assert_eq!(actual.layer_visibility.get("water"), None);
    assert_eq!(actual.layer_stroke_scale, base.layer_stroke_scale);
}

#[test]
fn invalid_numeric_delta_cannot_be_hidden_by_a_later_write() {
    let base = populated();
    let mut invalid = Gis2dConfigDiff::from(Gis2dConfigDelta { layer_stroke_scale: BTreeMap::from([("roads".into(), Some(f64::NAN))]), ..Default::default() });
    invalid.absorb(Gis2dConfigDiff::from(Gis2dConfigDelta { layer_stroke_scale: BTreeMap::from([("roads".into(), Some(2.0))]), ..Default::default() }));
    assert_eq!(invalid.apply(&base).unwrap_err().code, "mutation.apply.invalid-number");
    assert_eq!(base, populated());
}

#[test]
fn non_finite_values_cannot_serialize_as_override_removals() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let operation = Gis2dConfigMutation::SetLayerStrokeScale(SetLayerStrokeScale { layer_id: "roads".into(), value: Some(value) });
        let base = populated();
        let outcome = operation.diff(&base);
        assert_eq!(outcome.worst_level(), Some(dsl::Severity::Fatal));
        assert_eq!(outcome.diff().apply(&base).unwrap(), base);
        assert!(serde_json::to_value(&operation).is_err());
        let delta = Gis2dConfigDelta { layer_stroke_scale: BTreeMap::from([("roads".into(), Some(value))]), ..Default::default() };
        assert!(serde_json::to_value(Gis2dConfigDiff::from(delta)).is_err());
    }
}
//#endregion 🧪️Composition
