"""✂️ Shows how a sheet PDF clips its hatch marks: sheet-clips.py <pdf> <x-mm> <y-mm> prints the drawing entries (with clip levels and scissors, PyMuPDF extended mode) that have a point within 0.6 mm of the given paper point, and the raw content operators around the first hatch."""
import sys

import pymupdf

MM_PER_PT = 25.4 / 72


def show(pdf: str, x_mm: float, y_mm: float) -> None:
    """🔎️ Print the entries near the point and the clip entries that enclose them."""
    page = pymupdf.open(pdf)[0]
    entries = page.get_drawings(extended=True)
    for index, entry in enumerate(entries):
        rect = entry.get("rect") or entry.get("scissor")
        near = rect is not None and rect.x0 * MM_PER_PT - 0.6 <= x_mm <= rect.x1 * MM_PER_PT + 0.6 and rect.y0 * MM_PER_PT - 0.6 <= y_mm <= rect.y1 * MM_PER_PT + 0.6
        if near:
            box = [round(v * MM_PER_PT, 3) for v in (rect.x0, rect.y0, rect.x1, rect.y1)]
            print(index, entry.get("type"), "level", entry.get("level"), "layer", entry.get("layer"), "box mm", box, "fill", entry.get("fill"), "even_odd", entry.get("even_odd"))


if __name__ == "__main__":
    show(sys.argv[1], float(sys.argv[2]), float(sys.argv[3]))
