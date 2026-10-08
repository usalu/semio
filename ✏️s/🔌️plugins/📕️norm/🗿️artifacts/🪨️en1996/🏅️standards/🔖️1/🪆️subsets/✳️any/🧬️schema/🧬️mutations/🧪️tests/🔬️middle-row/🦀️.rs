//! 🧪️ Ordered-collection law fixtures: removing or inserting a MIDDLE row restores that row at its original index, and the
//! inverse diffs sum to the negative of the forward diff, for every list the document owns.

use super::*;
use crate::standards::v1::subsets::any::io::{apply_en1996_mutation, inverse_en1996_mutation};
use crate::{ConcentratedLoad, WallOpening};

fn three_row_document() -> En1996Snapshot {
    let mut base = En1996Snapshot::compliant_clay_wall();
    let template = base.walls[0].clone();
    for id in ["wall-b", "wall-c"] {
        let mut wall = template.clone();
        wall.id = id.into();
        base.walls.push(wall);
    }
    for wall in &mut base.walls {
        for id in ["o1", "o2", "o3"] {
            wall.openings.push(WallOpening { id: id.into(), width_m: 1.0, height_m: 1.0, sill_height_m: 0.5 });
        }
        for id in ["c1", "c2", "c3"] {
            wall.load_cases[0].concentrated.push(ConcentratedLoad { id: id.into(), force_n: 1.0, bearing_area_m2: 0.1, bearing_length_m: 0.2 });
        }
        let template = wall.load_cases[0].clone();
        for id in ["lc2", "lc3"] {
            let mut load_case = template.clone();
            load_case.id = id.into();
            wall.load_cases.push(load_case);
        }
    }
    base
}

#[semio_framework_async_macros::async_test]
async fn removing_a_middle_row_obeys_the_inverse_sum_law() {
    let base = three_row_document();
    let removals = [
        En1996Mutation::RemoveWall(remove_wall::RemoveWall { index: 1 }),
        En1996Mutation::RemoveOpening(remove_opening::RemoveOpening { wall_index: 1, index: 1 }),
        En1996Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { wall_index: 1, index: 1 }),
        En1996Mutation::RemoveConcentrated(remove_concentrated::RemoveConcentrated { wall_index: 1, load_case_index: 0, index: 1 }),
    ];
    for mutation in &removals {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &base).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_row_obeys_the_inverse_sum_law() {
    let base = three_row_document();
    let mut wall = base.walls[0].clone();
    wall.id = "wall-new".into();
    let insertions = [
        En1996Mutation::InsertWall(insert_wall::InsertWall { index: 1, wall }),
        En1996Mutation::InsertOpening(insert_opening::InsertOpening { wall_index: 1, index: 1, opening: WallOpening { id: "o-new".into(), width_m: 0.9, height_m: 2.1, sill_height_m: 0.0 } }),
        En1996Mutation::InsertConcentrated(insert_concentrated::InsertConcentrated { wall_index: 1, load_case_index: 0, index: 1, load: ConcentratedLoad { id: "c-new".into(), force_n: 5.0, bearing_area_m2: 0.1, bearing_length_m: 0.2 } }),
    ];
    for mutation in &insertions {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &base).await;
    }
}

#[test]
fn a_removed_middle_row_returns_to_its_original_index() {
    let base = three_row_document();
    let removal = En1996Mutation::RemoveWall(remove_wall::RemoveWall { index: 1 });
    let (after, _) = apply_en1996_mutation(&base, &removal).expect("removal applies");
    let restored = inverse_en1996_mutation(&removal, &base).expect("inverse").iter().fold(after, |document, step| apply_en1996_mutation(&document, step).expect("inverse applies").0);
    assert_eq!(restored, base);
    assert_eq!(restored.walls[1].id, "wall-b");
}
