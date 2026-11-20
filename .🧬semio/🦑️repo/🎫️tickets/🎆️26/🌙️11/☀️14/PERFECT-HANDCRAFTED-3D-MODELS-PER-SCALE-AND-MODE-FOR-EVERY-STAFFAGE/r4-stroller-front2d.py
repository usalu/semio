"""🪟 Redraws the seat fabric of `humans/parent-with-stroller` in the front drawing of every scale so that the toddler's face shows under the hood rim.

The front shape `seat-fabric` was the full rectangle of the sling up to the top of the backrest. In the pushing pose the product paints it over the toddler's head and the hood (its nearest point, the leg rest, lies ahead of both), so the front view showed a headless child. The seat is now drawn as what the front view really shows of it: the sling below the hood arch, with the outline of the toddler's head left open. The 1:500 drawing keeps its plain rectangle: at that size the hood covers the head and a notch would fall under the print minimum. Points are stated by hand per scale from the hood arch and head outline of that scale, in the frame of the stroller bone (the hood arch lies 0.085 m and the head outline 0.11 m higher there than in their own bone frames).

Usage: `python3 r4-stroller-front2d.py` writes the candidate `🗑️generated/r4-g08/stroller/🔣️.json`; `--commit` writes the catalog file (re-read right before).
"""
import glob
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CAT = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/parent-with-stroller/🔣️.json")[0]
OUT = os.path.join(T, "🗑️generated", "r4-g08", "stroller", "🔣️.json")

SEAT = {
    "50": [[-0.1781, -0.94], [0.1781, -0.94], [0.1781, -0.31], [0.1682, -0.297], [0.1223, -0.286], [0.062, -0.2825], [0.062, -0.31], [0.056, -0.335], [0.04, -0.356], [0.02, -0.365], [-0.02, -0.365], [-0.04, -0.356], [-0.056, -0.335], [-0.062, -0.31], [-0.062, -0.2825], [-0.1223, -0.286], [-0.1682, -0.297], [-0.1781, -0.31]],
    "100": [[-0.178, -0.9404], [0.1782, -0.9404], [0.1782, -0.32], [0.15, -0.295], [0.0825, -0.286], [0.062, -0.286], [0.06, -0.32], [0.045, -0.352], [0.022, -0.372], [-0.022, -0.372], [-0.045, -0.352], [-0.06, -0.32], [-0.062, -0.286], [-0.0825, -0.286], [-0.15, -0.295], [-0.178, -0.32]],
    "200": [[-0.19, -0.9534], [0.1887, -0.9534], [0.1887, -0.406], [0.165, -0.406], [0.165, -0.328], [0.135, -0.302], [0.064, -0.296], [0.06, -0.33], [0.044, -0.36], [0, -0.378], [-0.044, -0.36], [-0.06, -0.33], [-0.064, -0.296], [-0.135, -0.302], [-0.165, -0.328], [-0.165, -0.406], [-0.19, -0.406]],
}
ORDER = ["50", "100", "200"]


def points_text(points, pad):
    rows = []
    for x, y in points:
        rows.append("%s  [\n%s    %s,\n%s    %s\n%s  ]" % (pad, pad, repr(x), pad, repr(y), pad))
    return '"points": [\n' + ",\n".join(rows) + "\n" + pad + "]"


def redraw(text):
    marker = '"id": "seat-fabric"'
    hits = []
    at = text.find(marker)
    while at != -1:
        hits.append(at)
        at = text.find(marker, at + 1)
    if len(hits) != 12:
        raise SystemExit("expected 12 seat-fabric shapes (4 scales x 3 views), found %d" % len(hits))
    for scale, hit in reversed(list(zip(ORDER, hits[1::3]))):
        start = text.index('"points": [', hit)
        end = text.index('"closed"', start)
        pad = " " * (start - text.rindex("\n", 0, start) - 1)
        tail = text[text.rindex("]", start, end) + 1 : end]
        text = text[:start] + points_text(SEAT[scale], pad) + tail + text[end:]
    return text


if __name__ == "__main__":
    fresh = redraw(open(CAT, encoding="utf-8").read())
    target = CAT if "--commit" in sys.argv else OUT
    os.makedirs(os.path.dirname(target), exist_ok=True)
    open(target, "w", encoding="utf-8").write(fresh)
    print("written", target)
