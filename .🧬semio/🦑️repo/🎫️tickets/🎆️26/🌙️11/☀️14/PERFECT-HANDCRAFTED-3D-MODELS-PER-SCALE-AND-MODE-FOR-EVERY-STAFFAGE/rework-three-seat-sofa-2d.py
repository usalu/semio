"""✏️ Three-seat sofa, author pass 1: the seat cushions of the 2D drawings follow the domed crown of the 3D body (1:50 and 1:100).

Side elevation (seat-1..3, bone-local: x along the cushion, y above the deck) and Section A-A (seat-2: x = world z, y = world height).
Only the points between the two seam corners are redrawn by hand; the file keeps its formatting (text splice of the point arrays).
Usage: python3 rework-three-seat-sofa-2d.py <catalog asset json> <output json>   (output = the catalog file to write in place)
@see ./rework-three-seat-sofa-cushions.py
"""
import json
import sys

DOME = {
    "50": (0.17, [(0.012, 0.189), (0.030, 0.196), (0.065, 0.202), (0.115, 0.206), (0.180, 0.2085), (0.235, 0.21)]),
    "100": (0.175, [(0.012, 0.19), (0.045, 0.2), (0.12, 0.207), (0.22, 0.21)]),
}
DECK = 0.23


def redraw(points, seam, dome, base):
    """🛋️ Keep the boxing below the seam, draw the dome between the two seam corners."""
    lo, hi = min(p[0] for p in points), max(p[0] for p in points)
    left = max(i for i, p in enumerate(points) if abs(p[0] - lo) < 1e-9 and p[1] <= base + seam + 1e-9)
    right = min(i for i, p in enumerate(points) if abs(p[0] - hi) < 1e-9 and p[1] <= base + seam + 1e-9)
    up = [[round(lo + inset, 4), round(base + y, 4)] for inset, y in dome]
    down = [[round(hi - inset, 4), round(base + y, 4)] for inset, y in reversed(dome)]
    return points[: left + 1] + up + down + points[right:]


def splice(text, old, new):
    """🧵 Replace every point array equal to old by new, at the indentation it has in the file."""
    decoder, at, hits = json.JSONDecoder(), 0, []
    while (at := text.find('"points": [', at)) >= 0:
        start = at + len('"points": ')
        value, end = decoder.raw_decode(text, start)
        if value == old:
            hits.append((start, end))
        at = end
    assert hits, "point array not found"
    for start, end in reversed(hits):
        pad = text[text.rfind("\n", 0, end) + 1 : end - 1]
        text = text[:start] + json.dumps(new, indent=2).replace("\n", "\n" + pad) + text[end:]
    return text, len(hits)


def main():
    """✍️ Re-read the catalog file, redraw the seat cushions, write."""
    text = open(sys.argv[1], encoding="utf-8").read()
    representations = json.loads(text)["asset"]["representations"]
    count, done = 0, []
    for scale, (seam, dome) in DOME.items():
        geometry = representations[scale]["geometry"]
        for view, base in (("elevation", 0.0), ("front", DECK)):
            for shape in geometry[view]:
                if shape["id"] in ("seat-1", "seat-2", "seat-3") and shape["kind"] == "poly":
                    new = redraw(shape["points"], seam, dome, base)
                    if new != shape["points"] and shape["points"] not in done:
                        text, hits = splice(text, shape["points"], new)
                        done.append(shape["points"])
                        count += hits
    json.loads(text)
    open(sys.argv[2], "w", encoding="utf-8").write(text)
    print("seat shapes redrawn", count)


main()
