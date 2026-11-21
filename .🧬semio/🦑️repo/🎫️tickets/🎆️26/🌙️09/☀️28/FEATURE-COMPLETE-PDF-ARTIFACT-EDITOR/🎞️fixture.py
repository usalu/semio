"""🎞️ Builds the shared sampled-image PDF and checks its rendered colors with MuPDF."""

from pathlib import Path
import json
import zlib


ROOT = Path(__file__).parent
GENERATED = ROOT / "🗑️generated"


def packed_samples(image, width, height):
    quadrants = list(map(bytes, image["samples"]))
    top = quadrants[0] * (width // 2) + quadrants[1] * (width // 2)
    bottom = quadrants[2] * (width // 2) + quadrants[3] * (width // 2)
    return top * (height // 2) + bottom * (height // 2)


def pdf_stream(body, fields=""):
    return f"<< /Length {len(body)}{fields} >>\nstream\n".encode() + body + b"\nendstream"


def make_pdf(spec):
    page_width, page_height = spec["pageSize"]
    width, height = spec["imageSize"]
    x, y = spec["imageOrigin"]
    first, second, third = spec["images"]

    def resources(image):
        return f"<< /XObject << /{image['name']} {image['object']} 0 R >> >>"

    def page(contents, resource_entry=""):
        return f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {page_width} {page_height}] /Contents {contents} 0 R{resource_entry} >>".encode()

    def content(image):
        return pdf_stream(f"q {width} 0 0 {height} {x} {y} cm /{image['name']} Do Q\n".encode())

    def image_stream(image):
        samples = packed_samples(image, width, height)
        fields = f" /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /{image['space']} /BitsPerComponent 8"
        if image["filter"]:
            fields += f" /Filter /{image['filter']}"
            samples = zlib.compress(samples, level=0)
        return pdf_stream(samples, fields)

    rows = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        f"<< /Type /Pages /Kids [3 0 R 6 0 R 9 0 R 13 0 R] /Count 4 /Resources {resources(first)} >>".encode(),
        page(4), content(first), image_stream(first),
        page(7, f" /Resources {resources(second)}"), content(second), image_stream(second),
        page(12, " /Resources 10 0 R"), resources(third).encode(), image_stream(third), content(third),
        page(14), content(first),
    ]
    output = bytearray(b"%PDF-1.7\n")
    offsets = []
    for number, row in enumerate(rows, start=1):
        offsets.append(len(output))
        output.extend(f"{number} 0 obj\n".encode() + row + b"\nendobj\n")
    xref = len(output)
    size = len(rows) + 1
    output.extend(f"xref\n0 {size}\n0000000000 65535 f \n".encode())
    for offset in offsets:
        output.extend(f"{offset:010d} 00000 n \n".encode())
    output.extend(f"trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode())
    return output


def verify(spec, file):
    import fitz

    width, height = spec["imageSize"]
    with fitz.open(file) as pdf:
        assert len(pdf) == len(spec["pages"])
        for page, expected in zip(pdf, spec["pages"]):
            image = spec["images"][expected["image"]]
            rendered = page.get_pixmap(colorspace=fitz.csRGB)
            rendered_colors = [rendered.pixel(*pixel) for pixel in spec["pixelLocations"]]
            expected_colors = [tuple(values if len(values) == 3 else values * 3) for values in image["samples"]]
            assert rendered_colors == expected_colors, (rendered_colors, expected_colors)
            assert rendered.pixel(0, 0) == (255, 255, 255)
            [xobject] = page.get_images()
            assert tuple(xobject[2:5]) == (width, height, 8)
            original = fitz.Pixmap(pdf, xobject[0])
            assert original.samples == packed_samples(image, width, height)
    return len(spec["pages"])


if __name__ == "__main__":
    import sys

    corpus = json.loads((ROOT / "🎞️fixture.json").read_text())
    if len(sys.argv) > 1:
        destination = Path(sys.argv[1])
    else:
        destination = GENERATED / "🎞️controlled-images.pdf"
        destination.parent.mkdir(exist_ok=True)
        destination.write_bytes(make_pdf(corpus))
        import fitz
        with fitz.open(destination) as original:
            for image in corpus["images"]:
                xref = image["object"]
                (GENERATED / f"{xref}.samples").write_bytes(original.xref_stream(xref))
        assert destination.stat().st_size > corpus["minimumByteLength"]
    print(f"[DEBUG] MuPDF verified {verify(corpus, destination)} sample-backed pages in {destination.name}")
