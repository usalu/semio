//! 🖼️ Materializes the pinned SDK input through the native bitmap and journal owners.

use semio_framework_artifact_reference::ArtifactDialect;
use semio_os_store_text_io::text_format::{empty_document_spr, stamp_document_spr_identity};
use semio_s_artifact_wfc_bitmap_snapshot::{BitmapSnapshot, standards::v1::subsets::any::io::{binary::snapshot::{encode, decode}, text::snapshot::bitmap_json_decode}};
use std::{io::Read, path::PathBuf};

fn main() {
    let Some(folder) = std::env::args_os().nth(1) else { return };
    let id = std::env::args().nth(2).expect("artifact id");
    let mut json = String::new();
    std::io::stdin().read_to_string(&mut json).unwrap();
    let snapshot: BitmapSnapshot = bitmap_json_decode(&json).expect("valid owner JSON");
    if std::env::args().nth(3).as_deref()==Some("--solve") {
        let result=semio_s_artifact_wfc_bitmap_inferences::host::inferences::solve_with_job(&snapshot);
        let commit=result.expect("actual native WFC solve");
        println!("{}",semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot::bitmap_json_encode(&commit));
    }
    let pack = encode(&snapshot);
    assert_eq!(decode(&pack).unwrap(), snapshot);
    let schema = snapshot.schema.as_str();
    let dialect = ArtifactDialect {artifact_kind:schema.into(), standard:"1".into(), subset:"*".into()};
    let spr = semio_framework_async::block_on(async {
        let empty = empty_document_spr(&id, schema).await;
        stamp_document_spr_identity(&empty,&id,schema,&dialect,None).await.unwrap()
    });
    let folder = PathBuf::from(folder);
    std::fs::write(folder.join("PACK"),pack).unwrap();
    std::fs::write(folder.join("SPR"),spr).unwrap();
}
