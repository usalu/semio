"""Check the PDF ordered filter fixture against the independent MuPDF decoder."""
import base64
import json
import sys
from pathlib import Path

import fitz

case = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))[0]
wrapped = base64.a85encode(bytes.fromhex(case["hex"])) + b"~>"
outer = wrapped.hex().encode() + b">"
if "chain" not in case:
    print(outer.decode())
    raise SystemExit
assert outer.decode() == case["chain"]

objects = [
    b"<< /Type /Catalog /Pages 2 0 R >>",
    b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
    b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 20 20] >>",
    (
        b"<< /Filter [/ASCIIHexDecode /ASCII85Decode /FlateDecode] "
        b"/DecodeParms [null null << /Predictor 15 /Columns 3 >>]"
        + f" /Length {len(outer)} >>\nstream\n".encode()
        + outer
        + b"\nendstream"
    ),
]
pdf = bytearray(b"%PDF-1.7\n")
offsets = [0]
for num, body in enumerate(objects, 1):
    offsets.append(len(pdf))
    pdf += f"{num} 0 obj\n".encode() + body + b"\nendobj\n"
xref = len(pdf)
pdf += f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode()
for offset in offsets[1:]:
    pdf += f"{offset:010d} 00000 n \n".encode()
pdf += (
    f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n"
).encode()
doc = fitz.open(stream=pdf, filetype="pdf")
assert not doc.is_repaired
assert doc.xref_stream(4) == bytes.fromhex(case["decoded"])
print(f"MuPDF decoded {doc.xref_stream(4).hex()} from all three filters")
