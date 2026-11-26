use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};
use semio_os_store_backbone::{Backbones, MemoryBackbone, BackboneMessage, original_outbox::OriginalBackboneOutbox};
use semio_os_spr::CausalPayload;

#[test]
fn memory_document_outbox_donates_fixture_bytes_once() {
    let law: serde_json::Value = serde_json::from_str(include_str!(env!("SEMIO_UI_ACTOR_TRANSFER_FIXTURE"))).unwrap();
    let pattern = law["pattern"].as_array().unwrap();
    let bytes: Vec<_> = (0..law["bytes"].as_u64().unwrap() as usize).map(|i| pattern[i % pattern.len()].as_u64().unwrap() as u8).collect();
    let mut original = Some(CausalPayload::try_from_slice(&bytes).unwrap());
    let backing = original.as_ref().unwrap().retained_bytes().backing_ptr(0).unwrap();
    let queue = std::sync::Arc::new(std::sync::Mutex::new(std::collections::VecDeque::new()));
    let endpoint = Backbones::Memory(MemoryBackbone { uri: String::new(), inbox: None, outbox: Some(queue.clone()) });
    let mut source = OriginalBackboneOutbox::default();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 16 };
    while !source.can_append(grant).unwrap() {
        AuthoringWallet::new(grant, 1024).run(grant, 64, |effect| source.reserve_step(effect)).unwrap().unwrap().output.unwrap();
    }
    source.append_reserved(&mut original, grant).unwrap().unwrap();
    for _ in 0..3 {
        let receipt = AuthoringWallet::new(grant, 1024).run(grant, 64, |effect| endpoint.send_original(&mut source, effect)).unwrap().unwrap().output.unwrap();
        if let Some((true, paid)) = receipt { assert!(paid.fits(grant)); break; }
    }
    let Some(BackboneMessage::Mutations { envelopes }) = queue.lock().unwrap().pop_front() else { panic!("missing frame"); };
    assert_eq!(envelopes.retained_bytes().backing_ptr(0).unwrap(), backing);
    assert_eq!(envelopes.to_vec_owner(), bytes);
    assert_eq!(source.len(), 0);
    assert!(queue.lock().unwrap().is_empty());
    while !source.terminal_is_empty() {
        AuthoringWallet::new(grant, 1024).run(grant, 64, |effect| { source.recycle_step(effect).ok().unwrap(); }).unwrap().unwrap();
    }
}
