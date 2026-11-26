import json
import re
import sys
from pathlib import Path

from PIL import Image
from pypdf import PdfReader


def rgb(hexadecimal):
    return tuple(bytes.fromhex(hexadecimal.lstrip("#")))


def pixel_at(image, x, y):
    return image.getpixel((round(image.width * x), round(image.height * y)))


def close(actual, expected):
    return all(abs(left - right) <= 2 for left, right in zip(actual, expected))


def sheet_image(directory, page):
    candidates = (directory / f"p-{page:02}.png", directory / f"p-{page}.png")
    return Image.open(next(path for path in candidates if path.exists())).convert("RGB")


def require_sheet_roles(image, margin, interior, body_y=0.65):
    for x, y in [(0.01, 0.5), (0.99, 0.5), (0.5, 0.01), (0.5, 0.99)]:
        assert close(pixel_at(image, x, y), margin), (x, y, pixel_at(image, x, y))
    assert close(pixel_at(image, 0.5, body_y), interior), pixel_at(image, 0.5, body_y)


def require_shared_frame(page):
    content = page.get_contents().get_data()
    rectangle = re.search(rb"cm [\s\S]*? rg q (-?[\d.]+) (-?[\d.]+) ([\d.]+) ([\d.]+) re f Q", content)
    assert rectangle, content[:300]
    x, bottom, width, height = map(float, rectangle.groups())
    rules = []
    for line in re.finditer(rb"q (\.?[\d.]+) w (-?[\d.]+) (-?[\d.]+) m (-?[\d.]+) (-?[\d.]+) l S Q", content):
        stroke, left, start, right, end = map(float, line.groups())
        if abs(start - end) < 0.01 and abs(left - x) < 0.01 and abs(right - x - width) < 0.01:
            rules.append((start, stroke))
    assert rules
    low, low_stroke = min(rules)
    high, high_stroke = max(rules)
    assert abs(bottom - (low - low_stroke / 2)) < 0.02
    assert abs(bottom + height - (high + high_stroke / 2)) < 0.02
    return x, content


def main(run):
    source = Path("🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧪️tests/🔣️.json")
    appearance = {row["theme"]: row for row in json.loads(source.read_text())["chips"]}
    for document, theme, file in [("report", "light", "report"), ("report-dark", "dark", "report")]:
        directory = run / document
        palette = appearance[theme]
        margin, interior = rgb(palette["margin"]), rgb(palette["interior"])
        for index in [1, 2, 3, 9]:
            require_sheet_roles(sheet_image(directory, index), margin, interior)
        document_pdf = PdfReader(directory / (file + ".pdf"))
        verso, _ = require_shared_frame(document_pdf.pages[1])
        recto, body = require_shared_frame(document_pdf.pages[2])
        assert recto - verso > 20
        assert b"/pgf@ca0.76 gs" in body and b"/pgf@ca0.1 gs" in body
        center = pixel_at(sheet_image(directory, 4), 0.5, 0.5)
        assert close(center, margin)
        print("[DEBUG]", document, "cover and body colors, blank verso, matched border coordinates, binding offsets and chip opacities verified")
    palette = appearance["light"]
    for document in ["thesis", "flyer"]:
        require_sheet_roles(sheet_image(run / document, 1), rgb(palette["margin"]), rgb(palette["interior"]))
        print("[DEBUG]", document, "cover sheet roles verified")


def require_articles(run):
    palette = rgb("#f7f3e3"), rgb("#e9e6d7")
    for name, pages in [("proof", 9), ("notes", 3)]:
        directory = run / name
        document = PdfReader(directory / "_.pdf")
        assert len(document.pages) == pages
        for index in range(1, pages + 1):
            image = sheet_image(directory, index)
            require_sheet_roles(image, *palette, body_y=0.91)
        fonts = document.pages[0]["/Resources"]["/Font"].get_object()
        assert any("LMRoman" in str(font.get_object().get("/BaseFont")) for font in fonts.values())
        if name == "notes":
            image = sheet_image(directory, 1)
            for y in [0.05, 0.95]:
                assert close(pixel_at(image, 0.5, y), palette[1])
        print("[DEBUG]", name, pages, "article pages preserve Latin Modern and distinguish the running interior from the physical margins")


main(Path(sys.argv[1]))
if len(sys.argv) > 2:
    require_articles(Path(sys.argv[2]))
