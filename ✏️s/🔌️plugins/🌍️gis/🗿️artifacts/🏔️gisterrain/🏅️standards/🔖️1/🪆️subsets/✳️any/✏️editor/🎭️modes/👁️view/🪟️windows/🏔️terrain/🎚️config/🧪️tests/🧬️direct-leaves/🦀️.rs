//! 🧪️ Shared native consumers of the schema-first GIS 3D configuration vectors.

use super::{GisTerrainWindowConfig, GisTerrainWindowConfigDiff, GisTerrainWindowConfigMutation};
use protocol::{Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};
use serde_json::Value;
use std::fmt::Debug;
use store::{ArtifactDsl, ArtifactPack};

//#region 🧫️NeutralFixture
fn vectors() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧬️direct-leaves/🔣️.json")).expect("domain neutral vectors")
}

fn decode<T: dsl::FromValue>(value: &Value) -> T {
    dsl::json::from_json_str(&(value.clone()).to_string()).expect("typed neutral value")
}

fn assert_schema_cases<T>(cases: &Value)
where
    T: dsl::FromValue + dsl::ToValue + PartialEq + Debug,
{
    for value in cases["valid"].as_array().expect("valid cases") {
        let decoded: T = decode(value);
        let encoded = serde_json::from_str::<Value>(&dsl::json::to_json_string(&decoded)).expect("encode typed value");
        assert_eq!(decode::<T>(&encoded), decoded);
    }
    for value in cases["invalid"].as_array().expect("invalid cases") {
        assert!(dsl::json::from_json_str::<T>(&(value.clone()).to_string()).is_err(), "{} accepted {value}", std::any::type_name::<T>());
    }
}
//#endregion 🧫️NeutralFixture

//#region 🔺️SnapshotAndDiff
#[test]
fn strict_snapshot_and_aggregate_json_vectors() {
    let fixture = vectors();
    assert_schema_cases::<GisTerrainWindowConfig>(&fixture["config"]);
    assert_schema_cases::<GisTerrainWindowConfigMutation>(&fixture["mutations"]);
    for value in fixture["config"]["valid"].as_array().expect("valid snapshots") {
        let snapshot: GisTerrainWindowConfig = decode(value);
        assert_eq!(serde_json::from_str::<Value>(&dsl::json::to_json_string(&snapshot)).expect("snapshot JSON"), *value);
    }
    for value in fixture["mutations"]["valid"].as_array().expect("valid operations") {
        let operation: GisTerrainWindowConfigMutation = decode(value);
        assert_eq!(serde_json::from_str::<Value>(&dsl::json::to_json_string(&operation)).expect("operation JSON"), *value);
    }
}

#[test]
fn sparse_diff_vectors_and_ordered_absorption() {
    let fixture = vectors();
    assert_schema_cases::<GisTerrainWindowConfigDiff>(&fixture["diff"]);
    for law in fixture["diff"]["laws"].as_array().expect("diff laws") {
        let before: GisTerrainWindowConfig = decode(&law["before"]);
        let diff: GisTerrainWindowConfigDiff = decode(&law["diff"]);
        let after: GisTerrainWindowConfig = decode(&law["after"]);
        assert_eq!(diff.apply(&before), Ok(after), "{}", law["id"]);
        assert_eq!(before, decode::<GisTerrainWindowConfig>(&law["before"]));
        assert_eq!(GisTerrainWindowConfigDiff::default().apply(&before), Ok(before));
    }
    for law in fixture["diff"]["absorption"].as_array().expect("absorption laws") {
        let before: GisTerrainWindowConfig = decode(&law["before"]);
        let left: GisTerrainWindowConfigDiff = decode(&law["left"]);
        let right: GisTerrainWindowConfigDiff = decode(&law["right"]);
        let expected: GisTerrainWindowConfigDiff = decode(&law["expected"]);
        let after: GisTerrainWindowConfig = decode(&law["after"]);
        let sequential = left.apply(&before).and_then(|mid| right.apply(&mid));
        let mut combined = left.clone();
        combined.absorb(right.clone());
        assert_eq!(combined, expected, "{}", law["id"]);
        assert_eq!(combined.apply(&before), sequential, "{}", law["id"]);
        assert_eq!(combined.apply(&before), Ok(after), "{}", law["id"]);
        let mut with_identity = GisTerrainWindowConfigDiff::default();
        with_identity.absorb(combined.clone());
        with_identity.absorb(GisTerrainWindowConfigDiff::default());
        assert_eq!(with_identity, combined);
    }
}

#[test]
fn snapshot_document_text_and_binary_roundtrips() {
    let fixture = vectors();
    for value in fixture["config"]["valid"].as_array().expect("valid snapshots") {
        let snapshot: GisTerrainWindowConfig = decode(value);
        let text = snapshot.print_dsl();
        assert_eq!(GisTerrainWindowConfig::parse_dsl(&text).expect("snapshot text"), snapshot);
        let bytes = snapshot.encode_pack_with(&store::PackEncodeOptions::default()).expect("snapshot pack");
        assert_eq!(GisTerrainWindowConfig::decode_pack_with(&bytes, &store::PackDecodeOptions::default()).expect("snapshot binary"), snapshot);
        for end in 0..bytes.len() {
            assert!(GisTerrainWindowConfig::decode_pack_with(&bytes[..end], &store::PackDecodeOptions::default()).is_err(), "truncated snapshot at {end}");
        }
    }
    assert!(GisTerrainWindowConfig::parse_dsl("").is_err());
    assert!(GisTerrainWindowConfig::parse_dsl("unknown-field 1").is_err());
}
//#endregion 🔺️SnapshotAndDiff

//#region 🧬️GenericLeafContract
fn assert_operation_codecs(operation: &GisTerrainWindowConfigMutation, keyword: &str, tag: u32) {
    let line = operation.print_op();
    assert!(line == keyword || line.starts_with(&format!("{keyword} ")));
    let parsed = GisTerrainWindowConfigMutation::parse_op(&line).expect("operation text");
    assert_eq!(&parsed, operation);
    assert_eq!(parsed.print_op(), line);
    let bytes = operation.encode_op().expect("operation binary");
    let decoded = GisTerrainWindowConfigMutation::decode_op(&bytes).expect("operation binary roundtrip");
    assert_eq!(&decoded, operation);
    assert_eq!(decoded.encode_op().expect("canonical binary"), bytes);
    assert_eq!(GisTerrainWindowConfigMutation::parse_op(&decoded.print_op()).expect("binary to text"), *operation);
    assert_eq!(GisTerrainWindowConfigMutation::decode_op(&parsed.encode_op().expect("text to binary")).expect("text to binary roundtrip"), *operation);
    assert_eq!(bytes[0], dsl::variants_binary::OP_BINARY_FORMAT);
    assert_eq!(bytes[1], u8::try_from(tag).expect("fixture one-byte ordinal"));
    for end in 0..bytes.len() {
        assert!(GisTerrainWindowConfigMutation::decode_op(&bytes[..end]).is_err(), "truncated {keyword} at {end}");
    }
    let mut bad_format = bytes.clone();
    bad_format[0] = 0;
    assert!(matches!(GisTerrainWindowConfigMutation::decode_op(&bad_format), Err(protocol::ProtocolError::Malformed { what: "op format", .. })));
    let mut unknown_ordinal = bytes.clone();
    unknown_ordinal[1] = u8::try_from(<GisTerrainWindowConfigMutation as dsl::DslVariants>::variants().len()).expect("fixture roster fits one byte");
    assert!(matches!(GisTerrainWindowConfigMutation::decode_op(&unknown_ordinal), Err(protocol::ProtocolError::Malformed { what: "op variant", .. })));
    assert!(GisTerrainWindowConfigMutation::decode_op(&[dsl::variants_binary::OP_BINARY_FORMAT, 0x80]).is_err());
    assert!(GisTerrainWindowConfigMutation::parse_op("").is_err());
    assert!(GisTerrainWindowConfigMutation::parse_op(keyword).is_err());
    assert!(GisTerrainWindowConfigMutation::parse_op(&format!("unknown-{line}")).is_err());
    assert!(GisTerrainWindowConfigMutation::parse_op(&format!("{line} unknown-field 1")).is_err());
    assert!(GisTerrainWindowConfigMutation::parse_op(&format!("{line} \"")).is_err());
}

/// 🧬️ Executes a leaf's exact neutral cases through its intrinsic and aggregate contracts.
pub(crate) fn assert_leaf_contract<T>(key: &str, authored_descriptor: &str)
where
    T: MutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation> + MutationLeaf + dsl::DslField + Into<GisTerrainWindowConfigMutation> + PartialEq + Debug,
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
    assert_eq!(<T as MutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation>>::SEMANTICS.kind, T::DESCRIPTOR.semantic_kind);
    assert_eq!(provenance.owner, T::DESCRIPTOR.owner);
    assert_eq!(provenance.source_path, format!("{}/🦀️.rs", T::DESCRIPTOR.owner));
    assert_eq!(provenance.descriptor_path, format!("{}/../../🧫️fixtures/🧬️direct-leaves/🔣️.json", T::DESCRIPTOR.owner));
    let mutation_root: &'static str = T::DESCRIPTOR.owner.rsplit_once('/').expect("direct leaf parent").0;
    let scope = protocol::MutationLeafSourceScope {
        workspace_token: provenance.workspace_token,
        mutation_root,
        owner_layout: protocol::MutationOwnerLayout::Flat,
        taxonomy_path: provenance.taxonomy_path,
        mutation_payload_facet: "🦠️mutation",
        source_filename: "🦀️.rs",
        descriptor_filename: "../../🧫️fixtures/🧬️direct-leaves/🔣️.json",
    };
    assert!(protocol::validate_mutation_leaf_source(&T::DESCRIPTOR, &provenance, &scope).is_ok());
    assert_eq!(<GisTerrainWindowConfigMutation as Mutation<GisTerrainWindowConfig>>::DESCRIPTORS.iter().filter(|entry| entry.owner == T::DESCRIPTOR.owner).count(), 1);
    assert_schema_cases::<T>(&fixture["payloads"][key]);
    for payload in fixture["payloads"][key]["valid"].as_array().expect("valid payloads") {
        let leaf: T = decode(payload);
        assert_eq!(serde_json::from_str::<Value>(&dsl::json::to_json_string(&leaf)).expect("payload JSON"), *payload);
        assert_eq!(<T as dsl::DslField>::from_value(&<T as dsl::DslField>::to_value(&leaf)).expect("intrinsic record roundtrip"), leaf);
    }
    for law in row["cases"].as_array().expect("leaf cases") {
        let before: GisTerrainWindowConfig = decode(&law["before"]);
        let after: GisTerrainWindowConfig = decode(&law["after"]);
        let expected_diff: GisTerrainWindowConfigDiff = decode(&law["diff"]);
        let leaf: T = decode(&law["payload"]);
        let operation: GisTerrainWindowConfigMutation = leaf.clone().into();
        let outcome = <T as MutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation>>::diff(&leaf, &before);
        assert_eq!(&T::DESCRIPTOR, operation.descriptor());
        assert_eq!(outcome.diff(), &expected_diff, "{}", law["id"]);
        assert_eq!(operation.diff(&before), outcome, "{}", law["id"]);
        assert_eq!(outcome.diff().apply(&before), Ok(after.clone()), "{}", law["id"]);
        assert_eq!(<T as MutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation>>::target(&leaf), vec![row["target"].as_str().expect("target").to_owned()]);
        if let Some(code) = law["warning"].as_str() {
            assert_eq!(outcome.messages().len(), 1, "{}", law["id"]);
            assert_eq!(outcome.messages()[0].code.0, code);
            assert_eq!(outcome.messages()[0].level, dsl::Severity::Warning);
            assert_eq!(outcome.diff(), &GisTerrainWindowConfigDiff::default());
        } else {
            assert!(outcome.messages().is_empty(), "{}", law["id"]);
        }
        let expected_inverse: Vec<GisTerrainWindowConfigMutation> = decode(&law["inverse"]);
        let stored = operation.inverse(&before);
        assert_eq!(stored, expected_inverse, "{}", law["id"]);
        assert_eq!(<T as MutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation>>::inverse(&leaf, &before), stored, "{}", law["id"]);
        assert_eq!(serde_json::from_str::<Value>(&dsl::json::to_json_string(&stored)).expect("stored inverse JSON"), law["inverse"]);
        let restored = stored.iter().rev().try_fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state)).expect("stored inverse application");
        assert_eq!(restored, before, "{}", law["id"]);
        let mut envelope = law["payload"].as_object().expect("payload object").clone();
        envelope.insert("operation".into(), law["inverse"][0]["operation"].clone());
        assert_eq!(serde_json::from_str::<Value>(&dsl::json::to_json_string(&operation)).expect("forward JSON"), Value::Object(envelope));
        assert_operation_codecs(&operation, T::DESCRIPTOR.text_opcode.expect("text opcode"), T::DESCRIPTOR.binary_tag.expect("binary ordinal"));
    }
}
//#endregion 🧬️GenericLeafContract
