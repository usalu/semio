//! 🧪️ Example round trip, plus the raster laws only this example can prove.

use crate::schema::snapshot::Wfc2dTileMedia;
use crate::standards::v1::subsets::any::io::binary::snapshot::tile_media_png_data_url;

#[test]
fn printed_text_parses_back_to_the_authored_document() {
    let document = crate::examples::terrain_ring::document();
    let text = store::ArtifactDsl::print_dsl(&document);
    let parsed: crate::Wfc2dSnapshot = <crate::Wfc2dSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("the printed example parses");
    assert_eq!(parsed, document, "the printed text is not the document it was printed from");
}

#[test]
fn the_example_source_carries_the_printed_document() {
    let source = crate::examples::terrain_ring::source();
    assert_eq!(source.id(), crate::examples::terrain_ring::ID);
    assert!(source.document_json().len() > 8, "the example payload must not be empty");
}

/// 🖼️ Every tile really is an 8×8 palette-indexed bitmap — the branch no other example exercises.
#[test]
fn every_tile_is_an_indexed_bitmap() {
    let document = crate::examples::terrain_ring::document();
    assert_eq!(document.tiles.len(), 4);
    for tile in &document.tiles {
        let Wfc2dTileMedia::Bitmap { width, height, palette, pixels } = &tile.media else { panic!("{} is not a bitmap tile", tile.id) };
        assert_eq!((*width, *height), (crate::examples::terrain_ring::TILE_PIXELS, crate::examples::terrain_ring::TILE_PIXELS));
        assert_eq!(palette.len(), 2);
        assert!(!pixels.is_empty());
    }
}

/// 🎨 Each tile encodes to a real PNG data URL — not a label, not a placeholder.
#[test]
fn every_tile_encodes_to_a_png_data_url() {
    for tile in &crate::examples::terrain_ring::document().tiles {
        let url = tile_media_png_data_url(&tile.media).expect("a bitmap tile encodes");
        assert!(url.starts_with("data:image/png;base64,"), "{} did not encode to a png data url", tile.id);
        assert!(url.len() > 64, "{} encoded to a suspiciously short payload", tile.id);
    }
}

/// 🚫️ A malformed bitmap draws its outline rather than failing the whole surface refresh.
#[test]
fn a_malformed_bitmap_refuses_rather_than_panicking() {
    let short = Wfc2dTileMedia::Bitmap { width: 8, height: 8, palette: vec![Default::default()], pixels: "AAAA".into() };
    assert_eq!(tile_media_png_data_url(&short), None);
    let empty = Wfc2dTileMedia::Bitmap { width: 0, height: 0, palette: Vec::new(), pixels: String::new() };
    assert_eq!(tile_media_png_data_url(&empty), None);
    assert_eq!(tile_media_png_data_url(&Wfc2dTileMedia::Empty), None);
}

/// ⬡️ The raster ring still solves — the media kind is orthogonal to the propagation.
#[test]
fn the_terrain_ring_solves() {
    let commit = crate::host::inferences::solve_with_clock(&crate::examples::terrain_ring::document(), semio_framework_job::logical_now_us).expect("the terrain ring solves");
    assert!(!commit.contradiction);
    assert_eq!(commit.assignments.len(), 6);
}

/// 🎨️ The declared palette fixture reaches the actual PNG author and checked raster view.
#[test]
fn raster_preserves_clamped_palette_and_unknown_index_transparency() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🎨️raster/🔣️.json")).unwrap();
    let number = |value: &serde_json::Value| u32::try_from(value.as_u64().unwrap()).unwrap();
    let indices: Vec<u8> = fixture["indices"].as_array().unwrap().iter().map(|v| u8::try_from(v.as_u64().unwrap()).unwrap()).collect();
    let palette = fixture["palette"].as_array().unwrap().iter().map(|color| crate::schema::snapshot::Wfc2dColor { r: number(&color["r"]), g: number(&color["g"]), b: number(&color["b"]), a: number(&color["a"]) }).collect();
    let media = Wfc2dTileMedia::Bitmap { width: number(&fixture["width"]), height: number(&fixture["height"]), palette, pixels: base64_codec::base64_standard_encode(indices) };
    let url = tile_media_png_data_url(&media).expect("authored raster encodes through stdio PNG");
    let bytes = base64_codec::base64_standard_decode(url.strip_prefix("data:image/png;base64,").unwrap()).unwrap();
    let raster = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::project_png(&bytes).unwrap();
    assert_eq!((raster.width, raster.height), (number(&fixture["width"]), number(&fixture["height"])));
    let expected: Vec<u8> = fixture["rgba"].as_array().unwrap().iter().map(|v| u8::try_from(v.as_u64().unwrap()).unwrap()).collect();
    assert_eq!(raster.pixels, expected);
}
