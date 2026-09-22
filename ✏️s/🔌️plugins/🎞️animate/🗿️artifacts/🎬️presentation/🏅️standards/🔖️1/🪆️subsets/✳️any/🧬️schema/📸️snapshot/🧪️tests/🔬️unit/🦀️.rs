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

/// 🖼️ The demo figure must be a file this repository actually ships under the shared asset root that
/// `semioAssetsVitePlugin` serves (and a release build copies) at `/🖼️assets/*`. The predecessor of
/// this law was a bare `/🖼️bauteilbörse.png` that existed nowhere: every host answered the figure
/// request with its SPA fallback (`200 text/html`), so the booted `demo` deck showed fifteen blank
/// tiles with no console error to explain it.
#[test]
fn the_demo_figure_source_names_a_shipped_asset() {
    const SEMIO_ASSET_ROUTE: &str = "/🖼️assets/";
    let source = crate::default_figure_tile_source();
    let relative = source.src.strip_prefix(SEMIO_ASSET_ROUTE).unwrap_or_else(|| panic!("the demo figure must live under the shared asset route {SEMIO_ASSET_ROUTE}, not at {}", source.src));
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../..");
    let path = repo_root.join("🧰️framework/🔨️modules/🖼️assets").join(relative);
    assert!(path.is_file(), "the demo figure {} resolves to {}, which this repository does not ship", source.src, path.display());
    assert_eq!(source.source_aspect, Some(crate::DEMO_FIGURE_PIXEL_WIDTH / crate::DEMO_FIGURE_PIXEL_HEIGHT), "the declared physical aspect must be the shipped figure's own pixel aspect");
}
