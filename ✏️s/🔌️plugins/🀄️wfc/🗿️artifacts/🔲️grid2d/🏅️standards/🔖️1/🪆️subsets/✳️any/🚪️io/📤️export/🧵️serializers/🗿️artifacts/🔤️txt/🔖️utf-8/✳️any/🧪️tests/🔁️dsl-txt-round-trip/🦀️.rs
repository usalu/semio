use crate::Grid2dSnapshot;
use semio_os_kernel::io::io_mechanism::ArchiveChildren;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::Grid2dIntoTxt;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::TxtIntoGrid2d;
use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload;

/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    let snapshot = <crate::Grid2dSnapshot as Default>::default();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 16 << 20, maximum_capacity_bytes: 16 << 20, maximum_release_bytes: 16 << 20, maximum_depth: 512 };
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, grant);
    let exported = Grid2dIntoTxt::serialize(&snapshot, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = TxtIntoGrid2d::deserialize(&IoPayload::Text(text), &mut control).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}
