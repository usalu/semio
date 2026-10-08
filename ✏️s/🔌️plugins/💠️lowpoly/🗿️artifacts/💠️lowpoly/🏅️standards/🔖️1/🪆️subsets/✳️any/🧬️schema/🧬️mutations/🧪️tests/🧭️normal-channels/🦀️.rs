//! 🧭️ Selection motions keep transforming the named Normal attribute channels: the rows `rotate-selection` and `scale-selection` emit
//! carry exactly the channels the mesh kernel's own `rotate_vertices`/`scale_vertices` rewrite (the kernel is the independent
//! implementation), the central applier writes them before it recomputes the vertex normals, and the position-and-channel undo
//! restores the base.

use crate::mutations::{lowpoly_selection_motion_diff, lowpoly_selection_normal_channels, LowpolyMutation, LowpolySelectionMotion};
use crate::{LowpolyMeshAttribute, LowpolyMeshAttributeDomain, LowpolyMeshAttributeSemantic, LowpolyMeshState, LowpolySnapshot};
use semio_framework_3d::mesh::{Vec3, VertexId};
use semio_framework_value::DslValue;

const BEFORE: &str = include_str!("../../../../🧫️fixtures/🧬️mutations/🌀️rotate-selection/📐️lifts/📸️snapshot/⬅️before/🔣️.json");
const SELECTED: [u32; 3] = [0, 1, 2];

fn authored_channel(state: &LowpolyMeshState) -> LowpolyMeshAttribute {
    let normal = |x: f64, y: f64, z: f64| DslValue::Array(vec![DslValue::float(x), DslValue::float(y), DslValue::float(z)]);
    LowpolyMeshAttribute {
        name: "authored".into(),
        domain: LowpolyMeshAttributeDomain::Corner,
        semantic: LowpolyMeshAttributeSemantic::Normal,
        interpolation: crate::LowpolyMeshAttributeInterpolation::Linear,
        values: (0..state.halfedges.len()).map(|corner| if corner % 2 == 0 { normal(0.0, 1.0, 0.0) } else { normal(0.6, 0.0, 0.8) }).collect(),
        indices: None,
    }
}

fn base() -> LowpolySnapshot {
    let mut snapshot: LowpolySnapshot = crate::standards::v1::subsets::any::io::text::lowpoly_json_decode(BEFORE).expect("fixture decodes");
    let object = snapshot.objects.iter_mut().find(|object| object.mesh_state.is_some()).expect("a meshed object");
    let state = object.mesh_state.as_mut().expect("mesh state");
    let channel = authored_channel(state);
    state.attributes.push(channel);
    snapshot
}

fn meshed(snapshot: &LowpolySnapshot) -> (&str, &LowpolyMeshState) {
    let object = snapshot.objects.iter().find(|object| object.mesh_state.is_some()).expect("a meshed object");
    (object.id.as_str(), object.mesh_state.as_ref().expect("mesh state"))
}

fn kernel_after(state: &LowpolyMeshState, edit: impl Fn(&mut semio_framework_3d::mesh::HalfedgeMesh, &[VertexId])) -> LowpolyMeshState {
    let mut mesh = state.clone().into_mesh().expect("the base mesh decodes");
    let ids: Vec<VertexId> = SELECTED.iter().map(|id| VertexId(*id)).collect();
    edit(&mut mesh, &ids);
    LowpolyMeshState::from_mesh(mesh)
}

fn assert_matches_kernel(motion: LowpolySelectionMotion, edit: impl Fn(&mut semio_framework_3d::mesh::HalfedgeMesh, &[VertexId])) {
    let snapshot = base();
    let (id, state) = meshed(&snapshot);
    let expected = kernel_after(state, edit);
    let channels = lowpoly_selection_normal_channels(state, &SELECTED, &motion).expect("a clean motion");
    assert_eq!(channels.len(), 1, "exactly the one authored Normal channel is rewritten");
    assert_eq!(Some(&channels[0]), expected.attributes.iter().find(|held| held.name == "authored"), "the emitted channel equals the kernel's own rewrite");
    assert_ne!(Some(&channels[0]), state.attributes.iter().find(|held| held.name == "authored"), "the motion really changed the channel");
    let raised = lowpoly_selection_motion_diff(&snapshot, id, &SELECTED, &motion);
    assert!(raised.messages().iter().all(|message| message.code.0 != "mutation.invariant"), "the motion is clean");
    let after = protocol::apply_diff(raised.diff(), &snapshot).expect("the diff applies");
    let applied = after.objects.iter().find(|object| object.id == id).and_then(|object| object.mesh_state.clone());
    assert_eq!(applied, Some(expected), "applying the rows yields the kernel's mesh, channels and recomputed normals included");
}

/// ▶️ A turn rewrites the Normal channel exactly as `HalfedgeMesh::rotate_vertices` does.
#[test]
fn a_turn_rotates_the_named_normal_channel_like_the_kernel() {
    let (pivot, axis, angle) = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.7_f32);
    assert_matches_kernel(LowpolySelectionMotion::Turn { pivot, axis, angle }, |mesh, ids| mesh.rotate_vertices(ids, Vec3(axis), angle, Vec3(pivot)).expect("kernel rotation"));
}

/// ▶️ A stretch rewrites the Normal channel exactly as `HalfedgeMesh::scale_vertices` does.
#[test]
fn a_stretch_scales_the_named_normal_channel_like_the_kernel() {
    let (pivot, factor) = ([0.0, 0.0, 0.0], [2.0, 1.0, 0.5]);
    assert_matches_kernel(LowpolySelectionMotion::Stretch { pivot, factor }, |mesh, ids| mesh.scale_vertices(ids, Vec3(factor), Vec3(pivot)).expect("kernel scale"));
}

/// ▶️ An offset leaves the Normal channels alone, exactly as `HalfedgeMesh::move_vertices` does.
#[test]
fn an_offset_leaves_the_normal_channels_alone() {
    let snapshot = base();
    let (_, state) = meshed(&snapshot);
    assert_eq!(lowpoly_selection_normal_channels(state, &SELECTED, &LowpolySelectionMotion::Offset([1.0, 0.0, 0.0])), Some(Vec::new()));
}

/// ⚖️ The rotate and scale leaves still satisfy the inverse-sum law with a Normal channel present: the undo restores the channel.
#[semio_framework_async_macros::async_test]
async fn the_inverse_restores_the_normal_channels_and_sums_to_the_negative_diff() {
    let snapshot = base();
    let (id, _) = meshed(&snapshot);
    let rotate = LowpolyMutation::RotateSelection(crate::mutations::rotate_selection::RotateSelection { object_id: id.into(), vertex_ids: SELECTED.to_vec(), pivot: [0.0; 3], axis: [1.0, 0.0, 0.0], angle: 0.7 });
    let scale = LowpolyMutation::ScaleSelection(crate::mutations::scale_selection::ScaleSelection { object_id: id.into(), vertex_ids: SELECTED.to_vec(), pivot: [0.0; 3], factor: [2.0, 1.0, 0.5] });
    for mutation in [rotate, scale] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &snapshot).await;
        let forward = protocol::apply_diff(<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation, &snapshot).diff(), &snapshot).expect("forward applies");
        let mut restored = forward;
        for step in <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::inverse(&mutation, &snapshot).expect("inverse") {
            restored = protocol::apply_diff(<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&step, &restored).diff(), &restored).expect("inverse step applies");
        }
        assert_eq!(meshed(&restored).1.attributes, meshed(&snapshot).1.attributes, "the Normal channels are back to the base");
    }
}
