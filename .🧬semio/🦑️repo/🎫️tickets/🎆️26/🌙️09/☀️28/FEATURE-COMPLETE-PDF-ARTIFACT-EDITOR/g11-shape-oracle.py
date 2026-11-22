"""🧭 G11 per-shape oracle: writes one small PDF per deferred controlled-import shape and the typed projection an independent reader (pikepdf, else PyMuPDF) reports, so the governed lifter can be held to it.

Usage: python3 g11-shape-oracle.py [--write DIR]   (prints the projection; --write also stores the PDFs and `projection.json`)
Shapes: resources (fonts and images on annotated pages), text-program (text with a non-admitted font), roles (content array, indirect Length, inherited resources), reference-required (cross-reference stream plus object stream).
"""
import json
import sys
import zlib
from pathlib import Path


def build(objects, trailer_extra=""):
    LAST["objects"] = objects
    out = bytearray(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n")
    offsets = {}
    for number, body in sorted(objects.items()):
        offsets[number] = len(out)
        out += f"{number} 0 obj\n".encode() + (body if isinstance(body, bytes) else body.encode()) + b"\nendobj\n"
    size = max(objects) + 1
    start = len(out)
    out += f"xref\n0 {size}\n0000000000 65535 f \n".encode()
    for number in range(1, size):
        out += (f"{offsets[number]:010d} 00000 n \n" if number in offsets else "0000000000 65535 f \n").encode()
    out += f"trailer\n<< /Size {size} /Root 1 0 R {trailer_extra}>>\nstartxref\n{start}\n%%EOF\n".encode()
    return bytes(out)


LAST = {}


def stream(data, extra=""):
    return f"<< /Length {len(data)} {extra}>>\nstream\n".encode() + data + b"\nendstream"


def shape_resources():
    content = b"BT /F1 12 Tf 20 100 Td (Resource page) Tj ET q 40 0 0 40 20 20 cm /Im1 Do Q"
    image = stream(b"ABCDEFGHIJKL", "/Type /XObject /Subtype /Image /Width 2 /Height 2 /ColorSpace /DeviceRGB /BitsPerComponent 8 ")
    return build({
        1: "<< /Type /Catalog /Pages 2 0 R >>",
        2: "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        3: "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> /XObject << /Im1 6 0 R >> >> /Annots [7 0 R] >>",
        4: stream(content),
        5: "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
        6: image,
        7: "<< /Type /Annot /Subtype /Text /Rect [10 10 30 30] /Contents (Note) /C [1 1 0] >>",
    })


def shape_text_program():
    content = b"BT /F2 14 Tf 10 50 Td [(Kern) -120 (ing)] TJ T* (next) ' ET"
    return build({
        1: "<< /Type /Catalog /Pages 2 0 R >>",
        2: "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        3: "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Contents 4 0 R /Resources << /Font << /F2 5 0 R >> >> >>",
        4: stream(content),
        5: "<< /Type /Font /Subtype /TrueType /BaseFont /Courier-Bold /FirstChar 32 /LastChar 33 /Widths [600 600] /Encoding /WinAnsiEncoding >>",
    })


def shape_roles():
    first, second = b"q 1 0 0 rg 0 0 50 50 re f Q ", b"q 0 0 1 rg 60 0 50 50 re f Q"
    return build({
        1: "<< /Type /Catalog /Pages 2 0 R >>",
        2: "<< /Type /Pages /Kids [3 0 R] /Count 1 /MediaBox [0 0 150 60] /Resources << >> /Rotate 90 >>",
        3: "<< /Type /Page /Parent 2 0 R /Contents [4 0 R 5 0 R] >>",
        4: f"<< /Length 6 0 R >>\nstream\n".encode() + first + b"\nendstream",
        5: stream(second),
        6: str(len(first)),
    })


def shape_reference_required():
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 120 80] /Contents 4 0 R >>",
    ]
    head, body = b"", b""
    for index, item in enumerate(objects, 1):
        head += f"{index} {len(body)} ".encode()
        body += item + b"\n"
    packed = zlib.compress(head + body)
    content = zlib.compress(b"0 0 1 rg 10 10 60 30 re f")
    out = bytearray(b"%PDF-1.7\n")
    off_objstm = len(out)
    out += f"5 0 obj\n<< /Type /ObjStm /N 3 /First {len(head)} /Filter /FlateDecode /Length {len(packed)} >>\nstream\n".encode() + packed + b"\nendstream\nendobj\n"
    off_content = len(out)
    out += f"4 0 obj\n<< /Filter /FlateDecode /Length {len(content)} >>\nstream\n".encode() + content + b"\nendstream\nendobj\n"
    off_xref = len(out)
    rows = [(0, 0, 65535), (2, 5, 0), (2, 5, 1), (2, 5, 2), (1, off_content, 0), (1, off_objstm, 0), (1, off_xref, 0)]
    table = b"".join(bytes([kind]) + field.to_bytes(4, "big") + gen.to_bytes(2, "big") for kind, field, gen in rows)
    xref = zlib.compress(table)
    out += f"6 0 obj\n<< /Type /XRef /Size 7 /W [1 4 2] /Root 1 0 R /Filter /FlateDecode /Length {len(xref)} >>\nstream\n".encode() + xref + b"\nendstream\nendobj\n"
    out += f"startxref\n{off_xref}\n%%EOF\n".encode()
    return bytes(out)


SHAPES = {"resources": shape_resources, "text-program": shape_text_program, "roles": shape_roles, "reference-required": shape_reference_required}


def project(data: bytes):
    try:
        import pikepdf
    except ImportError:
        pikepdf = None
    if pikepdf:
        import io
        pdf = pikepdf.open(io.BytesIO(data))
        pages = []
        for page in pdf.pages:
            ops = [str(operator) for _, operator in pikepdf.parse_content_stream(page)]
            resources = page.get("/Resources", {})
            pages.append({
                "mediaBox": [float(v) for v in page.MediaBox],
                "rotate": int(page.get("/Rotate", 0)),
                "operators": ops,
                "fonts": sorted(str(k) for k in resources.get("/Font", {}).keys()),
                "xobjects": sorted(str(k) for k in resources.get("/XObject", {}).keys()),
                "annotations": [str(a.Subtype) for a in page.get("/Annots", [])],
            })
        return {"reader": "pikepdf", "pages": pages}
    try:
        import fitz
    except ImportError:
        return {"reader": None, "pages": []}
    document = fitz.open(stream=data, filetype="pdf")
    return {"reader": "pymupdf", "pages": [{"mediaBox": list(page.mediabox), "rotate": page.rotation, "text": page.get_text().strip(), "annotations": [a.type[1] for a in page.annots() or []], "images": len(page.get_images())} for page in document]}


def rows(make):
    make()
    return [[number, 0, (body if isinstance(body, str) else body.decode("latin-1"))] for number, body in sorted(LAST["objects"].items())]


if __name__ == "__main__":
    if "--fixture" in sys.argv:
        shapes = {name: {"objects": rows(make), "projection": project(make())} for name, make in SHAPES.items() if name != "reference-required"}
        Path(sys.argv[sys.argv.index("--fixture") + 1]).write_text(json.dumps(shapes, indent=2) + "\n")
        sys.exit(0)
    projection = {name: project(make()) for name, make in SHAPES.items()}
    if "--write" in sys.argv:
        target = Path(sys.argv[sys.argv.index("--write") + 1])
        target.mkdir(parents=True, exist_ok=True)
        for name, make in SHAPES.items():
            (target / f"{name}.pdf").write_bytes(make())
        (target / "projection.json").write_text(json.dumps(projection, indent=2) + "\n")
    print(json.dumps(projection, indent=2))
    if all(item["reader"] is None for item in projection.values()):
        print("no independent reader (pikepdf or PyMuPDF) is installed; only the PDFs were generated", file=sys.stderr)
