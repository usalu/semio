
#[semio_framework_async_macros::async_test]
async fn dump_example_dsl_and_pack_assets() {
    use crate::example_subjects::{de_office_compliant, multi_fail_noncompliant};
    use crate::standards::v1::subsets::any::schema::snapshot::{encode_en1991_dsl, encode_en1991_pack};
    use std::path::PathBuf;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/Nest");
    // resolve via relative from package
    let any = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/Nest");
}
