"""Writes the form-images fixture: images typed inside Form XObjects, as pdfTeX, Illustrator and Word export them, and the spec (schema first) the Rust laws and the pymupdf oracle both read.

Page 1 paints two forms and one page-level image. Form `Fm1` types a Flate RGB image and a Flate gray image with a soft mask; form `Fm2` types a JPEG whose authored name `Im0` collides with `Fm1`'s (the reference lifter renames it `Im0_2`) and a nested form `Fn` that re-types `Fm1`'s `Im0` object, an Indexed 1-bit image and a stencil mask. Page 2 paints `Fm1` again (a form shared between pages).

usage: uv run --offline --with pymupdf==1.26.7 python form-images-fixture.py <fixture directory>
"""
import json
import pathlib
import sys
import zlib

import pymupdf


def flate(data: bytes) -> bytes:
    return zlib.compress(data, 9)


def stream(entries: str, body: bytes) -> tuple[bytes, bytes]:
    return (f"<< {entries} /Length {len(body)} >>").encode(), body


def image(width: int, height: int, space: str, body: bytes, extra: str = "", bits: int = 8) -> tuple[bytes, bytes]:
    return stream(f"/Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace {space} /BitsPerComponent {bits} {extra} /Filter /FlateDecode", flate(body))


def jpeg(width: int, height: int) -> tuple[bytes, bytes]:
    pixmap = pymupdf.Pixmap(pymupdf.csRGB, pymupdf.IRect(0, 0, width, height), False)
    pixmap.set_rect(pixmap.irect, (200, 40, 40))
    body = pixmap.tobytes("jpeg")
    return stream(f"/Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode", body)


def form(box: str, resources: str, content: bytes) -> tuple[bytes, bytes]:
    return stream(f"/Type /XObject /Subtype /Form /BBox {box} /Resources << {resources} >>", content)


def build() -> bytes:
    gradient = bytes(range(0, 48, 1))
    objects: dict[int, bytes | tuple[bytes, bytes]] = {
        1: b"<< /Type /Catalog /Pages 2 0 R >>",
        2: b"<< /Type /Pages /Kids [3 0 R 16 0 R] /Count 2 >>",
        3: b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 240 240] /Resources 4 0 R /Contents 5 0 R >>",
        4: b"<< /XObject << /Fm1 6 0 R /Fm2 7 0 R /Pg 10 0 R >> /ProcSet [/PDF /ImageC] >>",
        5: stream("", b"q 1 0 0 1 10 130 cm /Fm1 Do Q q 1 0 0 1 10 10 cm /Fm2 Do Q q 60 0 0 60 170 170 cm /Pg Do Q"),
        6: form("[0 0 100 60]", "/XObject << /Im0 8 0 R /Im1 9 0 R >>", b"q 40 0 0 40 0 0 cm /Im0 Do Q q 40 0 0 40 50 0 cm /Im1 Do Q"),
        7: form("[0 0 100 100]", "/XObject << /Im0 11 0 R /Fn 12 0 R >>", b"q 40 0 0 40 0 0 cm /Im0 Do Q q 1 0 0 1 50 50 cm /Fn Do Q"),
        8: image(4, 4, "/DeviceRGB", gradient),
        9: image(4, 4, "/DeviceGray", bytes(range(16)), "/SMask 13 0 R"),
        10: image(3, 3, "/DeviceRGB", bytes(range(27))),
        11: jpeg(8, 8),
        12: form("[0 0 50 50]", "/XObject << /Im0 8 0 R /Ix 14 0 R /Mk 15 0 R >>", b"q 20 0 0 20 0 0 cm /Im0 Do Q q 20 0 0 20 25 0 cm /Ix Do Q q 20 0 0 20 0 25 cm /Mk Do Q"),
        13: image(4, 4, "/DeviceGray", bytes(range(0, 256, 16))),
        14: image(8, 8, "[/Indexed /DeviceRGB 1 <ff000000ff00>]", bytes([0xAA, 0x55] * 4), bits=1),
        15: stream("/Type /XObject /Subtype /Image /Width 8 /Height 8 /ImageMask true /BitsPerComponent 1", bytes([0xF0] * 8)),
        16: b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 120 120] /Resources << /XObject << /Fm1 6 0 R >> >> /Contents 17 0 R >>",
        17: stream("", b"q 1 0 0 1 10 10 cm /Fm1 Do Q"),
    }
    out = bytearray(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n")
    offsets = {}
    for number in sorted(objects):
        offsets[number] = len(out)
        value = objects[number]
        if isinstance(value, tuple):
            head, body = value
            out += b"%d 0 obj\n%s\nstream\n%s\nendstream\nendobj\n" % (number, head, body)
        else:
            out += b"%d 0 obj\n%s\nendobj\n" % (number, value)
    size = max(objects) + 1
    xref = len(out)
    out += b"xref\n0 %d\n0000000000 65535 f \n" % size
    for number in range(1, size):
        out += b"%010d 00000 n \n" % offsets[number]
    out += b"trailer\n<< /Size %d /Root 1 0 R >>\nstartxref\n%d\n%%%%EOF\n" % (size, xref)
    return bytes(out)


SPEC = {
    "pages": 2,
    "forms": ["Fm1", "Fn", "Fm2"],
    "images": ["Im0", "Im1", "Im1SMask", "Im0_2", "Ix", "Mk", "Pg"],
    "formImages": {"Fm1": ["Im0", "Im1"], "Fm2": ["Im0_2"], "Fn": ["Im0", "Ix", "Mk"]},
    "imageObjects": [8, 9, 10, 11, 13, 14, 15],
    "formObjects": [6, 7, 12],
}


def main():
    directory = pathlib.Path(sys.argv[1])
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / "form-images.pdf"
    path.write_bytes(build())
    (directory / "spec.json").write_text(json.dumps(SPEC, indent=2) + "\n")
    document = pymupdf.open(path)
    assert document.page_count == SPEC["pages"]
    for number in SPEC["imageObjects"]:
        assert "/Subtype/Image" in document.xref_object(number, compressed=True).replace(" ", ""), number
    for number in SPEC["formObjects"]:
        assert "/Subtype/Form" in document.xref_object(number, compressed=True).replace(" ", ""), number
    xrefs = {image[0] for page in document for image in page.get_images(full=True)}
    assert xrefs >= {8, 9, 10, 11, 14}, xrefs
    pixmap = document[0].get_pixmap()
    assert pixmap.width == 240 and pixmap.height == 240
    print(f"{path}: {document.page_count} pages, images seen by pymupdf {sorted(xrefs)}")


if __name__ == "__main__":
    main()
