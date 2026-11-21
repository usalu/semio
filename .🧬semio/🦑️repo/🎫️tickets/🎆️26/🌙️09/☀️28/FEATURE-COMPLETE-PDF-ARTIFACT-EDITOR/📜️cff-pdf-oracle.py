"""📰 Builds and independently opens a small PDF with an OpenType CFF font."""
from io import BytesIO
import json
from pathlib import Path
import sys

import fitz
from fontTools import subset
from fontTools.ttLib import TTFont

fixture, generated = map(Path, sys.argv[1:])
case = json.loads(fixture.read_text(encoding="utf-8"))
text = "AgRé"
font = TTFont(case["fonts"][0])
options = subset.Options()
options.glyph_names = True
subsetter = subset.Subsetter(options=options)
subsetter.populate(text=text)
subsetter.subset(font)
output = BytesIO()
font.save(output)
font = output.getvalue()

def show_stream(body, extra=b""):
    return b"<< /Length " + str(len(body)).encode() + b" " + extra + b">>\nstream\n" + body + b"\nendstream"

char_bytes = text.encode("cp1252")
lines = [
    b"<< /Type /Catalog /Pages 2 0 R >>",
    b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
    b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 160 110] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>",
    show_stream(b"BT /F1 32 Tf 1 0 0 1 20 40 Tm <" + char_bytes.hex().encode() + b"> Tj ET"),
    b"<< /Type /Font /Subtype /Type1 /BaseFont /LibertinusSerif-Regular /Encoding /WinAnsiEncoding /FontDescriptor 6 0 R /FirstChar 65 /LastChar 233 /Widths [" + b"500 " * 169 + b"] >>",
    b"<< /Type /FontDescriptor /FontName /LibertinusSerif-Regular /Flags 34 /FontBBox [-660 -445 1400 1055] /ItalicAngle 0 /Ascent 800 /Descent -200 /CapHeight 675 /StemV 90 /FontFile3 7 0 R >>",
    show_stream(font, b"/Subtype /OpenType "),
]
data = bytearray(b"%PDF-1.7\n%\xB5\xED\xAE\xFB\n")
offsets = [0]
for index, row in enumerate(lines, 1):
    offsets.append(len(data))
    data.extend(str(index).encode() + b" 0 obj\n" + row + b"\nendobj\n")
xref = len(data)
data.extend(f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode())
for offset in offsets[1:]:
    data.extend(f"{offset:010} 00000 n \n".encode())
data.extend(f"trailer\n<< /Root 1 0 R /Size {len(offsets)} >>\nstartxref\n{xref}\n%%EOF\n".encode())
assert 0 < len(data) < 32 * 1024

with fitz.open(stream=bytes(data), filetype="pdf") as pdf:
    page = pdf[0]
    assert page.get_text().strip() == text
    chars = [ch for block in page.get_text("rawdict")["blocks"] for line in block["lines"] for span in line["spans"] for ch in span["chars"]]
    assert [ch["origin"] for ch in chars] == [(20, 70), (36, 70), (52, 70), (68, 70)]
    bitmap = page.get_pixmap(colorspace=fitz.csRGB)
    black_pixels = [
        sum(bitmap.pixel(x, y) == (0, 0, 0) for y in range(42, 81) for x in range(left, left + 15))
        for left in [20, 36, 52, 68]
    ]
    assert all(count > 8 for count in black_pixels), black_pixels
    bitmap.save(generated / "cff-pdf-oracle.png")
(generated / "cff-opentype.pdf").write_bytes(data)
print(f"[DEBUG] MuPDF verified four OpenType CFF glyphs and their positions in a {len(data)}-byte PDF. Black pixel counts {black_pixels}.")
