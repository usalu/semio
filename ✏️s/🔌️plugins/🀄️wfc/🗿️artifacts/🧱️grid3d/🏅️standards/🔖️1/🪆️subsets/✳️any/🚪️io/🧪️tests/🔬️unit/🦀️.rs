use super::*;
use crate::examples::blocks;
use crate::Grid3dSnapshot;
use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeSource, Dialect, StandardId, SubsetId};

#[test]
fn io_declares_the_stdio_txt_channel_and_no_foreign_entries() {
    let declaration = crate::standards::v1::subsets::any::io();
    assert!(declaration.entries.is_empty(), "grid3d ships no foreign hop of its own");
    assert_eq!(import_stdio_kinds(), &["stdio.txt"]);
    assert_eq!(export_stdio_kinds(), &["stdio.txt"]);
    assert!(declaration.native.snapshot.text.is_some());
    assert!(declaration.native.snapshot.binary.is_some());
}

#[test]
fn the_derived_composition_rebuilds_blocks_from_the_native_dialect() {
    let document = blocks::snapshot();
    let pack = <Grid3dSnapshot as store::ArtifactPack>::encode_pack(&document);
    let dialect = Dialect { artifact_kind: "s.wfc.grid3d", standard: StandardId("1"), subset: SubsetId("*") };
    let sources = [ComposeSource { dialect, payload: AnalyzeSource::Binary(&pack) }];
    let composition = Grid3dComposerComposition::compose(&sources).expect("native dialect composes");
    assert_eq!(composition.snapshot.seed, document.seed);
    assert_eq!(composition.snapshot.width, document.width);
    assert_eq!(composition.snapshot.tiles.len(), document.tiles.len());
    assert_eq!(composition.snapshot.rules.len(), document.rules.len());
}
