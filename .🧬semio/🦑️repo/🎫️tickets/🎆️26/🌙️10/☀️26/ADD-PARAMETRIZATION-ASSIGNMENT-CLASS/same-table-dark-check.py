"""🌘 Verify native German dark assignment bounds and metadata."""
from pathlib import Path
import hashlib
import json
import pdfplumber
from pypdf import PdfReader

ticket = Path(__file__).resolve().parent
pdf = ticket / "🗑️generated/dark-compiled/parametrization-de-dark.pdf"
reader = PdfReader(pdf)
log = pdf.with_suffix(".log").read_text(encoding="utf-8")
assert len(reader.pages) == 3
assert reader.trailer["/Root"].get("/Lang") == "de-DE"
assert reader.metadata.title == "Fingerübung Parametrisierung"
assert reader.metadata.author == "Architektur"
assert "Overfull" not in log and "Underfull" not in log
with pdfplumber.open(pdf) as document:
    for index, page in enumerate(document.pages):
        assert all(-.5 <= c["x0"] <= c["x1"] <= page.width+.5 and -.5 <= c["top"] <= c["bottom"] <= page.height+.5 for c in page.chars), index+1
    text = [page.extract_text() for page in document.pages]
    assert "Beispiel" in text[0]
    assert "Wohnkapseln" in text[1]
    assert "20" in text[2]
result = {"pages": 3, "bytes": pdf.stat().st_size, "language": reader.trailer["/Root"].get("/Lang"), "overfull": 0, "underfull": 0, "page_text_bounds": "pass"}
print(json.dumps(result))
