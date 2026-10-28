"""↗️ Verify directed ledger labels and writing gutters in the native PDF."""
from pathlib import Path
import json
import re
import sys
import pdfplumber

MM = 72 / 25.4
fixture = json.loads((Path(__file__).parent / "directed-geometry.fixture.json").read_text(encoding="utf-8"))
with pdfplumber.open(sys.argv[1]) as pdf:
    page = pdf.pages[2]
    rules = sorted([line for line in page.lines if abs(line.get("linewidth", 0)-.3) < .02 and 20*MM < line["width"] < 90*MM and line["height"] < .1], key=lambda line: (round(line["x0"], 1), line["top"]))
    assert len(rules) == 20, len(rules)
    labels = []
    starts = []
    for line in rules:
        crop = page.crop((max(0, line["x0"]-23*MM), line["top"]-5*MM, line["x0"]+1, line["top"]+1))
        text = crop.extract_text() or ""
        digits = re.findall(r"\d+", text)
        assert len(digits) == 2 and "→" in text, text
        labels.append(list(map(int, digits)))
        starts.append(min(char["x0"] for char in crop.chars if char["text"].strip()))
        blank = page.crop((line["x0"]+.5, line["top"]-3*MM, line["x1"]-.5, line["top"]-.2))
        assert not blank.chars, text
    assert labels == fixture["selectedDirections"], labels
    gaps = [(starts[index+10]-rules[index]["x1"])/MM for index in range(10)]
    assert min(gaps) >= 3, gaps
    width = min(line["width"] for line in rules)/MM
    print(f"PASS directed ledger: 20 arrow labels match ordered fixture; minimum gutter {min(gaps):.2f}mm; blank rules {width:.2f}mm")
