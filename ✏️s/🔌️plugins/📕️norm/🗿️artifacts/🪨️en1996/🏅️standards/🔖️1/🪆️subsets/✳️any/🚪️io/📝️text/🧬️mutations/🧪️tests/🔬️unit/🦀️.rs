use crate::mutations::*;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1996_model::MortarClass;
use semio_s_artifact_norm_en1996_mut_project::change_annex;
use semio_s_artifact_norm_en1996_mut_mortar::change_mortar_class;
use semio_s_artifact_norm_en1996_mut_walls::{change_support_sides, change_wall_height, change_wall_thickness, insert_wall, remove_wall};
use semio_s_artifact_norm_en1996_mut_units::change_unit_fb;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

fn every_mutation() -> Vec<En1996Mutation> {
    let wall = En1996Snapshot::compliant_clay_wall().walls[0].clone();
    vec![
        En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1996Mutation::InsertWall(insert_wall::InsertWall { index: 1, wall }),
        En1996Mutation::RemoveWall(remove_wall::RemoveWall { index: 0 }),
        En1996Mutation::ChangeWallThickness(change_wall_thickness::ChangeWallThickness { index: 0, new_thickness_m: 0.49 }),
        En1996Mutation::ChangeWallHeight(change_wall_height::ChangeWallHeight { index: 0, new_height_m: 2.5 }),
        En1996Mutation::ChangeUnitFb(change_unit_fb::ChangeUnitFb { index: 0, new_f_b_pa: 28e6 }),
        En1996Mutation::ChangeMortarClass(change_mortar_class::ChangeMortarClass { index: 0, new_mortar_class: MortarClass::M20 }),
        En1996Mutation::ChangeSupportSides(change_support_sides::ChangeSupportSides { index: 0, new_support_sides: 4 }),
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
