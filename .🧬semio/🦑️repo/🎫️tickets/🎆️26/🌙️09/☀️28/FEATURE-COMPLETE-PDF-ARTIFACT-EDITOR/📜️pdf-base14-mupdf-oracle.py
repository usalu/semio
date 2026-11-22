"""🔠 Compare production CFF contour addresses and extents with independent PDF and font libraries."""
from pathlib import Path
import json
from math import isclose
import sys

from fontTools.pens.boundsPen import BoundsPen
from fontTools.ttLib import TTFont
import pymupdf

ticket = Path(__file__).resolve().parent
assets = ticket / "🗑️generated" / "pdf-base14"
case = json.loads((ticket / "🔣️pdf-base14-outlines.json").read_text(encoding="utf8"))
fonts = {}
for face in case["faces"]:
    path = assets / (face["program"] + ".otf")
    sfnt = TTFont(path)
    assert sfnt["head"].unitsPerEm == 1000
    fonts[face["font"]] = (sfnt, sfnt.getGlyphSet(), pymupdf.Font(face["font"]), face["glyphs"])
seen = set()
maximum = 0.0
reference_difference = 0.0
for line in Path(sys.argv[1]).read_text(encoding="utf8").splitlines():
    face, glyph, code, glyph_id, *measured = line.split("\t")
    sfnt, glyphs, reference, expected = fonts[face]
    code = int(code)
    assert {"name": glyph, "unicode": code} in expected
    assert (face, glyph) not in seen
    seen.add((face, glyph))
    assert sfnt.getGlyphID(glyph) == int(glyph_id), (face, glyph, glyph_id, sfnt.getGlyphID(glyph))
    pen = BoundsPen(glyphs)
    glyphs[glyph].draw(pen)
    actual = tuple(float(value) / 1000 for value in measured)
    oracle = tuple(value / 1000 for value in pen.bounds)
    maximum = max(maximum, *(abs(left - right) for left, right in zip(actual, oracle)))
    assert all(isclose(a, b, abs_tol=0.00003) for a, b in zip(actual, oracle)), (face, glyph, actual, oracle)
    assert reference.has_glyph(code, fallback=False), (face, glyph)
    built_in_box = tuple(reference.glyph_bbox(code))
    reference_difference = max(reference_difference, *(abs(left - right) for left, right in zip(actual, built_in_box)))
assert len(seen) == sum(len(row["glyphs"]) for row in case["faces"])
print(f"[DEBUG] FontTools confirmed {len(seen)} glyph IDs and contour boxes across {len(fonts)} fonts; maximum difference {maximum:.6f} em.")
print(f"[DEBUG] MuPDF {pymupdf.__version__} built-in replacement faces cover the same {len(seen)} Unicode probes; contour box maximum difference {reference_difference:.6f} em.")

from fontTools.pens.recordingPen import RecordingPen

paint_overlay = assets / "font-code-overlay.json"
if paint_overlay.exists():
    records = json.loads(paint_overlay.read_text(encoding="utf8"))
    fixture = json.loads(next(body for path, _, body in records if path.endswith("🧪️text-paint.json")))
    with pymupdf.open() as pdf:
        page = pdf.new_page(width=fixture["page"][0], height=fixture["page"][1])
        to_view = pymupdf.Matrix(fixture["textMatrix"]) * pymupdf.Matrix(fixture["ctm"]) * page.transformation_matrix
        rise = fixture["textRise"]
        origin = pymupdf.Point(0, rise) * to_view
        x_axis = pymupdf.Point(1, rise) * to_view - origin
        y_axis = pymupdf.Point(0, rise + 1) * to_view - origin
        assert [*x_axis,*y_axis,*origin] == fixture["expected"]["transform"]
    sfnt, glyphs, _, _ = fonts[fixture["baseFont"]]
    cmap = sfnt.getBestCmap()
    pen = RecordingPen()
    for char in fixture["expected"]["text"]["content"]:
        glyphs[cmap[ord(char)]].draw(pen)
    assert sum(command == "closePath" for command, _ in pen.value) == fixture["expected"]["contours"]
    print("[DEBUG] MuPDF confirmed the staged scene baseline and FontTools confirmed its contour count.")
