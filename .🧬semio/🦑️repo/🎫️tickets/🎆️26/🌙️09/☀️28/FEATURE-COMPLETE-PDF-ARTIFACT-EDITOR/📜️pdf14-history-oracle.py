"""📚 Verify PDF 1.4 history fixture documents with MuPDF."""

from pathlib import Path
import json
import math

import fitz

TICKET = Path(__file__).resolve().parent


def write(pages):
    """📄 Make a PDF from the language-neutral document snapshot."""
    document = fitz.open()
    for row in pages:
        page = document.new_page(width=row["width"], height=row["height"])
        page.insert_text((24, 48), row["text"])
    return document


def observed(document, expected):
    """📏 Check reopened page sizes, text and unaffected pages."""
    assert len(document) == len(expected)
    for page, row in zip(document, expected, strict=True):
        assert math.isclose(page.rect.width, row["width"], abs_tol=0.001)
        assert math.isclose(page.rect.height, row["height"], abs_tol=0.001)
        assert page.get_text().strip() == row["text"]


def run(name):
    """🧪 Apply the declared edit to PDF objects through MuPDF."""
    fixture = json.loads((TICKET / name).read_text(encoding="utf8"))
    snapshot = fixture["before"]["pages"]
    with write(snapshot) as original:
        observed(original, snapshot)
        before = original[0].get_pixmap().samples
        spec = fixture["mutation"]
        args = spec["payload"]
        page = original[args["index"]]
        if spec["mutation"] == "replace-page-text":
            (bounds,) = page.search_for(snapshot[args["index"]]["text"])
            page.add_redact_annot(bounds)
            assert page.apply_redactions()
            page.insert_text((24, 48), args["text"])
        else:
            assert spec["mutation"] == "resize-page"
            page.set_mediabox(fitz.Rect(0, 0, args["width"], args["height"]))
        with fitz.open(stream=original.tobytes(), filetype="pdf") as reopened:
            observed(reopened, fixture["after"]["pages"])
            assert reopened[0].get_pixmap().samples == before
    print(f"[DEBUG] Verified {name}")


if __name__ == "__main__":
    for fixture in ("🔣️pdf14a-history.json", "🔣️pdf14x-history.json"):
        run(fixture)
