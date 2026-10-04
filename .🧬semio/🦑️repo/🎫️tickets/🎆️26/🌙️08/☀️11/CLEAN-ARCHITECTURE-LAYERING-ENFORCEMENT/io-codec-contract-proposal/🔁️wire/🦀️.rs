#[semio_framework_async_macros::async_test]
async fn canonical_compose_confidence_wire_preserves_none_and_every_existing_variant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️codec/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["confidence"].as_array().unwrap() {
        let confidence = match row["value"].as_str().unwrap() { "None" => Confidence::None, "Low" => Confidence::Low, "Medium" => Confidence::Medium, "High" => Confidence::High, _ => panic!("closed confidence") };
        let wire = WireComposedArtifact { dialect: ArtifactDialect { artifact_kind: "test.codec-confidence.wire".into(), standard: "1".into(), subset: "*".into() }, payload: IoPayload::Text("preserved confidence payload".into()), diagnostics: Vec::new(), confidence };
        let bytes = encode_wire_json("composed-artifact", &wire).await.unwrap();
        let independent: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(independent["confidence"], row["value"]);
        let result = wire_decode_composed_artifact(&bytes).await.unwrap();
        assert_eq!(result.confidence, confidence);
        assert_eq!(result.payload, wire.payload);
        assert_eq!(result.diagnostics, wire.diagnostics);
        assert_eq!(ArtifactDialect::from(result.dialect), wire.dialect);
    }
    println!("[DEBUG] actual compose wire confidence preserves None/Low/Medium/High plus dialect/payload/diagnostics; independent serde_json equality");
}
