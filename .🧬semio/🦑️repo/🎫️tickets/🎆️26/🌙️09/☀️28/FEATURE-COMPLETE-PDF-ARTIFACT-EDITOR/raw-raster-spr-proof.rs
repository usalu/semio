//! 🧮 Proves the exact existing SPR limit for canonical PDF sampled-image payloads.

use semio_os_spr::{
    ActorId, ArtifactDiff, ArtifactId, CausalPayload, DocumentBackboneBatchLimitsV1, HybridLogicalTimestamp, InverseMutation,
    MutationEnvelope, MutationId, SchemaId, decode_document_backbone_envelopes_exact,
    decode_document_backbone_envelopes_exact_with_limits, encode_envelopes,
};

fn envelope(bytes: &[u8]) -> MutationEnvelope {
    let schema = SchemaId("stdio.pdf.v1.7.base".to_owned());
    MutationEnvelope {
        mutation_id: MutationId("set-image".to_owned()),
        document_id: ArtifactId("pdf-raster-replay".to_owned()),
        actor: ActorId("local".into()),
        dependencies: vec![],
        observed: None,
        target: vec!["image-1".to_owned()],
        diff: ArtifactDiff { schema: schema.clone(), payload: CausalPayload::try_from_slice(bytes).unwrap() },
        inverse: InverseMutation { schema, payload: CausalPayload::default() },
        timestamp: HybridLogicalTimestamp::new(1, 2),
        transaction: None,
        verb: Some("set".to_owned()),
        line: None,
    }
}

#[test]
fn live_backbone_round_trips_small_sample_payload_and_refuses_large() {
    let small = include_bytes!("🗑️generated/raw-raster-spr-proof/small.json");
    let large = include_bytes!("🗑️generated/raw-raster-spr-proof/large.json");
    assert!(small.len() < semio_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES);
    assert!(large.len() > semio_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES);

    let operation = envelope(small);
    let encoded = encode_envelopes(std::slice::from_ref(&operation));
    let restored = decode_document_backbone_envelopes_exact(encoded.as_slice()).unwrap();
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].diff, operation.diff);
    assert_eq!(encode_envelopes(&restored), encoded);

    let too_large = encode_envelopes(&[envelope(large)]);
    let refusal = decode_document_backbone_envelopes_exact(too_large.as_slice()).unwrap_err();
    assert!(matches!(refusal, semio_os_spr::ProtocolError::LimitExceeded("batch-bytes")));
    let smaller_payload = DocumentBackboneBatchLimitsV1 { maximum_payload_bytes: small.len() - 1, ..Default::default() };
    let payload_refusal = decode_document_backbone_envelopes_exact_with_limits(encoded.as_slice(), smaller_payload).unwrap_err();
    assert!(matches!(payload_refusal, semio_os_spr::ProtocolError::LimitExceeded("payload-bytes")));
    println!("[DEBUG] small PDF color sample array {} bytes; large {} bytes; exact SPR refusals: {refusal:?}, {payload_refusal:?}",
        small.len(), large.len());
}
