"""🧫️ W3-T2-STROKES: writes the committed `paint-input-stroke` quintets of `s.wfc.bitmap` from an independent
Python computation of the leaf (Bresenham cell walk, bounding-box region, outcome codes), in the exact canonical
form the artifact's own Rust generator (`emit_committed_fixtures`) prints, so the Rust fixture laws and this
computation must agree byte for byte.

Run from the repository root: `python3 <this file>`.
"""

import base64
import json
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
SUBSET = os.path.join(ROOT, "✏️s", "🔌️plugins", "🀄️wfc", "🗿️artifacts", "🖼️bitmap", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any")
FIXTURES = os.path.join(SUBSET, "🧫️fixtures", "🧬️mutations")
BASE = os.path.join(FIXTURES, "🎲️change-seed", "🎲️reseeds-the-solve-from-7-to-99", "📸️snapshot", "⬅️before", "🔣️.json")

CASES = [
    ("✍️paints-a-diagonal-stroke-in-colour-1", [(0, 0), (3, 2)], 1),
    ("⚠️clips-a-stroke-leaving-the-sample", [(2, 0), (5, 0)], 0),
]


def cells(points):
    out, seen = [], set()
    def visit(cell):
        if cell not in seen:
            seen.add(cell)
            out.append(cell)
    if not points:
        return out
    visit(points[0])
    for (x, y), (ex, ey) in zip(points, points[1:]):
        dx, dy = abs(ex - x), -abs(ey - y)
        sx, sy = (1 if x < ex else -1), (1 if y < ey else -1)
        err = dx + dy
        while True:
            visit((x, y))
            if (x, y) == (ex, ey):
                break
            e2 = 2 * err
            if e2 >= dy:
                err += dy
                x += sx
            if e2 <= dx:
                err += dx
                y += sy
    return out


def write(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def main():
    with open(BASE, encoding="utf-8") as handle:
        before = json.load(handle)
    width, height = before["input"]["width"], before["input"]["height"]
    buffer = bytearray(base64.b64decode(before["input"]["pixels"]))
    for case, points, color in CASES:
        walked = cells(points)
        inside = [(x, y) for x, y in walked if x < width and y < height]
        min_x, min_y = min(x for x, _ in inside), min(y for _, y in inside)
        max_x, max_y = max(x for x, _ in inside), max(y for _, y in inside)
        w, h = max_x - min_x + 1, max_y - min_y + 1
        region = bytearray(buffer[(min_y + row) * width + min_x + col] for row in range(h) for col in range(w))
        for x, y in inside:
            region[(y - min_y) * w + (x - min_x)] = color
        after_buffer = bytearray(buffer)
        for row in range(h):
            after_buffer[(min_y + row) * width + min_x : (min_y + row) * width + min_x + w] = region[row * w : (row + 1) * w]
        after = json.loads(json.dumps(before))
        after["input"]["pixels"] = base64.b64encode(bytes(after_buffer)).decode("ascii")
        diff = {
            "schema": None, "seed": None, "inputWidth": None, "inputHeight": None, "inputPixels": None,
            "inputRegions": [{"x": min_x, "y": min_y, "width": w, "height": h, "pixels": base64.b64encode(bytes(region)).decode("ascii")}],
            "palette": None, "output": None, "model": None, "pinnedRemoved": [], "pinnedUpserted": [],
        }
        outcome = {"status": "applied"}
        if len(inside) < len(walked):
            outcome["messages"] = [{"level": "warning", "code": "mutation.partial"}]
        mutation = {"PaintInputStroke": {"points": [{"x": x, "y": y} for x, y in points], "color": color}}
        directory = os.path.join(FIXTURES, "✍️paint-input-stroke", case)
        write(os.path.join(directory, "📸️snapshot", "⬅️before", "🔣️.json"), before)
        write(os.path.join(directory, "📸️snapshot", "➡️after", "🔣️.json"), after)
        write(os.path.join(directory, "🦠️mutation", "🔣️.json"), mutation)
        write(os.path.join(directory, "🔺️diff", "🔣️.json"), diff)
        write(os.path.join(directory, "🎯️outcome", "🔣️.json"), outcome)
        print("wrote", case, "cells", walked, "inside", inside)


if __name__ == "__main__":
    main()
