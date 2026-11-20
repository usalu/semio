"""🔠 Project the real TrueType outlines through the production PDF path transform."""
from pathlib import Path
import json
import re
import subprocess
import sys

root, fixture, generated = map(Path, sys.argv[1:])
data = json.loads(fixture.read_text(encoding="utf-8"))
font = (root / "🔨️modules/🔤️fonts/🔠️truetype/🦀️.rs").resolve()
coordinates = (root / "✏️editor/🖼️page/🧭️geometry/🧭️coordinates/🦀️.rs").resolve()
render = (root / "✏️editor/🖼️page/🗺️scene/🔠️truetype-text/🦀️.rs").read_text()
projection = render[render.index("fn append_outline("):]
rust_string = lambda value: json.dumps(str(value), ensure_ascii=False)
size = float(re.search(r"/F1 (\d+) Tf", data["clip"]["content"])[1])
runs = re.findall(r"1 0 0 1 (\d+) (\d+) Tm \((\w)\) Tj", data["clip"]["content"])
assert len(runs) == 2
glyph_runs = "\n".join(f"    glyph({float(x)}, {float(y)}, '{glyph}');" for x, y, glyph in runs)
width, height = [float(value) for value in data["page"]]
rust = '''
#![allow(dead_code)]
#[path = @FONT@] mod font;
#[path = @COORDS@] mod coordinates;
use font::{TrueTypeFont, TrueTypeOutlineSegment};
use coordinates::{PageViewport, VectorSegment};

@PROJECTION@

fn main() {
    let bytes = std::fs::read(@DATA@).unwrap();
    let font = TrueTypeFont::parse(&bytes).unwrap();
    let viewport = PageViewport::new([0.0, 0.0, @WIDTH@, @HEIGHT@], None, 0, None, 0.0);
    let mut path = Vec::new();
    let mut glyph = |x, y, glyph| {
        let id = font.glyph_for_char(glyph).unwrap();
        let outline = font.glyph_outline(id).unwrap();
        append_outline(&mut path, &outline, @SIZE@ / f64::from(font.units_per_em), 0.0,
            |sx, sy| viewport.point(x + sx, y + sy));
    };
@RUNS@
    let point = |(x, y)| format!("[{x},{y}]");
    let rows = path.into_iter().map(|segment| match segment {
        VectorSegment::Move(to) => format!(r#"{{"kind":"move","to":{}}}"#, point(to)),
        VectorSegment::Line(to) => format!(r#"{{"kind":"line","to":{}}}"#, point(to)),
        VectorSegment::Cubic { first, second, end } => format!(r#"{{"kind":"cubic","ctrl1":{},"ctrl2":{},"to":{}}}"#, point(first), point(second), point(end)),
        VectorSegment::Close => r#"{"kind":"close"}"#.into(),
    }).collect::<Vec<_>>();
    println!("[{}]", rows.join(","));
}
'''
for label, value in {
    "FONT": rust_string(font), "COORDS": rust_string(coordinates), "PROJECTION": projection,
    "DATA": rust_string(Path(data["font"]).resolve()), "WIDTH": width, "HEIGHT": height, "SIZE": size,
    "RUNS": glyph_runs,
}.items():
    rust = rust.replace(f"@{label}@", str(value))
source = generated / "glyph-clip-native.rs"
binary = generated / ("glyph-clip-native.exe" if sys.platform == "win32" else "glyph-clip-native")
source.write_text(rust, encoding="utf-8")
subprocess.run(["rustc", "--edition", "2021", str(source), "-o", str(binary)], check=True)
with (generated / "glyph-clip-segments.json").open("w") as output:
    subprocess.run([str(binary)], stdout=output, check=True)
