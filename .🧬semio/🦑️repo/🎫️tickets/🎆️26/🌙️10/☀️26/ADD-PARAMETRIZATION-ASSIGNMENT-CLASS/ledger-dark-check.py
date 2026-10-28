"""🌘 Verify the dark worked example and exercise share ruled ledgers."""
from pathlib import Path
import json
import pdfplumber
from pypdf import PdfReader

ticket = Path(__file__).resolve().parent
pdf = ticket / "🗑️generated/dark-compiled/parametrization-de-dark.pdf"
reader = PdfReader(pdf)
assert len(reader.pages) == 3
assert reader.trailer["/Root"].get("/Lang") == "de-DE"
assert reader.metadata.title == "Fingerübung Parametrisierung"
log = pdf.with_suffix(".log").read_text(encoding="utf-8")
assert "Overfull" not in log and "Underfull" not in log
rule_length = 64.6 * 72 / 25.4
report = {"pages": 3, "language": "de-DE", "ruled_ledgers": []}
with pdfplumber.open(pdf) as document:
    for index, page in enumerate(document.pages):
        assert all(-.5 <= char["x0"] <= char["x1"] <= page.width+.5 and -.5 <= char["top"] <= char["bottom"] <= page.height+.5 for char in page.chars), index + 1
        rules = [line for line in page.lines if abs(line["x1"]-line["x0"]-rule_length) < .5 and abs(line["y1"]-line["y0"]) < .1]
        if index in (0, 2):
            assert len(rules) == (6 if index == 0 else 20), (index+1, len(rules))
            columns = sorted(set(round(line["x0"], 1) for line in rules))
            assert len(columns) == 2, columns
            rows = sorted(set(round(line["top"], 1) for line in rules))
            assert len(rows) == (3 if index == 0 else 10), rows
            assert all(abs(rows[i+1]-rows[i]-6.3*72/25.4) < .2 for i in range(len(rows)-1)), rows
            for line in rules:
                chars = [char for char in page.chars if line["x0"]-.2 <= char["x0"] and char["x1"] <= line["x1"]+.2 and line["top"]-11 < char["top"] < line["top"]+1]
                assert bool(chars) == (index == 0), (index+1, line, chars)
                if chars:
                    assert max(char["bottom"] for char in chars) < line["top"]+.2, (line, chars)
                    assert min(char["size"] for char in chars if char["text"].strip()) >= 8.5, [(char["text"], char["size"]) for char in chars]
            report["ruled_ledgers"].append({"page": index+1, "rules": len(rules), "columns": len(columns), "rows": len(rows), "length_mm": 64.6, "row_pitch_mm": 6.3})
print(json.dumps(report))
