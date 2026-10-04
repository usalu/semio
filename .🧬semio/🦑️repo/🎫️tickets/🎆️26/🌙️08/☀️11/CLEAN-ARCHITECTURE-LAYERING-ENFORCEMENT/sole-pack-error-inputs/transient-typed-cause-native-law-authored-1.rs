/// 🧷️ Preserves each decoder's typed cause through the generated transient codecs.
#[test]
fn transient_codecs_preserve_typed_causes_and_protocol_envelopes() {
    use protocol::{OpBinary, OpText};
    use semio_framework_pack_error::{PackError, PackRefusal};
    use semio_framework_value::{ValueError, ValueRefusalKind};
    use store::{ArtifactDsl, ArtifactPack};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("language-neutral fixture");
    let cases = fixture["cases"].as_array().expect("fixture cases");
    assert_eq!(cases.len(), 10);
    for case in cases {
        let bytes: Vec<u8> = case["bytes"].as_array().expect("input bytes").iter().map(|value| value.as_u64().expect("byte") as u8).collect();
        let route = case["route"].as_str().expect("codec route");
        let expected = match case["sourceOwner"].as_str().expect("defining cause owner") {
            "Utf8Error" => ValueError::from(std::str::from_utf8(&bytes).expect_err("rejected UTF8")),
            "JsonError" => {
                let text = std::str::from_utf8(&bytes).expect("JSON text input");
                assert!(serde_json::from_slice::<serde_json::Value>(&bytes).is_err(), "independent JSON decoder refuses the same fixture");
                if route.starts_with("op") {
                    crate::__pack_json::from_json_str::<ProbeTransientMutation>(text, crate::__pack_json::JsonMemberPolicy::Reject).expect_err("typed mutation JSON cause").into_value_error()
                } else {
                    crate::__pack_json::from_json_str::<ProbeTransient>(text, crate::__pack_json::JsonMemberPolicy::Reject).expect_err("typed root JSON cause").into_value_error()
                }
            }
            "SemioError" => store::semio_format::unwrap_binary(&bytes).expect_err("typed envelope cause").into_value_error(),
            "ValueError" => ValueError::new(ValueRefusalKind::InvalidValue, "s.test.probe.windowtransient pack envelope mismatch"),
            other => panic!("unknown cause owner {other}"),
        };
        let actual = match route {
            "op-utf8" | "op-json" => {
                assert_eq!(case["protocolEnvelope"], true);
                match ProbeTransientMutation::decode_op(&bytes).expect_err("rejected mutation") {
                    protocol::ProtocolError::Pack(PackError::Refusal(refusal)) => refusal.into_value_error(),
                    error => panic!("mutation lost its semantic protocol cause: {error:?}"),
                }
            }
            "pack-utf8" | "pack-json" | "pack-identity" => {
                assert_eq!(case["protocolEnvelope"], false);
                let identity = if route == "pack-identity" { "s.test.other.windowtransient" } else { "s.test.probe.windowtransient" };
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(identity, store::semio_format::Component::Pack, 1).expect("valid test envelope");
                let encoded = store::semio_format::wrap_binary(&envelope, &bytes);
                let refusal: PackRefusal = ProbeTransient::decode_pack(&encoded).expect_err("rejected pure artifact pack");
                refusal.into_value_error()
            }
            "pack-header" => {
                let refusal: PackRefusal = ProbeTransient::decode_pack(&bytes).expect_err("rejected pure header");
                refusal.into_value_error()
            }
            "op-text-json" => {
                let error = ProbeTransientMutation::parse_op(std::str::from_utf8(&bytes).expect("text input")).expect_err("rejected mutation text");
                ValueError::new(error.kind, error.message)
            }
            "dsl-json" => {
                let error = ProbeTransient::parse_dsl(std::str::from_utf8(&bytes).expect("text input")).expect_err("rejected root text");
                ValueError::new(error.kind, error.message)
            }
            other => panic!("unknown codec route {other}"),
        };
        assert_eq!(actual.kind, expected.kind, "{} kind", case["id"]);
        assert_eq!(actual.message, expected.message, "{} owned display", case["id"]);
    }
    eprintln!("[DEBUG] transient typed codec law completed {} language-neutral cause vectors", cases.len());
}
