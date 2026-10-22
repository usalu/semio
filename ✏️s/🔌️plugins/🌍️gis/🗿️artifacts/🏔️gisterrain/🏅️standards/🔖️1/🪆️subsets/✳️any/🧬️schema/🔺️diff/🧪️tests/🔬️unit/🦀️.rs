use crate::GisTerrainSnapshot;
#[allow(unused_imports)]
use ::semio_framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use protocol::MutationDiff;
use crate::standards::v1::subsets::any::schema::diff::*;

#[semio_framework_async_macros::async_test]
async fn field_diffs_absorb_last_writer_wins_and_apply_onto_the_snapshot() {
    let base = GisTerrainSnapshot { exaggeration: 1.0, imported_map: None, ..Default::default() };
    let mut diff = GisTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
    diff.absorb(GisTerrainDiff { exaggeration: Some(3.0), imported_map: Some(crate::schema::diff::ImportedMapChange{value:None}), ..Default::default() });
    let next = protocol::apply_diff_cold(&diff, &base).expect("valid mutation diff");
    assert_eq!(next.exaggeration, 3.0);
    assert_eq!(next.imported_map, None);
}

