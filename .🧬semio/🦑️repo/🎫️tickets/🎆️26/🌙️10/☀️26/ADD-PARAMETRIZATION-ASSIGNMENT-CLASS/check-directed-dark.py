"""🌘 Verify the final native directed dark PDF and its retained typography."""
import json
from pathlib import Path
import pdfplumber
from pypdf import PdfReader

ticket = Path(__file__).resolve().parent
pdf = ticket / "🗑️generated/compiled-dark/parametrization-de-dark.pdf"
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
            assert character["x0"] >= 0 and character["x1"] <= page.width
            assert character["top"] >= 0 and character["bottom"] <= page.height
    ledger = [c for c in document.pages[2].chars if 380 < c["top"] < 575]
    digits = [c for c in ledger if c["text"].isdigit()]
    assert digits and all("Anta" in c["fontname"] for c in digits)
    arrows = [c for c in ledger if c["text"] == "→"]
    assert len(arrows) == 20
    page_one_labels = [c for c in document.pages[0].chars if 300 < c["top"] < 495]
    labels = [c for c in page_one_labels if c["text"] == "F"]
    assert len(labels) == 6
    result = {"pages": len(document.pages), "bytes": pdf.stat().st_size, "ledger_arrows": len(arrows), "sans_ledger_digits": len(digits), "worked_example_form_labels": len(labels), "overfull": 0, "underfull": 0}
print(json.dumps(result))
