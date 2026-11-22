//! 📋️ Isolated production clipboard probe when the artifact workspace's unrelated DXF test dependency is unavailable.

extern crate pack_json as semio_framework_pack_json;
extern crate puzzle2d_model as semio_s_artifact_puzzle_2d_mut_any_10;
extern crate semio_os_kernel as protocol;
extern crate value as semio_framework_value;

pub use puzzle2d_model::{standards, Puzzle2dSnapshot};

#[path = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🦀️.rs"]
pub mod actual_recorder;

pub mod editor {
    pub mod puzzle2d {
        pub use crate::actual_recorder as recorder;
        pub use puzzle2d_model::editor::puzzle2d::snapshot;
    }
}

use editor::puzzle2d::recorder::{puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_paste_operations_on, Puzzle2dRecorder};
use editor::puzzle2d::snapshot::puzzle2d_selection_is_locked;
use protocol::ColdMutation;
use puzzle2d_model::Puzzle2dPlaySnapshot;
use semio_plugin::kernel::PastePlacement;
use standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;

fn record_with_inverse(recorder: &mut Puzzle2dRecorder, mutations: Vec<Puzzle2dMutation>) -> Vec<Puzzle2dMutation> {
    let mut inverse = Vec::new();
    for mutation in mutations {
        inverse.extend(ColdMutation::<Puzzle2dSnapshot>::inverse(&mutation, recorder.typed()).expect("mutation inverse"));
        assert!(recorder.record(mutation));
    }
    inverse.reverse();
    inverse
}

fn main() {
    let input = include_str!("../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧫️fixtures/clipboard-topology.json");
    let before = pack_json::parse(input, pack_json::JsonMemberPolicy::Reject).expect("fixture");
    let oracle: serde_json::Value = serde_json::from_str(input).expect("external fixture decoder");
    let fragment = puzzle2d_copy_fragment_from(&before, &["a".into(), "b".into()]).expect("copy");
    let decoded: serde_json::Value = serde_json::from_str(&fragment.dsl_text).expect("external clipboard decoder");
    assert_eq!(decoded["nodes"][0]["id"], "a");
    assert_eq!(decoded["nodes"][1]["id"], "b");
    assert_eq!(decoded["edges"], oracle["edges"]);
    assert!(puzzle2d_selection_is_locked(&before, &["a".into()]));
    assert!(puzzle2d_selection_is_locked(&before, &["b".into()]));
    assert!(!puzzle2d_selection_is_locked(&before, &["c".into()]));

    let action = pack_json::json!({ "anchor": "centroid", "position": [100.0, -10.0, 0.0] });
    let placement = PastePlacement::from_action_args(&pack_json::to_dsl_value(&action)).expect("placement");
    let base = Puzzle2dPlaySnapshot::new(Default::default());
    let (mutations, ids) = puzzle2d_paste_operations_on(&base, &fragment, &placement).expect("paste");
    assert_eq!(ids.len(), 2);
    let mut recorder = Puzzle2dRecorder::new(&base);
    let undo_paste = record_with_inverse(&mut recorder, mutations);
    let pasted = recorder.typed();
    assert_eq!(pasted.nodes.len(), 2);
    for (index, ((node, id), [x, y])) in pasted.nodes.iter().zip(ids).zip([[99.0, -11.0], [101.0, -9.0]]).enumerate() {
        assert!(node.id.eq_str(id.as_str()));
        assert!(!matches!(id.as_str(), "a" | "b"));
        assert_eq!((node.x, node.y), (x, y));
        assert!(node.text.as_ref().unwrap().eq_str(oracle["labeling"]["implicit"][index].as_str().unwrap()));
    }
    let endpoints = oracle["endpoints"].as_array().expect("endpoint oracle");
    assert_eq!(pasted.edges.len(), endpoints.len());
    assert_ne!(pasted.nodes[0].handles[0].id, pasted.nodes[0].handles[1].id);
    let handle = |point: &serde_json::Value| {
        let node = point[0].as_u64().unwrap() as usize;
        let port = point[1].as_u64().unwrap() as usize;
        &pasted.nodes[node].handles[port].id
    };
    for (index, edge) in pasted.edges.iter().enumerate() {
        assert_eq!(&edge.source, handle(&endpoints[index][0]));
        assert_eq!(&edge.target, handle(&endpoints[index][1]));
        assert_eq!(Some(edge.gap), oracle["edges"][index]["gap"].as_f64());
        assert_eq!(edge.locked, oracle["edges"][index]["locked"].as_bool());
    }
    for mutation in undo_paste {
        assert!(recorder.record(mutation));
    }
    assert_eq!(recorder.typed(), base.typed());

    let board = pack_json::json!({
        "schema": base.typed().schema.to_string_owner(), "nodes": before["nodes"], "edges": before["edges"]
    });
    let cut_base: Puzzle2dPlaySnapshot = value::FromValue::from_value(pack_json::to_dsl_value(&board)).expect("typed original board");
    let cut_ids: Vec<String> = serde_json::from_value(oracle["cutAllowed"].clone()).unwrap();
    let cut = puzzle2d_cut_operations_from(&cut_base, &cut_ids);
    assert_eq!(cut.len(), cut_ids.len());
    let mut recorder = Puzzle2dRecorder::new(&cut_base);
    let undo_cut = record_with_inverse(&mut recorder, cut);
    assert_eq!(recorder.typed().nodes.len(), cut_base.typed().nodes.len() - cut_ids.len());
    assert_eq!(recorder.typed().edges, cut_base.typed().edges);
    for mutation in undo_cut {
        assert!(recorder.record(mutation));
    }
    assert_eq!(recorder.typed(), cut_base.typed());
    println!("2D clipboard preserved relative placement, distinct handle endpoints, edge data and incident locks");
}
