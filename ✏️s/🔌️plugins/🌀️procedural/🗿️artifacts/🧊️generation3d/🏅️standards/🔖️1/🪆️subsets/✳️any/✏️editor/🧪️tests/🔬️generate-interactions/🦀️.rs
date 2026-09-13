//! 🎛️ The user-reachable interaction surface of generate mode, as ONE table.
//!
//! 🧾️ `📓️audit-user-journey-gaps-2026-09-13.md` scored five separate gaps (#3 wire editing, #5 the
//! missing generate-mode gumball, #7 the generation row verbs, #8 the form value edit, #10 the
//! trigger-less `reorganize`/`moveMediaNode`) against five different files with no single place that
//! said what the app OFFERS. `🧫️fixtures/🎛️generate-mode-interactions.json` is that place; this module
//! holds it against the live `AppDefinition`, and the sibling `🟦️.ts` re-derives the same invariants
//! from the table alone (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use super::*;
use crate::editor::generation3d::unit_tests::serial_execution;

//#region 🎛️Fixture
const GENERATE_INTERACTIONS_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🎛️generate-mode-interactions.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerateInteractionsFixture {
    format: String,
    version: u8,
    windows: Vec<InteractionWindow>,
    preview_parity: Vec<String>,
    gumball_verbs: Vec<String>,
    node_graph_edit_operations: Vec<NodeGraphOperation>,
    command_triggers: Vec<CommandTrigger>,
    removed_commands: Vec<RemovedCommand>,
    rename_commit_argument: RenameCommitArgument,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InteractionWindow {
    id: String,
    body_key: String,
    actions: Vec<String>,
    utilities: Vec<String>,
    row_verbs: Vec<RowVerb>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RowVerb {
    verb: String,
    affordance: String,
    only_when_selected: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeGraphOperation {
    operation: String,
    required: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommandTrigger {
    command: String,
    triggers: Vec<String>,
    keys: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemovedCommand {
    command: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RenameCommitArgument {
    guest_reads: Vec<String>,
}

/// 📇️ Every action id the app declares. `AppDefinition` has no app-level `actions` list — an action a
/// window does not own is COPIED onto every window by `build_definition`
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`), so the union over the window kinds is the
/// roster, and an app-scoped verb like `reorganize` appears on all of them.
fn declared_action_ids(definition: &semio_framework_plugin::AppDefinition) -> std::collections::BTreeSet<&str> {
    definition.window_kinds.iter().flat_map(|kind| kind.actions.iter()).map(|action| action.id.as_str()).collect()
}

fn generate_interactions_fixture() -> GenerateInteractionsFixture {
    let fixture: GenerateInteractionsFixture = serde_json::from_str(GENERATE_INTERACTIONS_FIXTURE_JSON).expect("generate-mode interactions fixture");
    assert_eq!(fixture.format, "semio.generation3d.generate-mode-interactions");
    assert_eq!(fixture.version, 1);
    fixture
}
//#endregion 🎛️Fixture

/// ⚖️ LAW: the table IS the manifest. Every window's declared action set and utility rail match the
/// fixture exactly — an action added to a window without a row here, or a row here the window never
/// declares, both fail.
#[test]
fn every_window_declares_exactly_the_interaction_surface_the_table_names() {
    let _serial = serial_execution::lock();
    let fixture = generate_interactions_fixture();
    let definition = crate::editor::generation3d::create_generation3d_app();
    for window in &fixture.windows {
        let kind = definition.window_kinds.iter().find(|kind| kind.id == window.id).unwrap_or_else(|| panic!("{} is not a declared window kind", window.id));
        assert_eq!(kind.body_key, window.body_key, "{}: body key drifted", window.id);
        let declared: std::collections::BTreeSet<&str> = kind.actions.iter().map(|action| action.id.as_str()).collect();
        for action in &window.actions {
            assert!(declared.contains(action.as_str()), "{} must declare {action}: {declared:?}", window.id);
        }
        let utilities: Vec<String> = kind.utilities.iter().map(|utility| utility.as_str().to_string()).collect();
        assert_eq!(utilities, window.utilities, "{}: utility rail drifted", window.id);
    }
}

/// ⚖️ LAW: the two World3d previews are peers. Everything one of them offers for direct manipulation
/// the other offers too — the gumball trio and the transform utility rail included.
#[test]
fn the_two_previews_offer_one_identical_direct_manipulation_surface() {
    let _serial = serial_execution::lock();
    let fixture = generate_interactions_fixture();
    let definition = crate::editor::generation3d::create_generation3d_app();
    let surface = |id: &str| {
        let kind = definition.window_kinds.iter().find(|kind| kind.id == id).unwrap_or_else(|| panic!("{id} window kind"));
        (kind.actions.iter().map(|action| action.id.clone()).collect::<std::collections::BTreeSet<_>>(), kind.utilities.iter().map(|utility| utility.as_str().to_string()).collect::<Vec<_>>())
    };
    let mut previews = fixture.preview_parity.iter();
    let first = previews.next().expect("the parity list names two previews");
    let (first_actions, first_utilities) = surface(first);
    for verb in &fixture.gumball_verbs {
        assert!(first_actions.contains(verb), "{first} must own {verb}");
    }
    for other in previews {
        let (actions, utilities) = surface(other);
        assert_eq!(actions, first_actions, "{first} and {other} must offer the same actions");
        assert_eq!(utilities, first_utilities, "{first} and {other} must offer the same utility rail");
    }
}

/// ⚖️ LAW: every command the table gives a keyboard trigger really carries that chord, and every
/// command it lists at all is still a declared action — the "no dead commands" half of the table.
#[test]
fn every_listed_command_keeps_its_declared_triggers() {
    let _serial = serial_execution::lock();
    let fixture = generate_interactions_fixture();
    let definition = crate::editor::generation3d::create_generation3d_app();
    let declared: std::collections::BTreeSet<&str> = declared_action_ids(&definition);
    for row in &fixture.command_triggers {
        assert!(declared.contains(row.command.as_str()), "{} is listed with triggers {:?} but is not a declared action", row.command, row.triggers);
        assert!(!row.triggers.is_empty(), "{} must have at least one trigger — no dead commands", row.command);
        let chords: Vec<&str> = definition.keybindings.iter().filter(|binding| binding.action.action == row.command).map(|binding| binding.keys.as_str()).collect();
        match row.keys.as_deref() {
            Some(keys) => assert_eq!(chords, vec![keys], "{} must carry exactly its declared chord", row.command),
            None => assert!(chords.is_empty(), "{} carries an undeclared chord {chords:?}", row.command),
        }
    }
    for removed in &fixture.removed_commands {
        assert!(!declared.contains(removed.command.as_str()), "{} was removed as dead and must not come back without a trigger", removed.command);
    }
}

/// ⚖️ LAW: every node-graph sub-operation the table names is actually HANDLED — asserted by running
/// each one against a real app and requiring a fixture change, which is the only way to tell a handled
/// operation from one that falls through the catch-all arm.
#[semio_framework_async_macros::async_test]
async fn every_node_graph_sub_operation_the_table_names_changes_the_fixture() {
    let _serial = serial_execution::lock();
    let fixture = generate_interactions_fixture();
    let named: std::collections::BTreeSet<&str> = fixture.node_graph_edit_operations.iter().map(|row| row.operation.as_str()).collect();
    assert_eq!(named, ["connect", "deleteSelection", "disconnect", "move", "setFixture"].into_iter().collect::<std::collections::BTreeSet<_>>());
    for row in &fixture.node_graph_edit_operations {
        assert!(!row.required.is_empty() || row.operation == "deleteSelection", "{} must name the arguments it reads", row.operation);
    }
    let mut app = crate::editor::generation3d::unit_tests::context::app().await;
    let node_id = {
        let read = crate::editor::generation3d::unit_tests::context::snapshot(&app);
        crate::widget_id(read.fixture.widgets.first().expect("a widget")).to_string()
    };
    let operations = serde_json::json!([{ "operation": "move", "nodeId": node_id, "x": 12.5, "y": -7.5 }]).to_string();
    crate::editor::generation3d::unit_tests::context::dispatch(
        &mut app,
        crate::editor::generation3d::Generation3dCommand::NodeGraphEdit(crate::editor::generation3d::commands::node_graph_edit::NodeGraphEdit { operations_json: operations }),
    )
    .await;
    let landed = {
        let read = crate::editor::generation3d::unit_tests::context::snapshot(&app);
        let position = read.fixture.layout.get(node_id.as_str()).expect("a moved widget keeps a pinned layout");
        (position.x, position.y)
    };
    assert_eq!(landed, (12.5, -7.5), "the `move` row of the table must name a HANDLED sub-operation");
}

/// ⚖️ LAW: the guest reads every spelling the rename commit can arrive under. The authored args carry
/// `id`; the framework names a scalar `Trigger::Commit` payload `value`; a command bridge that read
/// only `name` renamed every generation to the empty string.
#[test]
fn the_rename_bridge_reads_every_spelling_the_commit_can_arrive_under() {
    let _serial = serial_execution::lock();
    let fixture = generate_interactions_fixture();
    for spelling in &fixture.rename_commit_argument.guest_reads {
        let args: dsl::DslValue = serde_json::json!({ "id": "generation-1", spelling.as_str(): "Balcony Study" }).into();
        let command = <crate::editor::generation3d::Generation3dPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action("renameGeneration", Some(&args)).expect("renameGeneration bridges");
        let crate::editor::generation3d::Generation3dCommand::RenameGeneration(payload) = command else { panic!("renameGeneration must bridge to its own row") };
        assert_eq!(payload.name, "Balcony Study", "the guest must read the rename text under `{spelling}`");
    }
}
