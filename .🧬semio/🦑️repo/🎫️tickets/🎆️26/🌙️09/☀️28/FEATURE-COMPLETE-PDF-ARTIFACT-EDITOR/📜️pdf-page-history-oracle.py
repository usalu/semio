"""📄 Build and independently verify the mounted page-operation witness."""

import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
import pymupdf

ROOT = Path(__file__).resolve().parent
OUT = ROOT / "🗑️generated"
CASES = json.loads((ROOT / "🔣️mounted-page-history.json").read_text(encoding="utf8"))
PAGES = {page["id"]: page for page in CASES["pages"]}
SOURCE = OUT / "pdf-page-history-source.pdf"


def build():
    """🏗️ Generate original pages with distinct content and actual display dimensions."""
    spec = importlib.util.spec_from_file_location("page_pdf_oracle", ROOT / "controlled-dct-oracle.py")
    helper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helper)
    contents = [PAGES[name] for name in CASES["states"][0]]
    page_ids = list(range(4, 4 + len(contents)))
    kids = " ".join(f"{num} 0 R" for num in page_ids)
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        (f"<< /Type /Pages /Kids [{kids}] /Count {len(contents)} >>").encode(),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
    ]
    for i, page in enumerate(contents):
        leaf = str(page["width"]) + " " + str(page["height"])
        obj = f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {leaf}] /Resources << /Font << /F 3 0 R >> >> /Contents {4 + len(contents) + i} 0 R >>"
        objects.append(obj.encode("ascii"))
    for page in contents:
        label = page["text"]
        assert label.isascii() and label.replace(" ", "").isalnum()
        objects.append(helper.stream(b"", f"BT /F 18 Tf 25 80 Td ({label}) Tj ET".encode("ascii")))
    SOURCE.write_bytes(helper.document(objects))
    with pymupdf.open(SOURCE) as original:
        verify_order(original, CASES["states"][0])
    print(f"[DEBUG] Generated an independent {SOURCE.stat().st_size}-byte fixture with distinguishable pages")


def verify_order(document, state):
    assert not document.is_repaired
    assert document.page_count == len(state)
    for observed, identifier in zip(document, state, strict=True):
        expected = PAGES[identifier]
        assert observed.rect == pymupdf.Rect(0, 0, expected["width"], expected["height"]), identifier
        assert observed.get_text().strip() == (expected["text"] or ""), identifier


def verify():
    """🎯 Check the delivered PDF and compare the preserved page's actual pixels."""
    state = CASES["states"][-1]
    saved = Path(sys.argv[2]) if len(sys.argv) > 2 else OUT / "pdf-mounted-page-history.pdf"
    with pymupdf.open(saved) as actual, pymupdf.open(SOURCE) as before:
        verify_order(actual, state)
        untouched = CASES["states"][0].index(state[0])
        assert actual[0].get_pixmap().samples == before[untouched].get_pixmap().samples
    print("[DEBUG] Page order, dimensions, content and untouched-page raster matched the independent oracle")


if __name__ == "__main__":
    {"build": build, "verify": verify}[sys.argv[1]]()
