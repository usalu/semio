"""🔍 Render compiled assignment PDFs to page images and report pages, metadata, overfull boxes and out-of-page glyphs."""

import json
import sys
from pathlib import Path

import pdfplumber
import pypdfium2 as pdfium
from pypdf import PdfReader

sys.stdout.reconfigure(encoding="utf-8")
STAGE = Path(sys.argv[1])
OUTPUT = Path(sys.argv[2])
OUTPUT.mkdir(parents=True, exist_ok=True)
for language in sys.argv[3:] or ["en", "de"]:
    pdf = STAGE / "compiled" / language / f"{language}.pdf"
    reader = PdfReader(pdf)
    document = pdfium.PdfDocument(pdf)
    for index in range(len(document)):
        document[index].render(scale=1.6).to_pil().save(OUTPUT / f"{language}-{index + 1}.png")
    with pdfplumber.open(pdf) as plumber:
        outside = [char["text"] for page in plumber.pages for char in page.chars if char["text"].strip() and (char["x0"] < 0 or char["x1"] > page.width + 1)]
        text = "\n".join(page.extract_text() or "" for page in plumber.pages)
    log = (pdf.with_suffix(".log")).read_text(encoding="utf-8", errors="replace")
    print(json.dumps({"language": language, "pages": len(reader.pages), "bytes": pdf.stat().st_size, "title": reader.metadata.title, "lang": str(reader.trailer["/Root"].get("/Lang")), "overfull": log.count("Overfull"), "outside": len(outside), "missing": log.count("Missing character")}, ensure_ascii=False))
    (OUTPUT / f"{language}.txt").write_text(text, encoding="utf-8")
