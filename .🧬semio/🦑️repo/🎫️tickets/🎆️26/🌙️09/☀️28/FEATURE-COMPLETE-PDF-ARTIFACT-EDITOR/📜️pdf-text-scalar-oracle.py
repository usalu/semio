"""🔤 Verify authored PDF text string fixtures with MuPDF."""
from pathlib import Path
import sys

import fitz

def document(text_hex: str) -> bytes:
    rows = (
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 100 100] >>",
        b"<< /Title <" + text_hex.encode("ascii") + b"> >>",
    )
    payload = bytearray(b"%PDF-1.7\n")
    positions = []
    for number, row in enumerate(rows, 1):
        positions.append(len(payload))
        payload.extend(f"{number} 0 obj\n".encode("ascii") + row + b"\nendobj\n")
    xref = len(payload)
    payload.extend(b"xref\n0 5\n0000000000 65535 f \n")
    for offset in positions:
        payload.extend(f"{offset:010} 00000 n \n".encode("ascii"))
    payload.extend(f"trailer\n<< /Size 5 /Root 1 0 R /Info 4 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode("ascii"))
    return bytes(payload)

for line in Path(sys.argv[1]).read_text(encoding="utf-8").splitlines():
    if line.startswith("#"):
        continue
    authored, expected = line.split()
    with fitz.open(stream=document(authored), filetype="pdf") as pdf:
        assert not pdf.is_repaired
        actual = pdf.metadata["title"].encode("utf-8").hex()
        assert actual == expected, (authored, actual, expected)
        print(f"{authored} → {actual}")
