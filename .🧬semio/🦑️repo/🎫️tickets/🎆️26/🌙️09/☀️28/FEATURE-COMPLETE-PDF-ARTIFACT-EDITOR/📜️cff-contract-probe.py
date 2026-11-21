"""🧪️ Compiles the staged native reader and applies the neutral CFF contract."""

from pathlib import Path
import json
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
stage = ticket / "🗑️generated/simple-cff-stage"
case = json.loads((stage / "🧫️contracts.json").read_text())
base = Path(sys.argv[1]).resolve() / "🔨️modules/🔤️fonts"
table = (base / "🅰️tables/🦀️.rs").read_text()
encoding = next(line for line in table.splitlines() if line.startswith("pub const STANDARD_ENCODING: "))
sfnt = (base / "🔠️truetype/🦀️.rs").as_posix()
reader = (stage / "cff.rs").as_posix()
font = list(bytes.fromhex(case["font"]))
fixtures = {"@FONT@": str(font), "@NAMES@": json.dumps(case["names"]),
            "@ENCODING@": "[" + ", ".join(f'({code}, {json.dumps(name)})' for code, name in case["encoding"]) + "]",
            "@GLYPH@": str(case["glyph"]), "@EXPERT@": str(list(bytes.fromhex(case["expertFont"]))),
            "@EXPERT_ENCODING@": "[" + ", ".join(f'({code}, {json.dumps(name)})' for code, name in case["expertEncoding"]) + "]"}
segments = []
for name, points in case["outline"]:
    if name == "closePath":
        segments.append("Close")
    else:
        variant = {"moveTo": "MoveTo", "lineTo": "LineTo"}[name]
        segments.append(f"{variant}({points[0]!r})")
fixtures["@SEGMENTS@"] = "[" + ", ".join(segments) + "]"

src = r'''
#[path = "@SFNT@"] mod truetype;
@STANDARD_ENCODING@
#[path = "@READER@"] mod reader;
fn main() {
    let bytes = @FONT@;
    let font = reader::CffOutlineFont::new(&bytes).unwrap();
    let mut names = Vec::new();
    font.visit_glyph_names(|name, _| names.push(name)).unwrap();
    assert_eq!(names, @NAMES@);
    let map = font.builtin_encoding().unwrap();
    assert_eq!(map.iter().map(|(code, name)| (*code, name.as_str())).collect::<Vec<_>>(), @ENCODING@);
    use truetype::TrueTypeOutlineSegment::*;
    assert_eq!(font.raw_outline(@GLYPH@).unwrap(), @SEGMENTS@);
    let expert = @EXPERT@;
    let expert = reader::CffOutlineFont::new(&expert).unwrap();
    let map = expert.builtin_encoding().unwrap();
    for (code, name) in @EXPERT_ENCODING@ {
        assert_eq!(map.iter().find(|(key, _)| *key == code).map(|(_, name)| name.as_str()), Some(name));
    }
    println!("[DEBUG] Native CFF matches the neutral name, encoding and geometry fixture");
}
'''
src = src.replace("@SFNT@", sfnt).replace("@STANDARD_ENCODING@", encoding).replace("@READER@", reader)
for name, replacement in fixtures.items():
    src = src.replace(name, replacement)
source = stage / "contract-main.rs"
source.write_text(src)
binary = stage / ("contract-native.exe" if os.name == "nt" else "contract-native")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                "--edition=2021", "-Awarnings", str(source), "-o", str(binary)], check=True)
subprocess.run([str(binary)], check=True)
