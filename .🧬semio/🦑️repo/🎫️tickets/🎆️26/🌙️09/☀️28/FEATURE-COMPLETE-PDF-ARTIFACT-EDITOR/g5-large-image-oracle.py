"""🖼️ MuPDF oracle for the large original image law: independent generation, source digest, and open-save-reopen verification of a PDF saved by the product."""
import hashlib
import json
import sys
from pathlib import Path

import pymupdf

LAW = Path(sys.argv[1])
law = json.loads(LAW.read_text(encoding="utf8"))
MASK = (1 << 64) - 1


def fnv1a64(data):
    h = 0xCBF29CE484222325
    for byte in data:
        h = ((h ^ byte) * 0x100000001B3) & MASK
    return f"{h:016x}"


def stream(dictionary, data):
    return b"<< " + dictionary + f" /Length {len(data)} >>\nstream\n".encode() + data + b"\nendstream"


def image_samples():
    half = law["imageSide"] // 2
    a, b, c, d = law["grayscale"]
    return ((bytes([a]) * half + bytes([b]) * half) * half + (bytes([c]) * half + bytes([d]) * half) * half)


def source():
    side, media, image = law["imageSide"], law["mediaSide"], image_samples()
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
        (f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {media} {media}] /Contents 4 0 R /Resources << /XObject << /FixtureImage 5 0 R >> >> >>").encode(),
        stream(b"", f"q {media} 0 0 {media} 0 0 cm /FixtureImage Do Q\n".encode()),
        stream((f"/Type /XObject /Subtype /Image /Width {side} /Height {side} /ColorSpace /DeviceGray /BitsPerComponent 8").encode(), image),
    ]
    out, offsets = bytearray(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n"), []
    for index, value in enumerate(objects, 1):
        offsets.append(len(out))
        out += f"{index} 0 obj\n".encode() + value + b"\nendobj\n"
    xref = len(out)
    out += f"xref\n0 {len(offsets) + 1}\n0000000000 65535 f \n".encode()
    for offset in offsets:
        out += f"{offset:010d} 00000 n \n".encode()
    out += f"trailer\n<< /Size {len(offsets) + 1} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
    return bytes(out)


def verify(path):
    with pymupdf.open(path) as pdf:
        assert not pdf.is_repaired and len(pdf) == law["pageCount"], path
        (image,) = pdf[0].get_images()
        pixels = pymupdf.Pixmap(pdf, image[0])
        assert (pixels.width, pixels.height, pixels.n) == (law["imageSide"], law["imageSide"], 1), path
        assert hashlib.sha256(pixels.samples).hexdigest() == law["imageSha256"], path


data = source()
if "--write" in sys.argv:
    law.update(sourceBytes=len(data), sourceFnv1a64=fnv1a64(data), imageBytes=len(image_samples()), imageSha256=hashlib.sha256(image_samples()).hexdigest())
    LAW.write_text(json.dumps(law, indent=2) + "\n", encoding="utf8")
assert len(data) == law["sourceBytes"] and hashlib.sha256(data).hexdigest() == law["sourceSha256"] and fnv1a64(data) == law["sourceFnv1a64"]
scratch = Path(sys.argv[2])
scratch.mkdir(parents=True, exist_ok=True)
(scratch / "source.pdf").write_bytes(data)
verify(scratch / "source.pdf")
for saved in sys.argv[3:]:
    if saved != "--write":
        verify(saved)
        print("[DEBUG] MuPDF verified the product-saved PDF", saved)
print("[DEBUG] large original image law matches the independent generator and MuPDF")
