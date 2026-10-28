"""🔎 Inspect English outer-and-inner-cycle PDF content, fonts and writing space."""
from pathlib import Path
import json
import re
import sys
import pdfplumber
from pypdf import PdfReader

source = Path(sys.argv[1])
reader = PdfReader(source)
assert len(reader.pages) == 3
assert reader.metadata.title == "Finger Exercise: Parametrization"
assert reader.metadata.author == "Architecture"
assert reader.trailer["/Root"]["/Lang"] == "en-US"
log = source.with_suffix(".log").read_text(encoding="utf-8")
assert "Overfull" not in log and "Underfull" not in log
mm = 72 / 25.4
with pdfplumber.open(source) as pdf:
    for page in pdf.pages:
        assert all(0 <= c["x0"] <= c["x1"] <= page.width + 1 and 0 <= c["top"] <= c["bottom"] <= page.height + 1 for c in page.chars if c["text"].strip())
    texts = [" ".join((page.extract_text() or "").split()) for page in pdf.pages]
    assert all(word in texts[0] for word in ["Unique.", "Consistent.", "Minimal."])
    assert "Table A" in texts[1] and "Table B" in texts[1]
    assert "20 marked" in texts[2] and "edges." in texts[2] and "arrow direction" in texts[2]
    page = pdf.pages[2]
    rules = sorted([line for line in page.lines if abs(line.get("linewidth", 0) - .3) < .02 and 20 * mm < line["width"] < 90 * mm and line["height"] < .1], key=lambda line: (round(line["x0"], 1), line["top"]))
    assert len(rules) == 20
    labels, starts, digits, arrows = [], [], [], []
    for line in rules:
        crop = page.crop((max(0, line["x0"] - 23 * mm), line["top"] - 5 * mm, line["x0"] + 1, line["top"] + 1))
        text = crop.extract_text() or ""
        labels.append(tuple(map(int, re.findall(r"\d+", text))))
        starts.append(min(c["x0"] for c in crop.chars if c["text"].strip()))
        digits.extend(c["fontname"] for c in crop.chars if c["text"].isdigit())
        arrows.extend(c["fontname"] for c in crop.chars if c["text"] == "→")
        assert not page.crop((line["x0"] + .5, line["top"] - 3 * mm, line["x1"] - .5, line["top"] - .2)).chars
    assert labels == [(i, i + 1) for i in range(1, 16)] + [(16, 1), (1, 5), (5, 10), (10, 13), (13, 1)]
    gaps = [(starts[i + 10] - rules[i]["x1"]) / mm for i in range(10)]
    assert min(gaps) >= 3
    assert digits and all("Anta-Regular" in font for font in digits)
    assert len(arrows) == 20 and all("CMSY10" in font for font in arrows)
    print(json.dumps({"pages": 3, "bytes": source.stat().st_size, "language": "en-US", "overfull": 0, "underfull": 0, "ledger_labels": len(labels), "minimum_gutter_mm": round(min(gaps), 2), "minimum_blank_rule_mm": round(min(line["width"] for line in rules) / mm, 2), "digit_fonts": sorted(set(digits)), "arrow_fonts": sorted(set(arrows))}))
