//! 🧩️ Contribution tests of the CAD editor unit tests: the computer packs and the demonstrator packs admitted by the retained command. See [`super`].

use super::*;

//#region 🧩️Contributions
/// 🧩️ Law (e): a contributed `cad.computer` pack is accepted by `setContributions` — the payload
/// survives onto the config lane AND the app's own reader decodes it into a real pack. The four
/// `cad-extension-*` plugins wired into the demonstrator closure on 2026-09-16 push exactly this shape.
#[semio_framework_async_macros::async_test]
async fn a_contributed_cad_computer_pack_is_accepted_by_set_contributions() {
    let contributions = semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(vec![
        semio_framework_value::DslValue::object([
            ("pluginId".to_string(), semio_framework_value::DslValue::String("cad-extension-aec-building".into())),
            (
                "topicContribution".to_string(),
                semio_framework_value::DslValue::object([
                    ("topic".to_string(), semio_framework_value::DslValue::String("cad.computer".into())),
                    (
                        "payload".to_string(),
                        semio_framework_value::DslValue::object([
                            ("appId".to_string(), semio_framework_value::DslValue::String("cad-play".into())),
                            ("moduleId".to_string(), semio_framework_value::DslValue::String("aec.building".into())),
                            ("computersJson".to_string(), semio_framework_value::DslValue::String("[{\"id\":\"aec.building.storey-count\"}]".into())),
                        ]),
                    ),
                ]),
            ),
        ]),
        // 🧩️ A second entry on another topic: a mixed host payload must still install cad's own share.
        semio_framework_value::DslValue::object([
            ("pluginId".to_string(), semio_framework_value::DslValue::String("flow".into())),
            (
                "topicContribution".to_string(),
                semio_framework_value::DslValue::object([("topic".to_string(), semio_framework_value::DslValue::String("flow.extension".into())), ("payload".to_string(), semio_framework_value::DslValue::object([]))]),
            ),
        ]),
    ]));

    let accepted = semio_s_artifact_cad_cad_inferences::validate_cad_computer_contributions(&contributions);
    assert_eq!(accepted.len(), 1, "exactly the cad.computer pack addressed to cad-play is accepted: {accepted:?}");
    assert_eq!(accepted[0].module_id, "aec.building");
    assert!(accepted[0].computers_json.contains("aec.building.storey-count"), "the contributor's own computer roster survives verbatim");

    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let emit = drive(&app, &scene, "setContributions", Some(json!({ "json": contributions.clone() })));
    let config = config_after(&emit, &CadConfig::default());
    assert_eq!(config.contributions_json, contributions, "setContributions stores the pack on the config lane");
    assert_eq!(semio_s_artifact_cad_cad_inferences::validate_cad_computer_contributions(&config.contributions_json).len(), 1, "the stored payload is still an accepted pack");
}

/// 🧩️ The REAL host pack the four `cad-extension-*` plugins push — a Rust mirror of
/// `shippedCadComputerContributionsJson` (`✏️editor/⚙️engine/🏃️runtime/🟦️.ts:51`), module ids and
/// `computersJson` rosters verbatim.
fn shipped_cad_computer_contributions() -> String {
    let entry = |plugin_id: &str, module_id: &str, computers_json: &str| {
        semio_framework_value::DslValue::object([
            ("pluginId".to_string(), semio_framework_value::DslValue::String(plugin_id.into())),
            (
                "topicContribution".to_string(),
                semio_framework_value::DslValue::object([
                    ("topic".to_string(), semio_framework_value::DslValue::String("cad.computer".into())),
                    (
                        "payload".to_string(),
                        semio_framework_value::DslValue::object([
                            ("appId".to_string(), semio_framework_value::DslValue::String("cad-play".into())),
                            ("moduleId".to_string(), semio_framework_value::DslValue::String(module_id.into())),
                            ("computersJson".to_string(), semio_framework_value::DslValue::String(computers_json.into())),
                        ]),
                    ),
                ]),
            ),
        ])
    };
    semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(vec![
        entry("cad-extension-spatial-shape", "spatial-shape", "{\"modelDefinitionIds\":[\"spatial.shape\"],\"statComputers\":[\"spatial.shape.geometry\"],\"propertyComputers\":[\"spatial.shape.volume\"],\"importProfiles\":[],\"transformationAppliers\":[]}"),
        entry("cad-extension-aec-building", "aec-building", "{\"modelDefinitionIds\":[\"aec.building\"],\"statComputers\":[],\"propertyComputers\":[],\"importProfiles\":[{\"modelDefinitionId\":\"aec.building\",\"layerTypology\":{},\"fallbackTypology\":\"building.building.slab\"}],\"transformationAppliers\":[]}"),
        entry("cad-extension-aec-building-energy", "aec-building-energy", "{\"modelDefinitionIds\":[\"aec.building.energy\"],\"statComputers\":[\"energy.demand\"],\"propertyComputers\":[\"energy.heatedvolume\"],\"importProfiles\":[],\"transformationAppliers\":[]}"),
        entry("cad-extension-aec-building-structure", "aec-building-structure", "{\"modelDefinitionIds\":[\"aec.building.structure\"],\"statComputers\":[\"structure.stability\"],\"propertyComputers\":[],\"importProfiles\":[],\"transformationAppliers\":[\"aec.building.structure/from_building\"]}"),
    ]))
}

/// ⚖️ LAW: the REAL four-extension `cad.computer` pack the demonstrator's koordinator pane receives
/// is ADMITTED by the retained config envelope, lands on the config lane through the production
/// `setContributions` dispatch, and decodes back into all four computer modules. Unlike sourcing's
/// 96-byte filter-text envelope, cad already prices `SetContributions` on its own
/// `CAD_CONFIG_STORE_MAXIMUM_BYTES` lane — this law pins that the real pack fits it, and that the
/// `CAD_CONFIG_STORE_MAXIMUM_BYTES` lane — this law pins that the real pack fits it.
///
/// 🏁️ The one-page (4 KiB) close budget this law used to pin as well was the framework's retained-
/// config close livelock, fixed 2026-09-16 in `close_retained_fields_step`
/// (`📓️fix-2026-09-16-kernel-capability-contributions-and-close-cliff.md` §2); a 65 536-byte retained
/// config now reaches terminal-empty, so the declared lane is what bounds this pack and nothing else.
#[semio_framework_async_macros::async_test]
async fn the_shipped_cad_computer_pack_is_admitted_by_the_retained_config_envelope() {
    let contributions = shipped_cad_computer_contributions();
    assert!(contributions.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES, "the real pack is {} bytes against a {}-byte config lane", contributions.len(), CAD_CONFIG_STORE_MAXIMUM_BYTES);

    let mutation = CadConfigMutation::SetContributions(SetContributionsEdit { json: contributions.clone() });
    assert!(admit_cad_config_mutation(&mutation).is_ok(), "the retained config store must admit the real host pack");
    assert!(prepare_cad_config(&CadConfig::default(), mutation).is_ok(), "the whole preparation must admit the real host pack");
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let emit = drive(&app, &scene, "setContributions", Some(json!({ "json": contributions.clone() })));
    let config = config_after(&emit, &CadConfig::default());
    assert_eq!(config.contributions_json, contributions, "the real pack survives verbatim onto the config lane");
    let accepted = semio_s_artifact_cad_cad_inferences::validate_cad_computer_contributions(&config.contributions_json);
    assert_eq!(
        accepted.iter().map(|pack| pack.module_id.clone()).collect::<Vec<_>>(),
        vec!["spatial-shape".to_string(), "aec-building".to_string(), "aec-building-energy".to_string(), "aec-building-structure".to_string()],
        "every cad-extension-* module installs, in host order"
    );
    assert!(accepted.iter().all(|pack| pack.computers_json.contains("modelDefinitionIds")), "each module's own computer roster survives verbatim");
    // 🖼️ `render_body` runs `validate_cad_computer_contributions` on every body assembly
    // (`✏️editor/🦀️.rs:1278`), so a pack that made the panel refuse would surface here.
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    assert!(render_direct(&app, shape::BODY_KEY, &doc, &config, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).is_ok(), "the shape panel still assembles with the whole pack installed");
}

/// 🧩️ The pack the DEMONSTRATOR's koordinator pane actually receives: the demonstrator consumes
/// `cad.computer`, `process.machines` and `sourcing.module`, so the host hands this app all three
/// topics in ONE crossing and the guest ignores what is not addressed to it.
fn demonstrator_contributions_pack() -> String {
    let mut entries: Vec<semio_framework_value::DslValue> = serde_json::from_str::<Vec<serde_json::Value>>(&shipped_cad_computer_contributions())
        .expect("the shipped cad pack parses")
        .into_iter()
        .map(|value| semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("entry"))
        .collect();
    for (plugin_id, topic, app_id, bulk) in [
        ("process-extension-wood", "process.machines", "process3d-play", 4_705usize),
        ("process-extension-metal", "process.machines", "process3d-play", 4_011),
        ("process-extension-robotic", "process.machines", "process3d-play", 4_010),
        ("process-extension-concrete", "process.machines", "process3d-play", 3_311),
        ("sourcing-module-beams", "sourcing.module", "sourcing-curation", 780),
        ("sourcing-module-windows", "sourcing.module", "sourcing-curation", 669),
        ("sourcing-module-slabs", "sourcing.module", "sourcing-curation", 589),
    ] {
        entries.push(semio_framework_value::DslValue::object([
            ("pluginId".to_string(), semio_framework_value::DslValue::String(plugin_id.into())),
            (
                "topicContribution".to_string(),
                semio_framework_value::DslValue::object([
                    ("topic".to_string(), semio_framework_value::DslValue::String(topic.into())),
                    (
                        "payload".to_string(),
                        semio_framework_value::DslValue::object([
                            ("appId".to_string(), semio_framework_value::DslValue::String(app_id.into())),
                            ("moduleId".to_string(), semio_framework_value::DslValue::String(plugin_id.into())),
                            // 📐️ The REAL bulk string of that extension, measured off the built dev manifests 2026-09-16.
                            ("bulkJson".to_string(), semio_framework_value::DslValue::String("m".repeat(bulk))),
                        ]),
                    ),
                ]),
            ),
        ]));
    }
    semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(entries))
}

/// ⚖️ LAW: the REAL demonstrator pack crosses this app's registered `setContributions` admission and
/// is retained whole.
///
/// 🏁️ Before the per-app admission, cad priced `setContributions` on the 8 KiB gesture envelope every
/// retained tool shares, and the live push died with `typed command raw JSON exceeds its registered
/// retained-page admission` the moment the host stopped cutting capability packs to `[]`
/// (2026-09-16, ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS).
#[semio_framework_async_macros::async_test]
async fn the_real_demonstrator_pack_is_admitted_by_the_registered_contributions_wire() {
    let pack = demonstrator_contributions_pack();
    let wire = semio_framework_pack_json::to_json_string(&("setContributions", semio_framework_value::DslValue::object([("json".to_string(), semio_framework_value::DslValue::String(pack.clone()))])));
    println!("[STATS] cad demonstrator pack packChars={} wireChars={}", pack.len(), wire.len());
    assert!(pack.len() > CAD_RETAINED_RAW_BYTES, "the real pack is past the gesture envelope — that is why the app declares its own contributions wire");
    assert!(wire.len() <= semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, "the real pack's command wire ({} B) must fit the registered admission", wire.len());
    assert!(pack.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES, "the real pack ({} B) must fit the retained config lane", pack.len());
    let mutation = CadConfigMutation::SetContributions(SetContributionsEdit { json: pack.clone() });
    assert!(admit_cad_config_mutation(&mutation).is_ok());
    assert!(prepare_cad_config(&CadConfig::default(), mutation).is_ok());
    let accepted = semio_s_artifact_cad_cad_inferences::validate_cad_computer_contributions(&pack);
    assert_eq!(accepted.len(), 4, "only the four cad.computer modules install; the foreign topics are ignored, never refused");
}
//#endregion 🧩️Contributions
