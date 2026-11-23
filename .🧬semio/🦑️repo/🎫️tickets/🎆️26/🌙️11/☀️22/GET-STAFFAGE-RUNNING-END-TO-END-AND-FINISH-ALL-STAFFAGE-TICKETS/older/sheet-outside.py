"""📑️ Lists what a sheet PDF draws beyond the figure boxes its SVG twin reports, by the rules of the sheet oracle (`🧪️tests/📑️multiview-sheet/🐍️.py`): sheet-outside.py <svg> <pdf> prints, per view, every PDF drawing item outside the SVG box but within the 0.5 mm pad, with its layer, kind, colours and overshoot."""
import re
import sys

import pymupdf
from lxml import etree

VIEWS = ("elevation", "front", "plan")
SKIPPED_LAYERS = ("staffage-ground", "staffage-annotations", "staffage-legend", "staffage-background", "staffage-forces", "staffage-diagrams")
NUMBER = re.compile(r"-?\d+(?:\.\d+)?(?:e-?\d+)?")
MM_PER_PT = 25.4 / 72
PDF_OVERLAYS = ("Ground", "Forces and supports", "Diagrams", "Annotations", "Legend", "Background")


def svg_boxes(path: str) -> dict:
    """📐️ The figure box of every view of the SVG, and the layers that contribute to it."""
    boxes = {view: None for view in VIEWS}
    layers = {view: set() for view in VIEWS}
    for element in etree.parse(path).getroot().iter("{http://www.w3.org/2000/svg}path"):
        ids = [node.get("id", "") for node in [element, *element.iterancestors()]]
        layer = next((i for i in ids if i.startswith("staffage-")), "")
        view = next((v for i in ids for v in VIEWS if i.startswith(v + "-")), None)
        if layer in SKIPPED_LAYERS or view is None:
            continue
        layers[view].add(layer)
        numbers = [float(n) for n in NUMBER.findall(re.sub(r"[A-Za-z]", " ", element.get("d")))]
        for x, y in zip(numbers[0::2], numbers[1::2]):
            box = boxes[view]
            boxes[view] = [x, y, x, y] if box is None else [min(box[0], x), min(box[1], y), max(box[2], x), max(box[3], y)]
    return {"boxes": boxes, "layers": layers}


def outside(svg: str, pdf: str) -> None:
    """🔎️ Print the overshooting items of `pdf` against the boxes of `svg`."""
    report = svg_boxes(svg)
    page = pymupdf.open(pdf)[0]
    drawings = page.get_drawings()
    print("pdf layers", sorted({str(d.get("layer")) for d in drawings}))
    for view, region in report["boxes"].items():
        print(view, "svg box", [round(v, 3) for v in region], "svg layers", sorted(report["layers"][view]))
        for drawing in drawings:
            if drawing.get("layer") in PDF_OVERLAYS:
                continue
            for item in drawing["items"]:
                points = [p for p in item[1:] if hasattr(p, "x")] if item[0] != "re" else [item[1].tl, item[1].br]
                for p in points:
                    x, y = p.x * MM_PER_PT, p.y * MM_PER_PT
                    if region[0] - 0.5 <= x <= region[2] + 0.5 and region[1] - 0.5 <= y <= region[3] + 0.5:
                        over = max(region[0] - x, x - region[2], region[1] - y, y - region[3])
                        if over > 0.02:
                            print("  ", drawing.get("layer"), drawing.get("type"), item[0], "fill", drawing.get("fill"), "stroke", drawing.get("color"), "width", drawing.get("width"), "at", round(x, 3), round(y, 3), "over", round(over, 3))


if __name__ == "__main__":
    outside(sys.argv[1], sys.argv[2])
