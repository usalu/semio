/// 🔗️ Inference artifact binding: the bound inference rows and commands and the binding field, encoding and contract laws.

//#region 🔗️InferenceArtifactBinding

/// 🧾️ One declared inference row carrying a published contract, for the binding laws below. The
/// shape is exactly what a re-described `🀄️wfc` commits under
/// `contributions.inferenceServices[].payload`.
fn bound_inference_row(required: bool, encoding: &str) -> semio_framework::ContributedInferenceMetadata {
    semio_framework::ContributedInferenceMetadata {
        owner: "wfc".into(),
        artifact_kind: "s.wfc.bitmap".into(),
        artifact_schema: "s.wfc.bitmap".into(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.bitmap.solve".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: "wfc".into(),
        depends_on: Vec::new(),
        payload: Some(semio_framework::InferencePayloadContract {
            payload_schema_id: "s.wfc.bitmap.inference.request.v1".into(),
            input_schema: "{\"type\":\"object\"}".into(),
            output_schema: "{\"type\":\"object\"}".into(),
            progress_unit: "cells".into(),
            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".into(), encoding: encoding.into(), required }),
            commit: None,
        }),
    }
}

fn bound_inference_command(document: Option<ArtifactDocumentBinding>, payload: &[u8]) -> InferCommand {
    InferCommand { plugin_id: "wfc".into(), artifact_kind: "s.wfc.bitmap".into(), inference_schema: "s.wfc.bitmap.solve".into(), turn:semio_framework_actor::RetainedTurnInput{operation:71,generation:3,epoch:19,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:1671,absolute_deadline_us:1000000,fuel:64},grant:semio_framework_value::RetainedCloneGrant{maximum_items:7,maximum_copy_bytes:3,maximum_capacity_bytes:129,maximum_release_bytes:4096,maximum_depth:2}},retained_policy:semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:129,maximum_release_bytes:4096,maximum_depth:2},revision:7,cancellation_id:"original binding cancellation".into(),work_units:11,maximum_elapsed_milliseconds:13, canonical_payload: payload.to_vec(),artifact_id:String::new(), artifact_document: document,cancel:InferenceCancel::default() }
}

/// 🔗️ The artifact the caller named lands under the field the PLUGIN declared, base64, and nowhere
/// else — the fix for the gap `📓️pz2-puzzle-describe-under-budget.md` §5.2 named (`InferCommand`
/// carried no artifact binding at all, so the guest had no document to read a snapshot from).
///
/// 🔡️ …and what the host wrote is what a guest decodes back, byte for byte — the two halves of
///    `artifact-pack-base64` meet here and nowhere else.
#[test]
fn a_named_artifact_is_bound_under_the_field_the_contract_declares() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(ArtifactDocumentBinding { pack: b"PACK-BYTES".to_vec(), spr: b"SPR".to_vec() }), b"{}");
    let bound: serde_json::Value = serde_json::from_slice(&bind_inference_document(&declared, &command).expect("a named artifact binds")).expect("the bound body is JSON");
    assert_eq!(bound["document"]["pack"], serde_json::Value::String(encode_base64(b"PACK-BYTES")));
    assert_eq!(bound["document"]["spr"], serde_json::Value::String(encode_base64(b"SPR")));
    assert_eq!(decode_base64(bound["document"]["pack"].as_str().expect("a base64 string")), Some(b"PACK-BYTES".to_vec()));
}

/// ✍️ A caller who authored the bound field keeps it: the binding fills a GAP, it never overwrites a
/// body the caller wrote.
#[test]
fn a_caller_authored_binding_field_is_never_overwritten() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(ArtifactDocumentBinding { pack: b"HOST".to_vec(), spr: b"HOST".to_vec() }), br#"{"document":{"pack":"mine","spr":"mine"}}"#);
    let bound: serde_json::Value = serde_json::from_slice(&bind_inference_document(&declared, &command).expect("the caller's body survives")).expect("JSON");
    assert_eq!(bound["document"]["pack"], serde_json::Value::String("mine".to_string()));
}

/// 🚧️ A REQUIRED binding with no artifact named never reaches a guest: the refusal says which tool
/// argument fixes it, in milliseconds, instead of a 240 s dispatch that ends in a decode fault.
#[test]
fn a_required_binding_without_an_artifact_refuses_by_name() {
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let fault = bind_inference_document(&declared, &bound_inference_command(None, b"{}")).expect_err("a required binding with no artifact is refused");
    assert!(fault.message.contains("artifactId") && fault.message.contains("payload.document"), "{}", fault.message);
}

/// 🚧️ An encoding this gateway has no writer for is named, never silently written in the one shape
/// this host happens to know.
#[test]
fn an_unknown_binding_encoding_is_refused_rather_than_guessed() {
    let declared = bound_inference_row(true, "artifact-dsl-text");
    let fault = bind_inference_document(&declared, &bound_inference_command(Some(ArtifactDocumentBinding::default()), b"{}")).expect_err("an unwritable encoding is refused");
    assert!(fault.message.contains("artifact-dsl-text") && fault.message.contains(semio_framework::INFERENCE_ARTIFACT_PACK_BASE64), "{}", fault.message);
}

/// 🫙 An inference that publishes NO contract keeps the host-opaque behaviour it always had: the
/// caller's own body travels verbatim.
#[test]
fn an_inference_without_a_published_contract_passes_its_payload_through() {
    let mut declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    declared.payload = None;
    let bound = bind_inference_document(&declared, &bound_inference_command(None, br#"{"seed":7}"#)).expect("no contract, no binding");
    assert_eq!(bound, br#"{"seed":7}"#.to_vec());
}

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

//#endregion 🔗️InferenceArtifactBinding
