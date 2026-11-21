"""🎨 Verify mounted PDF text metrics, vector fill and document properties independently."""
import json
from pathlib import Path
import sys

import pymupdf

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated"
law = json.loads((TICKET / "🔣️mounted-style-properties.json").read_text(encoding="utf8"))
target = Path(sys.argv[1]) if len(sys.argv) > 1 else OUT / "pdf-mounted-style-properties.pdf"
source = OUT / "pdf-mounted-content-input.pdf"


def measure(path, phase):
    with pymupdf.open(path) as document:
        assert not document.is_repaired and document.page_count == 2
        assert document.metadata["author"] == law["author"]
        assert document.metadata["subject"] == law["subject"][phase]
        language = document.xref_get_key(document.pdf_catalog(), "Lang")
        assert language == (("string", law["language"][phase]) if law["language"][phase] else ("null", "null"))
        page = document[0]
        spans = [span for block in page.get_text("dict")["blocks"] for line in block.get("lines", []) for span in line["spans"]]
        text = [span for span in spans if span["text"] == law["text"]]
        assert len(text) == 1
        assert abs(text[0]["size"] - float(law["fontSize"][phase])) < 1e-5
        drawn = [drawing["fill"] for drawing in page.get_drawings() if drawing.get("fill") is not None]
        assert len(drawn) == 1
        assert all(abs(a - b) < 1e-5 for a, b in zip(drawn[0], [float(law["fillRed"][phase]), 0.3, 0.4], strict=True))
        return document[1].get_pixmap().samples


if __name__ == "__main__":
    before = measure(source, "before")
    if len(sys.argv) < 2 or sys.argv[1] != "baseline":
        assert measure(target, "after") == before
        print("[DEBUG] Mounted PDF retained precise font size, fill, language, subject and untouched-page pixels")
    else:
        print("[DEBUG] The original mounted style witness matches its initial font, vector and document properties")
