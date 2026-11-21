"""🖼️ Author large DCT-only PDF pages and obtain independent MuPDF evidence."""

import hashlib
import io
import json
from pathlib import Path
import random
import sys

import pymupdf
from PIL import Image

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "dct-assets"
WIDTH = HEIGHT = 176


def stream(attributes, data):
    return b"<< " + attributes + f" /Length {len(data)} >>\nstream\n".encode() + data + b"\nendstream"


def document(objects):
    output = b"%PDF-1.7\n"
    offsets = []
    for index, body in enumerate(objects, 1):
        offsets.append(len(output))
        output += f"{index} 0 obj\n".encode() + body + b"\nendobj\n"
    xref = len(output)
    total = len(objects) + 1
    output += f"xref\n0 {total}\n0000000000 65535 f \n".encode()
    output += b"".join(f"{offset:010} 00000 n \n".encode() for offset in offsets)
    return output + f"trailer\n<< /Root 1 0 R /Size {total} >>\nstartxref\n{xref}\n%%EOF\n".encode()


def jpeg(mode, seed):
    source = random.Random(seed).randbytes(WIDTH * HEIGHT * (3 if mode == "RGB" else 1))
    image = Image.frombytes(mode, (WIDTH, HEIGHT), source)
    output = io.BytesIO()
    image.save(output, format="JPEG", quality=83, subsampling=2)
    return output.getvalue()


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    native = [jpeg("RGB", 21), jpeg("L", 36)]
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R 6 0 R] /Count 2 >>",
    ]
    for index, (data, space, label, parameters) in enumerate(zip(
        native, ["/DeviceRGB", "/DeviceGray"], ["Photo", "Gray"], ["/DecodeParms << /ColorTransform 1 >>", ""],
    )):
        page = 3 + index * 3
        objects.extend([
            f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {WIDTH} {HEIGHT}] /Resources << /XObject << /{label} {page + 2} 0 R >> >> /Contents {page + 1} 0 R >>".encode(),
            stream(b"", f"q {WIDTH} 0 0 {HEIGHT} 0 0 cm /{label} Do Q\n".encode()),
            stream(f"/Type /XObject /Subtype /Image /Width {WIDTH} /Height {HEIGHT} /ColorSpace {space} /BitsPerComponent 8 /Filter /DCTDecode {parameters}".encode(), data),
        ])
    data = document(objects)
    assert len(data) > 32_768 and all(len(jpg) < 32_640 for jpg in native)
    (OUT / "large.pdf").write_bytes(data)
    for index, kind in ((4, b"UnknownXX"), (7, b"JPXDecode")):
        replacement = list(objects)
        replacement[index] = replacement[index].replace(b"DCTDecode", kind, 1)
        (OUT / (kind.decode() + ".pdf")).write_bytes(document(replacement))
    evidence = []
    with pymupdf.open(stream=data, filetype="pdf") as pdf:
        assert not pdf.is_repaired and len(pdf) == 2
        for index, page in enumerate(pdf):
            owner, *_ = page.get_images(full=True)[0]
            jpg = pdf.extract_image(owner)
            assert jpg["image"] == native[index]
            assert (jpg["width"], jpg["height"], jpg["ext"]) == (WIDTH, HEIGHT, "jpeg")
            raster = page.get_pixmap(alpha=False)
            (OUT / f"{owner}.jpg").write_bytes(jpg["image"])
            transform = 1 if index == 0 else None
            tail = json.dumps([{"kind": "dct", "colorTransform": transform}], separators=(",", ":")).encode()
            evidence.append({
                "owner": owner, "size": len(native[index]), "width": jpg["width"], "height": jpg["height"],
                "components": jpg["colorspace"], "colorTransform": transform,
                "jpegSha256": hashlib.sha256(jpg["image"]).hexdigest(),
                "identity": hashlib.sha256(b"s.stdio.jpeg" + jpg["image"] + tail).hexdigest(),
                "renderedRgbSha256": hashlib.sha256(raster.samples).hexdigest(),
            })
    contract = {"oracle": "PyMuPDF " + pymupdf.VersionBind, "fileBytes": len(data), "images": evidence}
    (OUT / "expect.json").write_text(json.dumps(contract, indent=2) + "\n")
    print(json.dumps(contract))


def verify_emitted(emitted=None):
    expected = json.loads((OUT / "expect.json").read_text())
    emitted = Path(emitted) if emitted else OUT / "emitted.pdf"
    with pymupdf.open(emitted) as pdf:
        assert not pdf.is_repaired and len(pdf) == 2
        for page, reference in zip(pdf, expected["images"]):
            owner, *_ = page.get_images(full=True)[0]
            assert owner == reference["owner"]
            jpg = pdf.extract_image(owner)
            assert hashlib.sha256(jpg["image"]).hexdigest() == reference["jpegSha256"]
            pixmap = page.get_pixmap(alpha=False)
            assert hashlib.sha256(pixmap.samples).hexdigest() == reference["renderedRgbSha256"]
            assert (jpg["width"], jpg["height"], jpg["colorspace"]) == (reference["width"], reference["height"], reference["components"])
    print(expected["oracle"], "reopened native output with both JPEGs and rendered pages byte-identical")


if __name__ == "__main__":
    if sys.argv[1:2] == ["verify-emitted"]:
        verify_emitted(sys.argv[2] if len(sys.argv) > 2 else None)
    else:
        prepare()
