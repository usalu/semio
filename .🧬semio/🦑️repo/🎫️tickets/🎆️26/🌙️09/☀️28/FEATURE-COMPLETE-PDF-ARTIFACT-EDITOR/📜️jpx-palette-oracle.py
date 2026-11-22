"""🧪️ Wraps an OpenJPEG gray codestream in a JP2 with pclr and cmap boxes and records MuPDF's rendering."""

import io
import struct
import sys

import fitz
from PIL import Image


def box(kind, body):
    return struct.pack(">I", 8 + len(body)) + kind + body


def main(path):
    width, height = 24, 16
    image = Image.new("L", (width, height))
    image.putdata([(x * 3 + y) % 7 for y in range(height) for x in range(width)])
    buffer = io.BytesIO()
    image.save(buffer, "JPEG2000", irreversible=False, no_jp2=True)
    codestream = buffer.getvalue()
    palette = [(255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0), (0, 255, 255), (255, 0, 255), (10, 20, 30)]
    pclr = struct.pack(">HB", len(palette), 3) + bytes([7, 7, 7]) + b"".join(bytes(entry) for entry in palette)
    cmap = b"".join(struct.pack(">HBB", 0, 1, column) for column in range(3))
    header = box(b"ihdr", struct.pack(">IIHBBBB", height, width, 1, 7, 7, 0, 0)) + box(b"colr", bytes([1, 0, 0]) + struct.pack(">I", 16)) + box(b"pclr", pclr) + box(b"cmap", cmap)
    jp2 = box(b"jP  ", b"\r\n\x87\n") + box(b"ftyp", b"jp2 \0\0\0\0jp2 ") + box(b"jp2h", header) + box(b"jp2c", codestream)
    objects = [b"<< /Type /Catalog /Pages 2 0 R >>", b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
               f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}] /Contents 4 0 R /Resources << /XObject << /Im1 5 0 R >> >> >>".encode(),
               f"<< /Length {len(f'q {width} 0 0 {height} 0 0 cm /Im1 Do Q')} >>\nstream\nq {width} 0 0 {height} 0 0 cm /Im1 Do Q\nendstream".encode(),
               f"<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /Filter /JPXDecode /Length {len(jp2)} >>".encode() + b"\nstream\n" + jp2 + b"\nendstream"]
    pdf = bytearray(b"%PDF-1.7\n")
    offsets = []
    for i, body in enumerate(objects):
        offsets.append(len(pdf))
        pdf += f"{i + 1} 0 obj\n".encode() + body + b"\nendobj\n"
    xref = len(pdf)
    pdf += f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode() + b"".join(f"{o:010} 00000 n \n".encode() for o in offsets) + f"trailer\n<< /Root 1 0 R /Size {len(objects) + 1} >>\nstartxref\n{xref}\n%%EOF\n".encode()
    with fitz.open(stream=bytes(pdf), filetype="pdf") as document:
        pixmap = document[0].get_pixmap(colorspace=fitz.csRGB)
        expected = bytes(c for y in range(height) for x in range(width) for c in pixmap.pixel(x, y))
    open(path, "w").write("\t".join(["jpx-palette", str(width), str(height), "3", "0", jp2.hex(), expected.hex()]))
    print("palette case")


main(sys.argv[1])
