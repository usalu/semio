"""🎨️ Compile the shared map scene assembly with minimal recording components."""

import json
import os
from pathlib import Path
import re
import subprocess


ticket = Path(__file__).resolve().parent
workspace = next(parent for parent in ticket.parents if (parent / "AGENTS.md").is_file())
tiled = workspace / "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map"
source = (tiled / "🏛️map-host/🖼️host-scene/🦀️.rs").read_text(encoding="utf-8")
fixture = json.loads((tiled / "🧪️tests/🧫️fixtures/🖼️basemap-layer-order/🔣️.json").read_text(encoding="utf-8"))
output = ticket / "🗑️generated" / "basemap-layer-order"
output.mkdir(parents=True, exist_ok=True)


def method(name):
    """🧩️ Read one Rust method including its complete body."""
    start = re.search(r"(?:pub )?fn " + name + r"\(", source)
    if start is None:
        return ""
    offset = source.index("{", start.start())
    depth = 0
    for index in range(offset, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
        if depth == 0:
            return source[start.start() : index + 1]
    raise ValueError(name)


assembly = method("build_vector_scene")
cache_method = method("append_vector_tiles_cached").split("let key = ", 1)[0]
cache_method += 'if self.vector { out.0.push("f"); } }'
backdrop = method("append_vector_tile_backdrop")
rows = ",\n".join(
    f'(MapTileMode::{case["mode"].capitalize()}, {str(case["raster"]).lower()}, '
    f'{str(case["vector"]).lower()}, &{json.dumps(case["commands"])}[..])'
    for case in fixture["cases"]
)
program = f"""
#[derive(Clone, Copy, Debug)]
enum MapTileMode {{ Combined, Vector, Image }}
struct Camera {{ zoom: f64 }}
struct Viewport;
struct MapHost {{ camera: Camera, viewport: Viewport, render_mode: MapTileMode, raster: bool, vector: bool }}
struct Scene(Vec<&'static str>);
impl Scene {{ fn new() -> Self {{ Self(Vec::new()) }} }}
fn vector_tiles_available_at_camera_zoom(zoom: f64) -> bool {{ zoom > 0.0 }}
fn viewport_lon_span_degrees(_: &Camera, _: &Viewport) -> f64 {{ 0.0 }}
impl MapHost {{
    fn vector_tile_render_z(&self) -> Option<u32> {{ self.vector.then_some(0) }}
    fn vector_tile_backdrop_fill(&self, _: f64, _: u32, _: Option<&str>) -> Option<()> {{ Some(()) }}
    fn pinned_band_id(&self) -> Option<&str> {{ None }}
    fn append_viewport_fill(&self, out: &mut Scene, _: ()) {{ out.0.push("f"); }}
    fn append_tiles(&self, out: &mut Scene) {{ if self.raster {{ out.0.push("i"); }} }}
    fn append_overlays(&self, _: &mut Scene) {{}}
    {assembly}
    {cache_method}
    {backdrop}
}}
#[test]
fn basemap_layer_order() {{
    for (mode, raster, vector, expected) in [{rows}] {{
        let host = MapHost {{ camera: Camera {{ zoom: 1.0 }}, viewport: Viewport, render_mode: mode, raster, vector }};
        assert_eq!(host.build_vector_scene().0, expected, "{{mode:?}}, raster={{raster}}, vector={{vector}}");
    }}
}}
"""
test_source = output / "fixture.rs"
executable = output / ("fixture.exe" if os.name == "nt" else "fixture")
test_source.write_text(program, encoding="utf-8")
rustup = ["repo", "tools", "exec", "--light", "rustup", "--"]
channel = subprocess.check_output([*rustup, "show", "active-toolchain"], cwd=workspace, text=True).split()[0]
subprocess.run([*rustup, "run", channel, "rustc", "--test", str(test_source), "-o", str(executable)], cwd=workspace, check=True)
subprocess.run([str(executable)], cwd=workspace, check=True)
