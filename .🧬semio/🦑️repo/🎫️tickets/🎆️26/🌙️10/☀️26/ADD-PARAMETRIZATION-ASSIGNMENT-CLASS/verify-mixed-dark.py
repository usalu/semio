"""🌘 Verify the settled mixed-form German dark PDF through independent PDF libraries."""
import json
import sys
from collections import Counter
from pathlib import Path
import pdfplumber
from pypdf import PdfReader

source = Path(sys.argv[1])
fixture = json.loads((Path(__file__).parent / "mixed-example.json").read_text(encoding="utf-8"))
reader = PdfReader(source)
log = source.with_suffix(".log").read_text(encoding="utf-8")
assert len(reader.pages) == 3
assert reader.trailer["/Root"].get("/Lang") == "de-DE"
assert "Overfull" not in log and "Underfull" not in log
with pdfplumber.open(source) as document:
    page = document.pages[0]
    horizontal = [edge for edge in page.edges if edge["orientation"] == "h" and edge["width"] > 400]
    boundaries = sorted({round(edge["top"], 2) for edge in horizontal})
    rows = []
    for top, bottom in zip(boundaries, boundaries[1:]):
        vertical = [edge for edge in page.edges if edge["orientation"] == "v" and abs(edge["top"] - top) < .5 and abs(edge["bottom"] - bottom) < .5]
        xs = sorted({round(edge["x0"], 2) for edge in vertical})
        if len(xs) == 5:
            rows.append((top, bottom, xs))
    assert len(rows) == 5, rows
    for index, parameter in enumerate(["L", "B", "H"]):
        top, bottom, xs = rows[index + 1]
        for column, shape in enumerate(fixture, 1):
            text = (page.crop((xs[column] + .5, top + .5, xs[column + 1] - .5, bottom - .5)).extract_text() or "").strip()
            assert text == str(shape[parameter]), (parameter, column, text, shape[parameter])
    top, bottom, xs = rows[4]
    categories = []
    for column in range(1, 4):
        categories.append((page.crop((xs[column] + .5, top + .5, xs[column + 1] - .5, bottom - .5)).extract_text() or "").strip())
    assert categories == ["Quader", "Pyramide", "ellipt. Zylinder"], categories
    edges = [line for line in page.lines if abs(line.get("linewidth", 0) - .8) < .02 and len(line.get("pts", [])) == 2]
    endpoints = Counter(tuple(round(value, 2) for value in point) for line in edges for point in line["pts"])
    assert len(edges) == 3 and len(endpoints) == 3 and set(endpoints.values()) == {2}
    curved = [curve for curve in page.curves if any(part[0] == "c" for part in curve.get("path", []))]
    assert len(curved) >= 4, len(curved)
    for index, current in enumerate(document.pages):
        assert all(-.5 <= char["x0"] <= char["x1"] <= current.width + .5 and -.5 <= char["top"] <= char["bottom"] <= current.height + .5 for char in current.chars), index + 1
    result = {"pages": len(reader.pages), "bytes": source.stat().st_size, "overfull": 0, "underfull": 0, "worked_rows": len(rows), "forms": categories, "native_k3_edges": len(edges), "elliptical_curves": len(curved), "page_text_bounds": "pass"}
print(json.dumps(result, ensure_ascii=False))
