#[path = "../../../../../../../../../🌎️hub/📇️directory/🦀️.rs"]
pub mod directory;

#[cfg(test)]
mod descriptor_contract_tests {
    use directory::os_directory::{descriptor_digest_encoding_v1, descriptor_digest_v1, hex_lower, ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DocumentDescriptor};
    use directory::{DslValue, FromValue, ToValue};

    fn ordered_json(value: &DslValue, out: &mut String) {
        match value {
            DslValue::Null => out.push_str("null"),
            DslValue::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            DslValue::Number(value) => out.push_str(&serde_json::Value::from(DslValue::Number(*value)).to_string()),
            DslValue::String(value) => out.push_str(&serde_json::to_string(value).unwrap()),
            DslValue::Array(values) => {
                out.push('[');
                for (index, value) in values.iter().enumerate() {
                    if index > 0 { out.push(','); }
                    ordered_json(value, out);
                }
                out.push(']');
            }
            DslValue::Object(entries) => {
                out.push('{');
                for (index, (key, value)) in entries.iter().enumerate() {
                    if index > 0 { out.push(','); }
                    out.push_str(&serde_json::to_string(key).unwrap());
                    out.push(':');
                    ordered_json(value, out);
                }
                out.push('}');
            }
        }
    }

    #[test]
    fn document_descriptor_matches_language_neutral_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️document-descriptor.json")).unwrap();
        let descriptor = DocumentDescriptor::from_value(DslValue::from(fixture["valid"].clone())).unwrap();
        let mut actual = String::new();
        ordered_json(&descriptor.to_value(), &mut actual);
        assert_eq!(actual, fixture["canonical"].as_str().unwrap());
    }

    #[test]
    fn document_descriptor_digest_matches_language_neutral_fixture_and_rejects_invalid_boundaries() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️artifact-authority.json")).unwrap();
        let descriptor = DocumentDescriptor::from_value(DslValue::from(fixture["descriptor"].clone())).unwrap();
        let expected = ArtifactHash::from_value(DslValue::from(fixture["descriptorDigestV1"].clone())).unwrap();
        assert_eq!(hex_lower(&descriptor_digest_encoding_v1(&descriptor).unwrap()), fixture["descriptorEncodingHex"].as_str().unwrap());
        assert_eq!(descriptor_digest_v1(&descriptor).unwrap(), expected);

        let mut empty = descriptor.clone();
        empty.space_id.clear();
        assert!(descriptor_digest_v1(&empty).is_err());
        let mut invalid_hash = descriptor;
        invalid_hash.owner.package_hash = "FF".repeat(32);
        assert!(descriptor_digest_v1(&invalid_hash).is_err());
        assert!(ArtifactHash::from_value(DslValue::from(serde_json::json!(vec![0; 31]))).is_err());
        let mut overflow = vec![0u64; 32];
        overflow[31] = 256;
        assert!(ArtifactHash::from_value(DslValue::from(serde_json::json!(overflow))).is_err());

        let checkpoint = ArtifactCheckpoint::from_value(DslValue::from(fixture["checkpoint"].clone())).unwrap();
        let retention = ArtifactRetention::from_value(DslValue::from(fixture["retention"].clone())).unwrap();
        assert_eq!(checkpoint.scope, retention.scope);
        assert_eq!(checkpoint.checkpoint_id, retention.retained_checkpoint_id);
        assert!(ArtifactHash::from_value(DslValue::from(fixture["invalidBoundaries"]["shortHash"].clone())).is_err());
        assert!(ArtifactHash::from_value(DslValue::from(fixture["invalidBoundaries"]["overflowHashByte"].clone())).is_err());
    }
}
