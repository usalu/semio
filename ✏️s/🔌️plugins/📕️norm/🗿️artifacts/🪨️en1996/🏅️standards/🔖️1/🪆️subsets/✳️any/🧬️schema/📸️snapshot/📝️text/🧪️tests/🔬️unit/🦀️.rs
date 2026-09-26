use super::*;
use crate::document::{AnnexChoice, DesignSituation};

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1996Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn loadbearing_wall_example_fixture_parses_and_round_trips() {
    let from_default = print_dsl(&En1996Snapshot::compliant_clay_wall());
    let document = parse_dsl(&from_default).expect("parse printed compliant wall");
    assert_eq!(document.annex, AnnexChoice::De);
    assert_eq!(document.masonry_class, crate::MasonryClass::Class1);
    assert_eq!(document.design_situation, DesignSituation::Persistent);
    assert_eq!(document.storeys, 2);
    assert_eq!(document.walls.len(), 1);
    assert_eq!(document.walls[0].id, "wall-north");
    store::os_store::test_support::assert_dsl_round_trip(&document);
    // Keep asset in sync with the printer (authoritative for the example pack).
    let _ = EN1996_LOADBEARING_WALL_EXAMPLE_TEXT;
}
