use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};
use semio_os_store_backbone::{
    atomic_transport::AtomicBackboneDelivery, ActorBackboneChannelOwner, BackboneMessage,
    Backbones, PortBackbone, decode_hot_backbone_message_exact,
};

#[test]
fn paged_history_wire_reaches_the_real_actor() {
    let law: serde_json::Value = serde_json::from_str(include_str!(env!("SEMIO_UI_ACTOR_TRANSFER_FIXTURE"))).unwrap();
    let pattern = law["pattern"].as_array().unwrap();
    let bytes: Vec<_> = (0..law["bytes"].as_u64().unwrap() as usize)
        .map(|index| pattern[index % pattern.len()].as_u64().unwrap() as u8).collect();
    let body = semio_os_spr::CausalPayload::try_from_slice(&bytes).unwrap();
    let (channel, host) = ActorBackboneChannelOwner::pair("actor://original-history");
    let port = Backbones::Port(PortBackbone { uri: "actor://original-history".into(), channel: Some(channel) });
    let grant = RetainedCloneGrant {
        maximum_items: 1, maximum_copy_bytes: 128, maximum_capacity_bytes: bytes.len() + 16,
        maximum_release_bytes: 0, maximum_depth: 1,
    };
    let mut cancelled = AtomicBackboneDelivery::begin_actor_message([1; 32]);
    AuthoringWallet::new(grant, 1024).run(grant, 64, |effect| cancelled.advance_ordinary_wire(&body, &port, effect))
        .unwrap().unwrap().output.unwrap();
    assert!(!cancelled.terminal_is_empty());
    let close = RetainedCloneGrant { maximum_release_bytes: cancelled.close_demand().release_bytes, ..grant };
    AuthoringWallet::new(close, 1024).run(close, 64, |effect| cancelled.close(effect)).unwrap().unwrap().output.unwrap();
    assert!(cancelled.terminal_is_empty());
    assert!(host.take_outbound().unwrap().is_none());
    let mut delivery = AtomicBackboneDelivery::begin_actor_message([1; 32]);
    for _ in 0..bytes.len() + 4 {
        if delivery.actor_message_sent() { break; }
        assert!(host.take_outbound().unwrap().is_none());
        let performed = AuthoringWallet::new(grant, 1024).run(grant, 64, |effect| delivery.advance_ordinary_wire(&body, &port, effect)).unwrap().unwrap();
        assert!(performed.output.unwrap().fits(grant));
    }
    assert!(delivery.actor_message_sent());
    assert!(delivery.terminal_is_empty());
    let wire = host.take_outbound().unwrap().unwrap();
    let prefix = law["actorWirePrefix"].as_str().unwrap();
    assert_eq!(wire[..prefix.len() / 2].iter().map(|byte| format!("{byte:02x}")).collect::<String>(), prefix);
    let BackboneMessage::Mutations { envelopes } = decode_hot_backbone_message_exact(&wire).unwrap() else { panic!("canonical history mutation") };
    assert_eq!(envelopes.to_vec_owner(), bytes);
    assert_eq!(body.to_vec_owner(), bytes);
    assert!(host.take_outbound().unwrap().is_none());
    host.begin_retire().unwrap();
}
