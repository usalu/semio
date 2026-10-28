"""🌘 Inspect final dark assignment metadata, diagnostics, and page bounds."""
import json
import sys
from pathlib import Path
import pdfplumber
from pypdf import PdfReader
source = Path(sys.argv[1])
reader = PdfReader(source)
log = source.with_suffix(".log").read_text(encoding="utf-8")
assert len(reader.pages) == 3
assert reader.trailer["/Root"].get("/Lang") == "de-DE"
assert "Overfull" not in log and "Underfull" not in log
with pdfplumber.open(source) as document:
    for index, page in enumerate(document.pages):
        assert all(-.5 <= char["x0"] <= char["x1"] <= page.width+.5 and -.5 <= char["top"] <= char["bottom"] <= page.height+.5 for char in page.chars), index+1
print(json.dumps({"pages": len(reader.pages), "bytes": source.stat().st_size, "language": reader.trailer["/Root"].get("/Lang"), "overfull": 0, "underfull": 0, "page_text_bounds": "pass"}, ensure_ascii=True))
