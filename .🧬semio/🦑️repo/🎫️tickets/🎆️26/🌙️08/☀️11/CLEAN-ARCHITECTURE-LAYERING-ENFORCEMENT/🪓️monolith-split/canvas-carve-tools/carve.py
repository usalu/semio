#!/usr/bin/env python3
# 🪚️ Carves the monolithic canvas `🦀️.rs` into the scene, camera, icon-codec crates and the thin facade root.
# Reads the original from CANVAS_ORIGINAL (a pristine copy), writes below CANVAS_OUT (default: the module itself).
import hashlib
import os
import sys

MODULE = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️canvas"
ORIGINAL = os.environ.get("CANVAS_ORIGINAL", MODULE + "/🦀️.rs")
OUT = os.environ.get("CANVAS_OUT", MODULE)
EXPECTED_SHA256 = "0fa23cfc2f879e8d66d1d819617b6128154ce251c6efca3dbb6eb64546150733"

text = open(ORIGINAL, encoding="utf8").read()
assert hashlib.sha256(text.encode("utf8")).hexdigest() == EXPECTED_SHA256, "the canvas root is not the pristine original"
L = text.split("\n")


def find(exact, start=0):
    hits = [i for i in range(start, len(L)) if L[i] == exact]
    assert hits, f"missing anchor {exact!r}"
    return hits[0]


def unique(exact):
    hits = [i for i, line in enumerate(L) if line == exact]
    assert len(hits) == 1, f"anchor {exact!r} occurs {len(hits)} times"
    return hits[0]


def block(open_line):
    start = unique(open_line)
    end = find("}", start + 1)
    return start, end


def dedent(lines):
    out = []
    for line in lines:
        if line.startswith("    "):
            out.append(line[4:])
        else:
            assert line.strip() == "", f"cannot dedent {line!r}"
            out.append(line)
    return out


def sub(lines, old, new, count=1):
    joined = "\n".join(lines)
    assert joined.count(old) == count, f"{old!r} occurs {joined.count(old)} times, expected {count}"
    return joined.replace(old, new).split("\n")


def write(path, content):
    full = os.path.join(OUT, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    while "\n\n\n" in content:
        content = content.replace("\n\n\n", "\n\n")
    with open(full, "w", encoding="utf8") as handle:
        handle.write(content if content.endswith("\n") else content + "\n")


P2 = '#[cfg(all(target_arch = "wasm32", target_env = "p2"))]'
NOT_P2 = '#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]'

# ---- boundaries -----------------------------------------------------------------------------------------
renderer_open, renderer_close = block("mod renderer {")
svg_open, svg_close = block("pub mod svg_icon {")
text_open, text_close = block("pub mod text {")
camera_open, camera_close = block("pub mod camera {")
lod_open, lod_close = block("pub mod lod {")
raster_open, raster_close = block("pub mod raster {")
render_open, render_close = block("pub mod render {")
content_open, content_close = block("pub mod canvas_content {")
gpu_open, gpu_close = block("pub mod gpu_session {")
codec_open, codec_close = block("pub mod icon_codec {")
theme_open, theme_close = block("pub mod theme {")
native_open, native_close = block("pub mod native_render {")
assets_open, assets_close = block("pub mod icon_assets {")

renderer_region_start = unique("// #region 🔖️Renderer")
renderer_region_end = unique("// #endregion 🔖️Renderer")
errors_start = unique("// #region ⚠️ Errors")
errors_end = unique("// #endregion ⚠️ Errors")

# ---- scene: renderer file -------------------------------------------------------------------------------
renderer_body = dedent(L[renderer_open + 1 : renderer_close])
renderer_body = sub(renderer_body, 'include!("🧪️tests/', 'include!("../../🧪️tests/', 2)
renderer_file = ["//! 🖌️ The first-party scene: the paint value types, the recorded command list `Scene`, its retirement registry, the portable draw list and the parsed SVG document.", ""] + renderer_body
write("🎬️scene/🖌️renderer/🦀️.rs", "\n".join(renderer_file))

# ---- scene: svg icon file -------------------------------------------------------------------------------
svg_body = dedent(L[svg_open + 1 : svg_close])
old_options = [
    NOT_P2,
    "static ICON_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();",
    "",
    "/// 🔤️ Shared `usvg` parse options with bundled Noto Color Emoji so `<text>` in Typst `emoji:` SVG matches the Typst font book; avoids system fallback glyphs.",
    NOT_P2,
    "pub(super) fn usvg_options_icons() -> &'static usvg::Options<'static> {",
    "    ICON_USVG_OPTIONS.get_or_init(|| {",
    "        let mut db = usvg::fontdb::Database::new();",
    "        db.load_font_data(super::icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF.to_vec());",
    "        usvg::Options { fontdb: Arc::new(db), font_family: ui_styling::canvas_fonts::NOTO_COLOR_EMOJI.into(), ..Default::default() }",
    "    })",
    "}",
]
new_options = [
    NOT_P2,
    "static ICON_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();",
    "",
    NOT_P2,
    "static ICON_USVG_WITHOUT_FACE: OnceLock<usvg::Options<'static>> = OnceLock::new();",
    "",
    "/// 🔤️ Shared `usvg` parse options with the installed Noto Color Emoji face (the compiler's emoji font, delivered through",
    "/// the runtime asset port) so `<text>` in `emoji:` SVG matches the compiler's font set; avoids system fallback glyphs.",
    "/// Until a host installs the runtime assets the options carry no face and such text paints no glyphs.",
    NOT_P2,
    "pub(super) fn usvg_options_icons() -> &'static usvg::Options<'static> {",
    "    match semio_framework_canvas_assets::emoji_font() {",
    "        Some(face) => ICON_USVG_OPTIONS.get_or_init(|| {",
    "            let mut db = usvg::fontdb::Database::new();",
    "            db.load_font_data(face.to_vec());",
    "            usvg::Options { fontdb: Arc::new(db), font_family: ui_styling::canvas_fonts::NOTO_COLOR_EMOJI.into(), ..Default::default() }",
    "        }),",
    "        None => ICON_USVG_WITHOUT_FACE.get_or_init(|| usvg::Options { font_family: ui_styling::canvas_fonts::NOTO_COLOR_EMOJI.into(), ..Default::default() }),",
    "    }",
    "}",
]
joined = "\n".join(svg_body)
old_text = "\n".join(old_options)
assert joined.count(old_text) == 1, "svg options block not found verbatim"
svg_body = joined.replace(old_text, "\n".join(new_options)).split("\n")
svg_file = ["//! 🖼️ SVG icon painting: the `usvg` parse options, themed and literal paint of a parsed tree, content bounds, and the `wasm32-wasip2` declared-size arm.", ""] + svg_body
write("🎬️scene/🖼️svg-icon/🦀️.rs", "\n".join(svg_file))

# ---- scene: text file -----------------------------------------------------------------------------------
text_body = dedent(L[text_open + 1 : text_close])
old_map = [
    NOT_P2,
    "static MAP_LABEL_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();",
    "",
    "/// 🔤️ `usvg` options with bundled map label sans for place-name labels.",
    NOT_P2,
    "pub(super) fn usvg_options_map_labels() -> &'static usvg::Options<'static> {",
    "    MAP_LABEL_USVG_OPTIONS.get_or_init(|| {",
    "        let mut db = usvg::fontdb::Database::new();",
    "        db.load_font_data(super::icon_assets::MAP_LABEL_SANS_TTF.to_vec());",
    "        let family = db.faces().next().and_then(|face| face.families.first().map(|(name, _)| name.clone())).unwrap_or_else(|| ui_styling::canvas_fonts::MAP_LABEL_SANS_FALLBACK.into());",
    "        usvg::Options { fontdb: Arc::new(db), font_family: family, ..Default::default() }",
    "    })",
    "}",
]
new_map = [
    NOT_P2,
    "static MAP_LABEL_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();",
    "",
    NOT_P2,
    "static MAP_LABEL_USVG_WITHOUT_FACE: OnceLock<usvg::Options<'static>> = OnceLock::new();",
    "",
    "/// 🔤️ `usvg` options with the installed map label sans (delivered through the runtime asset port) for place-name labels.",
    "/// Until a host installs the runtime assets the options carry no face and labels shape no glyphs.",
    NOT_P2,
    "pub(super) fn usvg_options_map_labels() -> &'static usvg::Options<'static> {",
    "    match semio_framework_canvas_assets::label_font() {",
    "        Some(face) => MAP_LABEL_USVG_OPTIONS.get_or_init(|| {",
    "            let mut db = usvg::fontdb::Database::new();",
    "            db.load_font_data(face.to_vec());",
    "            let family = db.faces().next().and_then(|loaded| loaded.families.first().map(|(name, _)| name.clone())).unwrap_or_else(|| ui_styling::canvas_fonts::MAP_LABEL_SANS_FALLBACK.into());",
    "            usvg::Options { fontdb: Arc::new(db), font_family: family, ..Default::default() }",
    "        }),",
    "        None => MAP_LABEL_USVG_WITHOUT_FACE.get_or_init(|| usvg::Options { font_family: ui_styling::canvas_fonts::MAP_LABEL_SANS_FALLBACK.into(), ..Default::default() }),",
    "    }",
    "}",
]
joined = "\n".join(text_body)
old_text = "\n".join(old_map)
assert joined.count(old_text) == 1, "map label options block not found verbatim"
text_body = joined.replace(old_text, "\n".join(new_map)).split("\n")
text_body = sub(text_body, 'include!("🧪️tests/🏷️label-shapes/🦀️.rs");', 'include!("../../🧪️tests/🏷️label-shapes/🦀️.rs");')
text_body = sub(text_body, 'include_str!("🧬️schema/🏷️label-shapes/🔣️.json")', 'include_str!("../../🧬️schema/🏷️label-shapes/🔣️.json")')
assert text_body[0].lstrip().startswith("//! 🏷️"), "the text module keeps its inner doc first"
text_file = []
index = 0
while text_body[index].startswith("//!"):
    text_file.append(text_body[index])
    index += 1
text_file += ["", "// #region 🔖️Text"] + text_body[index:] + ["// #endregion 🔖️Text"]
write("🎬️scene/🏷️text/🦀️.rs", "\n".join(text_file))

# ---- scene: gpu session file ----------------------------------------------------------------------------
gpu_start = unique("// #region 🔖️GpuSession")
gpu_end = unique("// #endregion 🔖️GpuSession")
gpu_cfg = L[gpu_open - 1]
assert gpu_cfg == '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]'
gpu_comment = L[gpu_start + 1 : gpu_open - 1]
gpu_body = dedent(L[gpu_open + 1 : gpu_close])
gpu_file = ["//! 🖥️ The browser WebGPU canvas session: surface bring-up, resize, detach and the frame render that blits a `Scene` onto the attached canvas.", "", "// #region 🔖️GpuSession"] + gpu_body + ["// #endregion 🔖️GpuSession"]
write("🎬️scene/🖥️gpu-session/🦀️.rs", "\n".join(gpu_file))

# ---- scene: root ----------------------------------------------------------------------------------------
renderer_reexports = L[renderer_close + 1 : renderer_region_end]
renderer_reexports = sub(renderer_reexports, '#[path = "♻️retirement/', '#[path = "../♻️retirement/', 2)
intrinsic_doc_start = renderer_region_end + 2
assert L[intrinsic_doc_start].startswith("/// 📐️ First-party intrinsic-dimension reader")
intrinsic_end = find("use semio_framework_intrinsic_size as intrinsic_size;", intrinsic_doc_start)
intrinsic = L[intrinsic_doc_start : intrinsic_end + 1]
intrinsic = [
    "/// 📐️ First-party intrinsic-dimension reader (`🧰️framework/🔨️modules/📏️intrinsic-size`) — the",
    "/// `wasm32-wasip2` arm of `svg_icon::svg_icon_content_bounds_from_str` uses it instead of `usvg` so that",
    "/// dimension-only call site stops pulling the SVG pipeline onto the guest component. Native/browser keep the",
    "/// real `usvg` parse unchanged. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,",
    "/// `🔍️research/📓️intrinsic-size-wiring.md`.",
    P2,
    "use semio_framework_intrinsic_size as intrinsic_size;",
]
native = L[native_open - 2 : native_close + 1]
assert native[0].startswith("/// 🖥️ Framework GPU device boundary")
errors = L[errors_start : errors_end + 1]
theme = L[theme_open : theme_close + 1]
theme = sub(theme, 'include!("🧪️tests/🎨️theme-color-fields/🦀️.rs");', 'include!("../🧪️tests/🎨️theme-color-fields/🦀️.rs");')
impl_start = find(NOT_P2, svg_close + 1)
assert L[impl_start + 1] == "impl SvgDocument {"
impl_end = find("}", impl_start + 2)
svg_document_impl = L[impl_start : impl_end + 1]
small_modules = L[unique("// #region 🔖️Raster") : unique("// #endregion 🔖️CanvasContent") + 1]
gpu_root = gpu_comment + [gpu_cfg, '#[path = "🖥️gpu-session/🦀️.rs"]', "pub mod gpu_session;"]

scene_root = (
    [
        "//! 🎬️ Canvas scene: the first-party command list `Scene` and its paint types, the portable draw list, SVG icon painting, shaped labels, theme paints and the GPU sessions that rasterize a scene. Application-neutral; extend through `CanvasExtension` in the facade.",
        '#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]',
        "",
        '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]',
        "extern crate semio_framework_async as wasm_bindgen_futures;",
        "",
        "// #region 🔖️Renderer",
        '#[path = "🖌️renderer/🦀️.rs"]',
        "mod renderer;",
        "",
    ]
    + renderer_reexports
    + ["// #endregion 🔖️Renderer", ""]
    + intrinsic
    + [""]
    + native
    + [""]
    + errors
    + [""]
    + theme
    + ["", '#[path = "🖼️svg-icon/🦀️.rs"]', "pub mod svg_icon;", ""]
    + svg_document_impl
    + ["", '#[path = "🏷️text/🦀️.rs"]', "pub mod text;", ""]
    + small_modules
    + [""]
    + gpu_root
    + [
        "",
        "// #region 🧪️Tests",
        "#[cfg(test)]",
        '#[path = "../🧪️tests/🔬️unit/🦀️.rs"]',
        "mod tests;",
        "",
        "#[cfg(test)]",
        '#[path = "../🧪️tests/🏷️label-fit/🦀️.rs"]',
        "mod label_fit_laws;",
        "",
        "#[cfg(test)]",
        '#[path = "../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]',
        "mod test_allocation;",
        "",
        "#[cfg(test)]",
        "#[global_allocator]",
        "static TEST_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;",
        "// #endregion 🧪️Tests",
    ]
)
write("🎬️scene/🦀️.rs", "\n".join(scene_root))

# ---- camera crate ---------------------------------------------------------------------------------------
camera_region_start = unique("// #region 🔖️Camera")
camera_region_end = unique("// #endregion 🔖️Camera")
lod_region_start = unique("// #region 🔖️Lod")
lod_region_end = unique("// #endregion 🔖️Lod")
camera_block = L[camera_region_start : camera_region_end + 1]
camera_block = sub(camera_block, "    use super::{Affine, Point};", "    use geometry::{Affine, Point};")
lod_block = L[lod_region_start : lod_region_end + 1]
camera_root = (
    ["//! 📷️ Canvas camera: the pan-and-zoom transform between world and screen space, the opening-camera fit of a content extent, and the level-of-detail bands zoom selects.", ""]
    + camera_block
    + [""]
    + lod_block
    + [
        "",
        "// #region 🧪️Tests",
        "#[cfg(test)]",
        '#[path = "../🧪️tests/📷️camera-fit/🦀️.rs"]',
        "mod camera_fit_laws;",
        "",
        "#[cfg(test)]",
        '#[path = "../🧪️tests/🧭️camera-unit/🦀️.rs"]',
        "mod unit_tests;",
        "// #endregion 🧪️Tests",
    ]
)
write("📷️camera/🦀️.rs", "\n".join(camera_root))

# ---- icons crate ----------------------------------------------------------------------------------------
codec_region_start = unique("// #region 🔖️IconCodec")
codec_region_end = unique("// #endregion 🔖️IconCodec")
codec_lines = L[codec_region_start : codec_region_end + 1]
codec_text = "\n".join(codec_lines)


def swap(old, new, count=1):
    global codec_text
    assert codec_text.count(old) == count, f"{old!r} occurs {codec_text.count(old)} times, expected {count}"
    codec_text = codec_text.replace(old, new)


swap('#[path = "../🖼️assets/🌱️metabolism/', '#[path = "../../🖼️assets/🌱️metabolism/')
swap('include!("🌉️icon-name-value/🦀️.rs");', 'include!("../🌉️icon-name-value/🦀️.rs");')
swap('include!("../🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs");', 'include!("../../🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs");')
swap('include!("🧪️tests/🔬️icon-codec-unit/🦀️.rs");', 'include!("../🧪️tests/🔬️icon-codec-unit/🦀️.rs");')
swap("    pub use compiler::world::{FontRole, FontSet};\n", "")
swap("    use serde::{Deserialize, Serialize};\n", "    use semio_framework_canvas_assets::{compiler_fonts, icon_pack};\n    use serde::{Deserialize, Serialize};\n")

fonts_start = codec_text.index("    #[cfg(not(all(target_arch = \"wasm32\", target_env = \"p2\")))]\n    static COMPILER_FONTS:")
fonts_end_anchor = "        Some(&ESTIMATE_ONLY)\n    }\n\n"
fonts_end = codec_text.index(fonts_end_anchor, fonts_start) + len(fonts_end_anchor)
removed = codec_text[fonts_start:fonts_end]
assert removed.count("pub fn install_compiler_fonts") == 2 and removed.count("fn compiler_fonts()") == 2, "unexpected font port block"
codec_text = codec_text[:fonts_start] + codec_text[fonts_end:]

shortcode_anchor = "    mod icon_shortcodes {\n        include!(\"../../🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs\");\n    }\n"
shortcode_resolve = shortcode_anchor + """
    /// 🔖️ What one shortcode body names: an emoji glyph from the compiled table, or an SVG source from the installed icon pack.
    enum ShortcodeResolved {
        Emoji(&'static str),
        SvgPlain(&'static str),
        SvgThemed(&'static str),
    }

    /// 🔍️ Resolves a shortcode body against the compiled emoji table first, then the installed pack's themed and plain SVG;
    /// `None` when the code is unknown or no host has installed the icon pack.
    fn icon_shortcode_resolve(code: &str) -> Option<ShortcodeResolved> {
        let code = code.trim();
        if let Some(glyph) = icon_shortcodes::icon_shortcode_emoji(code) {
            return Some(ShortcodeResolved::Emoji(glyph));
        }
        let pack = icon_pack()?;
        pack.themed(code).map(ShortcodeResolved::SvgThemed).or_else(|| pack.plain(code).map(ShortcodeResolved::SvgPlain))
    }
"""
swap(shortcode_anchor, shortcode_resolve)
swap("icon_shortcodes::icon_shortcode_resolve(", "icon_shortcode_resolve(", 3)
swap("icon_shortcodes::ShortcodeResolved::", "ShortcodeResolved::", 6)
swap("pub use icon_codec::{resolve_icon_kind, decode_icon, encode_icon, install_compiler_fonts, install_compiler_font_bytes, CompilerFontsInstallError, FontRole, FontSet, ResolvedIcon, Icon, ThemedSvgLookup};", "pub use icon_codec::{resolve_icon_kind, decode_icon, encode_icon, ResolvedIcon, Icon, ThemedSvgLookup};")

icons_root = """//! 🪪️ Canvas icon identity and codec: the structured `Icon` union and its canonical wire string, the metabolism icon names, and the resolver that turns an encoded icon into paintable SVG or raster content through the compiler's installed fonts and the installed icon pack.
#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]

/// 📐️ First-party intrinsic-dimension reader (`🧰️framework/🔨️modules/📏️intrinsic-size`) — the `wasm32-wasip2` arm of
/// `icon_codec::decode_raster_icon_bytes` uses it instead of `image` so that dimension-only call site stops pulling the
/// raster pipeline onto the guest component. Native/browser keep the real `image` decode unchanged. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`, `🔍️research/📓️intrinsic-size-wiring.md`.
""" + P2 + """
use semio_framework_intrinsic_size as intrinsic_size;

""" + codec_text
write("🪪️icon-codec/🦀️.rs", icons_root)

# ---- facade root ----------------------------------------------------------------------------------------
extension_start = unique("// #region 🔖️CanvasExtension")
extension_end = unique("// #endregion 🔖️CanvasExtension")
extension = L[extension_start : extension_end + 1]
facade = (
    [
        "//! 🖼️ Application-neutral tile-based infinite canvas; extend via `CanvasExtension`. A thin facade over the producer crates: the scene,",
        "//! the camera, the icon codec and the runtime asset port.",
        "",
        "// #region 🔖️Reexports",
        "pub use semio_framework_canvas_scene::{",
        "    advance_opaque_scene_retirement, append_shape_to_path, canvas_content, draw_list, geom_sel, opaque_scene_retirement_status, publish_opaque_scene_retirement, raster, render, reserve_opaque_scene_retirement, svg_icon, text, theme, Affine, Arc, BezPath, BlendMode, Cap, CanvasError,",
        "    CanvasPathRetirement, CanvasSceneRetirement, CanvasStrokeRetirement, Circle, Color, CubicBez, FillRule, Line, OpaqueSceneRetirementStep, OpaqueSceneRetirementToken, Paint, PathEl, Point, RasterImage, Rect, Rgba8, RoundedRect, RoundedRectRadii, Scene, ShapeRef, Stroke, Vec2,",
        "};",
        NOT_P2,
        "pub use semio_framework_canvas_scene::{append_svg_document, native_render, SvgDocument};",
        '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]',
        "pub use semio_framework_canvas_scene::gpu_session;",
        "pub use semio_framework_canvas_camera::{camera, lod};",
        "pub use semio_framework_canvas_icons::{decode_icon, encode_icon, icon_codec, resolve_icon_kind, Icon, IconName, MetabolismIconName, ResolvedIcon, ThemedSvgLookup};",
        "pub use semio_framework_canvas_assets::{install_runtime_assets, FontRole, FontSet, RuntimeAsset, RuntimeAssets, RuntimeAssetsInstallError};",
        "// #endregion 🔖️Reexports",
        "",
    ]
    + extension
    + [
        "",
        "// #region 🔖️Tests",
        "#[cfg(test)]",
        '#[path = "🧪️tests/🎬️draw-list/🦀️.rs"]',
        "mod draw_list_tests;",
        "// #endregion 🔖️Tests",
    ]
)
write("🦀️.rs", "\n".join(facade))

print("carved:", {"renderer": renderer_close - renderer_open - 1, "svg_icon": svg_close - svg_open - 1, "text": text_close - text_open - 1, "gpu": gpu_close - gpu_open - 1, "camera": camera_close - camera_open - 1, "lod": lod_close - lod_open - 1, "codec": codec_close - codec_open - 1})
