#!/usr/bin/env python3
"""🖼️ R10 item 2 (row 1.10, oracle conflict `rust:png`): the os host encodes its rasterized SVG with the framework's own
PNG codec (`semio-framework-pixels::encode_png`, RFC 1950/1951 via `semio-framework-deflate`) instead of the third-party
`png` crate, which stays only as the dev-dependency oracle of a new native law. Guest-linked tree (os host is built for
the browser wasm32 target) → prepared patch, lands in window 3; idempotent.

Usage: python3 owned-png-host.py [--apply] [--root <repo or overlay root>]
"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
HOST = ROOT / "🧰️framework/🛍️products/💻️os/🖥️host"
SOURCE = HOST / "🦀️.rs"
MANIFEST = HOST / "📦️packages/🦀️rust/Cargo.toml"
LAW_DIR = HOST / "🧪️tests/🔬️media-export-raster-unit"
LAW = LAW_DIR / "🦀️.rs"

SOURCE_EDITS = [
    (
        '    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]\n    use png::{BitDepth, ColorType, Encoder};\n    use serde_json::Value;\n',
        "    use serde_json::Value;\n",
    ),
    (
        """    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        {
            let mut encoder = Encoder::new(&mut bytes, width, height);
            encoder.set_color(ColorType::Rgba);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().map_err(|error| error.to_string())?;
            writer.write_image_data(pixels).map_err(|error| error.to_string())?;
        }
        Ok(bytes)
    }
""",
        """    /// 📤️ Encodes the renderer's RGBA8 pixmap with the framework's own PNG codec.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage { width, height, pixels: pixels.to_vec() }).map_err(|error| error.to_string())
    }
""",
    ),
    (
        """    #[cfg(all(test, target_arch = "wasm32", target_env = "p2"))]
    include!("🧪️tests/🔬️media-export-raster-wasip2/🦀️.rs");
""",
        """    #[cfg(all(test, target_arch = "wasm32", target_env = "p2"))]
    include!("🧪️tests/🔬️media-export-raster-wasip2/🦀️.rs");

    #[cfg(all(test, not(target_arch = "wasm32")))]
    include!("🧪️tests/🔬️media-export-raster-unit/🦀️.rs");
""",
    ),
]

MANIFEST_EDITS = [
    ('png = "0.17.16"\nresvg = "0.45.1"\n', 'semio-framework-pixels = { path = "../../../../../🔨️modules/🔲️pixels/📦️packages/🦀️rust" }\nresvg = "0.45.1"\n'),
    ('[dev-dependencies]\n', '[dev-dependencies]\npng = "0.17.16"\n'),
]

LAW_TEXT = '''mod native_raster_tests {
    use super::*;

    /// 🖼️ The owned PNG encoder behind `rasterize_svg_to_png_base64` writes a stream the third-party `png` crate (the
    /// oracle) decodes to exactly what the renderer drew: a 4×2 opaque red rectangle is 4×2 RGBA8, every pixel
    /// (255, 0, 0, 255).
    #[test]
    fn rasterized_svg_png_decodes_to_the_rendered_pixels_under_the_png_crate() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="4" height="2"><rect width="4" height="2" fill="#ff0000"/></svg>"##;
        let bytes = base64_codec::base64_standard_decode(rasterize_svg_to_png_base64(svg, 4, 2).expect("the native host renders SVG")).expect("a base64 payload");
        let mut reader = png::Decoder::new(std::io::Cursor::new(bytes)).read_info().expect("the oracle reads the header");
        let mut pixels = vec![0; reader.output_buffer_size()];
        let frame = reader.next_frame(&mut pixels).expect("the oracle inflates the IDAT stream");
        assert_eq!((frame.width, frame.height, frame.color_type, frame.bit_depth), (4, 2, png::ColorType::Rgba, png::BitDepth::Eight));
        assert!(pixels[..frame.buffer_size()].chunks(4).all(|pixel| pixel == [255, 0, 0, 255]));
    }
}
'''


def plan(path, edits):
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if new in text and old not in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"{path.name}: anchor found {text.count(old)} times: {old[:60]!r}")
        text = text.replace(old, new)
    return text


def main():
    apply = "--apply" in sys.argv
    source = plan(SOURCE, SOURCE_EDITS)
    manifest = plan(MANIFEST, MANIFEST_EDITS)
    law_present = LAW.exists() and LAW.read_text(encoding="utf-8") == LAW_TEXT
    print(f"source: {'unchanged' if source == SOURCE.read_text(encoding='utf-8') else 'edits ready'}; manifest: {'unchanged' if manifest == MANIFEST.read_text(encoding='utf-8') else 'edits ready'}; law: {'present' if law_present else 'to write'}")
    if "use png::" in source or "png::" in source.split("mod native_raster_tests")[0].replace("semio_framework_pixels", ""):
        raise SystemExit("a production png:: use survives")
    if not apply:
        print("dry run clean")
        return
    SOURCE.write_text(source, encoding="utf-8")
    MANIFEST.write_text(manifest, encoding="utf-8")
    LAW_DIR.mkdir(parents=True, exist_ok=True)
    LAW.write_text(LAW_TEXT, encoding="utf-8")
    print("applied")


main()
