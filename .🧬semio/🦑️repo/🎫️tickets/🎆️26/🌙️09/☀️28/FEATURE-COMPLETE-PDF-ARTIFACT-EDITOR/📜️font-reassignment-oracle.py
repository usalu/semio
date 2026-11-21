"""🔠 Authors and verifies a PDF font change that preserves the shown text."""

import json
import sys
from pathlib import Path

import fitz

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
case = {
    "schema": "semio.stdio.pdf.font-reassignment.v1",
    "sourceCode": 233,
    "sourceAlternateCode": 70,
    "targetCode": 142,
    "sourceFont": "Helvetica",
    "targetFont": "Times-Roman",
    "firstLine": "éé",
    "secondLine": "Tail",
    "kerning": -50,
}

def pdf_stream(body):
    return f"<< /Length {len(body)} >>\nstream\n".encode() + body + b"\nendstream"

def pdf(font, code, restore, program=None):
    source = f"BT /{font} 20 Tf 10 50 Td 24 TL [<{code:02X}> {case['kerning']} <{code:02X}>] TJ {restore} T* (Tail) Tj ET".encode()
    rows = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 180 90] /Resources << /Font << /Fsrc 5 0 R /Fdst 6 0 R >> >> /Contents 4 0 R >>",
        pdf_stream(source if program is None else program),
        f"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding << /BaseEncoding /WinAnsiEncoding /Differences [{case['sourceAlternateCode']} /eacute] >> >>".encode(),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Times-Roman /Encoding /MacRomanEncoding >>",
    ]
    data = bytearray(b"%PDF-1.7\n")
    offsets = [b"0000000000 65535 f \n"]
    for owner, row in enumerate(rows, 1):
        offsets.append(f"{len(data):010d} 00000 n \n".encode())
        data.extend(f"{owner} 0 obj\n".encode() + row + b"\nendobj\n")
    xref = len(data)
    data.extend(f"xref\n0 {len(offsets)}\n".encode() + b"".join(offsets)
                + f"trailer\n<< /Root 1 0 R /Size {len(offsets)} >>\nstartxref\n{xref}\n%%EOF\n".encode())
    return data

def verify(data, before=None):
    with fitz.open(stream=data, filetype="pdf") as document:
        page = document[0]
        assert page.get_text().splitlines() == [case["firstLine"], case["secondLine"]], page.get_text()
        spans = [span for block in page.get_text("dict")["blocks"] for line in block["lines"] for span in line["spans"]]
        assert len(spans) == 2
        assert spans[0]["font"] == (case["sourceFont"] if before is None else case["targetFont"]), spans
        assert spans[1]["font"] == case["sourceFont"]
        if before:
            assert spans[0]["origin"] == before[0]["origin"]
            assert spans[1] == before[1]
    return spans

def generate():
    output.mkdir(exist_ok=True)
    before = pdf("Fsrc", case["sourceCode"], "")
    after = pdf("Fdst", case["targetCode"], "/Fsrc 20 Tf")
    original = verify(before)
    verify(after, original)
    native = output / "font-reassignment-native.stream"
    if native.exists():
        rendered = pdf("Fdst", case["targetCode"], "/Fsrc 20 Tf", native.read_bytes())
        verify(rendered, original)
        (output / "font-reassignment-native.pdf").write_bytes(rendered)
    (output / "font-reassignment-before.pdf").write_bytes(before)
    (output / "font-reassignment-after.pdf").write_bytes(after)
    (output / "font-reassignment.json").write_text(json.dumps(case, ensure_ascii=False, indent=2) + "\n")
    print("[DEBUG] MuPDF confirmed identical text, a changed first-line face, and an unchanged second line")


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "--verify":
        verify(Path(sys.argv[2]).read_bytes(), verify(pdf("Fsrc", case["sourceCode"], "")))
        print("[DEBUG] MuPDF verified the downloaded PDF font change and preserved the neighboring line")
    else:
        generate()
