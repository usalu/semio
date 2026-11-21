"""🖼️ Create and independently verify a visible sampled-and-DCT PDF graph."""
import hashlib
import io
import json
from pathlib import Path
import sys

import fitz
from PIL import Image

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "visible-save"
CASE = json.loads((TICKET / "🔣️pdf-save-visible-graph.json").read_text(encoding="utf8"))
JPEG = TICKET / "🗑️generated" / "paged-jpeg-assets" / "large.dct-placement.jpg"


def stream(body, extra=""):
    """📦 Frame a PDF stream using its actual encoded byte count."""
    return f"<< /Length {len(body)} {extra} >>\nstream\n".encode() + body + b"\nendstream"


def sampled():
    """🎨 Author independently reproducible straight RGB samples."""
    width, height = CASE["sampled"]["dimensions"]
    mult = CASE["sampled"]["multipliers"]
    return bytes(((index * mult[channel] + channel * 53) & 255)
                 for index in range(width * height) for channel in range(3))


def fixture():
    """📄 Frame two visible pages with a stroked path and one image each."""
    photo = JPEG.read_bytes()
    assert hashlib.sha256(photo).hexdigest() == CASE["coded"]["encodedSha256"]
    with Image.open(io.BytesIO(photo)) as opened:
        assert list(opened.size) == CASE["coded"]["dimensions"] and opened.mode == "RGB"
    rows = [b"<< /Type /Catalog /Pages 2 0 R >>",
            b"<< /Type /Pages /Kids [3 0 R 6 0 R] /Count 2 >>"]
    width, height = CASE["pageSize"]
    path = CASE["path"]
    for profile, body in (("sampled", sampled()), ("coded", photo)):
        page = len(rows) + 1
        name = CASE[profile]["name"]
        rows.append((f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}]"
                     f" /Resources << /XObject << /{name} {page + 1} 0 R >> >>"
                     f" /Contents {page + 2} 0 R >>").encode())
        w, h = CASE[profile]["dimensions"]
        extra = "/Filter /DCTDecode /DecodeParms << /ColorTransform 1 >>" if profile == "coded" else ""
        rows.append(stream(body, f"/Type /XObject /Subtype /Image /ColorSpace /DeviceRGB /BitsPerComponent 8 /Width {w} /Height {h} {extra}"))
        color = " ".join(str(value) for value in path["color"])
        start = " ".join(str(value) for value in path["start"])
        end = " ".join(str(value) for value in path["end"])
        rows.append(stream((f"{color} RG {path['width']} w {start} m {end} l S\n"
                            f"q 96 0 0 96 28 96 cm /{name} Do Q\n").encode()))
    output = bytearray(b"%PDF-1.7\n")
    offsets = []
    for number, body in enumerate(rows, 1):
        offsets.append(len(output))
        output.extend(f"{number} 0 obj\n".encode() + body + b"\nendobj\n")
    xref = len(output)
    size = len(rows) + 1
    output.extend(f"xref\n0 {size}\n0000000000 65535 f \n".encode())
    output.extend("".join(f"{value:010} 00000 n \n" for value in offsets).encode())
    output.extend(f"trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode())
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "in.pdf").write_bytes(output)


def observe(path):
    """🔬 Read raw and DCT bytes, path stroke and exact visible raster with MuPDF."""
    with fitz.open(path) as book:
        assert not book.is_repaired and len(book) == 2
        rasters = []
        for key, page in zip(("sampled", "coded"), book, strict=True):
            (info,) = page.get_images(full=True)
            assert list(info[2:4]) == CASE[key]["dimensions"] and info[7] == CASE[key]["name"]
            assert list(page.rect)[2:] == CASE["pageSize"]
            (line,) = page.get_drawings()
            assert abs(line["width"] - CASE["path"]["width"]) < 0.001
            if key == "sampled":
                assert book.xref_stream(info[0]) == sampled()
            else:
                assert hashlib.sha256(book.xref_stream_raw(info[0])).hexdigest() == CASE[key]["encodedSha256"]
            rasters.append(page.get_pixmap().samples)
        return rasters


if __name__ == "__main__":
    fixture()
    original = observe(OUT / "in.pdf")
    if "--compare" in sys.argv:
        assert observe(OUT / "out.pdf") == original
    print("[DEBUG] MuPDF and Pillow verified the sampled image, DCT bytes, stroke, and visible pages")
