use super::*;

#[test]
fn pack_round_trips() {
    let snap = PresentationSnapshot::default();
    let bytes = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <PresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[test]
fn dsl_text_round_trips() {
    let snap = PresentationSnapshot::default();
    let text = <PresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <PresentationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[test]
fn populated_snapshot_pack_and_dsl_round_trip() {
    let source = crate::default_figure_tile_source();
    let tiles = vec![crate::FigureTileDraft { id: "t1".into(), name: "Tile One".into(), crop: crate::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }];
    let snap = crate::presentation_snapshot_with_tiles(&source, &tiles);
    let bytes = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <PresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);

    let text = <PresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <PresentationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}
