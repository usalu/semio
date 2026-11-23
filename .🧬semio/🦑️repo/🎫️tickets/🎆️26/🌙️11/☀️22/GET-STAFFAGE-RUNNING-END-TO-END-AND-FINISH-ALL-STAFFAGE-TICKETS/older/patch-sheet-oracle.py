"""🩹️ One-time edit of the sheet oracle `🧪️tests/📑️multiview-sheet/🐍️.py`: the PDF figure box is measured from what the page shows, so marks that a clip path cuts away (hatch dots and hatch lines of a filled shape) count only inside their clip. Re-reads the file, replaces `pdf_report` exactly once, writes atomically."""
import os
import pathlib

FILE = pathlib.Path(__file__).resolve().parents[8] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🧪️tests" / "📑️multiview-sheet" / "🐍️.py"

OLD = '''def pdf_report(path, regions):
    document = pymupdf.open(path)
    page = document[0]
    pad = 0.5 / MM_PER_PT
    found = {view: None for view in VIEWS}
    for drawing in page.get_drawings():
        if drawing.get("layer") in PDF_OVERLAYS:
            continue
        for item in drawing["items"]:
            points = [p for p in item[1:] if hasattr(p, "x")] if item[0] != "re" else [item[1].tl, item[1].br]
            for p in points:
                for view, region in regions.items():
                    x, y = p.x * MM_PER_PT, p.y * MM_PER_PT
                    if region[0] - 0.5 <= x <= region[2] + 0.5 and region[1] - 0.5 <= y <= region[3] + 0.5:
                        found[view] = grow(found[view], x, y)
    return'''

NEW = '''def visible(points, clips):
    """👁️ The corner points of what a clipped item shows: its points as they are without a clip, else the corners of its box cut by every enclosing clip rectangle (none when nothing of it shows)."""
    if not clips or not points:
        return points
    box = [min(p[0] for p in points), min(p[1] for p in points), max(p[0] for p in points), max(p[1] for p in points)]
    for clip in clips:
        box = [max(box[0], clip.x0), max(box[1], clip.y0), min(box[2], clip.x1), min(box[3], clip.y1)]
    return [] if box[0] > box[2] or box[1] > box[3] else [(box[0], box[1]), (box[2], box[3])]


def pdf_report(path, regions):
    document = pymupdf.open(path)
    page = document[0]
    found = {view: None for view in VIEWS}
    clips = {}
    for drawing in page.get_drawings(extended=True):
        level = drawing["level"]
        clips = {depth: rect for depth, rect in clips.items() if depth < level}
        if drawing["type"] == "clip":
            clips[level] = drawing["scissor"]
            continue
        if drawing["type"] == "group" or drawing.get("layer") in PDF_OVERLAYS:
            continue
        for item in drawing["items"]:
            points = [(p.x, p.y) for p in item[1:] if hasattr(p, "x")] if item[0] != "re" else [(item[1].x0, item[1].y0), (item[1].x1, item[1].y1)]
            for px, py in visible(points, list(clips.values())):
                for view, region in regions.items():
                    x, y = px * MM_PER_PT, py * MM_PER_PT
                    if region[0] - 0.5 <= x <= region[2] + 0.5 and region[1] - 0.5 <= y <= region[3] + 0.5:
                        found[view] = grow(found[view], x, y)
    return'''

text = FILE.read_text(encoding="utf-8")
assert text.count(OLD) == 1, "pdf_report changed since this edit was written"
temporary = FILE.with_name(FILE.name + ".tmp")
temporary.write_text(text.replace(OLD, NEW), encoding="utf-8")
os.replace(temporary, FILE)
print("patched", FILE)
