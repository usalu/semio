"""🪟️ Build the mounted PDF content-edit specimen and check saved edits with MuPDF."""
import argparse
import json
import math
from pathlib import Path

import fitz


def pdf_numbers(numbers):
    return " ".join(str(value) for value in numbers)


def pdf_array(values):
    return f"[{pdf_numbers(values)}]".encode("ascii")


def stream(dictionary, contents):
    return b"<< " + dictionary + f" /Length {len(contents)} >>\nstream\n".encode() + contents + b"\nendstream"


def write_input(path, spec):
    page, text, vector, image, info = (spec[key] for key in ("page", "text", "vector", "image", "info"))
    content = (
        f"BT /F1 {text['size']} Tf {pdf_numbers(text['at'])} Td ({text['before']}) Tj ET\n"
        f"q {pdf_numbers(vector['fill'])} rg {pdf_numbers(vector['at'])} {vector['width']} {vector['height']} re f Q\n"
        f"q {image['width']} 0 0 {image['height']} {pdf_numbers(image['at'])} cm /Im0 Do Q\n"
    ).encode("ascii")
    media = pdf_array(page["media"])
    boundaries = b" /MediaBox " + media + b" /CropBox " + pdf_array(page["crop"]["before"]) + b" /TrimBox " + pdf_array(page["trim"]["before"])
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R 8 0 R] /Count 2 >>",
        b"<< /Type /Page /Parent 2 0 R" + boundaries
        + b" /Resources << /Font << /F1 5 0 R >> /XObject << /Im0 6 0 R >> >> /Contents 4 0 R >>",
        stream(b"", content),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>",
        stream(b"/Type /XObject /Subtype /Image /Width 2 /Height 2 /ColorSpace /DeviceRGB /BitsPerComponent 8", bytes(image["samples"])),
        f"<< /Title ({info['before']}) /Author ({info['author']}) >>".encode("ascii"),
        b"<< /Type /Page /Parent 2 0 R /MediaBox " + media + b" >>",
    ]
    output = bytearray(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n")
    offsets = []
    for number, body in enumerate(objects, 1):
        offsets.append(len(output))
        output.extend(f"{number} 0 obj\n".encode() + body + b"\nendobj\n")
    xref = len(output)
    output.extend(f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode())
    for offset in offsets:
        output.extend(f"{offset:010} 00000 n \n".encode())
    output.extend(f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R /Info 7 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode())
    path.write_bytes(output)


def write_reference(path, spec):
    page_spec, text, vector, image, info = (spec[key] for key in ("page", "text", "vector", "image", "info"))
    width, height = page_spec["media"][2:]
    with fitz.open() as document:
        page = document.new_page(width=width, height=height)
        page.insert_text((text["at"][0], height - text["at"][1]), text["after"], fontsize=text["size"])
        x, y = vector["newX"], vector["at"][1]
        rectangle = fitz.Rect(x, height - y - vector["height"], x + vector["width"], height - y)
        page.draw_rect(rectangle, color=None, fill=vector["fill"])
        x, y = image["at"]
        placement = fitz.Rect(x, height - y - image["height"], x + image["newWidth"], height - y)
        pixels = fitz.Pixmap(fitz.csRGB, 2, 2, bytes(image["samples"]), False)
        page.insert_image(placement, pixmap=pixels)
        page.set_cropbox(fitz.Rect(page_spec["crop"]["after"]))
        page.set_trimbox(fitz.Rect(page_spec["trim"]["after"]))
        document.new_page(width=width, height=height)
        document.set_metadata({"title": info["after"], "author": info["author"]})
        document.save(path)


def verify(path, spec, changed):
    phase = "after" if changed else "before"
    page_spec, text, vector, image, info = (spec[key] for key in ("page", "text", "vector", "image", "info"))
    with fitz.open(path) as pdf:
        assert not pdf.is_repaired and pdf.page_count == 2
        assert pdf.metadata["title"] == info[phase] and pdf.metadata["author"] == info["author"]
        page = pdf[0]
        assert list(page.mediabox) == page_spec["media"]
        for key in ("crop", "trim"):
            kind, value = pdf.xref_get_key(page.xref, f"{key.title()}Box")
            assert kind == "array" and [float(part) for part in value.strip("[]").split()] == page_spec[key][phase]
        page.set_cropbox(page.mediabox)
        spans = [span for block in page.get_text("dict")["blocks"] if block["type"] == 0
                 for line in block["lines"] for span in line["spans"]]
        span, = spans
        assert span["text"] == text[phase] and span["size"] == text["size"]
        width, height = page.mediabox_size
        assert span["origin"] == (text["at"][0], height - text["at"][1])
        assert width == page_spec["media"][2]
        drawing, = page.get_drawings()
        x, y = vector["at"]
        if changed:
            x = vector["newX"]
        assert list(drawing["rect"]) == [x, height - y - vector["height"], x + vector["width"], height - y]
        assert all(math.isclose(before, after, abs_tol=1e-6) for before, after in zip(drawing["fill"], vector["fill"]))
        placed, = page.get_image_info(xrefs=True)
        x, y = image["at"]
        width = image["newWidth"] if changed else image["width"]
        assert list(placed["bbox"]) == [x, height - y - image["height"], x + width, height - y]
        pixels = fitz.Pixmap(pdf, placed["xref"])
        assert (pixels.width, pixels.height, pixels.n) == (2, 2, 3)
        assert pixels.samples == bytes(image["samples"])
        assert not pdf[1].get_text() and not pdf[1].get_drawings() and not pdf[1].get_images()
        assert list(pdf[1].mediabox) == page_spec["media"]
    print(f"MuPDF verified the {phase} text, rectangle, RGB image, page boxes and metadata")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("saved", type=Path, nargs="?")
    parser.add_argument("--reference", type=Path)
    settings = parser.parse_args()
    spec = json.loads(Path(__file__).with_name("🔣️mounted-content-edit.json").read_text(encoding="utf-8"))
    if settings.saved is None:
        write_input(settings.input, spec)
    verify(settings.input, spec, False)
    if settings.saved is not None:
        verify(settings.saved, spec, True)
    if settings.reference is not None:
        write_reference(settings.reference, spec)
        verify(settings.reference, spec, True)
