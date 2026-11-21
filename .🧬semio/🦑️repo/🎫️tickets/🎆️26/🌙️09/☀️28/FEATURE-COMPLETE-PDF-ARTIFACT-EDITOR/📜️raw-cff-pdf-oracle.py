"""🌐 Builds a raw CID-keyed CFF PDF and verifies it with MuPDF."""

import json
from pathlib import Path

import fitz
from fontTools import subset
from fontTools.misc.transform import Transform
from fontTools.pens.recordingPen import RecordingPen
from fontTools.ttLib import TTFont

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
case = json.loads((ticket / "🧫️cff-outlines.json").read_text())["cidFont"]
text = "".join(case["characters"][:4])
font = TTFont(case["path"])
subsetter = subset.Subsetter()
subsetter.populate(text=text)
subsetter.subset(font)
chars = font.getBestCmap()
cids = [int(chars[ord(ch)].removeprefix("cid")) for ch in text]
gids = [font.getGlyphID(chars[ord(ch)]) for ch in text]
source = b"".join(cid.to_bytes(2, "big") for cid in cids)
encoded = source.hex().encode()
program = font.getTableData("CFF ")
top = font["CFF "].cff.topDictIndex[0]
registry, ordering, supplement = top.ROS
system = f"<< /Registry ({registry}) /Ordering ({ordering}) /Supplement {supplement} >>".encode()
assert cids != list(range(1, 5))

def pdf_stream(body, extra=b""):
    return b"<< /Length " + str(len(body)).encode() + b" " + extra + b">>\nstream\n" + body + b"\nendstream"

mapping = b"\n".join(f"<{cid:04X}> <{ch.encode('utf-16-be').hex()}>".encode() for cid, ch in zip(cids, text))
cmap = (b"/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n"
        b"/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n"
        b"/CMapName /CustomToUnicode def\n/CMapType 2 def\n"
        b"1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n4 beginbfchar\n"
        + mapping + b"\nendbfchar\nendcmap\nCMapName currentdict /CMap defineresource pop\nend end")
rows = [
    b"<< /Type /Catalog /Pages 2 0 R >>",
    b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
    b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 170 110] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>",
    pdf_stream(b"BT /F1 32 Tf 1 0 0 1 20 40 Tm <" + encoded + b"> Tj ET"),
    b"<< /Type /Font /Subtype /Type0 /BaseFont /NotoSansCJKsc-Regular-Identity-H /Encoding /Identity-H /DescendantFonts [8 0 R] /ToUnicode 9 0 R >>",
    b"<< /Type /FontDescriptor /FontName /NotoSansCJKsc-Regular /Flags 4 /FontBBox [-1002 -1048 2928 1808] /ItalicAngle 0 /Ascent 1160 /Descent -288 /CapHeight 733 /StemV 90 /FontFile3 7 0 R >>",
    pdf_stream(program, b"/Subtype /CIDFontType0C "),
    b"<< /Type /Font /Subtype /CIDFontType0 /BaseFont /NotoSansCJKsc-Regular /CIDSystemInfo " + system + b" /FontDescriptor 6 0 R /DW 1000 >>",
    pdf_stream(cmap),
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
with fitz.open(stream=bytes(data), filetype="pdf") as doc:
    page = doc[0]
    assert page.get_text().strip() == text
    positions = [20, 52, 84, 116]
    result = page.get_text("rawdict")
    extracted = [ch for block in result["blocks"] for line in block["lines"] for span in line["spans"] for ch in span["chars"]]
    assert [ch["origin"] for ch in extracted] == [(x, 70) for x in positions]
    rgb = page.get_pixmap(colorspace=fitz.csRGB)
    ink = [sum(rgb.pixel(x, y) == (0, 0, 0) for y in range(36, 80) for x in range(left, left + 30)) for left in positions]
    assert min(ink) > 30, ink
output.mkdir(exist_ok=True)
(output / "raw-cff.pdf").write_bytes(data)
(output / "raw-cff-cids.bin").write_bytes(source)
(output / "raw-cff-gids.bin").write_bytes(b"".join(gid.to_bytes(2, "big") for gid in gids))
lines = []
for ch, gid in zip(text, gids):
    pen = RecordingPen()
    glyph = top.CharStrings[chars[ord(ch)]]
    glyph.draw(pen)
    top_matrix = top.rawDict.get("FontMatrix")
    fd_matrix = top.FDArray[top.FDSelect[gid]].rawDict.get("FontMatrix")
    transform = Transform(*(top_matrix or fd_matrix or (0.001, 0, 0, 0.001, 0, 0)))
    if top_matrix and fd_matrix:
        transform = Transform(*top_matrix).transform(Transform(*fd_matrix))
    for name, points in pen.value:
        coords = [value * 1000 for point in points for value in transform.transformPoint(point)]
        lines.append(" ".join([str(gid), name, *(f"{v:.13g}" for v in coords)]))
(output / "raw-cff-outline.txt").write_text("\n".join(lines) + "\n")
print(f"[DEBUG] MuPDF matched {text!r}, sparse CIDs {cids} at gids {gids}, and per-glyph ink {ink} in {len(data)} bytes")
