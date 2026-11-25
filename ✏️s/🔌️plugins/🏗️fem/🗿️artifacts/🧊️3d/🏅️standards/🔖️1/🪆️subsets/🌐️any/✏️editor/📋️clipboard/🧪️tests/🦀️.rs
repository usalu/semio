use super::*;
use semio_plugin::AppOperationContext;
use semio_plugin::HistoryView;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_GRANULARITY_SOLID;

#[path = "../../🕹️interaction/🧪️tests/♻️retiring/🦀️.rs"]
mod retiring;
use retiring::Retiring;

fn boot() -> Retiring<Fem3dSnapshot> {
    Retiring::new(semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::fem3d_boot_snapshot().expect("boot snapshot"))
}

fn created(mutations: &[Fem3dMutation]) -> Vec<String> {
    mutations.iter().filter_map(|mutation| match mutation {
        Fem3dMutation::CreateNode(create) => Some(create.node.id.clone()),
        Fem3dMutation::CreateElement(create) => Some(element_id(&create.element).to_string()),
        Fem3dMutation::CreateSolid(create) => Some(create.solid.id.clone()),
        _ => None,
    }).collect()
}

fn operation(seed: &str) -> AppOperationContext {
    AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: [0; 32], retained: Default::default(), authoring_seed: seed.into() }
}

fn target(kind: &str, id: &str) -> String {
    protocol::wire::encode_scoped_identity(&[kind, id]).expect("scoped identity")
}

/// 📋️ LAW: copying a selected member and pasting it adds the member and its end nodes under fresh ids that wire the pasted member, and pasting twice under two admissions never reuses an id.
#[test]
fn copy_then_paste_adds_the_selected_member_under_fresh_ids() {
    let snapshot = boot();
    let element = snapshot.elements.first().expect("boot member");
    let fragment = fem3d_copy_fragment(&snapshot, &[target(FEM3D_GRANULARITY_ELEMENT, element_id(element))]).expect("copy");
    assert_eq!(fragment.schema, FEM_3D_SCHEMA);
    let history = HistoryView::empty();
    let placement = PastePlacement::default();
    let first = fem3d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("a")), &fragment, &placement).expect("paste");
    let second = fem3d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("b")), &fragment, &placement).expect("paste again");
    let (first_ids, second_ids) = (created(&first), created(&second));
    assert_eq!(first_ids.len(), 3);
    assert!(first_ids.iter().chain(second_ids.iter()).all(|id| snapshot.nodes.iter().all(|node| node.id.as_str() != id) && snapshot.elements.iter().all(|row| element_id(row) != id)));
    assert!(first_ids.iter().all(|id| !second_ids.contains(id)));
    let Some(Fem3dMutation::CreateElement(pasted)) = first.last() else { panic!("the member is created after its nodes") };
    let (start, end) = fem3d_element_endpoints(&pasted.element);
    assert!(first_ids.iter().any(|id| id == start) && first_ids.iter().any(|id| id == end));
}

/// 📋️ LAW: a placement position moves the pasted nodes so their bounds are centered on it, and an empty selection copies nothing.
#[test]
fn paste_centers_on_the_placement_position_and_an_empty_selection_copies_nothing() {
    let snapshot = boot();
    assert!(matches!(fem3d_copy_fragment(&snapshot, &[]), Err(ClipboardError::EmptySelection)));
    let node = snapshot.nodes.iter().next().expect("boot node");
    let fragment = fem3d_copy_fragment(&snapshot, &[target(FEM3D_GRANULARITY_NODE, node.id.as_str())]).expect("copy");
    let history = HistoryView::empty();
    let placement = PastePlacement { anchor: PasteAnchor::Middle, position: Some([10.0, 20.0, 30.0]) };
    let pasted = fem3d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("c")), &fragment, &placement).expect("paste");
    let [Fem3dMutation::CreateNode(create)] = pasted.as_slice() else { panic!("one node pasted") };
    assert_eq!((create.node.x, create.node.y, create.node.z), (10.0, 20.0, 30.0));
}

fn solid_row(snapshot: &Fem3dSnapshot) -> FemSolid {
    let material = snapshot.materials.iter().next().expect("boot material").id.as_str().to_string();
    FemSolid { id: "sol_src".into(), name: "Source".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 4.0], [0.0, 4.0]], holes: vec![vec![[0.5, 0.5], [1.0, 0.5], [1.0, 1.0]]], base_z: 1.0, height: 6.0, layers: 3, mesh_size: 0.5, material_id: material, axis: FemAxis::X }
}

/// 📋️ LAW: a copied solid pastes under a fresh id with its footprint, holes and offset moved so the solid bounds are centered on the placement, keeping every other field, and the remapped axis moves the extrusion offset.
#[test]
fn paste_moves_a_solid_through_its_extrusion_axis() {
    let mut snapshot = boot();
    let solid = solid_row(&snapshot);
    snapshot.solids.push(solid.clone());
    let fragment = fem3d_copy_fragment(&snapshot, &[target(FEM3D_GRANULARITY_SOLID, "sol_src")]).expect("copy");
    let history = HistoryView::empty();
    let placement = PastePlacement { anchor: PasteAnchor::Middle, position: Some([10.0, 20.0, 30.0]) };
    let pasted = fem3d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("s")), &fragment, &placement).expect("paste");
    let [Fem3dMutation::CreateSolid(create)] = pasted.as_slice() else { panic!("one solid pasted") };
    let moved = &create.solid;
    assert_ne!(moved.id, solid.id);
    assert_eq!((moved.name.as_str(), moved.layers, moved.height, moved.mesh_size, moved.axis, moved.material_id.as_str()), (solid.name.as_str(), solid.layers, solid.height, solid.mesh_size, solid.axis, solid.material_id.as_str()));
    let corners = moved.corners();
    let center = |axis: usize| corners.iter().map(|point| point[axis]).fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| (low.min(value), high.max(value)));
    assert_eq!([0, 1, 2].map(|axis| (center(axis).0 + center(axis).1) * 0.5), [10.0, 20.0, 30.0]);
    assert_eq!(moved.holes[0].len(), 3);
    assert_eq!(moved.holes[0][0][0] - moved.outline[0][0], 0.5);
}

/// ✂️ LAW: cut removes exactly what copy copied, pasting what was cut re-creates it under fresh ids, and removing the pasted ids restores the document (the revert a history undo performs).
#[test]
fn cut_then_paste_recreates_the_member_and_removing_the_paste_reverts_it() {
    let snapshot = boot();
    let element = snapshot.elements.first().expect("boot member");
    let selected = [target(FEM3D_GRANULARITY_ELEMENT, element_id(element))];
    let fragment = fem3d_copy_fragment(&snapshot, &selected).expect("copy");
    let history = HistoryView::empty();
    let config = NoConfig::default();
    let cut = fem3d_cut_operations(&ArtifactView::new(&snapshot, &history), &ConfigView { snapshot: &config, window: None }, &selected);
    assert!(matches!(cut.as_slice(), [Fem3dMutation::DeleteElement(removed)] if removed.id == element_id(element)));
    let pasted = fem3d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("p")), &fragment, &PastePlacement::default()).expect("paste");
    let mut document = Retiring::new((*snapshot).clone());
    for mutation in &pasted {
        let next = Retiring::new(semio_os_kernel::apply_mutation(&*document, mutation).expect("apply paste").0);
        document = next;
    }
    assert_eq!((document.nodes.len(), document.elements.len()), (snapshot.nodes.len() + 2, snapshot.elements.len() + 1));
    let ids = created(&pasted);
    let identities = ids.iter().map(|id| target(if id.starts_with('e') { FEM3D_GRANULARITY_ELEMENT } else { FEM3D_GRANULARITY_NODE }, id)).collect::<Vec<_>>();
    let removal = fem3d_cut_operations(&ArtifactView::new(&document, &history), &ConfigView { snapshot: &config, window: None }, &identities);
    for mutation in &removal {
        let next = Retiring::new(semio_os_kernel::apply_mutation(&*document, mutation).expect("apply removal").0);
        document = next;
    }
    assert_eq!(*document, *snapshot);
}
