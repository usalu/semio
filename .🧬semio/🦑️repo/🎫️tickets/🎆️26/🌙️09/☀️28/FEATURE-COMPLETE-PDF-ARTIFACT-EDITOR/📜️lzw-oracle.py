"""🧪 Build and verify the language-independent LZW PDF fixture."""
from pathlib import Path
import sys


def pack(codes, early_change):
    value = bits = 0
    result = bytearray()
    width, next_code, previous = 9, 258, False
    for code in codes:
        value = (value << width) | code
        bits += width
        while bits >= 8:
            bits -= 8
            result.append((value >> bits) & 255)
        value &= (1 << bits) - 1
        if code == 256:
            width, next_code, previous = 9, 258, False
        elif code != 257:
            if previous and next_code < 4096:
                next_code += 1
                if width < 12 and next_code + early_change == 1 << width:
                    width += 1
            previous = True
    if bits:
        result.append(value << (8 - bits))
    return bytes(result)


def fixtures():
    ramp = bytes(i % 251 for i in range(3839))
    for early_change in (0, 1):
        code = pack([256, *ramp, 256, 65, 258, 259, 257], early_change)
        yield code, ramp + b"AAAAAA", f"/EarlyChange {early_change}"
    png = [0, 10, 30, 50, 1, 20, 20, 20]
    yield pack([256, *png, 257], 1), bytes([10, 30, 50, 20, 40, 60]), "/Predictor 15 /Columns 3"
    tiff = [0, 10, 0, 20, 0, 20]
    yield pack([256, *tiff, 257], 0), bytes([0, 10, 0, 30, 0, 50]), "/EarlyChange 0 /Predictor 2 /Columns 3 /BitsPerComponent 16"


def build(path):
    entries = [
        b"<< /Type /Catalog /Pages 2 0 R /Probe [4 0 R 5 0 R 6 0 R 7 0 R] >>",
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 20 20] >>",
    ]
    for raw, _, params in fixtures():
        entries.append(f"<< /Length {len(raw)} /Filter /LZWDecode /DecodeParms << {params} >> >>\nstream\n".encode("ascii") + raw + b"\nendstream")
    body = bytearray(b"%PDF-1.7\n")
    offsets = []
    for i, obj in enumerate(entries, 1):
        offsets.append(len(body))
        body.extend(f"{i} 0 obj\n".encode() + obj + b"\nendobj\n")
    start = len(body)
    body.extend(f"xref\n0 {len(entries) + 1}\n0000000000 65535 f \n".encode())
    for offset in offsets:
        body.extend(f"{offset:010} 00000 n \n".encode())
    body.extend(f"trailer\n<< /Size {len(entries) + 1} /Root 1 0 R >>\nstartxref\n{start}\n%%EOF\n".encode())
    path.write_bytes(body)


def verify(path):
    import fitz

    with fitz.open(path) as pdf:
        assert not pdf.is_repaired
        for number, (encoded, expected, _) in enumerate(fixtures(), 4):
            assert pdf.xref_stream_raw(number) == encoded
            decoded = pdf.xref_stream(number)
            assert decoded == expected, (number, len(decoded), len(expected))
        print("MuPDF matched both EarlyChange widths, resets, special codes, and PNG/TIFF output")


if __name__ == "__main__":
    path = Path(sys.argv[2])
    if sys.argv[1] == "build":
        build(path)
    else:
        verify(path)
