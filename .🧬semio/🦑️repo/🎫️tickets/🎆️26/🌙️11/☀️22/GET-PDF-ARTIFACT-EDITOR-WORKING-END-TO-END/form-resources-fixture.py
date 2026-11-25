"""🖼️ Writes the form-resources fixture: one page painting a form XObject whose own resources declare an axial shading, a shading pattern and a named ICC-free colour space, as illustration tools export them; pymupdf checks the written file.

usage: uv run --offline --with pymupdf==1.26.7 python -I form-resources-fixture.py <fixture directory>
"""
import pathlib
import sys

import pymupdf


def build():
    objects = {
        1: b"<< /Type /Catalog /Pages 2 0 R >>",
        2: b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        3: b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << /XObject << /Fm1 5 0 R >> /ProcSet [/PDF] >> /Contents 4 0 R >>",
        6: b"<< /FunctionType 2 /Domain [0 1] /C0 [1 0 0] /C1 [0 0 1] /N 1 >>",
        7: b"<< /ShadingType 2 /ColorSpace /DeviceRGB /Coords [0 0 100 0] /Function 6 0 R /Extend [true true] >>",
        8: b"<< /PatternType 2 /Shading 7 0 R /Matrix [1 0 0 1 0 0] >>",
        9: b"<< /Shading << /Sh1 7 0 R >> /Pattern << /P1 8 0 R >> /ColorSpace << /CS1 /DeviceRGB >> >>",
    }
    page = b"q 1 0 0 1 20 20 cm /Fm1 Do Q"
    form = b"q 0 0 100 100 re W n /Sh1 sh Q /CS1 cs 0 1 0 sc 0 110 40 40 re f /Pattern cs /P1 scn 50 110 40 40 re f"
    streams = {
        4: (b"<< /Length %d >>" % len(page), page),
        5: (b"<< /Type /XObject /Subtype /Form /BBox [0 0 160 160] /Resources 9 0 R /Length %d >>" % len(form), form),
    }
    out = bytearray(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n")
    offsets = {}
    for number in range(1, 10):
        offsets[number] = len(out)
        if number in streams:
            head, body = streams[number]
            out += b"%d 0 obj\n%s\nstream\n%s\nendstream\nendobj\n" % (number, head, body)
        else:
            out += b"%d 0 obj\n%s\nendobj\n" % (number, objects[number])
    xref = len(out)
    out += b"xref\n0 10\n0000000000 65535 f \n"
    for number in range(1, 10):
        out += b"%010d 00000 n \n" % offsets[number]
    out += b"trailer\n<< /Size 10 /Root 1 0 R >>\nstartxref\n%d\n%%%%EOF\n" % xref
    return bytes(out)


def main():
    directory = pathlib.Path(sys.argv[1])
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / "form-resources.pdf"
    path.write_bytes(build())
    document = pymupdf.open(path)
    assert document.page_count == 1
    form = document.xref_object(5, compressed=True)
    assert "/Resources 9 0 R" in form, form
    resources = document.xref_object(9, compressed=True)
    assert all(key in resources for key in ("/Shading", "/Pattern", "/ColorSpace")), resources
    drawings = document[0].get_drawings()
    assert drawings, "pymupdf paints the form's pattern and colour-space fills"
    print(f"{path}: 1 page, form resources {resources}, {len(drawings)} drawings")


if __name__ == "__main__":
    main()
