use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::example_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::{Generation3dMutation};
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::central_apply::{apply_generation3d_mutation};

/// 🧭️ The document the host would normalise `bundled` to — the base every replay law is stated on, so a normalisation the
/// host applies on open is not mistaken for an edit.
fn normalised_example(example: &str) -> Generation3dSnapshotRead {
    let mut snapshot = example_snapshot(example).expect("bundled example");
    let host = with_host(&snapshot.host_snapshot, |host| host.host_snapshot.clone());
    std::mem::replace(&mut snapshot.host_snapshot, host).retire_cold();
    Generation3dSnapshotRead::new(snapshot)
}

/// 🎬️ Runs `edit` through a recorder, then replays the recorded leaves on `base`: the replayed graph must be the host's own.
fn recorded_replay_reaches_the_host(base: &Generation3dSnapshotRead, edit: impl FnOnce(&mut GraphEditor<'_>)) -> (Vec<Generation3dMutation>, Generation3dSnapshotRead) {
    let (leaves, expected) = with_host(&base.host_snapshot, |host| {
        let mut editor = GraphEditor::new(host);
        edit(&mut editor);
        let expected = editor.snapshot().clone();
        (editor.finish(), expected)
    });
    let mut replayed = Generation3dSnapshotRead::new((**base).clone());
    for leaf in &leaves {
        apply_generation3d_mutation(&mut replayed, leaf).unwrap_or_else(|refused| panic!("{leaf:?} refused: {refused:?}"));
    }
    assert_eq!(replayed.host_snapshot.widgets, expected.widgets, "the replayed widgets are the host's");
    assert_eq!(replayed.host_snapshot.synapses, expected.synapses, "the replayed wires are the host's");
    assert_eq!(replayed.host_snapshot.layout, expected.layout, "the replayed positions are the host's");
    expected.retire_cold();
    (leaves, replayed)
}

fn kinds(leaves: &[Generation3dMutation]) -> Vec<&'static str> {
    leaves.iter().map(|leaf| protocol::SemanticMutation::semantics(leaf).kind).collect()
}

fn retire(leaves: Vec<Generation3dMutation>) {
    for leaf in leaves {
        leaf.retire_cold();
    }
}

/// ➖️ Removing a wired, positioned widget records the cascade — every wire naming it, its position, then the widget — and
/// nothing else.
#[test]
fn graph_editor_replay_reaches_the_host_snapshot_for_a_removal() {
    let _serial = crate::test_serial::lock();
    let base = normalised_example(PROCEDURAL_EXAMPLE_BOX_FILLET);
    let doomed = base.host_snapshot.synapses.first().expect("the example wires something").to.clone();
    let wires = base.host_snapshot.synapses.iter().filter(|synapse| synapse.from == doomed || synapse.to == doomed).count();
    let (leaves, replayed) = recorded_replay_reaches_the_host(&base, |editor| editor.remove_widget(&doomed).expect("the widget is removed"));
    let kinds = kinds(&leaves);
    assert_eq!(kinds.iter().filter(|kind| **kind == "disconnect-synapse").count(), wires);
    assert_eq!(kinds.last(), Some(&"delete-widget"));
    assert!(replayed.host_snapshot.synapses.iter().all(|synapse| synapse.from != doomed && synapse.to != doomed) && !replayed.host_snapshot.layout.contains_key(&doomed));
    retire(leaves);
}

/// 🔗️ Cutting a wire and wiring it again is one `disconnect-synapse` and one `connect-synapse`.
#[test]
fn graph_editor_replay_reaches_the_host_snapshot_for_wiring() {
    let _serial = crate::test_serial::lock();
    let base = normalised_example(PROCEDURAL_EXAMPLE_BOX_FILLET);
    let wire = base.host_snapshot.synapses.first().expect("the example wires something").clone();
    let (leaves, _) = recorded_replay_reaches_the_host(&base, |editor| {
        editor.disconnect(&wire.id).expect("the wire is cut");
        editor.connect_ports(&wire.from, &wire.from_port, &wire.to, &wire.to_port).expect("the wire is restored");
    });
    assert_eq!(kinds(&leaves), ["disconnect-synapse", "connect-synapse"]);
    retire(leaves);
}

/// 🧩️ Creating a widget records the `create-widget` and the `move-widget` that places it — once, however often the command
/// places it again.
#[test]
fn graph_editor_replay_reaches_the_host_snapshot_for_a_creation() {
    let _serial = crate::test_serial::lock();
    let base = normalised_example(PROCEDURAL_EXAMPLE_BOX_FILLET);
    let (leaves, replayed) = recorded_replay_reaches_the_host(&base, |editor| {
        let id = editor.add_widget(r#"{"kind":"inputNote"}"#, 10.0, 20.0).expect("the note is created");
        editor.move_widget(&id, 30.0, 40.0).expect("the note is placed");
    });
    assert_eq!(kinds(&leaves), ["create-widget", "move-widget"]);
    assert_eq!(replayed.host_snapshot.widgets.len(), base.host_snapshot.widgets.len() + 1);
    retire(leaves);
}

/// 🗺️ Reorganising authors a `move-widget` per widget that moves, and the replayed layout is the host's.
#[test]
fn graph_editor_replay_reaches_the_host_snapshot_for_a_reorganisation() {
    let _serial = crate::test_serial::lock();
    let base = normalised_example(PROCEDURAL_EXAMPLE_BOX_FILLET);
    let (leaves, _) = recorded_replay_reaches_the_host(&base, |editor| editor.reorganize(r#"{"orientation":"leftRight"}"#).expect("the graph is laid out"));
    assert!(kinds(&leaves).iter().all(|kind| *kind == "move-widget"));
    retire(leaves);
}

/// 🔀️ A gumball splice — the transform operator created, placed and spliced into the wire it transforms, the preview moved
/// onto it — is exactly the leaves the host's own splice amounts to.
#[test]
fn graph_editor_replay_reaches_the_host_snapshot_for_a_splice() {
    let _serial = crate::test_serial::lock();
    let base = normalised_example(PROCEDURAL_EXAMPLE_BOX_FILLET);
    let source = base.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::Neuron { id, preview: true, .. } => Some(id.clone()),
        _ => None,
    }).expect("the example previews an operator");
    let (leaves, _) = recorded_replay_reaches_the_host(&base, |editor| {
        crate::standards::v1::subsets::any::io::text::snapshot::ensure_gumball_node(editor, &source, "translate").expect("the translate operator is spliced");
    });
    let kinds = kinds(&leaves);
    assert!(kinds.starts_with(&["create-widget", "move-widget"]) && kinds.contains(&"update-widget"), "{kinds:?}");
    retire(leaves);
}
