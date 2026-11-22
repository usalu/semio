"""📚 Build and verify a natural PDF whose source exceeds one shell command."""

import hashlib
import json
from pathlib import Path
import sys

import pymupdf

law = json.loads(Path(__file__).with_name("🔣️segmented-pdf-source.json").read_text(encoding="utf8"))


def stream(dictionary, data):
    """📦 Frame one PDF stream with its byte length."""
    return b"<< " + dictionary + f" /Length {len(data)} >>\nstream\n".encode() + data + b"\nendstream"


def generate(path):
    """🖨️ Write the page, uncompressed image and exact classic xref offsets."""
    side = law["imageSide"]
    media = law["mediaSide"]
    half = side // 2
    first, second, third, fourth = law["grayscale"]
    image = ((bytes([first]) * half + bytes([second]) * half) * half
             + (bytes([third]) * half + bytes([fourth]) * half) * half)
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
        (f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {media} {media}] "
         "/Contents 4 0 R /Resources << /XObject << /FixtureImage 5 0 R >> >> >>").encode(),
        stream(b"", f"q {media} 0 0 {media} 0 0 cm /FixtureImage Do Q\n".encode()),
        stream((f"/Type /XObject /Subtype /Image /Width {side} /Height {side} "
                "/ColorSpace /DeviceGray /BitsPerComponent 8").encode(), image),
    ]
    with path.open("xb") as output:
        output.write(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n")
        offsets = [0]
        for index, value in enumerate(objects, 1):
            offsets.append(output.tell())
            output.write(f"{index} 0 obj\n".encode())
            output.write(value)
            output.write(b"\nendobj\n")
        xref = output.tell()
        output.write(f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode())
        for offset in offsets[1:]:
            output.write(f"{offset:010d} 00000 n \n".encode())
        output.write(f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode())
    assert xref == law["xrefOffset"] > law["shellCommandBytes"]
    assert offsets[1] == law["catalogOffset"]


def verify_source(path):
    """🪪 Check the neutral source extent, digest, layout and samples."""
    assert path.stat().st_size == law["sourceBytes"]
    with path.open("rb") as source:
        source.seek(law["headerOffset"])
        assert source.readline().rstrip() == ("%PDF-" + law["version"]).encode("ascii")
        source.seek(law["catalogOffset"])
        assert source.readline().rstrip() == b"1 0 obj"
        source.seek(law["xrefOffset"])
        assert source.readline().rstrip() == b"xref"
        source.seek(0)
        assert hashlib.file_digest(source, "sha256").hexdigest() == law["sourceSha256"]
    verify(path, 0)


def verify(path, rotation):
    """🔍 Read the page and sample each quadrant with MuPDF."""
    media = law["mediaSide"]
    image = law["imageSide"]
    first, second, third, fourth = law["grayscale"]
    pixels = (first, second, fourth, third)
    quarters = rotation // 90
    expected = tuple(pixels[(index - quarters) % 4] for index in range(4))
    with pymupdf.open(path) as document:
        assert not document.is_repaired and document.page_count == 1
        page = document[0]
        assert page.rotation == rotation and page.mediabox == pymupdf.Rect(0, 0, media, media)
        assert [(value["width"], value["height"]) for value in page.get_image_info()] == [(image, image)]
        raster = page.get_pixmap(colorspace=pymupdf.csGRAY, alpha=False)
        low, high = media // 4, 3 * media // 4
        corners = ((low, low), (high, low), (high, high), (low, high))
        assert tuple(raster.pixel(x, y)[0] for x, y in corners) == expected


if __name__ == "__main__":
    mode, source = sys.argv[1:3]
    source = Path(source)
    if mode == "generate":
        generate(source)
        verify_source(source)
        print(f"[DEBUG] MuPDF read {source.stat().st_size} bytes and rasterized all four quadrants")
    elif mode == "verify":
        verify_source(source)
        verify(Path(sys.argv[3]), law["rotation"])
        print("[DEBUG] MuPDF verified the saved rotation, page box, image size and pixels")
    else:
        raise ValueError("Select generate or verify")
