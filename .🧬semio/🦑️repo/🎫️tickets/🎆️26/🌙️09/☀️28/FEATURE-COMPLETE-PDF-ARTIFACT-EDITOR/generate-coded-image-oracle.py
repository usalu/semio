"""Generate PDF DCT preview decisions from independent PyMuPDF rendering."""

import base64
import io
import json
import pathlib
import sys

import pymupdf
from PIL import Image


def stream(attributes, data):
    return b"<< " + attributes + f" /Length {len(data)} >>\nstream\n".encode() + data + b"\nendstream"


def pdf(objects):
    output = b"%PDF-1.7\n"
    offsets = []
    for index, body in enumerate(objects, start=1):
        offsets.append(len(output))
        output += f"{index} 0 obj\n".encode() + body + b"\nendobj\n"
    xref = len(output)
    size = len(objects) + 1
    output += f"xref\n0 {size}\n0000000000 65535 f \n".encode()
    output += b"".join(f"{offset:010} 00000 n \n".encode() for offset in offsets)
    return output + f"trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()


def jpeg(mode, left, right, **options):
    image = Image.new(mode, (16, 8), left)
    image.paste(right, (8, 0, 16, 8))
    buffer = io.BytesIO()
    image.save(buffer, format="JPEG", quality=100, subsampling=0, **options)
    return buffer.getvalue()


def document(content, resources=b"<< >>", *objects):
    return pdf([
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 16 8] /Contents 4 0 R /Resources " + resources + b" >>",
        stream(b"", content),
        *objects,
    ])


def xobject(content, compressed, options=b""):
    return document(
        content,
        b"<< /XObject << /Photo 5 0 R >> >>",
        stream(b"/Type /XObject /Subtype /Image /Width 16 /Height 8 /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode " + options, compressed),
    )


def expected(name, artifact, tolerance):
    with pymupdf.open(stream=artifact, filetype="pdf") as doc:
        image = doc[0].get_pixmap(alpha=True)
        samples = image.samples
        sample_points = [(3, 4), (12, 4)]
        return {
            "name": name,
            "pdfBase64": base64.b64encode(artifact).decode(),
            "dimensions": [image.width, image.height],
            "samples": [
                {"at": [x, y], "rgba": list(samples[(y * image.width + x) * 4:][:4])}
                for x, y in sample_points
            ],
            "tolerance": tolerance,
        }


rgb = jpeg("RGB", (210, 42, 25), (17, 116, 213))
adobe_rgb = jpeg("RGB", (210, 42, 25), (17, 116, 213), keep_rgb=True)
gray = jpeg("L", 34, 211)
cases = [
    expected("JFIF XObject with mirrored placement", xobject(b"q -16 0 0 8 16 0 cm /Photo Do Q", rgb), 2),
    expected("Adobe RGB marker overrides the PDF color transform", xobject(b"q 16 0 0 8 0 0 cm /Photo Do Q", adobe_rgb, b"/DecodeParms << /ColorTransform 1 >>"), 0),
    expected("wrapped inline grayscale DCT with inverted Decode", document(
        b"q 16 0 0 8 0 0 cm BI /W 16 /H 8 /BPC 8 /CS /G /D [1 0] /F [/AHx /DCT] ID\n"
        + gray.hex().encode() + b">\nEI Q",
    ), 0),
]
pathlib.Path(sys.argv[1]).write_text(json.dumps({"oracle": f"PyMuPDF {pymupdf.VersionBind}", "cases": cases}, indent=2) + "\n")
