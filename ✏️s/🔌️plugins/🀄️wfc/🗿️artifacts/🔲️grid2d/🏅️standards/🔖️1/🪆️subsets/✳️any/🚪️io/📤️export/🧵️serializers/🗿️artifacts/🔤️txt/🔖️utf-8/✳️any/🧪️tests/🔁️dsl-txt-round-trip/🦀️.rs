use super::Grid2dIntoTxt;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::TxtIntoGrid2d;
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload;

/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    let snapshot = <crate::Grid2dSnapshot as Default>::default();
    let exported = Grid2dIntoTxt::serialize(&snapshot).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = TxtIntoGrid2d::deserialize(&IoPayload::Text(text)).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}
