//! 🧩️ Process3d editor unit tests: the real contribution pack admission and the host contribution output cap.

use super::*;

//#region 🧩️RealContributionsPackAdmission
/// ⚖️ A demonstrator-shaped pack crosses this app's registered `setContributions` admission, distils
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
    assert!(wire.len() <= semio_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, "the real pack's command wire ({} B) must fit the registered admission", wire.len());
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
    assert_eq!(ids, ["geometry", "wood", "concrete", "metal", "robotic"]);
}

/// 🎁️ The four active contributions each provide exactly one catalog section.
#[test]
fn active_contributions_install_one_section_each() {
    let pack = context::demonstrator_contributions_pack();
    let after: Vec<(String, String)> = installed_catalogs(&pack).iter().map(|catalog| (catalog.catalog_id().to_string(), catalog.label().to_string())).collect();
    for id in ["wood", "metal", "concrete", "robotic"] {
        assert_eq!(after.iter().filter(|(catalog_id, _)| catalog_id == id).count(), 1, "{id} must name exactly one section");
    }
}

/// 🔄️ An installed extension supplies its own machine data for its catalog id.
#[test]
fn a_contribution_supplies_its_catalog() {
    let machine = WorkshopMachine { id: "customWoodMachine".into(), label: "Custom Wood Machine".into(), icon_id: "wrench".into(), catalog_id: None, capabilities: Vec::new() };
    let entry = semio_framework::ProgramContributionEntry {
        plugin_id: "custom-wood".into(),
        topic_contribution: Some(semio_framework::TopicContribution::new("process.machines", DslValue::object([
            ("appId".into(), DslValue::String(PROCESS_3D_PLAY_APP_ID.into())),
            ("moduleId".into(), DslValue::String("wood".into())),
            ("label".into(), DslValue::String("Custom Wood".into())),
            ("iconId".into(), DslValue::String("wrench".into())),
            ("machinesJson".into(), DslValue::String(semio_framework_pack_json::to_json_string(&vec![machine]))),
        ]))),
    };
    let pack = semio_framework_pack_json::to_json_string(&vec![entry]);
    assert!(catalog_machine(&pack, "wood", "customWoodMachine").is_some());
    assert!(catalog_machine(&pack, "wood", "circularSaw").is_none());
    assert_eq!(installed_catalogs(&pack).iter().filter(|catalog| catalog.catalog_id() == "wood").count(), 1);
}

/// ✨️ A genuinely new catalog id still installs — dedupe drops repeats, never the contribution
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
    let expected = vec!["geometry", "wood", "concrete", "metal", "robotic", "glass"];
    assert_eq!(ids, expected, "a new catalog id appends one section after the built-ins, in host order");
    assert_eq!(catalog_machine(&pack, "glass", "glassCutter").expect("the new catalog's machine").catalog_id.as_deref(), Some("glass"));
}
//#endregion 🧩️RealContributionsPackAdmission

//#region 🧩️HostContributionsOutputCap
/// 🏠️ LAW: the host's REAL `setContributions` push crosses this app's host-configuration gate. The
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

    let mut app = context::unseeded_app_with_registry();
    let receipt = context::install_contributions(&mut app, pack);
    assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Config));
    assert!(app.0.tools.original_scrub_dispatch.is_none(), "the host-action ingress takes its tool dispatch shell on re-entry");
    assert!(app.0.tools.original_emission_receipt.is_none() && app.0.tools.original_emission_construction.as_ref().is_none_or(|construction| construction.terminal_is_empty()), "the host emission hands back its receipt with its construction retired");
    assert!(PluginApp::maintenance_terminal_is_empty(&app.0), "no host-action owner lingers for maintenance after the command settled");
}

/// 🧊️ LAW (browser hang 2026-11-22): a scrub-tagged host-configuration press reached while a history edit freezes the
/// document answers `timeTravel.frozen` and retires its tool dispatch shell with the refusal. No shell, emission receipt or
/// held result lingers, maintenance turns terminal once the edit exits, and the next host command still installs.
#[test]
fn a_frozen_scrub_tagged_host_configuration_press_retires_its_dispatch_shell() {
    let mut app = context::app();
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
    assert!(context::published_a_document_mutation(&receipt), "the history edit needs one committed document mutation");
    let target = app.0.stores.store.mutation_ops().expect("committed mutation log").last().expect("the committed step").mutation_id.0.clone();
    action(&mut app, semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID, Some(&DslValue::object([(semio_framework::history_edit::HISTORY_EDIT_ARG_MUTATION_ID.into(), DslValue::String(target))])));
    assert!(app.0.time_travel.editor().is_some(), "the history edit freezes the document");
    let pack = context::demonstrator_contributions_pack();
    let press = DslValue::object([("json".into(), DslValue::String(installable_contributions(&pack, PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES))), (semio_plugin::SCRUB_GESTURE_ARG.into(), DslValue::String("frozen-press".into()))]);
    let failure = context::contributions_command(&mut app, press).expect_err("a frozen document refuses the scrub-tagged host press");
    assert!(format!("{failure:?}").contains("timeTravel.frozen"), "the refusal is the frozen-press fault: {failure:?}");
    app.1.retire_failure(failure);
    assert!(app.0.tools.original_scrub_dispatch.is_none(), "the frozen tool dispatch shell retires with its refusal");
    assert!(app.0.tools.original_emission_receipt.is_none() && app.0.tools.original_emission_result.is_none(), "the refused press leaves no emission handoff behind");
    action(&mut app, semio_framework::history_edit::HISTORY_EDIT_EXIT_ACTION_ID, None);
    assert!(app.0.time_travel.editor().is_none(), "the history edit exits");
    let installed = context::install_contributions(&mut app, pack);
    assert!(installed.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Config), "the next host command still installs after the frozen press");
    assert!(app.0.tools.original_scrub_dispatch.is_none() && PluginApp::maintenance_terminal_is_empty(&app.0), "no host-action owner lingers for maintenance");
}
//#endregion 🧩️HostContributionsOutputCap
