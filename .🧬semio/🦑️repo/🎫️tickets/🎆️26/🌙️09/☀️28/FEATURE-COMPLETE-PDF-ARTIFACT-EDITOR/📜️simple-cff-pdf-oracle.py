"""🗞️ Authored PDF fixtures for a raw CFF with an explicit custom character encoding."""

import json
from pathlib import Path

import fitz
from fontTools import subset
from fontTools.ttLib import TTFont

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
case = json.loads((ticket / "🧫️cff-outlines.json").read_text())
text = "AgRé"
codes = b"FGHI"

def stream(body, fields=b""):
    return b"<< /Length " + str(len(body)).encode() + b" " + fields + b">>\nstream\n" + body + b"\nendstream"

def specimen(builtin):
    font = TTFont(case["fonts"][0])
    options = subset.Options()
    options.glyph_names = True
    sub = subset.Subsetter(options=options)
    sub.populate(text=text)
    sub.subset(font)
    cmap = font.getBestCmap()
    names = [cmap[ord(ch)] for ch in text]
    glyphs = [font.getGlyphID(name) for name in names]
    encoding = b" /Encoding << /BaseEncoding /WinAnsiEncoding /Differences [ 70 /A /g /R /eacute ] >>"
    if builtin:
        cff = font["CFF "].cff.topDictIndex[0]
        cff.Encoding = [".notdef"] * 256
        for code, name in zip(codes, names):
            cff.Encoding[code] = name
        encoding = b""
    program = font.getTableData("CFF ")
    rows = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 160 110] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>",
        stream(b"BT /F1 32 Tf 1 0 0 1 20 40 Tm <" + codes.hex().encode() + b"> Tj ET"),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /LibertinusSerif-Regular" + encoding + b" /FontDescriptor 6 0 R /FirstChar 70 /LastChar 73 /Widths [500 500 500 500] >>",
        b"<< /Type /FontDescriptor /FontName /LibertinusSerif-Regular /Flags 34 /FontBBox [-660 -445 1400 1055] /ItalicAngle 0 /Ascent 800 /Descent -200 /CapHeight 675 /StemV 90 /FontFile3 7 0 R >>",
        stream(program, b"/Subtype /Type1C "),
    ]
    data = bytearray(b"%PDF-1.7\n")
    offsets = [b"0000000000 65535 f \n"]
    for owner, row in enumerate(rows, 1):
        offsets.append(f"{len(data):010d} 00000 n \n".encode())
        data.extend(f"{owner} 0 obj\n".encode() + row + b"\nendobj\n")
    xref = len(data)
    data.extend(f"xref\n0 {len(offsets)}\n".encode() + b"".join(offsets))
    data.extend(f"trailer\n<< /Root 1 0 R /Size {len(offsets)} >>\nstartxref\n{xref}\n%%EOF\n".encode())
    assert 0 < len(data) < 32_768
    return data, glyphs

output.mkdir(exist_ok=True)
for builtin in [False, True]:
    data, glyphs = specimen(builtin)
    position = "builtin" if builtin else "declared"
    with fitz.open(stream=bytes(data), filetype="pdf") as pdf:
        page = pdf[0]
        assert page.get_text().strip() == text, page.get_text()
        pixel = page.get_pixmap(colorspace=fitz.csRGB)
        ink = [sum(pixel.pixel(x, y) == (0, 0, 0) for y in range(42, 81) for x in range(left, left + 15)) for left in [20, 36, 52, 68]]
        assert all(count > 8 for count in ink), ink
    (output / f"raw-simple-{position}.pdf").write_bytes(data)
    (output / f"raw-simple-{position}-glyphs.bin").write_bytes(bytes(glyphs))
    print(f"[DEBUG] MuPDF matched {text!r} with {position} encoding and ink {ink} in {len(data)} bytes")
