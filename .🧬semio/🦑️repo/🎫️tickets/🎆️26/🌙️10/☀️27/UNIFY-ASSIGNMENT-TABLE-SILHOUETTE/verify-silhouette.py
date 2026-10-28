import json
import sys
import pdfplumber
from pypdf import PdfReader

source = sys.argv[1]
reader = PdfReader(source)
assert len(reader.pages) == 1
assert "xdvipdfmx" in reader.metadata.producer
with pdfplumber.open(source) as pdf:
    page = pdf.pages[0]
    words = page.extract_words()
    assert sum(word["text"] == "Parameter" for word in words) == 2
    horizontal = [edge for edge in page.edges if edge["orientation"] == "h" and edge["width"] > 475]
    ys = sorted({round(edge["top"], 2) for edge in horizontal})
    grids = []
    for start in range(len(ys) - 9):
        rows = ys[start:start + 10]
        gaps = [rows[index + 1] - rows[index] for index in range(9)]
        if 44 < gaps[0] < 47 and all(16 < gap < 19 for gap in gaps[1:]):
            grids.append(rows)
    assert len(grids) == 2, grids
    details = []
    for rows in grids:
        border = next(edge for edge in horizontal if abs(edge["top"] - rows[0]) < .3)
        left, right, top = border["x0"], border["x1"], rows[0]
        joins = []
        for x in [left, right]:
            join = [edge for edge in page.edges if edge["orientation"] == "v" and abs(edge["x0"] - x) < .6 and edge["top"] < top - 3 and edge["bottom"] >= top - .6]
            assert join, {"missing_caption_join": x, "top": top}
            ink_gap = max(0, top - border.get("linewidth", 0) / 2 - max(edge["bottom"] for edge in join))
            assert ink_gap < .03, {"open_caption_seam_pt": ink_gap}
            joins.append(ink_gap)
        details.append({"grid_top": top, "caption_ink_gaps_mm": [round(value * 25.4 / 72, 3) for value in joins], "rows": 9})
    print(json.dumps({"pages": 1, "tables": details, "producer": reader.metadata.producer}))
