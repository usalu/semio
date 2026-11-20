"""📐 Exercises the shared inline-image fixture in genuine PDF pages through MuPDF."""
import json
from pathlib import Path
import sys

import fitz


def pdf(stream):
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 120 40] /Resources << >> /Contents 4 0 R >>",
        f"<< /Length {len(stream)} >>\nstream\n".encode() + stream + b"endstream",
    ]
    data = bytearray(b"%PDF-1.7\n")
    offsets = []
    for number, body in enumerate(objects, 1):
        offsets.append(len(data))
        data.extend(f"{number} 0 obj\n".encode() + body + b"\nendobj\n")
    xref = len(data)
    data.extend(b"xref\n0 5\n0000000000 65535 f \n")
    for offset in offsets:
        data.extend(f"{offset:010d} 00000 n \n".encode())
    data.extend(f"trailer\n<< /Size 5 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode())
    return bytes(data)


def stream(case):
    prefix = f"q\n120 0 0 40 0 0 cm\nBI {case['dict']} ID{case['separator']}".encode()
    return prefix + bytes(case["raw"]) + case["tail"].encode() + b"Q\n"


def verify(fixture, output):
    images = pixels = 0
    for case in json.loads(fixture.read_text()):
        if "rgb" not in case:
            continue
        rendered_pdf = pdf(stream(case))
        (output / (case["name"] + ".pdf")).write_bytes(rendered_pdf)
        with fitz.open(stream=rendered_pdf, filetype="pdf") as document:
            pixmap = document[0].get_pixmap()
            width, height = case["grid"]
            actual = [list(pixmap.pixel((2*x+1)*60//width, (2*y+1)*20//height))
                      for y in range(height) for x in range(width)]
            assert actual == case["rgb"], (case["name"], actual, case["rgb"])
            images += 1
            pixels += len(actual)
    print(f"MuPDF matched the shared fixture for {images} inline images and {pixels} sample pixels.")


if __name__ == "__main__":
    verify(*map(Path, sys.argv[1:]))
