"""🔠 Check independent outline segments and glyph points with FontTools."""
import json
from pathlib import Path
import sys

from fontTools.ttLib import TTFont
from fontTools.pens.recordingPen import DecomposingRecordingPen

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
native = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
with TTFont(fixture["font"]) as font:
    mapping = font.getBestCmap()
    glyphs = font.getGlyphSet()
    for char, minimum, result in zip(fixture["characters"], fixture["minimumSegments"], native):
        pen = DecomposingRecordingPen(glyphs)
        glyph = mapping[ord(char)]
        glyphs[glyph].draw(pen)
        outline = native[fixture["characters"].index(char)]
        if not pen.value:
            assert outline == []
            continue
        assert len(result) >= minimum
        assert sum(row[0] == "moveTo" for row in result) == sum(row[0] == "moveTo" for row in pen.value)
        coordinates, _, _ = font["glyf"][glyph].getCoordinates(font["glyf"])
        bounds = lambda points: tuple(operation(point[axis] for point in points) for operation in (min, max) for axis in (0, 1))
        assert all(abs(a - b) < 1e-5 for a, b in zip(bounds(coordinates), bounds([point for _, *points in result for point in points])))
    print(f"FontTools matched contour count and control-point extent for {len(native)} glyph outlines.")
