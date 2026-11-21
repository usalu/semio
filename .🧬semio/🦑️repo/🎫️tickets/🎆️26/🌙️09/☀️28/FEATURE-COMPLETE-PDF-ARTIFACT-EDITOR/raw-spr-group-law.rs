//! 🧪 Drives the actual atomic SPR decoder over funded parts of a large fixture.

use semio_framework_replication_causal::{ArtifactDiff, CausalPayload, InverseMutation, MutationEnvelope};
use semio_framework_replication_causal::binary::{
    atomic_group::{ATOMIC_BACKBONE_TOOL, ATOMIC_EDIT_PREFIX_BYTES, AtomicBackboneKey, AtomicBackbonePart, AtomicBackboneSections, AtomicBackboneSender, AtomicPartStatus},
    decode_document_backbone_envelopes_exact, decode_verified_atomic_backbone, encode_envelopes,
};
use semio_framework_replication_edit::TransactionRef;
use semio_framework_replication_ids::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId};
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};

const PNG: &[u8] = include_bytes!("🗑️generated/large-picked/large-picked.png");
const HEADER_FIXTURE: &[u8] = include_bytes!("🗑️generated/raw-spr-group/header");
const GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 16, maximum_copy_bytes: 4096,
    maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 4 << 20, maximum_copy_bytes: 8 << 20,
    maximum_capacity_bytes: 8 << 20, maximum_release_bytes: 8 << 20, ..GRANT };

fn mutation(index: usize, bytes: &[u8]) -> MutationEnvelope {
    let schema = SchemaId("s.stdio.pdf.1.7".into());
    MutationEnvelope { mutation_id: MutationId(format!("source-{index}")), document_id: ArtifactId("pdf".into()),
        actor: ActorId("local".into()), dependencies: (index > 0).then(|| MutationId(format!("source-{}", index - 1))).into_iter().collect(),
        observed: None, target: vec![format!("png-{index}")],
        diff: ArtifactDiff { schema: schema.clone(), payload: CausalPayload::try_from_slice(bytes).unwrap() },
        inverse: InverseMutation { schema, payload: CausalPayload::default() },
        timestamp: HybridLogicalTimestamp::new(1, 2), transaction: Some(TransactionRef { id: "raw-pick".into(), tool: format!("s.stdio.pdf#{ATOMIC_BACKBONE_TOOL}") }),
        verb: Some("insert".into()), line: None }
}

fn close(wallet: &mut AuthoringWallet, mut retained: AtomicBackboneSections, extent: usize) {
    let mut release = 0;
    let turn = RetainedCloneGrant { maximum_items: 1 << 16, maximum_copy_bytes: 64 * 1024,
        maximum_capacity_bytes: 128 * 1024, maximum_release_bytes: 128 * 1024, ..GRANT };
    for _ in 0..4096 {
        if retained.terminal_is_empty() { break; }
        let (closed, _) = wallet.run_settled(turn, 1 << 16, |effect| retained.close(4096, effect)).unwrap().unwrap();
        release += closed.result.unwrap().progress().released_bytes;
    }
    assert!(retained.terminal_is_empty(), "returned {release} of {extent} initialized source bytes");
    assert!(release >= extent);
}

#[test]
fn registered_causal_codec_releases_only_an_exact_funded_large_group() {
    let fixture = AtomicBackbonePart { key: AtomicBackboneKey { digest: [0x17; 32], extent: 473_683 }, ordinal: 7 };
    assert_eq!(fixture.encode_header().unwrap().as_slice(), HEADER_FIXTURE);
    let mutations: Vec<_> = PNG.chunks(32 * 1024).enumerate().map(|(index, payload)| mutation(index, payload)).collect();
    let bytes = encode_envelopes(&mutations);
    assert!(decode_document_backbone_envelopes_exact(bytes.as_slice()).is_err());
    let mut prefix = [0; ATOMIC_EDIT_PREFIX_BYTES];
    let end = AtomicBackboneSender::edit_prefix("png-history", &mut prefix).unwrap();
    assert_eq!(&prefix[..end], b"\x80\x00G\x00\x0bpng-history");
    let framed: Vec<_> = prefix[..end].iter().chain(bytes.iter()).copied().collect();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 25);
    let mut hash = semio_framework_hash::Sha256::new();
    hash.update(&framed);
    let key = AtomicBackboneKey { digest: hash.finalize(), extent: framed.len() as u32 };
    let mut sender = AtomicBackboneSender::begin(&framed).unwrap();
    let mut frames = Vec::new();
    let sending = RetainedCloneGrant { maximum_copy_bytes: 128 * 1024, maximum_capacity_bytes: 128 * 1024, ..GRANT };
    for _ in 0..1024 {
        if sender.terminal_is_empty() { break; }
        let mut frame = None;
        wallet.run(sending, 1 << 17, |effect| sender.advance(&framed, &mut frame, effect)).unwrap().unwrap().output.unwrap();
        if let Some(frame) = frame { frames.push(frame); }
    }
    assert!(sender.terminal_is_empty());
    assert_eq!(frames.len(), key.count());
    let mut received = AtomicBackboneSections::new(key).unwrap();
    let mut altered = AtomicBackboneSections::new(key).unwrap();
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..GRANT };
    for (index, frame) in frames.iter().enumerate().rev() {
        assert!(received.complete().is_none());
        let offered = AtomicBackbonePart { key, ordinal: index as u16 };
        let (parsed, _) = AtomicBackbonePart::parse(frame).unwrap();
        assert_eq!(parsed.key, offered.key);
        assert!(AtomicBackbonePart::parse(&frame[..frame.len() - 1]).is_err());
        let mut part = Some(CausalPayload::try_from_slice(frame).unwrap());
        let mut wrong = parsed;
        wrong.key.digest[31] ^= 1;
        assert!(wallet.run(GRANT, 16, |effect| received.admit(wrong, &mut part, effect)).unwrap().unwrap().output.is_err());
        assert!(part.is_some());
        let (blocked, _) = wallet.run(denied, 16, |effect| received.admit(parsed, &mut part, effect)).unwrap().unwrap().output.unwrap();
        assert_eq!(blocked, AtomicPartStatus::Pending);
        assert!(part.is_some());
        let (status, _) = wallet.run(GRANT, 16, |effect| received.admit(parsed, &mut part, effect)).unwrap().unwrap().output.unwrap();
        assert_eq!(status, if index == 0 { AtomicPartStatus::Complete } else { AtomicPartStatus::Accepted });
        assert!(part.is_none());
        let mut bytes = frame.clone();
        if index == 0 { bytes[42] ^= 1; }
        let mut replaced = Some(CausalPayload::try_from_slice(&bytes).unwrap());
        assert!(wallet.run(GRANT, 16, |effect| received.admit(parsed, &mut replaced, effect)).unwrap().unwrap().output.is_err());
        assert!(replaced.is_some());
        wallet.run(GRANT, 16, |effect| altered.admit(parsed, &mut replaced, effect)).unwrap().unwrap().output.unwrap();
    }
    let mut digest = received.complete().unwrap();
    assert!(wallet.run(GRANT, 4096, |effect| digest.advance(&altered, 4096, effect)).unwrap().unwrap().output.is_err());
    let sealed = loop {
        let (sealed, _) = wallet.run(GRANT, 4096, |effect| digest.advance(&received, 4096, effect)).unwrap().unwrap().output.unwrap();
        if let Some(sealed) = sealed { break sealed; }
    };
    assert!(decode_verified_atomic_backbone(&sealed, "foreign", "local").is_err());
    assert!(decode_verified_atomic_backbone(&sealed, "pdf", "foreign").is_err());
    let restored = decode_verified_atomic_backbone(&sealed, "pdf", "local").unwrap();
    assert_eq!(restored.edit_id(), "png-history");
    assert_eq!(encode_envelopes(restored.events()), bytes);
    assert_eq!(restored.events().len(), mutations.len());
    let mut digest = altered.complete().unwrap();
    for turn in 0..framed.len().div_ceil(4096) {
        let result = wallet.run(GRANT, 4096, |effect| digest.advance(&altered, 4096, effect)).unwrap().unwrap().output;
        if turn + 1 == framed.len().div_ceil(4096) { assert!(result.is_err()); }
        else { assert!(result.unwrap().0.is_none()); }
    }
    close(&mut wallet, received, framed.len());
    close(&mut wallet, altered, framed.len());
    println!("[DEBUG] Replayed the intact 15-row atomic SPR group for {} original PNG bytes", PNG.len());
}
