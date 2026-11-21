"""✒️ Compares native CFF1 contours with FontTools on the same OpenType assets."""
import json
import os
from pathlib import Path
import subprocess
import sys

from fontTools.pens.recordingPen import RecordingPen
from fontTools.ttLib import TTFont


stage, truetype, raw_fixture, generated = map(Path, sys.argv[1:5])
fixture = json.loads(raw_fixture.read_text(encoding="utf-8"))
probe = generated / "cff-outline-probe.rs"
binary = generated / ("cff-outline-probe.exe" if os.name == "nt" else "cff-outline-probe")
quote = lambda path: json.dumps(str(path.resolve()), ensure_ascii=False)
script = r'''
#![allow(dead_code, unused_imports)]
#[path = @CFF@] mod outline;
#[path = @SFNT@] mod sfnt;
use outline::{CffOutlineFont, CffOutlineSegment::*};

fn main() {
    let mut args = std::env::args().skip(1);
    let file = args.next().unwrap();
    let bytes = std::fs::read(file).unwrap();
    let font = sfnt::TrueTypeFont::parse(&bytes).unwrap();
    let &(start, length) = font.tables.get("CFF ").unwrap();
    let cff = CffOutlineFont::new(&bytes[start..start + length]).unwrap();
    let mut cids = Vec::new();
    if cff.visit_cids(|cid, glyph| cids.push(format!("[{cid},{glyph}]"))).is_some() {
        println!("[{}]", cids.join(","));
    } else {
        println!("null");
    }
    let point = |[x,y]: [f64; 2]| format!("[{x},{y}]");
    for ch in args {
        let glyph = font.glyph_for_char(ch.chars().next().unwrap()).unwrap();
        let rows = cff.outline(glyph).unwrap_or_else(|| panic!("unable to outline {ch:?} with ID {glyph}"));
        let rows = rows.into_iter().map(|segment| match segment {
            MoveTo(to) => format!(r#"["moveTo",[{}]]"#, point(to)),
            LineTo(to) => format!(r#"["lineTo",[{}]]"#, point(to)),
            CubicTo { first, second, to } => format!(r#"["curveTo",[{},{},{}]]"#, point(first), point(second), point(to)),
            Close => r#"["closePath",[]]"#.to_string(),
        }).collect::<Vec<_>>();
        println!("[{}]", rows.join(","));
    }
}
'''
if "--integrated" in sys.argv[5:]:
    tables = (truetype.parent.parent / "🅰️tables/🦀️.rs").read_text()
    encoding = next(line for line in tables.splitlines() if line.startswith("pub const STANDARD_ENCODING: "))
    script = f"mod tables {{ {encoding} }}\n" + script.replace("#![allow(dead_code, unused_imports)]", "")
    script = script.replace("#[path = @CFF@] mod outline;", "mod font_codec { use super::sfnt as truetype; use super::tables::STANDARD_ENCODING; #[path = @CFF@] mod cff; pub use cff::CffOutlineFont; }")
    script = script.replace("use outline::{CffOutlineFont, CffOutlineSegment::*};", "use font_codec::CffOutlineFont; use sfnt::TrueTypeOutlineSegment::*;")
    script = script.replace("            Close =>", "            QuadraticTo { .. } => unreachable!(\"CFF produces cubic curves\"),\n            Close =>")
probe.write_text(script.replace("@CFF@", quote(stage)).replace("@SFNT@", quote(truetype)), encoding="utf-8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition", "2021", "-Awarnings", str(probe), "-o", str(binary)], check=True)
total = 0
fonts = [(path, fixture["characters"]) for path in fixture["fonts"]]
fonts.append((fixture["cidFont"]["path"], fixture["cidFont"]["characters"]))
for path, characters in fonts:
    font = TTFont(path)
    glyphs = font.getGlyphSet()
    cmap = font.getBestCmap()
    result = subprocess.check_output([str(binary), path, *characters], text=True).splitlines()
    [cid_rows, *outlines] = result
    actual_cids = json.loads(cid_rows)
    top = font["CFF "].cff.topDictIndex[0]
    expected_cids = [[0, 0]] + [[int(name[3:]), gid] for gid, name in enumerate(top.charset) if gid != 0] if hasattr(top, "ROS") else None
    assert actual_cids == expected_cids, path
    assert len(outlines) == len(characters)
    for ch, row in zip(characters, outlines):
        pen = RecordingPen()
        glyphs[cmap[ord(ch)]].draw(pen)
        expected = json.loads(json.dumps(pen.value))
        actual = json.loads(row)
        if actual != expected:
            for n, (one, two) in enumerate(zip(actual, expected)):
                if one != two:
                    raise AssertionError((path, ch, n, one, two))
            raise AssertionError((path, ch, len(actual), len(expected)))
        total += 1
print(f"[DEBUG] FontTools matched all outline commands and cubic control points in {total} glyphs and the complete CID charset.")
