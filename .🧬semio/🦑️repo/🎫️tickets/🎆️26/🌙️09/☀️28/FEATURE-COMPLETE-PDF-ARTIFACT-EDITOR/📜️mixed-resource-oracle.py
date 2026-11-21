"""🖼️ Generates and verifies PDF pages with both text and sampled images."""

import json
from pathlib import Path
import runpy
import sys

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
images = runpy.run_path(str(ticket / "🎞️fixture.py"))
corpus = json.loads((ticket / "🎞️fixture.json").read_text())
case = json.loads((ticket / "🧫️mixed-resources.json").read_text())

def specimen():
    pdf = images["make_pdf"](corpus)
    footer = pdf.rindex(b"startxref")
    xref = int(pdf[footer:].splitlines()[1])
    rows = pdf[xref:].splitlines()
    count = int(rows[1].split()[1])
    positions = [int(row.split()[0]) for row in rows[3:count + 2]] + [xref]
    objects = [bytes(pdf[start:end]).split(b"\n", 1)[1].rsplit(b"\nendobj", 1)[0]
               for start, end in zip(positions, positions[1:])]
    font_owner = len(objects) + 1
    objects.append(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>")
    for owner in case["resourceOwners"][:3]:
        declaration = f"/Font << /F1 {font_owner} 0 R >> /XObject".encode()
        objects[owner - 1] = objects[owner - 1].replace(b"/XObject", declaration, 1)
    for owner, text in zip(case["contentOwners"], case["lines"]):
        body = objects[owner - 1].split(b"\nstream\n")[1].rsplit(b"\nendstream", 1)[0]
        body += f"BT /F1 12 Tf 20 102 Td ({text}) Tj ET\n".encode()
        objects[owner - 1] = images["pdf_stream"](body)
    result = bytearray(b"%PDF-1.7\n")
    offsets = [b"0000000000 65535 f \n"]
    for owner, value in enumerate(objects, 1):
        offsets.append(f"{len(result):010d} 00000 n \n".encode())
        result.extend(f"{owner} 0 obj\n".encode() + value + b"\nendobj\n")
    xref = len(result)
    result.extend(f"xref\n0 {len(offsets)}\n".encode() + b"".join(offsets))
    result.extend(f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode())
    return result

def verify(path):
    import fitz
    assert images["verify"](corpus, path) == len(case["lines"])
    with fitz.open(path) as document:
        for page, expected in zip(document, case["lines"]):
            assert page.get_text().splitlines() == [expected]
            [font] = page.get_fonts()
            assert font[3:6] == ("Helvetica", "F1", "WinAnsiEncoding"), font
            spans = [span for block in page.get_text("dict", flags=fitz.TEXTFLAGS_TEXT)["blocks"] for line in block.get("lines", []) for span in line["spans"]]
            [span] = spans
            assert span["origin"] == tuple(case["expectedMuPdfBaseline"])
    print(f"[DEBUG] MuPDF verified {len(case['lines'])} mixed image and text pages in {path.name}")

if len(sys.argv) > 1:
    destination = Path(sys.argv[1])
else:
    destination = output / "mixed.pdf"
    output.mkdir(exist_ok=True)
    destination.write_bytes(specimen())
    assert destination.stat().st_size > corpus["minimumByteLength"]
verify(destination)
