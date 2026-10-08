//! 🧪️ Ordered-collection law fixtures: removing or inserting a MIDDLE row restores that row at its original index, and the
//! inverse diffs sum to the negative of the forward diff, for every list the document owns.

use super::*;
use crate::AccidentalCase;

fn three_row_document() -> En1991Snapshot {
    let mut base = En1991Snapshot::default();
    for id in ["b", "c"] {
        let mut floor = base.floors[0].clone();
        floor.id = format!("floor-{id}");
        base.floors.push(floor);
        let mut element = base.self_weight_elements[0].clone();
        element.id = format!("element-{id}");
        base.self_weight_elements.push(element);
        let mut roof = base.roofs[0].clone();
        roof.id = format!("roof-{id}");
        base.roofs.push(roof);
        let mut face = base.wind_faces[0].clone();
        face.id = format!("face-{id}");
        base.wind_faces.push(face);
    }
    base.accidental_cases = ["a", "b", "c"].into_iter().map(|id| AccidentalCase { id: format!("case-{id}"), impact: Vec::new(), explosion: Vec::new() }).collect();
    base
}

#[semio_framework_async_macros::async_test]
async fn removing_a_middle_row_obeys_the_inverse_sum_law() {
    let base = three_row_document();
    let removals = [
        En1991Mutation::RemoveFloors(remove_floors::RemoveFloors { index: 1 }),
        En1991Mutation::RemoveSelfWeightElements(remove_self_weight_elements::RemoveSelfWeightElements { index: 1 }),
        En1991Mutation::RemoveRoofs(remove_roofs::RemoveRoofs { index: 1 }),
        En1991Mutation::RemoveWindFaces(remove_wind_faces::RemoveWindFaces { index: 1 }),
        En1991Mutation::RemoveAccidentalCases(remove_accidental_cases::RemoveAccidentalCases { index: 1 }),
    ];
    for mutation in &removals {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &base).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_row_obeys_the_inverse_sum_law() {
    let base = three_row_document();
    let mut floor = base.floors[0].clone();
    floor.id = "floor-new".into();
    let mut roof = base.roofs[0].clone();
    roof.id = "roof-new".into();
    let insertions = [
        En1991Mutation::InsertFloors(insert_floors::InsertFloors { index: 1, item: floor }),
        En1991Mutation::InsertRoofs(insert_roofs::InsertRoofs { index: 1, item: roof }),
        En1991Mutation::InsertAccidentalCases(insert_accidental_cases::InsertAccidentalCases { index: 1, item: AccidentalCase { id: "case-new".into(), impact: Vec::new(), explosion: Vec::new() } }),
    ];
    for mutation in &insertions {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &base).await;
    }
}

#[test]
fn a_removed_middle_row_returns_to_its_original_index() {
    let base = three_row_document();
    let removal = En1991Mutation::RemoveFloors(remove_floors::RemoveFloors { index: 1 });
    let (after, _) = apply_en1991_mutation(&base, &removal).expect("removal applies");
    let restored = inverse_en1991_mutation(&removal, &base).expect("inverse").iter().fold(after, |document, step| apply_en1991_mutation(&document, step).expect("inverse applies").0);
    assert_eq!(restored, base);
    assert_eq!(restored.floors[1].id, "floor-b");
}
