
/// 🧮️ A duplicated member in the caller's body resolves to its last value, exactly as the reference JSON codec resolves it, and what the host writes parses to the same document.
#[test]
fn a_duplicated_body_member_keeps_its_last_value_like_the_reference_codec() {
    let declared = bound_inference_row(false, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let payload = br#"{"seed":1,"seed":2,"nested":{"a":[1,2.5,"x",null,true]}}"#;
    let bound = bind_inference_document(&declared, &bound_inference_command(None, payload)).expect("a body without the bound field passes");
    let ours: serde_json::Value = serde_json::from_slice(&bound).expect("the bound body is JSON");
    let reference: serde_json::Value = serde_json::from_slice(payload).expect("the reference codec reads the same body");
    assert_eq!(ours, reference);
    assert_eq!(ours["seed"], serde_json::json!(2));
}

/// 🛑️ A body that is not a JSON object is refused by name instead of being bound.
#[test]
fn a_body_that_is_not_a_json_object_is_refused_by_name() {
    let declared = bound_inference_row(false, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    for payload in [&b"[1,2]"[..], &b"7"[..], &b"\"text\""[..]] {
        let fault = bind_inference_document(&declared, &bound_inference_command(None, payload)).expect_err("a non-object body is refused");
        assert_eq!(fault.code, "app.command.rejected");
        assert!(fault.message.contains("is not a JSON object"), "{}", fault.message);
    }
}

/// 🔁️ The inference wire the owned codec writes and reads is the wire the reference codec writes and reads: the request, the roster rows and the result decode to the same documents.
#[test]
fn the_owned_codec_speaks_the_inference_wire_like_the_reference_codec() {
    use semio_framework_os_mcp_schema::schema::{ArtifactInferenceBudgetV1, ArtifactInferenceCacheModeV1, ArtifactInferenceRequestV1, ArtifactInferenceResultV1};
    let request = ArtifactInferenceRequestV1 {
        wire_version: 2,
        owner: "wfc".into(),
        artifact_kind: "s.wfc.bitmap".into(),
        artifact_schema: "s.wfc.bitmap".into(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.bitmap.solve".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        revision: 9,
        generation: 3,
        source_dialect: "s.wfc.bitmap".into(),
        policy: vec![1, 2, 3],
        budgets: ArtifactInferenceBudgetV1 { allocation_bytes: 1 << 20, work_units: 77, recursion_depth: 4 },
        retained: semio_framework_value::RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 128, maximum_release_bytes: 256, maximum_depth: 5 },
        cancellation_id: "cancel-1".into(),
        previous_state: None,
        requested_cache_mode: ArtifactInferenceCacheModeV1::Cold,
        canonical_payload: br#"{"seed":7}"#.to_vec(),
        dependencies: vec![("s.wfc.bitmap.prepare".into(), vec![9, 8])],
    };
    let ours: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&request)).expect("the owned codec writes JSON");
    assert_eq!(ours, serde_json::to_value(&request).expect("the reference codec writes JSON"));
    let rows = vec![bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64), { let mut plain = bound_inference_row(false, "artifact-pack-base64"); plain.payload = None; plain.depends_on = vec!["s.wfc.bitmap.prepare".into()]; plain }];
    let roster: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&rows)).expect("the owned codec writes the roster");
    assert_eq!(roster, serde_json::to_value(&rows).expect("the reference codec writes the roster"));
    let result_text = r#"{"wireVersion":2,"owner":"wfc","artifactKind":"s.wfc.bitmap","inferenceSchema":"s.wfc.bitmap.solve","revision":9,"generation":3,"complete":true,"actualCacheMode":"incremental","canonicalPayload":[1,2],"diagnostics":["ignored"]}"#;
    let decoded: ArtifactInferenceResultV1 = semio_framework_pack_json::from_json_str(result_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the owned codec reads the result");
    assert_eq!(decoded, serde_json::from_str::<ArtifactInferenceResultV1>(result_text).expect("the reference codec reads the result"));
    let absent: ArtifactInferenceResultV1 = semio_framework_pack_json::from_json_str(r#"{"wireVersion":2,"owner":"o","artifactKind":"k","inferenceSchema":"i","revision":1,"generation":1,"complete":false,"actualCacheMode":"cold"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("an absent payload reads as none");
    assert_eq!(absent.canonical_payload, None);
}
