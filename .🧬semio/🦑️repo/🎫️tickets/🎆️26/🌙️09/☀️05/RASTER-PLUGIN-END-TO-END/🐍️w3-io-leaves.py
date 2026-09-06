#!/usr/bin/env python3
"""🐍️ W3 generator for the 18 raster io leaves (9 export serializers + 9 import deserializers).

Every leaf is either a REAL hop (composite -> stdio's own `s.stdio.semio/v1/image` hub -> that
format's own byte encoder, or the reverse) or an HONEST refusal returning a typed `Err` naming the
exact missing capability. No leaf ever prints raster's own DSL text under a foreign extension.

Run from the repo root:  python3 <this file>
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
IO = ROOT / "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
EXPORT = IO / "📤️export/🧵️serializers/🗿️artifacts"
IMPORT = IO / "📥️import/🧩️deserializers/🗿️artifacts"

LEAF = {
    "gif": ("🎞️gif/🔖️87a/✳️any", "🎞️gif/🔖️87a/✳️any"),
    "svg": ("🎨️svg/🔖️1.1/✳️any", "🎨️svg/🔖️1.1/✳️any"),
    "pdf": ("📖️pdf/🔖️1.4/✳️any", "📖️pdf/🔖️1.4/✳️any"),
    "png": ("📷️png/🔖️1.2/✳️any", "📷️png/🔖️1.2/✳️any"),
    "jpg": ("📸️jpg/🔖️jfif-1.01/♾️any", "📸️jpg/🔖️jfif-1.01/♾️any"),
    "dwg": ("🖊️dwg/🔖️ac1018/✳️any", "🖊️dwg/🔖️ac1018/✳️any"),
    "tiff": ("🖼️tiff/🔖️6.0/✳️any", "🖼️tiff/🔖️6.0/✳️any"),
    "bmp": ("🪟️bmp/🔖️v3/✳️any", "🪟️bmp/🔖️v3/✳️any"),
}

# ── real pixel formats that go straight through the semio/image hub ───────────────────────────────
HUB = {
    "png": dict(dialect="PNG_DIALECT", snapshot="semio_s_plugin_stdio::artifacts::png::PngSnapshot", encode="semio_s_plugin_stdio::artifacts::png::io::encode_png", decode="semio_s_plugin_stdio::artifacts::png::io::decode_png", err="", note="`encode_png` always re-emits canonical RGBA8 (color type 6, bit depth 8), so this hop is lossless in both directions."),
    "bmp": dict(dialect="BMP_DIALECT", snapshot="semio_s_plugin_stdio::artifacts::bmp::BmpSnapshot", encode="semio_s_plugin_stdio::artifacts::bmp::io::encode_bmp", decode="semio_s_plugin_stdio::artifacts::bmp::io::decode_bmp", err="", note="BMP v3 carries no alpha channel: stdio's own `encode_bmp` writes 24bpp `BI_RGB` rows and drops alpha. That loss is the FORMAT's, documented by that codec, not a shortcut taken here."),
    "tiff": dict(dialect="TIFF_DIALECT", snapshot="semio_s_plugin_stdio::artifacts::tiff::TiffSnapshot", encode="semio_s_plugin_stdio::artifacts::tiff::io::encode_tiff", decode="semio_s_plugin_stdio::artifacts::tiff::io::decode_tiff", err="", note="stdio's TIFF codec decodes/encodes IFD 0 as canonical RGBA8 strips."),
    "jpg": dict(dialect="JPG_DIALECT", snapshot="semio_s_plugin_stdio::artifacts::jpg::JpgSnapshot", encode="semio_s_plugin_stdio::artifacts::jpg::io::encode_jpg", decode="semio_s_plugin_stdio::artifacts::jpg::io::decode_jpg", err="jpg", note="JPEG is lossy by construction and carries no alpha; stdio's own codec forces alpha opaque on decode and re-quantizes on encode. Both are the FORMAT's losses, documented by that codec."),
}

EXPORT_HUB = '''//! 📤️ raster → {fmt} — REAL. The document's visible layer stack is flattened to one canonical
//! RGBA8 canvas by `raster_composite_image`, handed to stdio's own registered
//! `s.stdio.semio/v1/image` → `{kind}` serializer, and written by stdio's own byte encoder.
//! This plugin owns no {fmt} byte codec and never will.
//!
//! 🧾️ {note}
use crate::artifacts::raster::io::{{raster_composite_image, semio_image_to_format, {dialect}}};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {{}}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {{
    let image = raster_composite_image(snapshot).map_err(|reason| format!("{fmt} export not available for this raster document: {{reason}}"))?;
    let target: {snapshot} = semio_image_to_format(&image, {dialect})?;
    {encode_call}
}}
'''

IMPORT_HUB = '''//! 📥️ raster ← {fmt} — REAL. stdio's own byte decoder produces the typed `{kind}` snapshot, stdio's
//! own registered `{kind}` → `s.stdio.semio/v1/image` deserializer turns it into canonical RGBA8,
//! and that content becomes one `Pixel` layer with a materialized asset child. The incoming bytes
//! are genuinely read — nothing is fabricated.
//!
//! 🧾️ {note}
use crate::artifacts::raster::io::{{raster_document_from_semio_image, semio_image_from_format, {dialect}}};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {{}}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {{
    let decoded = {decode_call};
    let image = semio_image_from_format(&decoded, {dialect})?;
    raster_document_from_semio_image(&image, "{fmt}-import", "Imported {fmt}")
}}
'''

REFUSE = '''//! {arrow} raster {word} {fmt} — HONESTLY UNSUPPORTED, registered so the router answers with THIS
//! sentence instead of a bare "no route". {why}
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {{}}
/// 🚫️ {short}
pub const {const_name}: &str = "{message}";
pub fn {fn_sig} {{
    let _ = {unused};
    Err({const_name}.to_string())
}}
'''


def write(path: Path, body: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body, encoding="utf-8")
    print(f"wrote {path.relative_to(ROOT)}")


def main() -> None:
    for fmt, cfg in HUB.items():
        exp, imp = LEAF[fmt]
        kind = f"s.stdio.{fmt}"
        encode_call = f"{cfg['encode']}(&target)"
        if cfg["err"]:
            encode_call += '.map_err(|error| format!("{error:?}"))'
        decode_call = f"{cfg['decode']}(bytes)"
        if cfg["err"]:
            decode_call += '.map_err(|error| format!("{error:?}"))?'
        else:
            decode_call += "?"
        write(EXPORT / exp / "🦀️.rs", EXPORT_HUB.format(fmt=fmt, kind=kind, note=cfg["note"], dialect=cfg["dialect"], snapshot=cfg["snapshot"], encode_call=encode_call))
        write(IMPORT / imp / "🦀️.rs", IMPORT_HUB.format(fmt=fmt, kind=kind, note=cfg["note"], dialect=cfg["dialect"], decode_call=decode_call))

    # ── gif: the hub is declared at 89a, this dialect is 87a ─────────────────────────────────────
    write(EXPORT / LEAF["gif"][0] / "🦀️.rs", '''//! 📤️ raster → gif (87a) — REAL. The composite goes through stdio's own registered
//! `s.stdio.semio/v1/image` → `s.stdio.gif@89a` serializer (whose `quantize` does the real,
//! exact 1:1 RGBA→palette reduction and errors past 256 distinct colors rather than approximating),
//! then `io::gif87a::from_89a` remaps that snapshot onto stdio's own `87a` model, which stdio's own
//! `encode_gif` writes as genuine `GIF87a` bytes.
//!
//! 🧾️ GIF87a has no Graphic Control Extension: per-frame delay, disposal and transparency cannot
//! exist in this dialect at all, and the remap drops them for that reason. A raster composite is a
//! single still frame, so only transparency is materially lost — a fully transparent source pixel
//! is normalized to opaque black by the 89a quantizer's own documented rule.
use crate::artifacts::raster::io::{gif87a, raster_composite_image, semio_image_to_format, GIF89A_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
use semio_s_plugin_stdio::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot as Gif89aSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let image = raster_composite_image(snapshot).map_err(|reason| format!("gif export not available for this raster document: {reason}"))?;
    let gif89a: Gif89aSnapshot = semio_image_to_format(&image, GIF89A_DIALECT)?;
    semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::io::encode_gif(&gif87a::from_89a(&gif89a))
}
''')
    write(IMPORT / LEAF["gif"][1] / "🦀️.rs", '''//! 📥️ raster ← gif (87a) — REAL. stdio's own `87a` `decode_gif` reads the file, `io::gif87a::to_89a`
//! remaps it onto the model stdio's own registered `s.stdio.gif@89a` → `s.stdio.semio/v1/image`
//! deserializer accepts (that leaf owns the palette→RGBA decode, via the codec's own `GifFrame::rgba`
//! accessor), and the resulting canvas becomes one `Pixel` layer with a materialized asset child.
//!
//! 🧾️ Only the FIRST image of a multi-image GIF87a reaches the document: a raster document has no
//! frame/animation concept, so the semio/image hub's own frame list collapses to its first frame.
use crate::artifacts::raster::io::{gif87a, raster_document_from_semio_image, semio_image_from_format, GIF89A_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let gif87a_snapshot = semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::io::decode_gif(bytes)?;
    let image = semio_image_from_format(&gif87a::to_89a(&gif87a_snapshot), GIF89A_DIALECT)?;
    raster_document_from_semio_image(&image, "gif-import", "Imported gif")
}
''')

    # ── svg ──────────────────────────────────────────────────────────────────────────────────────
    write(EXPORT / LEAF["svg"][0] / "🦀️.rs", '''//! 📤️ raster → svg (1.1) — REAL. `raster_document_json_to_svg` maps the document's own visible
//! layer stack into a real `SemioDrawingSnapshot` (one `DrawNode::Image` per pixel layer, carrying
//! that layer's actual asset bytes and its own transform) and composes it to SVG text through
//! stdio's registered `s.stdio.semio/v1/drawing` → `s.stdio.svg` serializer. The bytes are a bare
//! `<svg>…</svg>` XML document — never `print_dsl` output.
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let (svg, _width, _height) = crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?;
    Ok(svg.into_bytes())
}
''')
    write(IMPORT / LEAF["svg"][1] / "🦀️.rs", '''//! 📥️ raster ← svg (1.1) — REAL, host-tiered. A raster document holds pixels, so importing vector
//! markup requires actually RASTERIZING it. The only real vector renderer in this repo is
//! `semio_framework_os::rasterize_svg_to_png_base64` (usvg/resvg behind the framework's own
//! interface); its raw PNG output is then canonicalized through the real
//! `s.stdio.semio/v1/image` ↔ png codec, exactly as `raster_document_json_from_dwg` already does
//! for the DWG path (which is itself an SVG rasterization underneath).
//!
//! 🧾️ That renderer is native-tier only: inside a `wasm32-wasip2` guest it returns its own
//! "SVG rasterization requires the native semio-framework-os host" error, which this leaf
//! propagates verbatim rather than substituting a blank canvas.
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let svg = std::str::from_utf8(bytes).map_err(|error| format!("svg import: payload is not UTF-8 XML: {error}"))?;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(svg, 0, 0)?;
    let raw = base64_codec::base64_standard_decode(rendered.as_bytes()).map_err(|error| error.to_string())?;
    let image = crate::artifacts::raster::io::semio_image_from_png_bytes(&raw)?;
    crate::artifacts::raster::io::raster_document_from_semio_image(&image, "svg-import", "Imported svg")
}
''')

    # ── honest refusals ──────────────────────────────────────────────────────────────────────────
    refusals = [
        (
            EXPORT / LEAF["pdf"][0] / "🦀️.rs",
            dict(
                arrow="📤️", word="→", fmt="pdf (1.4)",
                why="`PdfSnapshot`'s entire per-page model is `{width, height, text}` (see stdio's own `📖️pdf/🏅️standards/4️⃣1.4/…/🧬️schema/📸️snapshot/🦀️.rs`) — there is no image XObject, no content-stream painting operator and no `encode_pdf` path that could carry one pixel. stdio's own `s.stdio.semio/v1/drawing` → pdf serializer says the same thing in its own module doc: it drops every `Path` and `Image` node and writes text only.",
                short="A raster composite has no representable form in this repo's PDF model.",
                const_name="RASTER_PDF_EXPORT_UNSUPPORTED",
                message="pdf export not supported for a raster document: this repo's PdfSnapshot models a page as {width, height, text} only — it has no image XObject or path-painting operator, so a pixel composite has nowhere to go. Export png/bmp/tiff/jpg/gif instead.",
                fn_sig="serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String>", unused="snapshot",
            ),
        ),
        (
            IMPORT / LEAF["pdf"][1] / "🦀️.rs",
            dict(
                arrow="📥️", word="←", fmt="pdf (1.4)",
                why="The same page model runs the other way: `decode_pdf` yields `{width, height, text}` per page and no pixels, so there is nothing a raster document could be built out of.",
                short="This repo's PDF model decodes no pixels.",
                const_name="RASTER_PDF_IMPORT_UNSUPPORTED",
                message="pdf import not supported for a raster document: this repo's PdfSnapshot decodes a page as {width, height, text} only and carries no pixel data or embedded image, so no raster layer can be built from it. Import png/bmp/tiff/jpg/gif/svg instead.",
                fn_sig="deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String>", unused="bytes",
            ),
        ),
        (
            EXPORT / LEAF["dwg"][0] / "🦀️.rs",
            dict(
                arrow="📤️", word="→", fmt="dwg (ac1018)",
                why="DWG is a vector entity model. stdio's own `s.stdio.semio/v1/drawing` → dwg serializer states in its own module doc that `Image` nodes have no DWG entity equivalent and are dropped — and a raster composite is nothing BUT image nodes, so the produced DWG would be an empty drawing. The IMPORT direction of this same format is genuinely real (see the sibling leaf); the asymmetry is the format's, not a gap here.",
                short="Every node a raster document contributes is an Image node, which DWG cannot hold.",
                const_name="RASTER_DWG_EXPORT_UNSUPPORTED",
                message="dwg export not supported for a raster document: a raster document contributes only DrawNode::Image nodes, and stdio's semio/drawing to dwg serializer has no DWG entity for those (its own module doc says they are dropped), so the result would be an empty drawing. Export png/bmp/tiff/jpg/gif for pixels, or svg for a vector container that does carry embedded images.",
                fn_sig="serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String>", unused="snapshot",
            ),
        ),
    ]
    for path, values in refusals:
        write(path, REFUSE.format(**values))


if __name__ == "__main__":
    main()
