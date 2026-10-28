"""🌘 Verify the actual German dark PDF metadata and compiler box diagnostics."""

import json
from pathlib import Path
from pypdf import PdfReader

ticket = Path(__file__).resolve().parent
pdf = ticket / "🗑️generated/compiled/parametrization-de-dark.pdf"
log = pdf.with_suffix(".log").read_text(encoding="utf-8")
reader = PdfReader(pdf)
result = {
    "pages": len(reader.pages),
    "bytes": pdf.stat().st_size,
    "title": reader.metadata.title,
    "author": reader.metadata.author,
    "language": reader.trailer["/Root"].get("/Lang"),
    "overfull": log.count("Overfull"),
    "underfull": log.count("Underfull"),
}
assert result["pages"] == 3, result
assert result["language"] == "de-DE", result
assert result["title"] == "Fingerübung Parametrisierung", result
assert result["author"] == "Architektur", result
assert result["overfull"] == result["underfull"] == 0, result
print(json.dumps(result, ensure_ascii=False))
