"""🎹 Validate the editor's PDF text baseline with PyMuPDF 1.24.14."""

import json
from math import hypot, isclose
from pathlib import Path
import sys

import fitz


def verify(path):
    fixture = json.loads(path.read_text(encoding="utf-8"))
    expected = fixture["expected"]
    content = expected["text"]["content"]
    ctm = " ".join(map(str, fixture["ctm"]))
    text_matrix = " ".join(map(str, fixture["textMatrix"]))
    with fitz.open() as document:
        page = document.new_page(width=fixture["page"][0], height=fixture["page"][1])
        font = document.get_new_xref()
        document.update_object(font, f"<< /Type /Font /Subtype /Type1 /BaseFont /{fixture['baseFont']} >>")
        document.xref_set_key(page.xref, "Resources", f"<< /Font << /F1 {font} 0 R >> >>")
        stream = document.get_new_xref()
        document.update_object(stream, "<<>>")
        document.update_stream(stream, f"{ctm} cm BT /F1 {expected['text']['size']} Tf {text_matrix} Tm {fixture['textRise']} Ts ({content}) Tj ET".encode("ascii"))
        page.set_contents(stream)
        line, = page.get_text("dict")["blocks"][0]["lines"]
        span, = line["spans"]
    assert span["text"] == content, span
    a, b, _, _, x, y = expected["transform"]
    assert span["origin"] == (x, y), span
    direction_length = hypot(a, b)
    assert all(isclose(actual, projected / direction_length, abs_tol=1e-6) for actual, projected in zip(line["dir"], (a, b))), line
    print("Text content, baseline origin, and direction match")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
