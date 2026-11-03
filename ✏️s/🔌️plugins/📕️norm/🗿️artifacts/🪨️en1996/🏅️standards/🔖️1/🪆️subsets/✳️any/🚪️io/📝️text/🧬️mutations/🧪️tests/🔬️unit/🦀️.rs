use crate::text::*;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1996_model::MortarClass;
use semio_s_artifact_norm_en1996_mut_project::change_annex;
use semio_s_artifact_norm_en1996_mut_mortar::change_mortar_class;
use semio_s_artifact_norm_en1996_mut_walls::{change_support_sides, change_wall_height, change_wall_thickness, insert_wall, remove_wall};
use semio_s_artifact_norm_en1996_mut_units::change_unit_fb;
use semio_s_artifact_norm_en1996_mut_design_cases::{change_case_entry_id, change_case_label_de, change_case_label_en, insert_case, remove_case, change_case_number, change_case_text};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

fn every_mutation() -> Vec<En1996Mutation> {
    let wall = En1996Snapshot::compliant_clay_wall().walls[0].clone();
    let wall_id = wall.id.clone();
    let case = En1996Snapshot::compliant_clay_wall().cases[0].clone();
    vec![
        En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1996Mutation::InsertWall(insert_wall::InsertWall { index: 1, wall }),
        En1996Mutation::RemoveWall(remove_wall::RemoveWall { id: wall_id.clone() }),
        En1996Mutation::ChangeWallThickness(change_wall_thickness::ChangeWallThickness { id: wall_id.clone(), new_thickness_m: 0.49 }),
        En1996Mutation::ChangeWallHeight(change_wall_height::ChangeWallHeight { id: wall_id.clone(), new_height_m: 2.5 }),
        En1996Mutation::ChangeUnitFb(change_unit_fb::ChangeUnitFb { id: wall_id.clone(), new_f_b_pa: 28e6 }),
        En1996Mutation::ChangeMortarClass(change_mortar_class::ChangeMortarClass { id: wall_id.clone(), new_mortar_class: MortarClass::M20 }),
        En1996Mutation::ChangeSupportSides(change_support_sides::ChangeSupportSides { id: wall_id.clone(), new_support_sides: 4 }),        En1996Mutation::InsertCase(insert_case::InsertCase { index: 0, case: case.clone() }),
        En1996Mutation::RemoveCase(remove_case::RemoveCase { id: "case-1".into() }),
        En1996Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: case.id.clone(), key: "la_m".into(), value: Some(3.0) }),
        En1996Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: case.id.clone(), key: "la_m".into(), value: None }),
        En1996Mutation::ChangeCaseText(change_case_text::ChangeCaseText { case_id: case.id.clone(), key: "support".into(), value: Some("simple".into()) }),
        En1996Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id: case.id.clone(), new_value: "Arching".into() }),
        En1996Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id: case.id.clone(), new_value: "Gewölbewirkung".into() }),
        En1996Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id: case.id.clone(), new_value: "M-32".into() }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_annex() {
    store::os_store::test_support::assert_op_line_round_trip(&En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }));
}

#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}
