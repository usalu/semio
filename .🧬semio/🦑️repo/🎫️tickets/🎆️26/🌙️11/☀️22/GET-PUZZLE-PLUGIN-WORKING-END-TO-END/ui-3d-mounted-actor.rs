use semio_os_kernel::os_store::backbone::{ActorBackboneChannelOwner, BackboneMessage, Backbones, PortBackbone, decode_hot_backbone_message_exact};
use semio_plugin::artifact_app_laws::{OriginalFixtureCaller, OriginalFixtureScope, close_registered_fixture_app, dispatch_fixture_action, fixture_identity, fixture_mounted_policy, fixture_renew_turn, meta, new_app_with_registry, settle_registered_typed_operation};
use semio_plugin::{App, EditorApp, PluginApp, ViewModel, ViewWindowInstance};
use semio_s_artifact_puzzle_3d_editor::{Puzzle3dPlayApp, create_puzzle3d_app};
use std::sync::Arc;

#[semio_framework_async_macros::async_test]
async fn mounted_puzzle_authors_and_drains_an_ordinary_actor_message() {
    let mut scope = OriginalFixtureScope::admit();
    let policy = fixture_mounted_policy();
    let mut app = Box::new(new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(
        || App { definition: create_puzzle3d_app(), examples: Vec::new() },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut fixture_identity(),
    ).await);
    app.bind_instance_id(1).await;
    let uri = "actor://puzzle3d-original-wire";
    let (channel, host) = ActorBackboneChannelOwner::pair(uri);
    app.attach_hot_backbone(Backbones::Port(PortBackbone::with_channel(uri, channel).await)).await.unwrap();
    let mut action_meta = meta("local");
    action_meta.view_state = Some(Arc::new(ViewModel {
        window_id: Some("puzzle3d-main".into()),
        window_instances: vec![ViewWindowInstance { id: "puzzle3d-main".into(), window_kind_id: "puzzle3d-main".into() }],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    }));
    assert!(!app.original_atomic_outbound_pending());
    let (action, args) = if std::env::var_os("PUZZLE_NATIVE_ACTOR_TARGET_VOLUME").is_some() {
        ("addTargetVolume", serde_json::json!({"origin": [1.0, 2.0, 3.0]}))
    } else {
        ("addObjectKind", serde_json::json!({"objectKind": "Object", "origin": [1.0, 2.0, 3.0]}))
    };
    dispatch_fixture_action(&mut *app, action, Some(args.into()), &action_meta, &mut fixture_identity(), scope.cx(), policy.close).await.unwrap();
    settle_registered_typed_operation(&mut *app, action_meta.instance_id, policy, &mut fixture_identity(), scope.cx()).await.unwrap();
    assert!(app.original_atomic_outbound_pending(), "the original edit must retain its document message");
    let mut delivery = None;
    let mut frames = Vec::new();
    for _ in 0..4096 {
        while let Some(frame) = host.take_outbound().unwrap() { frames.push(frame); }
        if delivery.is_none() && !app.original_atomic_outbound_pending() { break; }
        let cx = scope.cx();
        let grant = cx.original_turn_ceiling().0;
        fixture_renew_turn(cx, OriginalFixtureCaller::turn_fuel(), grant).unwrap();
        app.advance_original_atomic_outbound(&mut delivery, [1; 32], cx).unwrap();
    }
    assert!(delivery.is_none() && !app.original_atomic_outbound_pending(), "the exact original wire owner must settle");
    assert_eq!(frames.len(), 1);
    let BackboneMessage::Mutations { envelopes } = decode_hot_backbone_message_exact(&frames[0]).unwrap() else { panic!("the actor must receive its causal operations") };
    let operations = semio_os_kernel::os_spr::decode_envelopes(&envelopes).unwrap();
    assert_eq!(operations.len(), 1);
    assert!(!semio_os_kernel::os_spr::is_history_transition(&operations[0]));
    host.begin_retire().unwrap();
    app.detach_backbone().await.unwrap();
    close_registered_fixture_app(&mut *app, policy, scope.cx()).unwrap();
}
