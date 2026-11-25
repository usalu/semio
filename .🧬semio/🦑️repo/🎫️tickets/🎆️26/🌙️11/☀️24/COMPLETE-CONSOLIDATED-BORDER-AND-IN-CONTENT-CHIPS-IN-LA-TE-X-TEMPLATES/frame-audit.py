"""🧩 Audits the frames of PDFs from their vector drawings.

A rule is a filled rectangle or stroked line thinner than 1.5 pt; a neutral filled outline (a
tcolorbox frame) contributes its four hairline edges. A horizontal rule is a frame edge
when both of its ends meet a frame side, a vertical rule at least 12 pt tall. Per page it reports
  colour   frame edges that are not drawn in the neutral border colour,
  double   two frame edges that overlap horizontally with a gap between them (a border that is
           not shared),
  open     runs of stacked vertical frame rules in the neutral colour whose end meets no
           horizontal rule,
  erased   frame edges painted over in the canvas colour that are neither redrawn nor closed by
           the footer rule as the next edge below (a missing border).
Prints offenders and a verdict per document, and the totals.

usage: python frame-audit.py <neutral r,g,b> <pdf> [<pdf> ...]
"""
import sys
import pymupdf

neutral = tuple(float(value) for value in sys.argv[1].split(","))
near = lambda a, b, tolerance=1.0: abs(a - b) <= tolerance
same = lambda a, b: all(abs(x - y) < 0.02 for x, y in zip(a, b))


def rules(page):
    found = []
    for seq, drawing in enumerate(page.get_drawings()):
        color = drawing.get("fill") if drawing.get("fill") is not None else drawing.get("color")
        if color is None or len(color) != 3 or drawing.get("fill_opacity", 1) not in (None, 1, 1.0):
            continue
        outline = drawing["rect"]
        if drawing.get("fill") is not None and same(color, neutral) and outline.width >= 8 and outline.height >= 8:
            stroke = 0.75
            found += [("h", pymupdf.Rect(outline.x0, outline.y0, outline.x1, outline.y0 + stroke), tuple(color), seq),
                      ("h", pymupdf.Rect(outline.x0, outline.y1 - stroke, outline.x1, outline.y1), tuple(color), seq),
                      ("v", pymupdf.Rect(outline.x0, outline.y0, outline.x0 + stroke, outline.y1), tuple(color), seq),
                      ("v", pymupdf.Rect(outline.x1 - stroke, outline.y0, outline.x1, outline.y1), tuple(color), seq)]
            continue
        for item in drawing["items"]:
            if item[0] == "re":
                box = item[1]
            elif item[0] == "l" and drawing.get("fill") is None:
                box = pymupdf.Rect(item[1], item[2]).normalize()
                box.x1 += 0.01
                box.y1 += 0.01
            else:
                continue
            if box.height < 1.5 and box.width >= 8:
                found.append(("h", box, tuple(color), seq))
            elif box.width < 1.5 and box.height >= 8:
                found.append(("v", box, tuple(color), seq))
    return found


def audit(page):
    found = rules(page)
    horizontal = [(box, color) for kind, box, color, _ in found if kind == "h"]
    vertical = [(box, color) for kind, box, color, _ in found if kind == "v"]
    ordered = [(box, color, seq) for kind, box, color, seq in found if kind == "h"]
    level = lambda box: (box.y0 + box.y1) / 2
    spans = lambda box, edge, share: min(box.x1, edge.x1) - max(box.x0, edge.x0) > share * edge.width
    sides = [box for box, _ in vertical if box.height >= 12]
    meets = lambda x, y: any(near((box.x0 + box.x1) / 2, x, 1.2) and box.y0 - 1.2 <= y <= box.y1 + 1.2 for box in sides)
    erased = [(box, seq) for box, color, seq in ordered if same(color, page_canvas) and box.width >= 100]
    covered = lambda edge, at: any(abs(level(box) - level(edge)) < 0.6 and spans(box, edge, 0.9) for box, seq in erased if seq > at)
    edges = [(box, color) for box, color, seq in ordered if not same(color, page_canvas) and not covered(box, seq) and meets(box.x0, level(box)) and meets(box.x1, level(box))]
    issues = []
    for box, seq in erased:
        redrawn = any(later > seq and same(color, neutral) and abs(level(edge) - level(box)) < 0.6 and spans(edge, box, 0.9) for edge, color, later in ordered)
        below = [level(edge) for edge, color, _ in ordered if same(color, neutral) and level(edge) - level(box) > 0.5 and spans(edge, box, 0.9)]
        closed = bool(below) and min(below) > page.rect.height - 60
        if not redrawn and not closed:
            issues.append(("erased", round(box.y0, 2), round(box.x0, 2), round(box.width, 2), "no edge redrawn and no footer rule below"))
    for box, color in edges:
        if not same(color, neutral) and not same(color, page_canvas):
            issues.append(("colour", round(box.y0, 2), round(box.x0, 2), round(box.width, 2), tuple(round(c, 3) for c in color)))
    for index, (first, _) in enumerate(edges):
        for second, _ in edges[index + 1:]:
            gap = abs(second.y0 - first.y0)
            overlap = min(first.x1, second.x1) - max(first.x0, second.x0)
            if 1.0 < gap < 12 and overlap > 0.5 * min(first.width, second.width) and overlap > 20:
                issues.append(("double", round(min(first.y0, second.y0), 2), round(max(first.x0, second.x0), 2), round(overlap, 2), round(gap, 2)))
    live = [box for box, color, seq in ordered if not same(color, page_canvas) and not covered(box, seq)]
    touches = lambda x, y: any(box.x0 - 1.2 <= x <= box.x1 + 1.2 and near(level(box), y, 1.2) for box in live)
    runs = []
    for box in sorted((box for box, color in vertical if same(color, neutral)), key=lambda box: ((box.x0 + box.x1) / 2, box.y0)):
        x = (box.x0 + box.x1) / 2
        if runs and near(runs[-1][0], x, 0.3) and box.y0 <= runs[-1][2] + 1.2:
            runs[-1][2] = max(runs[-1][2], box.y1)
        else:
            runs.append([x, box.y0, box.y1])
    for x, y0, y1 in runs:
        box = pymupdf.Rect(x, y0, x, y1)
        for end in (box.y0, box.y1):
            if not touches(x, end):
                issues.append(("open", round(end, 2), round(x, 2), round(box.height, 2), "top" if end == box.y0 else "bottom"))
    return issues


totals = {"colour": 0, "double": 0, "open": 0, "erased": 0}
for path in sys.argv[2:]:
    document = pymupdf.open(path)
    counts = {"colour": 0, "double": 0, "open": 0, "erased": 0}
    shown = 0
    for number, page in enumerate(document, 1):
        pix = page.get_pixmap(clip=pymupdf.Rect(2, 2, 4, 4), dpi=72)
        page_canvas = tuple(channel / 255 for channel in pix.pixel(0, 0))
        for issue in audit(page):
            counts[issue[0]] += 1
            if shown < 12:
                print("  p%d" % number, *issue)
                shown += 1
    for key in totals:
        totals[key] += counts[key]
    print(path.replace("\\", "/").split("/out/")[-1], counts)
print("TOTAL", totals)
