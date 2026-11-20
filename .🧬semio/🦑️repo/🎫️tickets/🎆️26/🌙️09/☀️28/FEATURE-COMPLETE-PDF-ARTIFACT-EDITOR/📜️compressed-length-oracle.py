"""🔎️ Generate and independently verify the exact compressed PDF stream-length fixture."""
from pathlib import Path
import sys
import zlib

import fitz

CONTENT = b"BT /F1 12 Tf 10 70 Td (prefix\nendstream\nsuffix) Tj ET"

def fixture():
    data = bytearray(b"%PDF-1.7\n%\x80\x81\x82\x83\n")
    offsets = {}

    def put(number, value, generation=0):
        offsets[number] = len(data)
        data.extend(f"{number} {generation} obj\n".encode() + value + b"\nendobj\n")

    def stream(fields, body):
        return b"<< " + fields + b" >>\nstream\n" + body + b"\nendstream"

    put(1, b"<< /Type /Catalog /Pages 2 0 R >>")
    put(2, b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>")
    put(3, b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>")
    put(4, stream(b"/Length 7 0 R", CONTENT))
    put(5, b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>")

    first = b"6 0 7 3 " + b" " * 256
    packed = zlib.compress(first + b"42 +%d\n" % len(CONTENT))
    put(8, stream(b"/Type /ObjStm /N 2 /First %d /Length 9 2 R /Filter /FlateDecode" % len(first), packed))
    put(9, str(len(packed)).encode(), generation=2)
    put(10, stream(b"/Length 7 0 R", CONTENT))

    xref_offset = len(data)
    offsets[11] = xref_offset
    rows = bytearray()
    for number in range(12):
        if number == 0:
            kind, start, index = 0, 0, 65535
        elif number in (6, 7):
            kind, start, index = 2, 8, number - 6
        else:
            kind, start, index = 1, offsets[number], 2 if number == 9 else 0
        rows.extend(bytes([kind]) + start.to_bytes(4, "big") + index.to_bytes(2, "big"))
    put(11, stream(b"/Type /XRef /Size 12 /Root 1 0 R /W [1 4 2] /Length %d" % len(rows), rows))
    data.extend(f"startxref\n{xref_offset}\n%%EOF\n".encode())
    return bytes(data)

def verify(data):
    with fitz.open(stream=data, filetype="pdf") as pdf:
        assert not pdf.is_repaired
        assert pdf.xref_object(6) == "42"
        assert pdf.xref_object(7) == str(len(CONTENT))
        for number in (4, 10):
            assert pdf.xref_stream(number) == CONTENT
        assert pdf[0].get_text().strip() == "prefix\nendstream\nsuffix"

if __name__ == "__main__":
    path = Path(sys.argv[1])
    data = fixture() if "--write" in sys.argv else path.read_bytes()
    verify(data)
    if "--write" in sys.argv:
        path.write_bytes(data)
