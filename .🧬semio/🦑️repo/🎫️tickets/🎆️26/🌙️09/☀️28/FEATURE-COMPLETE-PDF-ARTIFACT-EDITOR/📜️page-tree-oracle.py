"""🌳 Verify page tree order and inherited semantics with MuPDF."""
from pathlib import Path
import sys

import fitz


def verify(path):
    with fitz.open(path) as pdf:
        assert not pdf.is_repaired
        expected = [
            (6, (0, 0, 200, 250), (10, 20, 190, 230), 180, "FInner", "Courier", "Override"),
            (5, (0, 0, 300, 400), (10, 20, 190, 230), 90, "FInner", "Courier", "Inner"),
            (3, (0, 0, 300, 400), (0, 0, 300, 400), 0, "FRoot", "Helvetica", "Root"),
        ]
        assert pdf.page_count == len(expected)
        for page, (reference, media, crop, rotation, name, font, text) in zip(pdf, expected):
            assert page.xref == reference
            assert tuple(page.mediabox) == media
            box = page.cropbox
            assert (box.x0, media[3] - box.y1, box.x1, media[3] - box.y0) == crop
            assert page.rotation == rotation
            assert [(entry[4], entry[3]) for entry in page.get_fonts()] == [(name, font)]
            assert page.get_text().strip() == text


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
    print("MuPDF matched all three pages in order, including inherited boxes, rotations, fonts, and text")
