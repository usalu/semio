#![allow(unused)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️prelude/🦀️.rs"));
mod context {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️context/🦀️.rs"));
}
use context::*;
use semio_plugin::artifact_app_laws;

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_object_edits_via_backbone() {
    use semio_plugin::artifact_app_laws;
    use store::os_store::backbone::MemoryBackbone;
    let mut instance_a = app_for_actor("alice").await;
    let mut instance_b = app_for_actor("bob").await;
    let seeded = object_count(&instance_a);
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence").await;
    let queue_a=backbone_a.outbox.as_ref().unwrap().clone();
    let queue_b=backbone_b.outbox.as_ref().unwrap().clone();
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("a adds object");
    dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("b adds object");

    let ids = |app: &Puzzle3dApp| app.snapshot().expect("scene").typed().objects.iter().map(|object| object.id.clone()).collect::<std::collections::BTreeSet<_>>();
    let expected = ids(&instance_a).union(&ids(&instance_b)).cloned().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(expected.len(), seeded + 2, "the two actors authored distinct objects");
    for (app,queue) in [(&mut instance_a,queue_a), (&mut instance_b,queue_b)] {
        let (raw, cx) = app.parts();
        let mut delivery = None;
        for _ in 0..4096 {
            if !raw.original_atomic_outbound_pending() { break; }
            let grant = cx.original_turn_ceiling().0;
            artifact_app_laws::fixture_renew_turn(cx, artifact_app_laws::OriginalFixtureCaller::turn_fuel(), grant).expect("renew outbound turn");
            let outbox=&mut raw.stores.store.wire.original_outbox;
            if outbox.front().is_some() {
                let Some((step,work))=cx.with_original_effect(grant, |_,effect| {
                    assert!(effect.performed(1).unwrap());
                    outbox.transfer_to_channel_step(&mut queue.lock().unwrap(),effect.grant()).unwrap()
                }).unwrap() else { continue; };
                cx.consume_work(work);
                if let Some((_,progress))=step {cx.consume_retained(progress).unwrap();}
            } else { raw.advance_original_atomic_outbound(&mut delivery,[1;32],cx).unwrap(); }
        }
        assert!(!raw.original_atomic_outbound_pending(), "the local queue takes the committed edit");
    }
    for app in [&mut instance_a, &mut instance_b] {
        let (raw, cx) = app.parts();
        let grant = cx.original_turn_ceiling().0;
        artifact_app_laws::fixture_renew_turn(cx, artifact_app_laws::OriginalFixtureCaller::turn_fuel(), grant).expect("renew inbound turn");
        raw.tick_backbone(&mut artifact_app_laws::fixture_identity(), cx).await.expect("receive the peer edit");
        assert_eq!(ids(app), expected);
    }
    assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
    assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
}

