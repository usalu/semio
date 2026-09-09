//! 🧪️ Direct GIS 2D presence camera laws.

use super::*;
use protocol::{Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};

//#region 🧪️Fixture
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧬️schema/🧫️fixtures/🔣️.json")).expect("presence law fixture")
}

fn base() -> Gis2dPresence {
    Gis2dPresence { camera_json: fixture()["laws"][0]["before"]["cameraJson"].as_str().expect("fixture camera").into() }
}

fn apply(base: &Gis2dPresence, operation: &Gis2dPresenceMutation) -> Gis2dPresence {
    operation.diff(base).diff().apply(base).expect("presence diff applies")
}

pub(crate) fn assert_set_camera_leaf(descriptor: &str) {
    let fixture = fixture();
    let envelope = fixture["aggregate"]["valid"][0].clone();
    let mut payload = envelope.clone();
    payload.as_object_mut().expect("envelope object").remove("operation");
    let leaf: SetCamera = dsl::json::from_json_str(&(payload.clone()).to_string()).expect("camera payload");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&leaf)).unwrap(), payload);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&SetCamera::DESCRIPTOR)).unwrap(), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
    assert_eq!(SetCamera::PROVENANCE.owner, SetCamera::DESCRIPTOR.owner);
    assert_eq!(SetCamera::PROVENANCE.source_path, format!("{}/🦀️.rs", SetCamera::DESCRIPTOR.owner));
    assert_eq!(SetCamera::PROVENANCE.descriptor_path, format!("{}/🔣️.json", SetCamera::DESCRIPTOR.owner));
    let operation = Gis2dPresenceMutation::SetCamera(leaf);
    assert_eq!(operation.descriptor(), &SetCamera::DESCRIPTOR);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&operation)).unwrap(), envelope);
    assert_eq!(dsl::json::from_json_str::<Gis2dPresenceMutation>(&(envelope).to_string()).unwrap(), operation);
    assert_eq!(operation.print_op().split_whitespace().next(), SetCamera::DESCRIPTOR.text_opcode);
    assert_eq!(Gis2dPresenceMutation::parse_op(&operation.print_op()).unwrap(), operation);
    let bytes = operation.encode_op().expect("camera binary");
    assert_eq!(bytes[0], dsl::variants_binary::OP_BINARY_FORMAT);
    assert_eq!(Some(u32::from(bytes[1])), SetCamera::DESCRIPTOR.binary_tag);
    assert_eq!(Gis2dPresenceMutation::decode_op(&bytes).unwrap(), operation);
    for law in fixture["laws"].as_array().expect("camera laws") {
        let before: Gis2dPresence = dsl::json::from_json_str(&(law["before"].clone()).to_string()).expect("before camera");
        let after: Gis2dPresence = dsl::json::from_json_str(&(law["after"].clone()).to_string()).expect("after camera");
        let operation: Gis2dPresenceMutation = dsl::json::from_json_str(&(law["operation"].clone()).to_string()).expect("camera mutation");
        let Gis2dPresenceMutation::SetCamera(leaf) = &operation;
        let expected_diff: Gis2dPresenceDiff = dsl::json::from_json_str(&(law.get("operationDiff").unwrap_or(&law["diff"]).clone()).to_string()).expect("operation diff");
        let outcome = operation.diff(&before);
        assert_eq!(outcome.diff(), &expected_diff, "{}", law["name"]);
        assert_eq!(<SetCamera as MutationKind<Gis2dPresence, Gis2dPresenceMutation>>::diff(leaf, &before), outcome);
        assert_eq!(outcome.diff().apply(&before).expect("camera applies"), after);
        assert_eq!(before != after, law["outcome"]["changed"].as_bool().expect("changed flag"));
        if let Some(code) = law["outcome"]["warningCode"].as_str() {
            assert_eq!(outcome.messages().len(), 1);
            assert_eq!(outcome.messages()[0].code.0, code);
            assert_eq!(outcome.messages()[0].level, dsl::Severity::Warning);
        } else {
            assert!(outcome.messages().is_empty());
        }
        let expected_inverse: Vec<Gis2dPresenceMutation> = dsl::json::from_json_str(&(law["inverse"].clone()).to_string()).expect("stored inverse");
        let inverse = operation.inverse(&before);
        assert_eq!(inverse, expected_inverse, "{}", law["name"]);
        assert_eq!(<SetCamera as MutationKind<Gis2dPresence, Gis2dPresenceMutation>>::inverse(leaf, &before), inverse);
        let restored = inverse.iter().rev().fold(after.clone(), |state, inverse| apply(&state, inverse));
        assert_eq!(restored, before);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&operation)).expect("mutation JSON"), law["operation"]);
        let text = operation.print_op();
        let parsed = Gis2dPresenceMutation::parse_op(&text).expect("camera text");
        assert_eq!(parsed, operation);
        assert_eq!(parsed.print_op(), text);
        let bytes = operation.encode_op().expect("camera binary");
        let decoded = Gis2dPresenceMutation::decode_op(&bytes).expect("camera binary roundtrip");
        assert_eq!(decoded, operation);
        assert_eq!(decoded.encode_op().expect("canonical binary"), bytes);
        for end in 0..bytes.len() {
            assert!(Gis2dPresenceMutation::decode_op(&bytes[..end]).is_err(), "{} prefix {end}", law["name"]);
        }
        let expected: Gis2dPresenceDiff = dsl::json::from_json_str(&(law["diff"].clone()).to_string()).expect("ordered diff");
        let mut combined = Gis2dPresenceDiff::default();
        let mut sequential = before.clone();
        for step in &expected.steps {
            let part = Gis2dPresenceDiff { steps: vec![step.clone()] };
            sequential = part.apply(&sequential).expect("ordered camera write");
            combined.absorb(part);
        }
        assert_eq!(combined, expected);
        assert_eq!(combined.apply(&before).expect("combined camera write"), sequential);
        assert_eq!(sequential, after);
        combined.absorb(Gis2dPresenceDiff::default());
        assert_eq!(combined, expected);
        assert_eq!(before, dsl::json::from_json_str::<Gis2dPresence>(&(law["before"].clone()).to_string()).expect("unchanged before"));
    }
}
//#endregion 🧪️Fixture

//#region 🧪️Contracts
#[test]
fn strict_state_and_payload_vectors_match_the_direct_camera_contract() {
    let fixture = fixture();
    for row in fixture["state"]["valid"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<Gis2dPresence>(&(row["value"].clone()).to_string()).is_ok(), "{}", row["name"]);
    }
    for row in fixture["state"]["invalid"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<Gis2dPresence>(&(row["value"].clone()).to_string()).is_err(), "{}", row["name"]);
    }
    for payload in fixture["payload"]["valid"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<SetCamera>(&(payload.clone()).to_string()).is_ok());
    }
    for payload in fixture["payload"]["invalid"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<SetCamera>(&(payload.clone()).to_string()).is_err());
    }
    for envelope in fixture["aggregate"]["valid"].as_array().unwrap() {
        let operation: Gis2dPresenceMutation = dsl::json::from_json_str(&(envelope.clone()).to_string()).expect("valid aggregate");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&operation)).expect("aggregate JSON"), *envelope);
    }
    for envelope in fixture["aggregate"]["invalid"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<Gis2dPresenceMutation>(&(envelope.clone()).to_string()).is_err());
    }
}

#[test]
fn direct_payload_metadata_text_binary_and_inverse_match_neutral_fixture() {
    assert_set_camera_leaf(include_str!("../../🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json"));
}

#[test]
fn sparse_camera_diff_has_an_empty_identity_and_preserves_the_no_op_warning() {
    let before = base();
    let operation = Gis2dPresenceMutation::SetCamera(SetCamera { camera_json: before.camera_json.clone() });
    let outcome = operation.diff(&before);
    assert_eq!(outcome.diff(), &Gis2dPresenceDiff::default());
    assert_eq!(outcome.worst_level(), Some(dsl::Severity::Warning));
    assert_eq!(outcome.diff().apply(&before).unwrap(), before);
}

#[test]
fn sparse_camera_diff_serde_order_noop_and_codec_rejections_match_neutral_fixture() {
    let fixture = fixture();
    for value in fixture["diff"]["valid"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<Gis2dPresenceDiff>(&(value.clone()).to_string()).is_ok());
    }
    for value in fixture["diff"]["invalid"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<Gis2dPresenceDiff>(&(value.clone()).to_string()).is_err());
    }
    let missing: Gis2dPresenceDiff = dsl::json::from_json_str(&(serde_json::json!({"steps":[{}]})).to_string()).unwrap();
    let explicit_null: Gis2dPresenceDiff = dsl::json::from_json_str(&(serde_json::json!({"steps":[{"cameraJson":null}]})).to_string()).unwrap();
    assert_eq!(missing, explicit_null);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&missing)).unwrap(), serde_json::json!({"steps":[{"cameraJson":null}]}));
    let law = &fixture["laws"][3];
    let before: Gis2dPresence = dsl::json::from_json_str(&(law["before"].clone()).to_string()).unwrap();
    let diff: Gis2dPresenceDiff = dsl::json::from_json_str(&(law["diff"].clone()).to_string()).unwrap();
    let after: Gis2dPresence = dsl::json::from_json_str(&(law["after"].clone()).to_string()).unwrap();
    assert_eq!(missing.apply(&before).unwrap(), before);
    assert_eq!(explicit_null.apply(&before).unwrap(), before);
    assert_eq!(diff.apply(&before).unwrap(), after);
    let no_op = Gis2dPresenceMutation::SetCamera(SetCamera { camera_json: before.camera_json.clone() });
    assert_eq!(no_op.diff(&before).diff(), &Gis2dPresenceDiff::default());
    assert_eq!(no_op.diff(&before).worst_level(), Some(dsl::Severity::Warning));
    assert!(Gis2dPresenceMutation::parse_op("camera cameraJson \"{}\"").is_err());
    assert!(Gis2dPresenceMutation::parse_op("set-camera cameraJson 1").is_err());
    assert!(Gis2dPresenceMutation::decode_op(&[]).is_err());
    assert!(Gis2dPresenceMutation::decode_op(&[dsl::variants_binary::OP_BINARY_FORMAT, 0, 0xff]).is_err());
    let mut truncated = no_op.encode_op().unwrap();
    truncated.pop();
    assert!(Gis2dPresenceMutation::decode_op(&truncated).is_err());
    let mut trailing = no_op.encode_op().unwrap();
    trailing.push(0xff);
    assert!(Gis2dPresenceMutation::decode_op(&trailing).is_err());
}
//#endregion 🧪️Contracts
