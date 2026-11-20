"""🗾 Compare PDF-oriented clipping across MuPDF, the browser host painter, and native scanlines."""

import json
from pathlib import Path
import subprocess
import sys

import fitz


def main(fixture_path, browser_path, native_binary, output):
    fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
    width, height = fixture["page"]
    with fitz.open() as document:
        page = document.new_page(width=width, height=height)
        form = document.get_new_xref()
        bbox = " ".join(map(str, fixture["bbox"]))
        matrix = " ".join(map(str, fixture["matrix"]))
        document.update_object(form, f"<< /Type /XObject /Subtype /Form /BBox [{bbox}] /Matrix [{matrix}] >>")
        document.update_stream(form, fixture["formContent"].encode("ascii"))
        stream = document.get_new_xref()
        document.update_object(stream, "<<>>")
        document.update_stream(stream, b"/Form Do")
        document.xref_set_key(page.xref, "Resources", f"<< /XObject << /Form {form} 0 R >> >>")
        page.set_contents(stream)
        reference = page.get_pixmap(colorspace=fitz.csRGB)

    browser = fitz.Pixmap(str(browser_path))
    arguments = [str(value) for point in fixture["polygon"] for value in point] + [str(width), str(height)]
    bands = [tuple(map(int, line.split())) for line in subprocess.check_output([str(native_binary), *arguments], text=True).splitlines()]
    native = fitz.Pixmap(fitz.csRGB, fitz.IRect(0, 0, width, height), False)
    native.clear_with(255)
    for band in bands:
        x, y, w, h = band
        native.set_rect(fitz.IRect(x, y, x + w, y + h), (255, 0, 0))
    for probe in fixture["probes"]:
        for label, image in zip(("MuPDF", "browser", "native"), (reference, browser, native)):
            actual = list(image.pixel(*probe["at"])[:3])
            assert actual == probe["rgb"], (label, probe, actual)

    gap = 8
    comparison = fitz.Pixmap(fitz.csRGB, fitz.IRect(0, 0, width * 3 + gap * 2, height), False)
    comparison.clear_with(228)
    for index, image in enumerate((reference, browser, native)):
        for row in range(height):
            for column in range(width):
                comparison.set_pixel(column + index * (width + gap), row, image.pixel(column, row)[:3])
    comparison.save(str(output))
    print(f"MuPDF, browser paint, and native scanline output match {len(fixture['probes'])} interior/exterior probes")


if __name__ == "__main__":
    main(*map(Path, sys.argv[1:]))
