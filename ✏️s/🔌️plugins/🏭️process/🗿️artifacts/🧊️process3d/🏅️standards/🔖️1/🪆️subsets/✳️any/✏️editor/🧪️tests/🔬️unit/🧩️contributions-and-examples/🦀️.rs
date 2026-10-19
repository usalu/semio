//! 🧩️ Process3d editor unit tests: the real contribution pack admission, the host contribution output cap and the member-less example archive door.

use super::*;

//#region 🧩️RealContributionsPackAdmission
/// ⚖️ The REAL demonstrator pack crosses this app's registered `setContributions` admission, distils
/// to the four `process.machines` entries addressed to this app, and is retained inside the
/// contributions lane — every hop the live push actually takes.
///
/// 🏁️ Before the per-tool contract this app priced `setContributions` on its 8 KiB GESTURE envelope,
/// and the live push died with `typed command raw JSON exceeds its registered retained-page
/// admission` the moment the host stopped cutting capability packs to `[]` (2026-09-16, ticket
/// 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS).
#[test]
fn the_real_demonstrator_pack_is_admitted_distilled_and_retained() {
    let pack = context::demonstrator_contributions_pack();
    let wire = semio_framework_pack_json::to_json_string(&("setContributions", semio_framework::DslValue::object([("json".to_string(), semio_framework::DslValue::String(pack.clone()))])));
    println!("[STATS] process3d demonstrator pack packChars={} wireChars={}", pack.len(), wire.len());
    assert!(wire.len() <= semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, "the real pack's command wire ({} B) must fit the registered admission", wire.len());
    assert!(pack.len() > PROCESS3D_RETAINED_RAW_BYTES, "the real pack is past the gesture envelope — that is the whole point of its own lane");

    let distilled = installable_contributions(&pack, PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES);
    let kept = semio_framework::parse_contributions(&distilled);
    assert_eq!(kept.len(), 4, "only the process.machines entries addressed to this app are retained");
    assert!(kept.iter().all(|entry| entry.topic_contribution.as_ref().is_some_and(|topic| topic.topic == "process.machines")));
    assert!(distilled.len() <= PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES);

    let mutation = Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: distilled.clone() });
    assert!(admit_process3d_config_mutation(&mutation).is_ok(), "the distilled roster fits the retained contributions lane");
    let (post, _) = prepare_process3d_config(&Process3dConfig::default(), &mutation).expect("the retained config admits the real roster");
    assert_eq!(post.contributions_json, distilled);

    let catalogs = installed_catalogs(&distilled);
    let ids: Vec<&str> = catalogs.iter().map(|catalog| catalog.catalog_id()).collect();
    let builtin_ids: Vec<String> = builtin_installed_catalogs().iter().map(|catalog| catalog.catalog_id().to_string()).collect();
    assert_eq!(ids, builtin_ids, "the four shipped packs re-contribute the built-in ids, so the installed build wins and nothing is appended");
}

/// ⚖️ A contribution pack repeating the ids this build already compiles in leaves the installed
/// catalogs untouched — the Werkstatt panel renders five sections, not nine.
#[test]
fn a_pack_repeating_the_builtin_ids_installs_exactly_the_builtin_catalogs() {
    let pack = context::demonstrator_contributions_pack();
    let before: Vec<(String, String)> = builtin_installed_catalogs().iter().map(|catalog| (catalog.catalog_id().to_string(), catalog.label().to_string())).collect();
    let after: Vec<(String, String)> = installed_catalogs(&pack).iter().map(|catalog| (catalog.catalog_id().to_string(), catalog.label().to_string())).collect();
    assert_eq!(after, before, "a repeated catalog id must not open a second section");
    for id in ["wood", "metal", "concrete", "robotic"] {
        assert_eq!(after.iter().filter(|(catalog_id, _)| catalog_id == id).count(), 1, "{id} must name exactly one section");
    }
}

/// ⚖️ A genuinely new catalog id still installs — dedupe drops repeats, never the contribution
/// channel itself — and its machines are reachable through the add-machine lookup.
#[test]
fn a_pack_with_a_new_catalog_id_appends_one_section_with_its_machines() {
    let machine = WorkshopMachine { id: "glassCutter".into(), label: "Glass Cutter".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: Vec::new() };
    let entry = semio_framework::ProgramContributionEntry {
        plugin_id: "process-extension-glass".into(),
        topic_contribution: Some(semio_framework::TopicContribution::new(
            "process.machines",
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String(PROCESS_3D_PLAY_APP_ID.to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String("glass".to_string())),
                ("label".to_string(), semio_framework::DslValue::String("Glass".to_string())),
                ("iconId".to_string(), semio_framework::DslValue::String("wrench".to_string())),
                ("machinesJson".to_string(), semio_framework::DslValue::String(semio_framework_pack_json::to_json_string(&vec![machine]))),
            ]),
        )),
    };
    let mut entries = semio_framework::parse_contributions(&context::demonstrator_contributions_pack());
    entries.push(entry);
    let pack = semio_framework_pack_json::to_json_string(&entries);
    let catalogs = installed_catalogs(&pack);
    let ids: Vec<&str> = catalogs.iter().map(|catalog| catalog.catalog_id()).collect();
    let mut expected: Vec<String> = builtin_installed_catalogs().iter().map(|catalog| catalog.catalog_id().to_string()).collect();
    expected.push("glass".to_string());
    assert_eq!(ids, expected, "a new catalog id appends one section after the built-ins, in host order");
    assert_eq!(catalog_machine(&pack, "glass", "glassCutter").expect("the new catalog's machine").catalog_id.as_deref(), Some("glass"));
}
//#endregion 🧩️RealContributionsPackAdmission

//#region 🧩️HostContributionsOutputCap
/// ⚖️ LAW: the host's REAL `setContributions` push crosses this app's host-configuration gate. The
/// gate prices the op-encoded config mutation against the proof contract's `max_output_bytes`; that
/// cap used to be the 16 KiB gesture output while the contributions lane beside it admits 24 KiB, so
/// the distilled four-extension roster was refused with `host configuration command
/// 'setContributions' exceeds its exact output cap` at every boot (ticket
/// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). Drives the registry-backed action path end to end.
#[test]
fn the_real_contributions_push_passes_the_host_configuration_output_gate() {
    use protocol::OpBinary;
    let pack = context::demonstrator_contributions_pack();
    let distilled = installable_contributions(&pack, PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES);
    let encoded = Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: distilled.clone() }).encode_op().expect("encode real roster");
    println!("[STATS] process3d contributions roster={} encoded={} cap={PROCESS3D_RESUMABLE_OUTPUT_BYTES}", distilled.len(), encoded.len());
    assert!(encoded.len() > 16_384, "the real roster is past the gesture output — the regression this law guards");
    assert!(encoded.len() <= process3d_resumable_contract().max_output_bytes, "the real roster must fit the declared output cap");
    let worst = Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: "x".repeat(PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES) }).encode_op().expect("encode lane-sized roster");
    assert!(worst.len() <= PROCESS3D_RESUMABLE_OUTPUT_BYTES, "a lane-filling roster encodes to {} B, past the declared {PROCESS3D_RESUMABLE_OUTPUT_BYTES} B", worst.len());

    use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
    let mut app = context::unseeded_app_with_registry();
    let invocation = CommandInvocation {
        address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: "process".into(), app_id: "s.process.process3d@1/*#editor".into() }, command_id: "setContributions".into() },
        arguments: [("json".to_string(), DslValue::String(pack))].into_iter().collect(),
    };
    let mut arguments = Some(DslValue::object(invocation.arguments.iter().map(|(name, value)| (name.clone(), value.clone()))));
    let mut fault = None;
    for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        let handled = ::semio_framework_async::poll::resolve_ready(PluginApp::handle_command(&mut app.0, &invocation.address, &mut arguments, &mut fault, None, &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("the host's setContributions command passes the host-configuration output gate");
        if handled.is_some() {
            return;
        }
        PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("setContributions maintenance step");
    }
    panic!("the host's setContributions command stayed pending under its original caller");
}
//#endregion 🧩️HostContributionsOutputCap

//#region 🚪️ExampleArchiveDoor
/// 🚪️ Settles one member-less archive load exactly the way the react shell's `loadDocumentPair` does.
fn settle_member_less_archive(app: &mut context::Process3dApp, operation: u64, parent_pack: Vec<u8>, parent_spr: Vec<u8>) -> protocol::DocumentArchiveLoadStatus {
    PluginApp::begin_document_archive_load(&mut app.0, operation, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
    for _ in 0..200_000 {
        let polled = ::semio_framework_async::poll::resolve_ready(PluginApp::poll_document_archive_load(&mut app.0, operation, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            PluginApp::acknowledge_document_archive_load(&mut app.0, operation).expect("archive acknowledgement");
            return polled;
        }
        let _ = PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("archive maintenance step");
    }
    panic!("archive load never reached a terminal state");
}

/// 🔁️ Dispatches `setActiveExample` and drives its typed operation to the published
/// `Effect::LoadDocument`, the way the host's publication ladder does.
fn published_example_load(app: &mut context::Process3dApp, example: &str) -> (Vec<u8>, Vec<u8>) {
    let instance = semio_framework_plugin::artifact_app_laws::meta("local").instance_id;
    let result = action(app, "setActiveExample", Some(&DslValue::from(&serde_json::json!({ "exampleId": example }))));
    let mut loaded = result.requested_effects.into_iter().find_map(|effect| match effect {
        Effect::LoadDocument { pack, spr } => Some((pack, spr)),
        _ => None,
    });
    for _ in 0..100_000 {
        if loaded.is_some() && !app.has_pending_typed_operations() {
            break;
        }
        PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("example maintenance step");
        ::semio_framework_async::poll::resolve_ready(app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("example publication");
        if let Some(page) = app.take_typed_operation_result_page(instance) {
            assert!(app.acknowledge_typed_operation_result(page.token).expect("example result ack"));
        }
        if let Some(Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
            loaded = Some((pack, spr));
        }
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
    }
    loaded.unwrap_or_else(|| panic!("{example}: setActiveExample must publish a LoadDocument effect"))
}

/// ⚖️ LAW: every registered example loads through the host's document archive door — `setActiveExample`
/// publishes `Effect::LoadDocument`, the shell hands its pack/spr back with NO members, and the
/// app's `genesis_child_pack` must complete the composed roster so the ownership closure is not
/// `Incomplete` (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP).
#[test]
fn every_example_loads_through_the_member_less_archive_door() {
    let _lane = semio_s_artifact_process_process3d_host::process3d_publication_authority_lane();
    for (operation, example) in [(71_u64, PROCESS3D_EXAMPLE_TIMBER), (72, PROCESS3D_EXAMPLE_PLATE), (73, PROCESS3D_EXAMPLE_CONCRETE_FOREST)] {
        let mut app = context::unseeded_app_with_registry();
        let (pack, spr) = published_example_load(&mut app, example);
        let expected = <Process3dSnapshot as ArtifactPack>::decode_pack(&pack).expect("example pack decodes");
        let status = settle_member_less_archive(&mut app, operation, pack, spr);
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{example}: {}", String::from_utf8_lossy(&status.fault));
        let loaded = app.snapshot().expect("loaded snapshot");
        assert_eq!(loaded.stock_solid.child_id, expected.stock_solid.child_id, "{example}: the example replaced the boot document");
        assert_eq!(loaded.step_payloads, expected.step_payloads, "{example}: the example's steps are live");
    }
}

/// ⚖️ LAW: every shipped example is CANONICAL — its composed-child handles are exactly the
/// content-addressed ids `process_working_scene_to_snapshot` mints from its own inline records today.
/// `genesis_process3d_child_pack` re-derives each `toolSolids` id from the step's solid, so a fixture
/// minted under an older B-Rep serialization names children genesis can no longer derive: the timber
/// and plate fixtures had drifted, and the member-less archive load of timber closed `Incomplete`
/// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). Regenerate with `print_dsl` of the remint.
#[test]
fn every_example_fixture_carries_its_canonical_child_handles() {
    for (example, document) in [(PROCESS3D_EXAMPLE_TIMBER, semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document()), (PROCESS3D_EXAMPLE_PLATE, semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::plate_document()), (PROCESS3D_EXAMPLE_CONCRETE_FOREST, semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::concrete_forest_document())] {
        let reminted = semio_s_artifact_process_process3d_model::snapshot::process_working_scene_to_snapshot(&semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(&document), document.workshop.clone());
        assert_eq!(document.stock_solid, reminted.stock_solid, "{example}: stale stockSolid handle");
        assert_eq!(document.steps, reminted.steps, "{example}: stale steps handle");
        assert_eq!(document.tool_solids, reminted.tool_solids, "{example}: stale toolSolids handles");
        assert_eq!(document, reminted, "{example}: the fixture is exactly its own remint");
    }
}
//#endregion 🚪️ExampleArchiveDoor
