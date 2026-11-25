"""Writes the page-resources fixture: page-level colour spaces, shadings and patterns, as Word, InDesign, Cairo and Illustrator export them, and the spec (schema first) the Rust laws and the pymupdf oracle both read.

Page 1: direct and indirect colour spaces (ICC based, Indexed, Separation, a device name), two shadings, a shading pattern and a tiling pattern whose own resources type an image, painted by `cs`/`sc`/`scn`/`sh`. Page 2: the same Indexed object under another name, and a direct colour space under the name page 1 already claimed (the reference lifter renames it `CsD_2`).

usage: uv run --offline --with pymupdf==1.26.7 python page-resources-fixture.py <fixture directory>
"""
import json
import pathlib
import struct
import sys
import zlib

import pymupdf


def s15(value: float) -> bytes:
    return struct.pack(">i", round(value * 65536))


def icc_profile() -> bytes:
    def xyz(x: float, y: float, z: float) -> bytes:
        return b"XYZ \0\0\0\0" + s15(x) + s15(y) + s15(z)

    description = b"semio test sRGB\0"
    desc = b"desc\0\0\0\0" + struct.pack(">I", len(description)) + description + struct.pack(">II", 0, 0) + struct.pack(">HB", 0, 0) + bytes(67)
    gamma = b"curv\0\0\0\0" + struct.pack(">IHxx", 1, 0x0233)
    text = b"text\0\0\0\0" + b"semio fixture\0"
    tags = [
        (b"desc", desc), (b"cprt", text), (b"wtpt", xyz(0.9642, 1.0, 0.8249)),
        (b"rXYZ", xyz(0.4361, 0.2225, 0.0139)), (b"gXYZ", xyz(0.3851, 0.7169, 0.0971)), (b"bXYZ", xyz(0.1431, 0.0606, 0.7141)),
        (b"rTRC", gamma), (b"gTRC", gamma), (b"bTRC", gamma),
    ]
    table_end = 128 + 4 + 12 * len(tags)
    table, body, offset = b"", b"", table_end
    for signature, data in tags:
        padded = data + bytes(-len(data) % 4)
        table += signature + struct.pack(">II", offset, len(data))
        body += padded
        offset += len(padded)
    header = bytearray(128)
    struct.pack_into(">I", header, 0, offset)
    header[8:12] = struct.pack(">I", 0x02100000)
    header[12:16], header[16:20], header[20:24] = b"mntr", b"RGB ", b"XYZ "
    header[36:40] = b"acsp"
    header[68:80] = s15(0.9642) + s15(1.0) + s15(0.8249)
    return bytes(header) + struct.pack(">I", len(tags)) + table + body


def stream(entries: str, body: bytes) -> tuple[bytes, bytes]:
    return (f"<< {entries} /Length {len(body)} >>").encode(), body


PAGE_ONE = b"""q
/CsD cs 0.2 0.4 0.6 sc 10 10 50 50 re f
/CsR cs 1 sc 70 10 50 50 re f
/CsS cs 0.7 scn 130 10 50 50 re f
/CsN cs 0.5 sc 190 10 40 50 re f
Q
q 10 70 220 60 re W n /Sh1 sh Q
q 10 140 100 60 re W n /Sh2 sh Q
/Pattern cs /P1 scn 120 140 50 60 re f
/Pattern cs /P2 scn 180 140 50 60 re f
"""
PAGE_TWO = b"q /Cs9 cs 0 sc 10 10 50 50 re f /CsD cs 0.5 0.5 0.5 sc 70 10 50 50 re f Q"


def build() -> bytes:
    pixel = zlib.compress(bytes(range(12)), 9)
    objects: dict[int, bytes | tuple[bytes, bytes]] = {
        1: b"<< /Type /Catalog /Pages 2 0 R >>",
        2: b"<< /Type /Pages /Kids [3 0 R 13 0 R] /Count 2 >>",
        3: b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 240 210] /Resources 4 0 R /Contents 5 0 R >>",
        4: b"<< /ColorSpace << /CsD [/ICCBased 6 0 R] /CsR 7 0 R /CsS 8 0 R /CsN /DeviceGray >> /Shading << /Sh1 10 0 R /Sh2 11 0 R >> /Pattern << /P1 12 0 R /P2 14 0 R >> /ProcSet [/PDF /Text] >>",
        5: stream("", PAGE_ONE),
        6: stream("/N 3 /Alternate /DeviceRGB", icc_profile()),
        7: b"[/Indexed /DeviceRGB 1 <ff000000ff00>]",
        8: b"[/Separation /Spot /DeviceCMYK 9 0 R]",
        9: b"<< /FunctionType 2 /Domain [0 1] /C0 [0 0 0 0] /C1 [0 1 1 0] /N 1 >>",
        10: b"<< /ShadingType 2 /ColorSpace /DeviceRGB /Coords [10 0 230 0] /Function 15 0 R /Extend [true true] >>",
        11: b"<< /ShadingType 3 /ColorSpace /DeviceRGB /Coords [60 170 0 60 170 40] /Function 15 0 R /Extend [false true] >>",
        12: b"<< /PatternType 2 /Shading 10 0 R /Matrix [1 0 0 1 0 0] >>",
        13: b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 130 70] /Resources << /ColorSpace << /Cs9 7 0 R /CsD [/CalRGB << /WhitePoint [0.9505 1 1.089] /Gamma [2.2 2.2 2.2] >>] >> >> /Contents 16 0 R >>",
        14: stream("/Type /Pattern /PatternType 1 /PaintType 1 /TilingType 1 /BBox [0 0 10 10] /XStep 10 /YStep 10 /Resources << /XObject << /Px 17 0 R >> >>", b"q 8 0 0 8 1 1 cm /Px Do Q"),
        15: b"<< /FunctionType 2 /Domain [0 1] /C0 [1 0 0] /C1 [0 0 1] /N 1 >>",
        16: stream("", PAGE_TWO),
        17: stream(f"/Type /XObject /Subtype /Image /Width 2 /Height 2 /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /FlateDecode", pixel),
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
    "declared": [
        {"ColorSpace": ["CsD", "CsN", "CsR", "CsS"], "Shading": ["Sh1", "Sh2"], "Pattern": ["P1", "P2"]},
        {"ColorSpace": ["Cs9", "CsD"], "Shading": [], "Pattern": []},
    ],
    "colorSpaces": ["CsD", "CsR", "CsS", "CsN", "CsD_2"],
    "shadings": ["Sh1", "Sh2"],
    "patterns": ["P1", "P2"],
    "images": ["Px"],
    "drawings": [4, 1],
}


def main():
    directory = pathlib.Path(sys.argv[1])
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / "page-resources.pdf"
    path.write_bytes(build())
    (directory / "spec.json").write_text(json.dumps(SPEC, indent=2) + "\n")
    document = pymupdf.open(path)
    assert document.page_count == SPEC["pages"]
    for page, expected in zip(document, SPEC["drawings"]):
        assert len(page.get_drawings()) >= expected, (page.number, len(page.get_drawings()))
        assert page.get_pixmap().width > 0
    print(f"{path}: {document.page_count} pages, drawings {[len(page.get_drawings()) for page in document]}")


if __name__ == "__main__":
    main()
