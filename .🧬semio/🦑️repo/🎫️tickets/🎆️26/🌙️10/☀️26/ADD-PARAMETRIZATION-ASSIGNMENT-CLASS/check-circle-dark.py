"""🌘 Verify metadata, page bounds, and native text in the circular dark assignment."""
import json
from pathlib import Path
import pdfplumber
from pypdf import PdfReader

pdf = Path(__file__).parent / "🗑️generated/compiled-dark/parametrization-de-dark.pdf"
reader = PdfReader(pdf)
log = pdf.with_suffix(".log").read_text(encoding="utf-8")
assert len(reader.pages) == 3
assert reader.trailer["/Root"].get("/Lang") == "de-DE"
assert reader.metadata.title == "Fingerübung Parametrisierung"
assert reader.metadata.author == "Architektur"
assert "Overfull" not in log and "Underfull" not in log
with pdfplumber.open(pdf) as document:
    for page in document.pages:
        for character in page.chars:
            assert 0 <= character["x0"] <= character["x1"] <= page.width
            assert 0 <= character["top"] <= character["bottom"] <= page.height
    arrows = [c for c in document.pages[2].chars if c["text"] == "→"]
    assert len(arrows) == 20
    ledger_top = min(c["top"] for c in arrows) - 1
    ledger_bottom = max(c["bottom"] for c in arrows) + 1
    digits = [c for c in document.pages[2].chars if c["text"].isdigit() and ledger_top < c["top"] < ledger_bottom]
    assert digits and all("Anta" in c["fontname"] for c in digits)
    print(json.dumps({"pages": len(document.pages), "bytes": pdf.stat().st_size, "ledger_arrows": len(arrows), "sans_ledger_digits": len(digits), "overfull": 0, "underfull": 0}))
