/// 📇️ Window action, archive import and hub genesis laws of the generation2d editor routes.

//#region 📇️WindowActionLawTests
/// 📇️ THE window-kind action law, generation2d's half (ticket 26/09/09/PROCEDURAL-3D-END-TO-END) —
/// the exact twin of generation3d's `every_emitted_action_is_declared_on_its_window_kind`, kept
/// assertion-for-assertion identical so the two apps cannot drift.
///
/// Every action id a rendered `UiNode` binding of a window emits must appear in that window kind's
/// `WindowKindDefinition.actions` — that list is what `ShellHost`'s `declaredAction` gate reads before
/// it will call `plugin.handleAction`
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5691`) —
/// and no window may declare a window-scoped action only some other window emits, which is what makes
/// `.window_kind_action_refs(...)` mean anything against `build_definition`'s unowned-action fallback
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`).
///
/// The two generations are dispatched, not hand-built: the tree only emits
/// `selectGeneration`/`renameGeneration`/`removeGeneration` and the form only emits
/// `updateGenerationValues` once a roster exists.
#[semio_framework_async_macros::async_test]
async fn every_emitted_action_is_declared_on_its_window_kind() {
    let definition = create_generation2d_app();
    let windows: Vec<(String, String, std::collections::BTreeSet<String>)> =
        definition.window_kinds.iter().map(|kind| (kind.id.clone(), kind.body_key.clone(), kind.actions.iter().map(|action| action.id.clone()).collect())).collect();
    assert_eq!(windows.len(), 5, "generation2d declares five window kinds");
    let mut app = app_with_registry().await;
    for _ in 0..2 {
        crate::app_fixture::dispatch(&mut app, Generation2dCommand::AddGeneration(add_generation::AddGeneration {})).await;
    }
    let mut emitted: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = Default::default();
    for (kind_id, body_key, _) in &windows {
        let projection = crate::app_fixture::render(&mut app, body_key).await;
        emitted.insert(kind_id.clone(), semio_framework_plugin::ui_action_laws::emitted_action_ids(&projection));
    }
    close(app);
    let window_scoped: std::collections::BTreeSet<String> = emitted.values().flatten().cloned().collect();
    for (kind_id, _, declared) in &windows {
        let emitted_here = &emitted[kind_id];
        println!("[STATS] window-actions kind={kind_id} declared={} emitted={} emits={emitted_here:?}", declared.len(), emitted_here.len());
        for action in emitted_here {
            assert!(declared.contains(action), "{kind_id} emits {action} but never declares it — ShellHost's declaredAction gate drops it");
        }
        for action in window_scoped.difference(emitted_here) {
            assert!(!declared.contains(action), "{kind_id} declares {action}, a window-scoped action only another window emits");
        }
    }
}
//#endregion 📇️WindowActionLawTests

//#region 🚪️ArchiveImportDoor
/// 🚪️ A framework document archive of this editor loads into a FRESH instance through the shell's Import Document door
/// (`createApp` → `loadDocumentArchive` → poll → acknowledge). Measured live 2026-09-28 (S20 io-matrix,
/// `s14-s20-io/s20b-local-en-2`): generation3d trapped the guest, generation2d faulted
/// `document-archive-replacement.initializer-failed`. Two faults: the initializer held no publication lease outside tests
/// (`generation2d-publication.authority-missing`) — it now grants itself the lease the host's replacement needs — and the kernel's
/// retained hydration plain-dropped every replayed diff, whose `host_snapshot.layout` `OrderedMap` aborts on drop — it now
/// retires them through `os_vcs::apply_mutation`. The source document is EDITED first so the archive carries history to
/// replay (ticket 26/09/23 slice S19).
#[semio_framework_async_macros::async_test]
async fn a_document_archive_loads_into_a_fresh_instance_through_the_import_door() {
    let _serial = semio_s_artifact_procedural_2d_testkit::publication_authority::lock();
    let mut source = app_with_registry().await;
    context::dispatch(&mut source, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, format: None, action: None, x: None, y: None })).await;
    let archive = PluginApp::document_archive(&mut source.0).await.expect("the open document exports as a framework archive");
    let expected: Vec<String> = context::snapshot_read(&source).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget).to_string()).collect();
    assert!(!expected.is_empty(), "the exported document carries widgets to compare");
    let mut target = app_with_registry().await;
    PluginApp::begin_document_archive_load(&mut target.0, 91, archive).expect("archive admission");
    let mut status = None;
    for _ in 0..200_000 {
        let polled = PluginApp::poll_document_archive_load(&mut target.0, 91, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), target.1.cx()).await.expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            PluginApp::acknowledge_document_archive_load(&mut target.0, 91).expect("archive acknowledgement");
            status = Some(polled);
            break;
        }
        PluginApp::maintenance_step(&mut target.0, target.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("archive maintenance step");
    }
    let status = status.expect("the archive load reaches a terminal state");
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    let loaded: Vec<String> = context::snapshot_read(&target).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget).to_string()).collect();
    assert_eq!(loaded, expected, "the fresh instance holds exactly the archived document");
    context::close(target);
    context::close(source);
}
//#endregion 🚪️ArchiveImportDoor

//#region 🌱️HubGenesis
/// 🌱️ The hub creates a `generation2d` document through this plugin's `codec.genesis` export — the SDK's one producer
/// `artifact_app_genesis_pair` — and the pair it prints must parse back to a live document. Measured on hub 7800
/// (2026-09-28, W4 open-plan probe): creation failed `ordered-map root must be explicitly retired before drop`, the
/// producer plain-dropped the initial snapshot whose `host_snapshot.layout` is an `OrderedMap` (ticket 26/09/23 slice S19).
#[semio_framework_async_macros::async_test]
async fn a_hub_genesis_pair_is_produced_and_parses_back_without_trapping() {
    let pair = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<Generation2dPlayApp>>("artifact-0123456789abcdef0123456789abcdef").await.expect("the genesis export produces a pair");
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "a genesis pair carries both a pack and an SPR");
    let snapshot = semio_framework_plugin::artifact_pair_snapshot::<semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot, semio_s_artifact_procedural_2d_mutations::Generation2dMutation>(&pair.pack, &pair.spr)
        .await
        .expect("the genesis pair parses back");
    let initial = <Generation2dPlayApp as semio_framework_plugin::ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")();
    let expected: Vec<String> = initial.host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget).to_string()).collect();
    let loaded: Vec<String> = snapshot.host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget).to_string()).collect();
    assert_eq!(loaded, expected, "the genesis document is exactly the editor's initial document");
    initial.retire_cold();
    snapshot.retire_cold();
}
//#endregion 🌱️HubGenesis
