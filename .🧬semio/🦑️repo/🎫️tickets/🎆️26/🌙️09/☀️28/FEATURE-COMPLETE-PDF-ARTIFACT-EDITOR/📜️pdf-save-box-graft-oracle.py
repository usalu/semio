"""📐 Verify paid Page Box Save output with MuPDF."""

import json
from pathlib import Path
import sys

import fitz

TICKET = Path(__file__).resolve().parent
CASE = json.loads((TICKET / "🔣️pdf-save-box-graft.json").read_text())


def verify(directory):
    """🔎 Compare all box coordinates, the untouched first page, rotation and independent text."""
    target = directory / "out.pdf"
    reference = directory / "in.pdf"
    with fitz.open(target) as actual, fitz.open(reference) as original:
        assert not actual.is_repaired and len(actual) == len(original) == 2
        page = actual[CASE["page"]]
        for name in ("mediaBox", "cropBox", "trimBox", "bleedBox", "artBox"):
            key = name[0].upper() + name[1:]
            tag, coordinates = actual.xref_get_key(page.xref, key)
            assert tag == "array" and [int(value) for value in coordinates.strip("[]").split()] == CASE[name], key
        assert int(actual.xref_get_key(page.xref, "UserUnit")[1]) == CASE["userUnit"]
        assert page.rotation == CASE["rotate"]
        assert CASE["preserveText"] in page.get_text()
        assert actual[0].get_pixmap().samples == original[0].get_pixmap().samples
        assert actual[1].get_text().strip() == original[1].get_text().strip()
    print("[DEBUG] MuPDF verified all five page boxes, UserUnit, rotation and untouched page")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
