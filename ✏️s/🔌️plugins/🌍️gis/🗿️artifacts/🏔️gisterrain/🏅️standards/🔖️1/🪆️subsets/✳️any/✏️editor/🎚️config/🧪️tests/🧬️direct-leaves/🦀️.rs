//! 🧪️ Shared native consumers of the schema-first GIS 3D configuration vectors.

use super::{Gis3dConfig, Gis3dConfigDiff, Gis3dConfigMutation};
use protocol::{Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use store::{ArtifactDsl, ArtifactPack};

//#region 🧫️NeutralFixture
fn vectors() -> Value {
    serde_json::from_str(include_str!("🔣️.json")).expect("domain neutral vectors")
}

fn decode<T: DeserializeOwned>(value: &Value) -> T {
    serde_json::from_value(value.clone()).expect("typed neutral value")
}

fn assert_schema_cases<T>(cases: &Value)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    for value in cases["valid"].as_array().expect("valid cases") {
        let decoded: T = decode(value);
        let encoded = serde_json::to_value(&decoded).expect("encode typed value");
        assert_eq!(decode::<T>(&encoded), decoded);
    }
    for value in cases["invalid"].as_array().expect("invalid cases") {
        assert!(serde_json::from_value::<T>(value.clone()).is_err(), "{} accepted {value}", std::any::type_name::<T>());
    }
}
//#endregion 🧫️NeutralFixture

//#region 🔺️SnapshotAndDiff
#[test]
fn strict_snapshot_and_aggregate_serde_vectors() {
    let fixture = vectors();
    assert_schema_cases::<Gis3dConfig>(&fixture["config"]);
    assert_schema_cases::<Gis3dConfigMutation>(&fixture["mutations"]);
    for value in fixture["config"]["valid"].as_array().expect("valid snapshots") {
        let snapshot: Gis3dConfig = decode(value);
        assert_eq!(serde_json::to_value(snapshot).expect("snapshot JSON"), *value);
    }
    for value in fixture["mutations"]["valid"].as_array().expect("valid operations") {
        let operation: Gis3dConfigMutation = decode(value);
        assert_eq!(serde_json::to_value(operation).expect("operation JSON"), *value);
    }
}

#[test]
fn sparse_diff_vectors_and_ordered_absorption() {
    let fixture = vectors();
    assert_schema_cases::<Gis3dConfigDiff>(&fixture["diff"]);
    for law in fixture["diff"]["laws"].as_array().expect("diff laws") {
        let before: Gis3dConfig = decode(&law["before"]);
        let diff: Gis3dConfigDiff = decode(&law["diff"]);
        let after: Gis3dConfig = decode(&law["after"]);
        assert_eq!(diff.apply(&before), Ok(after), "{}", law["id"]);
        assert_eq!(before, decode::<Gis3dConfig>(&law["before"]));
        assert_eq!(Gis3dConfigDiff::default().apply(&before), Ok(before));
    }
    for law in fixture["diff"]["absorption"].as_array().expect("absorption laws") {
        let before: Gis3dConfig = decode(&law["before"]);
        let left: Gis3dConfigDiff = decode(&law["left"]);
        let right: Gis3dConfigDiff = decode(&law["right"]);
        let expected: Gis3dConfigDiff = decode(&law["expected"]);
        let after: Gis3dConfig = decode(&law["after"]);
        let sequential = left.apply(&before).and_then(|mid| right.apply(&mid));
        let mut combined = left.clone();
        combined.absorb(right.clone());
        assert_eq!(combined, expected, "{}", law["id"]);
        assert_eq!(combined.apply(&before), sequential, "{}", law["id"]);
        assert_eq!(combined.apply(&before), Ok(after), "{}", law["id"]);
        let mut with_identity = Gis3dConfigDiff::default();
        with_identity.absorb(combined.clone());
        with_identity.absorb(Gis3dConfigDiff::default());
        assert_eq!(with_identity, combined);
    }
}

#[test]
fn snapshot_document_text_and_binary_roundtrips() {
    let fixture = vectors();
    for value in fixture["config"]["valid"].as_array().expect("valid snapshots") {
        let snapshot: Gis3dConfig = decode(value);
        let text = snapshot.print_dsl();
        assert_eq!(Gis3dConfig::parse_dsl(&text).expect("snapshot text"), snapshot);
        let bytes = snapshot.encode_pack_with(&store::PackEncodeOptions::default()).expect("snapshot pack");
        assert_eq!(Gis3dConfig::decode_pack_with(&bytes, &store::PackDecodeOptions::default()).expect("snapshot binary"), snapshot);
        for end in 0..bytes.len() {
            assert!(Gis3dConfig::decode_pack_with(&bytes[..end], &store::PackDecodeOptions::default()).is_err(), "truncated snapshot at {end}");
        }
    }
    assert!(Gis3dConfig::parse_dsl("").is_err());
    assert!(Gis3dConfig::parse_dsl("unknown-field 1").is_err());
}
//#endregion 🔺️SnapshotAndDiff

//#region 🧬️GenericLeafContract
fn assert_operation_codecs(operation: &Gis3dConfigMutation, keyword: &str, tag: u32) {
    let line = operation.print_op();
    assert!(line == keyword || line.starts_with(&format!("{keyword} ")));
    let parsed = Gis3dConfigMutation::parse_op(&line).expect("operation text");
    assert_eq!(&parsed, operation);
    assert_eq!(parsed.print_op(), line);
    let bytes = operation.encode_op().expect("operation binary");
    let decoded = Gis3dConfigMutation::decode_op(&bytes).expect("operation binary roundtrip");
    assert_eq!(&decoded, operation);
    assert_eq!(decoded.encode_op().expect("canonical binary"), bytes);
    assert_eq!(Gis3dConfigMutation::parse_op(&decoded.print_op()).expect("binary to text"), *operation);
    assert_eq!(Gis3dConfigMutation::decode_op(&parsed.encode_op().expect("text to binary")).expect("text to binary roundtrip"), *operation);
    assert_eq!(bytes[0], dsl::variants_binary::OP_BINARY_FORMAT);
    assert_eq!(bytes[1], u8::try_from(tag).expect("fixture one-byte ordinal"));
    for end in 0..bytes.len() {
        assert!(Gis3dConfigMutation::decode_op(&bytes[..end]).is_err(), "truncated {keyword} at {end}");
    }
    let mut bad_format = bytes.clone();
    bad_format[0] = 0;
    assert!(matches!(Gis3dConfigMutation::decode_op(&bad_format), Err(protocol::ProtocolError::Malformed { what: "op format", .. })));
    let mut unknown_ordinal = bytes.clone();
    unknown_ordinal[1] = u8::try_from(<Gis3dConfigMutation as dsl::DslVariants>::variants().len()).expect("fixture roster fits one byte");
    assert!(matches!(Gis3dConfigMutation::decode_op(&unknown_ordinal), Err(protocol::ProtocolError::Malformed { what: "op variant", .. })));
    assert!(Gis3dConfigMutation::decode_op(&[dsl::variants_binary::OP_BINARY_FORMAT, 0x80]).is_err());
    assert!(Gis3dConfigMutation::parse_op("").is_err());
    assert!(Gis3dConfigMutation::parse_op(keyword).is_err());
    assert!(Gis3dConfigMutation::parse_op(&format!("unknown-{line}")).is_err());
    assert!(Gis3dConfigMutation::parse_op(&format!("{line} unknown-field 1")).is_err());
    assert!(Gis3dConfigMutation::parse_op(&format!("{line} \"")).is_err());
}

/// 🧬️ Executes a leaf's exact neutral cases through its intrinsic and aggregate contracts.
pub(crate) fn assert_leaf_contract<T>(key: &str, authored_descriptor: &str)
where
    T: MutationKind<Gis3dConfig, Gis3dConfigMutation> + MutationLeaf + dsl::DslField + Into<Gis3dConfigMutation> + PartialEq + Debug,
{
    let fixture = vectors();
    let row = fixture["leaves"].as_array().expect("leaf roster").iter().find(|row| row["key"] == key).expect("leaf fixture");
    let descriptor: Value = serde_json::from_str(authored_descriptor).expect("actual leaf descriptor");
    let provenance = T::PROVENANCE;
    assert!(protocol::validate_mutation_leaf_descriptor(&T::DESCRIPTOR).is_ok());
    assert_eq!(descriptor["owner"], T::DESCRIPTOR.owner);
    assert_eq!(descriptor["semanticKind"], T::DESCRIPTOR.semantic_kind);
    assert_eq!(descriptor["aggregateVariant"], T::DESCRIPTOR.aggregate_variant);
    assert_eq!(descriptor["payloadSchema"], T::DESCRIPTOR.payload_schema);
    assert_eq!(descriptor["textOpcode"].as_str(), T::DESCRIPTOR.text_opcode);
    assert_eq!(descriptor["binaryTag"].as_u64(), T::DESCRIPTOR.binary_tag.map(u64::from));
    assert_eq!(T::DESCRIPTOR.semantic_kind, row["kind"].as_str().expect("semantic kind"));
    assert_eq!(T::DESCRIPTOR.aggregate_variant, row["variant"].as_str().expect("aggregate variant"));
    assert_eq!(T::DESCRIPTOR.text_opcode, row["kind"].as_str());
    assert_eq!(T::DESCRIPTOR.binary_tag.map(u64::from), row["tag"].as_u64());
    assert_eq!(<T as MutationKind<Gis3dConfig, Gis3dConfigMutation>>::SEMANTICS.kind, T::DESCRIPTOR.semantic_kind);
    assert_eq!(provenance.owner, T::DESCRIPTOR.owner);
    assert_eq!(provenance.source_path, format!("{}/🦀️.rs", T::DESCRIPTOR.owner));
    assert_eq!(provenance.descriptor_path, format!("{}/🔣️.json", T::DESCRIPTOR.owner));
    let mutation_root: &'static str = T::DESCRIPTOR.owner.rsplit_once('/').expect("direct leaf parent").0;
    let scope = protocol::MutationLeafSourceScope {
        workspace_token: provenance.workspace_token,
        mutation_root,
        owner_layout: protocol::MutationOwnerLayout::Flat,
        taxonomy_path: provenance.taxonomy_path,
        mutation_payload_facet: "🦠️mutation",
        source_filename: "🦀️.rs",
        descriptor_filename: "🔣️.json",
    };
    assert!(protocol::validate_mutation_leaf_source(&T::DESCRIPTOR, &provenance, &scope).is_ok());
    assert_eq!(<Gis3dConfigMutation as Mutation<Gis3dConfig>>::DESCRIPTORS.iter().filter(|entry| entry.owner == T::DESCRIPTOR.owner).count(), 1);
    assert_schema_cases::<T>(&fixture["payloads"][key]);
    for payload in fixture["payloads"][key]["valid"].as_array().expect("valid payloads") {
        let leaf: T = decode(payload);
        assert_eq!(serde_json::to_value(&leaf).expect("payload JSON"), *payload);
        assert_eq!(<T as dsl::DslField>::from_value(&leaf.to_value()).expect("intrinsic record roundtrip"), leaf);
    }
    for law in row["cases"].as_array().expect("leaf cases") {
        let before: Gis3dConfig = decode(&law["before"]);
        let after: Gis3dConfig = decode(&law["after"]);
        let expected_diff: Gis3dConfigDiff = decode(&law["diff"]);
        let leaf: T = decode(&law["payload"]);
        let operation: Gis3dConfigMutation = leaf.clone().into();
        let outcome = <T as MutationKind<Gis3dConfig, Gis3dConfigMutation>>::diff(&leaf, &before);
        assert_eq!(&T::DESCRIPTOR, operation.descriptor());
        assert_eq!(outcome.diff(), &expected_diff, "{}", law["id"]);
        assert_eq!(operation.diff(&before), outcome, "{}", law["id"]);
        assert_eq!(outcome.diff().apply(&before), Ok(after.clone()), "{}", law["id"]);
        assert_eq!(<T as MutationKind<Gis3dConfig, Gis3dConfigMutation>>::target(&leaf), vec![row["target"].as_str().expect("target").to_owned()]);
        if let Some(code) = law["warning"].as_str() {
            assert_eq!(outcome.messages().len(), 1, "{}", law["id"]);
            assert_eq!(outcome.messages()[0].code.0, code);
            assert_eq!(outcome.messages()[0].level, protocol::MutationMessage::warn(code, "").level);
            assert_eq!(outcome.diff(), &Gis3dConfigDiff::default());
        } else {
            assert!(outcome.messages().is_empty(), "{}", law["id"]);
        }
        let expected_inverse: Vec<Gis3dConfigMutation> = decode(&law["inverse"]);
        let stored = operation.inverse(&before);
        assert_eq!(stored, expected_inverse, "{}", law["id"]);
        assert_eq!(<T as MutationKind<Gis3dConfig, Gis3dConfigMutation>>::inverse(&leaf, &before), stored, "{}", law["id"]);
        assert_eq!(serde_json::to_value(&stored).expect("stored inverse JSON"), law["inverse"]);
        let restored = stored.iter().rev().try_fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state)).expect("stored inverse application");
        assert_eq!(restored, before, "{}", law["id"]);
        let mut envelope = law["payload"].as_object().expect("payload object").clone();
        envelope.insert("operation".into(), law["inverse"][0]["operation"].clone());
        assert_eq!(serde_json::to_value(&operation).expect("forward JSON"), Value::Object(envelope));
        assert_operation_codecs(&operation, T::DESCRIPTOR.text_opcode.expect("text opcode"), T::DESCRIPTOR.binary_tag.expect("binary ordinal"));
    }
}
//#endregion 🧬️GenericLeafContract
