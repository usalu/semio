"""🔎️ Analysis-only type-check probe for the relocation: the whole production tiled-map tree (host, scene, camera, policy port, decoder) plus the GIS policy builders.

Multi-crate since the wave-4 carve: the mvt decoder, the model crate (model root plus map-policy, map-state, projection, tiles) and the host crate are mounted as `tiled_map_mvt`, `tiled_map_model` and `tiled_map` with their `crate::` paths re-rooted. The full-resolution differential stops at name resolution because every external crate is unresolved, so it never reaches typeck. This probe copies those files into a scratch tree, swaps the external crates they touch (canvas, serde derives, serde_json, ui_styling, value derives, pixels, the surface crate path, the GIS document types) for signature-only local stand-ins, and runs `rustc --emit=metadata -o /dev/null` (no artifact written). Unit tests and the browser-only wasm session are not probed. `--seed` plants one type error to prove the probe reaches typeck; `--seed-privacy` narrows one moved function and one new method to private to prove field/method/item privacy is checked.

`PROBE_TESTS=1` also mounts every `🧪️tests` tree (with fixtures) and compiles with `--test`, stubbing the `image` oracle crate.

Usage: HO_WORK=<scratch> [TM_SRC=<tiled-map tree>] [PROBE_TESTS=1] python3 typeck_probe.py [--seed] [--seed-privacy]
"""
import os, re, subprocess, sys, shutil

REPO = "/Users/ueli/Documents/semio"
WORK = os.path.join(os.environ["HO_WORK"], "typeck-probe")
TM = os.environ.get("TM_SRC", REPO + "/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map")
GP = REPO + "/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🌍️gis-policy"

STUBS = r'''
#![allow(dead_code, unused)]
pub mod serde_json {
    #[derive(Clone, Default, Debug, PartialEq)]
    pub enum Value { #[default] Null, Bool(bool), String(String), Array(Vec<Value>), Object(Map) }
    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct Map;
    impl Map {
        pub fn new() -> Self { Map }
        pub fn insert(&mut self, _: String, _: Value) -> Option<Value> { None }
        pub fn get(&self, _: &str) -> Option<&Value> { None }
        pub fn values(&self) -> std::slice::Iter<'_, Value> { [].iter() }
        pub fn is_empty(&self) -> bool { true }
        pub fn len(&self) -> usize { 0 }
        pub fn keys(&self) -> std::slice::Iter<'_, String> { [].iter() }
        pub fn iter(&self) -> std::iter::Empty<(&String, &Value)> { std::iter::empty() }
        pub fn contains_key(&self, _: &str) -> bool { false }
    }
    macro_rules! value_eq { ($($t:ty),*) => { $(impl PartialEq<$t> for Value { fn eq(&self, _: &$t) -> bool { false } })* }; }
    value_eq!(&str, str, String, f64, f32, i32, i64, u32, u64, usize, bool);
    impl FromIterator<(String, Value)> for Map { fn from_iter<I: IntoIterator<Item = (String, Value)>>(_: I) -> Self { Map } }
    impl From<&str> for Value { fn from(_: &str) -> Self { Value::Null } }
    impl From<String> for Value { fn from(_: String) -> Self { Value::Null } }
    impl From<f64> for Value { fn from(_: f64) -> Self { Value::Null } }
    impl From<bool> for Value { fn from(_: bool) -> Self { Value::Null } }
    impl Value {
        pub fn get<I>(&self, _: I) -> Option<&Value> { None }
        pub fn as_array(&self) -> Option<&Vec<Value>> { None }
        pub fn as_object(&self) -> Option<&Map> { None }
        pub fn as_str(&self) -> Option<&str> { None }
        pub fn as_f64(&self) -> Option<f64> { None }
        pub fn as_u64(&self) -> Option<u64> { None }
        pub fn as_bool(&self) -> Option<bool> { None }
        pub fn is_null(&self) -> bool { true }
    }
    impl std::ops::Index<&str> for Value { type Output = Value; fn index(&self, _: &str) -> &Value { self } }
    impl std::ops::IndexMut<&str> for Value { fn index_mut(&mut self, _: &str) -> &mut Value { self } }
    impl std::ops::Index<usize> for Value { type Output = Value; fn index(&self, _: usize) -> &Value { self } }
    impl std::fmt::Display for Value { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("null") } }
    #[derive(Debug)]
    pub struct Error;
    impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("json") } }
    impl std::error::Error for Error {}
    pub fn to_string<T: ?Sized>(_: &T) -> Result<String, Error> { Ok(String::new()) }
    pub fn to_value<T>(_: T) -> Result<Value, Error> { Ok(Value::Null) }
    pub fn from_str<T>(_: &str) -> Result<T, Error> { Err(Error) }
    pub fn from_value<T>(_: Value) -> Result<T, Error> { Err(Error) }
    #[macro_export]
    macro_rules! json { ($($t:tt)*) => { $crate::serde_json::Value::Null }; }
    pub use crate::json;
}
pub mod semio_framework_value { pub trait FromValue {} pub trait ToValue {} }
pub mod semio_framework_pixels {
    pub struct Decoded { pub width: u32, pub height: u32, pub pixels: Vec<u8> }
    #[derive(Debug)]
    pub struct RasterError;
    impl std::fmt::Display for RasterError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("raster") } }
    pub fn decode_png(_: &[u8]) -> Result<Decoded, RasterError> { Err(RasterError) }
}
pub mod ui_styling {
    pub struct MapPalette { pub surface_clear: [f32; 4], pub land_fill: [f32; 4], pub land_stroke: [f32; 4], pub label_fill: [f32; 4], pub label_halo: [f32; 4], pub region_fill: [f32; 4], pub region_stroke: [f32; 4], pub route_stroke: [f32; 4], pub position_fill: [f32; 4], pub position_stroke: [f32; 4] }
    pub const MAP_LIGHT: MapPalette = MapPalette { surface_clear: [0.0; 4], land_fill: [0.0; 4], land_stroke: [0.0; 4], label_fill: [0.0; 4], label_halo: [0.0; 4], region_fill: [0.0; 4], region_stroke: [0.0; 4], route_stroke: [0.0; 4], position_fill: [0.0; 4], position_stroke: [0.0; 4] };
    pub mod metrics {
        pub mod map { pub const LAYER_WEIGHT_MIN: f64 = 0.25; pub const LAYER_WEIGHT_MAX: f64 = 3.0; }
        pub mod label { pub const CHAR_WIDTH_RATIO: f64 = 0.6; pub const DECLUTTER_CELL_MAX: f64 = 40.0; pub const DECLUTTER_CELL_MIN: f64 = 8.0; pub const DECLUTTER_CELL_RATIO: f64 = 1.5; pub const MAP_WIDTH_MAX: f64 = 400.0; pub const MAP_WIDTH_MIN: f64 = 8.0; pub const PAD_RATIO: f64 = 0.2; }
    }
}
pub mod canvas {
    use std::sync::Arc;
    #[derive(Clone, Copy, Debug, PartialEq, Default)]
    pub struct Point { pub x: f64, pub y: f64 }
    impl Point { pub fn new(x: f64, y: f64) -> Self { Self { x, y } } pub fn distance(self, _: Point) -> f64 { 0.0 } }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rect;
    impl Rect { pub fn new(_: f64, _: f64, _: f64, _: f64) -> Self { Rect } pub fn x0(&self) -> f64 { 0.0 } pub fn x1(&self) -> f64 { 0.0 } pub fn y0(&self) -> f64 { 0.0 } pub fn y1(&self) -> f64 { 0.0 } pub fn width(&self) -> f64 { 0.0 } }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Affine;
    impl Affine { pub const IDENTITY: Affine = Affine; pub fn new(_: [f64; 6]) -> Self { Affine } pub fn translate(self, _: (f64, f64)) -> Self { self } pub fn scale(self, _: f64) -> Self { self } pub fn as_coeffs(self) -> [f64; 6] { [0.0; 6] } }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rgba8 { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Color;
    impl Color { pub fn new(_: [f32; 4]) -> Self { Color } pub fn from_rgba8(_: u8, _: u8, _: u8, _: u8) -> Self { Color } pub fn to_rgba8(self) -> Rgba8 { Rgba8 { r: 0, g: 0, b: 0, a: 0 } } }
    pub trait Shape {}
    pub struct BezPath;
    impl Shape for BezPath {}
    impl BezPath { pub fn new() -> Self { BezPath } pub fn move_to(&mut self, _: Point) {} pub fn line_to(&mut self, _: Point) {} pub fn close_path(&mut self) {} }
    pub struct Circle;
    impl Shape for Circle {}
    impl Circle { pub fn new(_: Point, _: f64) -> Self { Circle } }
    pub struct Stroke;
    impl Stroke { pub fn new(_: f64) -> Self { Stroke } }
    #[derive(Clone, Copy)]
    pub enum FillRule { NonZero, EvenOdd }
    pub struct Scene;
    impl Scene {
        pub fn new() -> Self { Scene }
        pub fn fill(&mut self, _: FillRule, _: Affine, _: Color, _: Option<Affine>, _: &impl Shape) {}
        pub fn stroke(&mut self, _: &Stroke, _: Affine, _: Color, _: Option<Affine>, _: &impl Shape) {}
        pub fn append(&mut self, _: &Scene, _: Option<Affine>) {}
        pub fn is_empty(&self) -> bool { true }
        pub fn path_count(&self) -> usize { 0 }
    }
    pub struct RasterImage;
    impl RasterImage { pub fn rgba8(_: u32, _: u32, _: Arc<Vec<u8>>) -> Self { RasterImage } pub fn width(&self) -> u32 { 0 } pub fn height(&self) -> u32 { 0 } }
    pub mod camera {
        use super::Point;
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct Camera { pub x: f64, pub y: f64, pub zoom: f64 }
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Viewport { pub width: u32, pub height: u32, pub dpr: f64 }
        impl Viewport { pub fn set_size(&mut self, _: u32, _: u32, _: f64) {} }
        pub fn screen_to_world(_: &Camera, _: &Viewport, p: Point) -> Point { p }
        pub fn world_to_screen(_: &Camera, _: &Viewport, p: Point) -> Point { p }
    }
    pub mod text { pub fn append_label(_: &mut super::Scene, _: &str, _: super::Point, _: f64, _: super::Color, _: super::Color) {} }
    pub mod raster { pub fn draw_image_arc(_: &mut super::Scene, _: &std::sync::Arc<super::RasterImage>, _: super::Affine) {} }
    pub mod canvas_content { pub trait CanvasContent { fn build_scene(&self) -> super::Scene; fn clear_color(&self) -> super::Color; } }
    pub trait CanvasExtension: Send + Sync { fn extension_id(&self) -> &str; }
}
#[derive(Clone, Debug, Default)]
pub struct MapFeature { pub id: String }
#[derive(Clone, Debug, Default)]
pub struct GisMapSnapshot { pub positions: Vec<MapFeature>, pub routes: Vec<MapFeature>, pub regions: Vec<MapFeature> }
pub mod standards { pub mod v1 { pub mod subsets { pub mod any { pub mod io { pub mod text { pub mod snapshot {
    pub fn gis_map_descriptor_value(_: &crate::GisMapSnapshot) -> crate::serde_json::Value { crate::serde_json::Value::Null }
    pub fn default_document() -> crate::GisMapSnapshot { crate::GisMapSnapshot::default() }
} } } } } } }
pub mod image {
    #[derive(Clone, Copy, Debug)]
    pub enum ColorType { Rgba8, Rgb8, L8, La8 }
    pub struct ExtendedColorType;
    impl From<ColorType> for ExtendedColorType { fn from(_: ColorType) -> Self { ExtendedColorType } }
    #[derive(Debug)]
    pub struct ImageError;
    pub trait ImageEncoder { fn write_image(self, _: &[u8], _: u32, _: u32, _: ExtendedColorType) -> Result<(), ImageError>; }
    pub mod codecs { pub mod png {
        pub struct PngEncoder<W>(W);
        impl<W> PngEncoder<W> { pub fn new(w: W) -> Self { PngEncoder(w) } }
        impl<W> super::super::ImageEncoder for PngEncoder<W> { fn write_image(self, _: &[u8], _: u32, _: u32, _: super::super::ExtendedColorType) -> Result<(), super::super::ImageError> { Ok(()) } }
    } }
    pub struct RgbaImage;
    impl RgbaImage { pub fn dimensions(&self) -> (u32, u32) { (0, 0) } pub fn into_raw(self) -> Vec<u8> { Vec::new() } }
    pub struct DynamicImage;
    impl DynamicImage { pub fn to_rgba8(&self) -> RgbaImage { RgbaImage } }
    pub fn load_from_memory(_: &[u8]) -> Result<DynamicImage, ImageError> { Err(ImageError) }
}
#[path = "tm/🧱️vector-tiles/📦️mvt-decoder/🦀️.rs"]
pub mod tiled_map_mvt;
#[path = "tm/🧮️model/🦀️.rs"]
pub mod tiled_map_model;
#[path = "tm/🦀️.rs"]
pub mod tiled_map;
pub mod tile_coordinates {
    #[path = "../gis-policy/🦀️.rs"]
    pub mod gis_policy;
}
'''


def patch(text):
    text = re.sub(r"^[ \t]*#\[(?:serde|value)\((?:[^()]|\([^()]*\))*\)\]\n", "", text, flags=re.M)
    for derive in (", serde::Serialize", ", serde::Deserialize", ", semio_framework_value::ToValue", ", semio_framework_value::FromValue"):
        text = text.replace(derive, "")
    text = text.replace("pub use canvas::{self, *};", "pub use crate::canvas::{self, *};").replace("pub use canvas::{self};", "pub use crate::canvas::{self, *};").replace("pub use canvas::{self,};", "pub use crate::canvas::{self, *};")
    text = text.replace("semio_framework_surface::tiled_map", "crate::tiled_map")
    text = text.replace('include_bytes!("../../../../../../../../../🧰️framework/', 'include_bytes!("' + REPO + '/🧰️framework/')
    for name in ("serde_json", "ui_styling", "semio_framework_value", "semio_framework_pixels", "canvas", "image"):
        text = re.sub(r"(?<![\w:])" + name + r"::", "crate::" + name + "::", text)
    return text


MODEL_DIRS = ("🧮️model", "🎛️map-policy", "🪢️map-state", "🧭️projection", "🧩️tiles")


def crate_root(rel):
    if rel.startswith("🧱️vector-tiles/📦️mvt-decoder"):
        return "tiled_map_mvt"
    if rel.split(os.sep)[0] in MODEL_DIRS:
        return "tiled_map_model"
    return "tiled_map"


def externs(text):
    for name in ("tiled_map_model", "tiled_map_mvt"):
        text = re.sub(r"(?<![\w:])" + name + r"\b", "crate::" + name, text)
    return text


TESTS = os.environ.get("PROBE_TESTS") == "1"


def copy_tree(src, dst, skip, rooted):
    for directory, _, files in os.walk(src):
        rel = os.path.relpath(directory, src)
        if any(part in skip for part in rel.split(os.sep)):
            continue
        for name in files:
            if TESTS and not name.endswith(".rs"):
                target = os.path.join(dst, rel, name)
                os.makedirs(os.path.dirname(target), exist_ok=True)
                shutil.copy(os.path.join(directory, name), target)
            if name.endswith(".rs"):
                text = open(os.path.join(directory, name), encoding="utf-8").read()
                if rooted:
                    text = re.sub(r"(?<![\w$])crate::", "crate::" + crate_root(os.path.normpath(os.path.join(rel, name))) + "::", text)
                text = externs(patch(text))
                target = os.path.join(dst, rel, name)
                os.makedirs(os.path.dirname(target), exist_ok=True)
                open(target, "w", encoding="utf-8").write(text)


def main(seed, privacy):
    shutil.rmtree(WORK, ignore_errors=True)
    os.makedirs(os.path.join(WORK, "tile_coordinates"))
    open(os.path.join(WORK, "lib.rs"), "w", encoding="utf-8").write(STUBS)
    skip = set() if TESTS else {"🧪️tests"}
    copy_tree(TM, os.path.join(WORK, "tm"), skip, True)
    copy_tree(GP, os.path.join(WORK, "gis-policy"), skip, False)
    if seed:
        path = os.path.join(WORK, "tm", "🏛️map-host", "🖼️host-scene", "🦀️.rs")
        text = open(path, encoding="utf-8").read()
        open(path, "w", encoding="utf-8").write(text.replace("let road_lod_scale = band_row.road_stroke_scale;", "let road_lod_scale = band_row.road_spans;", 1))
    if privacy:
        for rel, old, new in (("gis-policy/🎨️vector-style/🦀️.rs", "pub(super) fn gis_map_band_gates(", "fn gis_map_band_gates("), ("tm/🏛️map-host/📷️camera-and-picking/🦀️.rs", "    pub fn pinned_band_id(", "    fn pinned_band_id("), ("tm/🧮️model/🦀️.rs", "    pub fn slot(&self, slot: MapPaletteSlot)", "    fn slot(&self, slot: MapPaletteSlot)")):
            path = os.path.join(WORK, rel)
            text = open(path, encoding="utf-8").read()
            assert text.count(old) == 1, old
            open(path, "w", encoding="utf-8").write(text.replace(old, new))
    for target in ([], ["--target", "wasm32-wasip2"]):
        cmd = ["rustc", "+nightly-2026-07-20", "--edition", "2021", *(["--test"] if TESTS else ["--crate-type", "lib"]), "--crate-name", "relo_typeck_probe", "--emit=metadata", "-o", "/dev/null", "--cap-lints", "warn", *target, os.path.join(WORK, "lib.rs")]
        result = subprocess.run(cmd, capture_output=True, text=True, env={**os.environ, "CARGO_MANIFEST_DIR": os.path.join(WORK, "tm", "🧱️vector-tiles", "📦️mvt-decoder", "📦️packages", "🦀️rust")})
        real = [line for line in result.stderr.splitlines() if line.startswith("error") and "couldn't create a temp dir" not in line and not line.startswith("error: aborting")]
        warnings = [line for line in result.stderr.splitlines() if line.startswith("warning")]
        print(f"[probe {' '.join(target) or 'native'}{' seeded' if seed else ''}] typeck errors={len(real)} warnings={len(warnings)}")
        for line in real:
            print("  ", line)
        if real:
            print(result.stderr[:12000])


if __name__ == "__main__":
    main("--seed" in sys.argv, "--seed-privacy" in sys.argv)
