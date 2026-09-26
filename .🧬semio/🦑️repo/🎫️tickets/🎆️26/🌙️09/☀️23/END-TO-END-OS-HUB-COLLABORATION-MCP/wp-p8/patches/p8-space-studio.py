#!/usr/bin/env python3
"""🪐️ P8 post-publish patch set: the studio's 24 dead verbs become reachable.

Every workflow-graph, parameter, instance and media verb of `s.space.studio@1/*#editor` was `BatchOnlyPendingRewrite`, so the
shell and every agent got `interactive-job.not-ui-safe` before any handler ran — published to agents, dead for everyone.
Each moves onto the studio's one bounded retained factory with the store lanes its handler actually emits; the six verbs
that read the live `graph` selection get it from the retained interaction state (the same ids `SpaceApp::handle` reads off
its `InteractionView`), through the selection-taking body each already has.

Parts: A tool ids + lanes + proofs; B reducer routes for the selection verbs; C selection bodies reachable; D
classification; E the language-neutral catalogue fixture and its law (40 routes, all bounded and migrated).
Usage: p8-space-studio.py --dry-run | --write"""
import json

from p8_patch import ROOT, finish, replace, text

SPACE = ROOT / "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space"
ENGINE = SPACE / "🦀️.rs"
COMMANDS = SPACE / "🎮️commands"
UNIT = SPACE / "🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = SPACE / "🧫️fixtures/🧫️retained-command-limits/🔣️.json"

LANES = {
    "patchParameter": ["Artifact"],
    "addParameter": ["Artifact"],
    "removeParameter": ["Artifact"],
    "moveMediaNode": ["Artifact"],
    "connectMediaPorts": ["Artifact"],
    "disconnectMediaEdge": ["Artifact"],
    "removeAppInstance": ["Artifact", "Config"],
    "copyAppInstance": ["Config"],
    "duplicateAppInstance": ["Artifact", "Config"],
    "pasteAppInstance": ["Artifact", "Config"],
    "renameAppInstance": ["Artifact"],
    "patchMediaNodes": ["Artifact"],
    "patchAppInstances": ["Artifact"],
    "bindParameterField": ["Artifact"],
    "unbindParameterField": ["Artifact"],
    "reorganizeWorkflow": ["Artifact"],
    "workflowEngagementSubmit": ["Artifact", "Config"],
    "compiledDagEngagementSubmit": ["HostOnly"],
    "nodeGraphEdit": ["Artifact", "Config"],
    "exportMedia": ["HostOnly"],
    "importMedia": ["Config"],
    "importMediaPayload": ["Config"],
    "exportStudioPack": ["HostOnly"],
    "exportStudioDsl": ["HostOnly"],
}
VERBS = list(LANES)

replace("A", ENGINE, '''    "importSpacePackPayload",
    "setAppRegistrations",
];
/// 🧵️ Still batch-only, honestly: every id here is a workflow-graph or media edit the shell never
/// dispatches on its own (no `🏛️ShellHost`/`🕸️NodeGraph` call site, verified by id), and each needs
/// its own reducer/extent review before it can claim a bounded first step.
#[cfg(test)]
const SPACE_BATCH_ONLY_TOOL_IDS: &[&str] = &[
''' + "".join(f'    "{verb}",\n' for verb in VERBS) + '''];''', '''    "importSpacePackPayload",
    "setAppRegistrations",
''' + "".join(f'    "{verb}",\n' for verb in VERBS) + '''];''')
contract_lines = "".join(
    f'        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "{verb}", lanes: &[{", ".join("semio_framework_plugin::ArtifactToolPublicationLane::" + lane for lane in lanes)}] }},\n'
    for verb, lanes in LANES.items()
)
replace("A", ENGINE, '''        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setAppRegistrations", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    ];''', '''        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setAppRegistrations", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
''' + contract_lines + '''    ];''')
replace("A", ENGINE, '''            "importSpacePackPayload",
            "setAppRegistrations",
        ]
    }''', '''            "importSpacePackPayload",
            "setAppRegistrations",
''' + "".join(f'            "{verb}",\n' for verb in VERBS) + '''        ]
    }''')

replace("B", ENGINE, '''    if let SpaceCommand::DeleteSelection(_) = command {
        return Ok(crate::engine::space::engine::resolve_future(delete_selection::delete_selected(config, &selected())));
    }''', '''    if let SpaceCommand::DeleteSelection(_) = command {
        return Ok(crate::engine::space::engine::resolve_future(delete_selection::delete_selected(config, &selected())));
    }
    match command {
        SpaceCommand::NodeGraphEdit(payload) => return Ok(crate::engine::space::engine::resolve_future(node_graph_edit::edit_with_selection(payload, snapshot, &selected()))),
        SpaceCommand::ReorganizeWorkflow(_) => return Ok(crate::engine::space::engine::resolve_future(reorganize_workflow::reorganize_selected(&doc, &selected()))),
        SpaceCommand::CopyAppInstance(_) => return Ok(Emit::config(vec![SpaceConfigMutation::SetClipboard { node_ids: selected() }])),
        SpaceCommand::DuplicateAppInstance(_) => return Ok(crate::engine::space::engine::resolve_future(duplicate_app_instance::duplicate_nodes(selected(), snapshot))),
        SpaceCommand::RemoveAppInstance(payload) => return Ok(crate::engine::space::engine::resolve_future(remove_app_instance::remove_with_selection(payload, config, &selected()))),
        SpaceCommand::RenameAppInstance(payload) => return Ok(crate::engine::space::engine::resolve_future(rename_app_instance::rename_with_selection(payload, &doc, config, &selected()))),
        _ => {}
    }''')

replace("C", COMMANDS / "✏️node-graph-edit/🦀️.rs", "async fn edit_with_selection(", "pub(crate) async fn edit_with_selection(")
replace("C", COMMANDS / "🗂️reorganize-workflow/🦀️.rs", "async fn reorganize_selected(", "pub(crate) async fn reorganize_selected(")
replace("C", COMMANDS / "👯️duplicate-app-instance/🦀️.rs", "async fn duplicate_nodes(", "pub(crate) async fn duplicate_nodes(")
replace("C", COMMANDS / "🚮️remove-app-instance/🦀️.rs", "async fn remove_with_selection(", "pub(crate) async fn remove_with_selection(")
replace("C", COMMANDS / "🏷️rename-app-instance/🦀️.rs", "async fn rename_with_selection(", "pub(crate) async fn rename_with_selection(")

for verb in VERBS:
    replace("D", ENGINE, f'''        .action_interactive_job("{verb}", InteractiveJobClassification::BatchOnlyPendingRewrite).await''', f'''        .action_interactive_job("{verb}", InteractiveJobClassification::Migrated).await''')

source = text(FIXTURE)
document = json.loads(source)
for route in document["routes"]:
    if route["id"] in LANES:
        route["execution"] = "bounded"
        route["status"] = "Migrated"
document["publicationContracts"] += [{"toolId": verb, "lanes": lanes} for verb, lanes in LANES.items()]
document["oracle"]["expected"] = {"routes": 40, "bounded": 40, "batch": 0, "migrated": 40, "unique": True}
replace("E", FIXTURE, source, json.dumps(document, indent=2, ensure_ascii=False) + "\n")
replace("E", UNIT, '''    assert_eq!(oracle, SpaceRetainedCatalogSummary { routes: 40, bounded: 16, batch: 24, migrated: 16, unique: true, bounded_ids: bounded_ids.clone(), migrated_ids: bounded_ids.clone(), host_only_ids: host_only_ids.clone() });''',
        '''    assert_eq!(oracle, SpaceRetainedCatalogSummary { routes: 40, bounded: 40, batch: 0, migrated: 40, unique: true, bounded_ids: bounded_ids.clone(), migrated_ids: bounded_ids.clone(), host_only_ids: host_only_ids.clone() });''')
replace("E", UNIT, '''    // 📣️ `presenceHeartbeat` joined the HostOnly lane (see `SpaceCommandJobFactory`'s own
    // `PUBLICATION_CONTRACTS` and the committed `🧫️retained-command-limits` fixture): seven now.
    assert_eq!(host_only_ids.len(), 7);
    assert_eq!(SPACE_BATCH_ONLY_TOOL_IDS.len(), 24);''', '''    // 📣️ The seven host relays plus the three studio exports and the compiled-DAG submit gesture, which touch no store.
    assert_eq!(host_only_ids.len(), 11);''')

finish(__doc__)
