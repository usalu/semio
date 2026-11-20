"""📐️ Check PDF viewport sizes and crop pixels with PyMuPDF 1.24.14."""

import json
from pathlib import Path
import sys

import fitz


def verify(fixture_path):
    fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
    for case in fixture["cases"]:
        with fitz.open() as document:
            page = document.new_page()
            for key, value in (
                ("MediaBox", str(fixture["mediaBox"]).replace(",", "")),
                ("CropBox", str(fixture["cropBox"]).replace(",", "")),
                ("UserUnit", str(fixture["userUnit"])),
                ("Rotate", str(case["rotate"])),
            ):
                document.xref_set_key(page.xref, key, value)
            stream = document.get_new_xref()
            document.update_object(stream, "<<>>")
            operations = []
            for rectangle in fixture["rectangles"]:
                color = " ".join(map(str, rectangle["rgb"]))
                bounds = " ".join(map(str, rectangle["bounds"]))
                operations.append(f"{color} rg\n{bounds} re\nf")
            document.update_stream(stream, "\n".join(operations).encode("ascii"))
            page.set_contents(stream)
            page = document.reload_page(page)
            raster = page.get_pixmap(colorspace=fitz.csRGB)
            assert [raster.width, raster.height] == case["size"], (case, raster)
            assert list(raster.pixel(*case["ink"])) == fixture["red"], case
            assert list(raster.pixel(*case["blank"])) == fixture["white"], case
            samples = raster.samples
            assert all(samples[index:index + 3] != bytes((0, 0, 255))
                       for index in range(0, len(samples), raster.n)), case
    with fitz.open() as document:
        page = document.new_page(width=100, height=100)
        x, y, width, height = fixture["secondPageInk"]
        page.draw_rect(fitz.Rect(x, 100 - y - height, x + width, 100 - y), color=None, fill=(0, 0, 0))
        raster = page.get_pixmap(colorspace=fitz.csRGB)
        x, y = fixture["onSecondPage"]
        assert list(raster.pixel(x, y - fixture["secondPageTop"])) == [0, 0, 0]
        assert fixture["cases"][1]["size"][1] < fixture["pageGap"][1] < fixture["secondPageTop"]
    print(f"{len(fixture['cases'])} rotated viewport sizes and crop pixels match")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
