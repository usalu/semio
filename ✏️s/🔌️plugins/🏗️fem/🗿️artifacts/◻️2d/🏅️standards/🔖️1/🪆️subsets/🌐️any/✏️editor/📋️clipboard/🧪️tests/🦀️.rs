use super::*;
use semio_plugin::AppOperationContext;
use semio_plugin::HistoryView;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring;

fn demo() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("demo snapshot"))
}

fn created(mutations: &[Fem2dMutation]) -> Vec<String> {
    mutations.iter().filter_map(|mutation| match mutation {
        Fem2dMutation::CreateNode(create) => Some(create.node.id.clone()),
        Fem2dMutation::CreateElement(create) => Some(element_id(&create.element).to_string()),
        Fem2dMutation::CreateRegion(create) => Some(create.region.id.clone()),
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
fn copy_then_paste_adds_the_selected_member_under_fresh_ids_2d() {
    let snapshot = demo();
    let element = snapshot.elements.first().expect("demo member");
    let fragment = fem2d_copy_fragment(&snapshot, &[target(FEM2D_GRANULARITY_ELEMENT, element_id(element))]).expect("copy");
    assert_eq!(fragment.schema, FEM_2D_SCHEMA);
    let history = HistoryView::empty();
    let placement = PastePlacement::default();
    let first = fem2d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("a")), &fragment, &placement).expect("paste");
    let second = fem2d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("b")), &fragment, &placement).expect("paste again");
    let (first_ids, second_ids) = (created(&first), created(&second));
    assert_eq!(first_ids.len(), 3);
    assert!(first_ids.iter().chain(second_ids.iter()).all(|id| snapshot.nodes.iter().all(|node| node.id.as_str() != id) && snapshot.elements.iter().all(|row| element_id(row) != id)));
    assert!(first_ids.iter().all(|id| !second_ids.contains(id)));
    let Some(Fem2dMutation::CreateElement(pasted)) = first.last() else { panic!("the member is created after its nodes") };
    let (start, end) = fem2d_element_endpoints(&pasted.element);
    assert!(first_ids.iter().any(|id| id == start) && first_ids.iter().any(|id| id == end));
}

/// 📋️ LAW: a placement position moves the pasted nodes so their bounds are centered on it, and an empty selection copies nothing.
#[test]
fn paste_centers_on_the_placement_position_and_an_empty_selection_copies_nothing_2d() {
    let snapshot = demo();
    assert!(matches!(fem2d_copy_fragment(&snapshot, &[]), Err(ClipboardError::EmptySelection)));
    let node = snapshot.nodes.iter().next().expect("demo node");
    let fragment = fem2d_copy_fragment(&snapshot, &[target(FEM2D_GRANULARITY_NODE, node.id.as_str())]).expect("copy");
    let history = HistoryView::empty();
    let placement = PastePlacement { anchor: PasteAnchor::Middle, position: Some([10.0, 20.0, 0.0]) };
    let pasted = fem2d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("c")), &fragment, &placement).expect("paste");
    let [Fem2dMutation::CreateNode(create)] = pasted.as_slice() else { panic!("one node pasted") };
    assert_eq!((create.node.x, create.node.y), (10.0, 20.0));
}

/// 📋️ LAW: a copied region pastes under a fresh id with its outline and holes moved so its bounds are centered on the placement, keeping every other field.
#[test]
fn paste_moves_a_region_and_keeps_its_fields_2d() {
    let snapshot = demo();
    let region = snapshot.regions.first().expect("demo region").clone();
    let fragment = fem2d_copy_fragment(&snapshot, &[target(FEM2D_GRANULARITY_REGION, &region.id)]).expect("copy a region");
    let history = HistoryView::empty();
    let placement = PastePlacement { anchor: PasteAnchor::Middle, position: Some([10.0, 20.0, 0.0]) };
    let pasted = fem2d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("r")), &fragment, &placement).expect("paste");
    let [Fem2dMutation::CreateRegion(create)] = pasted.as_slice() else { panic!("one region pasted") };
    let moved = &create.region;
    assert_ne!(moved.id, region.id);
    assert_eq!((moved.name.as_str(), moved.thickness, moved.mesh_size, moved.material_id.as_str()), (region.name.as_str(), region.thickness, region.mesh_size, region.material_id.as_str()));
    let bounds = |axis: usize| moved.outline.iter().map(|point| point[axis]).fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| (low.min(value), high.max(value)));
    assert_eq!([0, 1].map(|axis| (bounds(axis).0 + bounds(axis).1) * 0.5), [10.0, 20.0]);
}

/// ✂️ LAW: cut removes exactly what copy copied (a member, a region), pasting what was cut re-creates it under fresh ids, and removing the pasted ids restores the document (the revert a history undo performs).
#[test]
fn cut_then_paste_recreates_the_member_and_removing_the_paste_reverts_it_2d() {
    let snapshot = demo();
    let element = snapshot.elements.first().expect("demo member");
    let region = snapshot.regions.first().expect("demo region");
    let selected = [target(FEM2D_GRANULARITY_ELEMENT, element_id(element)), target(FEM2D_GRANULARITY_REGION, &region.id)];
    let fragment = fem2d_copy_fragment(&snapshot, &selected).expect("copy");
    let history = HistoryView::empty();
    let cut = fem2d_cut_operations(&ArtifactView::new(&snapshot, &history), &selected);
    assert!(cut.iter().any(|mutation| matches!(mutation, Fem2dMutation::DeleteElement(removed) if removed.id == element_id(element))));
    assert!(cut.iter().any(|mutation| matches!(mutation, Fem2dMutation::DeleteRegion(removed) if removed.id == region.id)));
    let pasted = fem2d_paste_operations(&ArtifactView::with_operation(&snapshot, &history, operation("p")), &fragment, &PastePlacement::default()).expect("paste");
    let mut document = Fem2dSnapshotRetiring::new((*snapshot).clone());
    for mutation in &pasted {
        let next = Fem2dSnapshotRetiring::new(semio_os_kernel::apply_mutation(&*document, mutation).expect("apply paste").0);
        document = next;
    }
    assert_eq!((document.elements.len(), document.regions.len()), (snapshot.elements.len() + 1, snapshot.regions.len() + 1));
    let identities = created(&pasted).iter().map(|id| target(if id.starts_with('e') { FEM2D_GRANULARITY_ELEMENT } else if id.starts_with('r') { FEM2D_GRANULARITY_REGION } else { FEM2D_GRANULARITY_NODE }, id)).collect::<Vec<_>>();
    let removal = fem2d_cut_operations(&ArtifactView::new(&document, &history), &identities);
    for mutation in &removal {
        let next = Fem2dSnapshotRetiring::new(semio_os_kernel::apply_mutation(&*document, mutation).expect("apply removal").0);
        document = next;
    }
    assert_eq!(*document, *snapshot);
}
