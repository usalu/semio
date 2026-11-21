use hub_causal::binary::atomic_hub::{client_atomic_part, decode_checkpoint_replay, encode_checkpoint_replay, AtomicSocketEdit, AtomicWalEdit, CheckpointReplayGroup};

fn main() {
    let source = include_bytes!("🗑️generated/large-picked/large-picked.png");
    let packets: &[&[u8]] = &[
        include_bytes!("🗑️generated/raw-hub-atomic/0"), include_bytes!("🗑️generated/raw-hub-atomic/1"),
        include_bytes!("🗑️generated/raw-hub-atomic/2"), include_bytes!("🗑️generated/raw-hub-atomic/3"),
        include_bytes!("🗑️generated/raw-hub-atomic/4"), include_bytes!("🗑️generated/raw-hub-atomic/5"),
        include_bytes!("🗑️generated/raw-hub-atomic/6"), include_bytes!("🗑️generated/raw-hub-atomic/7"),
    ];
    let mut inbound = AtomicSocketEdit::default();
    assert!(client_atomic_part(&[0, 1]).unwrap().is_none());
    assert!(client_atomic_part(&[0, 8, 0]).is_err());
    let (id, part) = client_atomic_part(packets[1]).unwrap().unwrap();
    assert!(inbound.push(id, part, "fixture-document", "fixture-actor").is_err());
    let mut invariant = None;
    for packet in packets {
        let (id, part) = client_atomic_part(packet).unwrap().unwrap();
        assert_eq!(id, 82);
        invariant = inbound.push(id, part, "fixture-document", "fixture-actor").unwrap();
        assert_eq!(invariant.is_some(), packet == packets.last().unwrap());
    }
    let edit = invariant.unwrap();
    assert_eq!(edit.edit_id(), "edit-png-82");
    assert_eq!(edit.events().len(), 15);
    assert!(edit.events().iter().flat_map(|row| row.diff.payload.iter()).eq(source.iter().copied()));
    let descriptor = include_bytes!("🗑️generated/raw-hub-atomic/descriptor");
    let stored = AtomicWalEdit::decode(descriptor).unwrap().unwrap();
    assert_eq!(stored.encode().unwrap(), descriptor);
    assert!(stored.admits(edit.events()));
    assert!(!stored.admits(&edit.events()[1..]));
    let replay = stored.relay(edit.events(), b"frontier").unwrap();
    for (i, (found, input)) in replay.iter().zip(packets).enumerate() {
        let metadata: &[u8] = if i == 0 { b"frontier" } else { b"" };
        assert_eq!(&found[..4], &[0, 16, metadata.len() as u8, 0]);
        assert_eq!(&found[4..4 + metadata.len()], metadata);
        assert_eq!(&found[4 + metadata.len()..], &input[10..]);
    }
    assert!(AtomicWalEdit::decode(&descriptor[..descriptor.len() - 1]).is_err());
    assert!(AtomicWalEdit::decode(b"ordinary").unwrap().is_none());
    let independent = include_bytes!("🗑️generated/raw-hub-atomic/checkpoint");
    assert_eq!(encode_checkpoint_replay([(Some(&stored), edit.events())]).unwrap(), independent);
    let mut ordinary = edit.events()[0].clone();
    ordinary.transaction = None;
    let ordinary = [ordinary];
    let mixed = encode_checkpoint_replay([(None, ordinary.as_slice()), (Some(&stored), edit.events())]).unwrap();
    let commits = decode_checkpoint_replay(&mixed, "fixture-document").unwrap().unwrap();
    assert!(matches!(&commits[..], [CheckpointReplayGroup::Ordinary(rows), CheckpointReplayGroup::Original(group)]
        if rows == &ordinary && group.edit_id() == "edit-png-82" && group.events() == edit.events()));
    assert!(decode_checkpoint_replay(&mixed, "other-document").is_err());
    assert!(decode_checkpoint_replay(&mixed[..mixed.len() - 1], "fixture-document").is_err());
    assert!(encode_checkpoint_replay([(None, edit.events())]).is_err());
    assert!(decode_checkpoint_replay(b"ordinary", "fixture-document").unwrap().is_none());

    let mut wrong_author = AtomicSocketEdit::default();
    let mut replaced_scope = AtomicSocketEdit::default();
    let mut tampered = AtomicSocketEdit::default();
    let mut wrong_batch = AtomicSocketEdit::default();
    for (i, packet) in packets.iter().enumerate() {
        let (id, part) = client_atomic_part(packet).unwrap().unwrap();
        let mut altered = part.to_vec();
        if i == 0 { *altered.last_mut().unwrap() ^= 1; }
        let tail = i + 1 == packets.len();
        assert_eq!(wrong_author.push(id, part, "fixture-document", "other-actor").is_err(), tail);
        assert_eq!(tampered.push(id, &altered, "fixture-document", "fixture-actor").is_err(), tail);
        if i < 2 {
            assert_eq!(wrong_batch.push(id + i as u64, part, "fixture-document", "fixture-actor").is_err(), i == 1);
            let expected = if i == 0 { "fixture-actor" } else { "another-actor" };
            assert_eq!(replaced_scope.push(id, part, "fixture-document", expected).is_err(), i == 1);
        }
    }
    println!("[DEBUG] Authenticated Hub exact socket framing and WAL descriptor passed for 473683 source octets");
}
