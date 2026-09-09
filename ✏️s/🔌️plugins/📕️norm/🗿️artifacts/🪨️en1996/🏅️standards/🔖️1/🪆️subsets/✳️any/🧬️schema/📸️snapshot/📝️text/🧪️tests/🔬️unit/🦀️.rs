use super::*;
use crate::document::{AnnexChoice, DesignSituation};
use crate::part_2;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1996Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn loadbearing_wall_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1996_LOADBEARING_WALL_EXAMPLE_TEXT).expect("parse loadbearing wall example");
    assert_eq!(document.annex, AnnexChoice::En);
    assert_eq!(document.masonry_class, crate::MasonryClass::Class2);
    assert_eq!(document.design_situation, DesignSituation::Transient);
    assert_eq!(document.exposure, part_2::ExposureClass::Mx3);
    assert_eq!(document.mortar, part_2::MortarClass::M10);
    assert_eq!(document.storeys, 4);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
