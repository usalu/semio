"""📐 Independent MuPDF checks for the shared retained page-content program."""
from pathlib import Path
import sys

import fitz


def create_pdf(source):
    with fitz.open() as document:
        page = document.new_page(width=100, height=90)
        fonts = []
        for base in ["Helvetica", "Courier"]:
            ref = document.get_new_xref()
            document.update_object(ref, f"<< /Type /Font /Subtype /Type1 /BaseFont /{base} >>")
            fonts.append(ref)
        content = document.get_new_xref()
        document.update_object(content, "<< >>")
        document.update_stream(content, source)
        document.xref_set_key(page.xref, "Resources", f"<< /Font << /F1 {fonts[0]} 0 R /F2 {fonts[1]} 0 R >> >>")
        document.xref_set_key(page.xref, "Contents", f"{content} 0 R")
        return document.tobytes()


def verify(fixture, output):
    rendered = create_pdf(fixture.read_bytes())
    output.write_bytes(rendered)
    with fitz.open(stream=rendered, filetype="pdf") as document:
        page = document[0]
        assert page.get_text().strip().splitlines() == ["Alpha", "Beta", "Gamma", "Delta"]
        spans = [span for block in page.get_text("dict")["blocks"] for line in block["lines"] for span in line["spans"]]
        assert [(span["font"], span["size"]) for span in spans] == [("Helvetica", 12.0)] * 4
        stroke, fill = page.get_drawings()
        assert stroke["type"] == "s" and stroke["color"] == (0.0, 0.0, 1.0)
        assert fill["type"] == "f" and fill["fill"] == (1.0, 0.0, 0.0)
        assert tuple(fill["rect"]) == (20.0, 55.0, 60.0, 70.0)
        pixels = page.get_pixmap()
        assert pixels.pixel(24, 80) == (0, 0, 255)
        assert pixels.pixel(32, 80) == (255, 255, 255)
        assert pixels.pixel(30, 62) == (255, 0, 0)
    print("MuPDF matched four restored-font text lines, the transformed rectangle, and the dash/on-gap pixels.")


if __name__ == "__main__":
    verify(*map(Path, sys.argv[1:]))
