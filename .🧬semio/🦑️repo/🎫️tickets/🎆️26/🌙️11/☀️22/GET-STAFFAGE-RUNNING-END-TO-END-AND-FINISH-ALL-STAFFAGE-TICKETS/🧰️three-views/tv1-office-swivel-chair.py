"""🪑️ Office swivel chair, three-views sweep (clip swivel-rotation, seat turned about 32 degrees): the side elevation drew backrest shell and pad as one prism over the full width, but the backrest is a shell curved in plan (its flanks stand up to 43 mm ahead of its middle), so the turned prism reached 47 mm further back than the plan and the 3D body. Each prism becomes five: the middle keeps the side profile, and two bands per side take it with the rear edge set forward to where the body stands at the outer edge of that band and foot and head cut at the heights the rounded corners leave the band (`tv1-local.ts`, 16 bands); every band lies inside the old outline, so the drawing at rest keeps it. The rim and texture lines of 1:50 are banded the same way. `python3 tv1-office-swivel-chair.py [target json]`."""
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("office-swivel-chair"))

BANDS = (0.105, 0.17)
SPAN = {"mid": (0.15, 0.635), "edge": (0.17, 0.6)}
LINES = {(50, "backrest-rim"): (-0.023, -0.046), (50, "backrest-texture"): (-0.019, -0.04)}
SHIFTS = {
    (50, "backrest-shell"): (-0.023, -0.046),
    (50, "backrest-pad"): (-0.019, -0.04),
    (100, "backrest-shell"): (-0.023, -0.046),
    (100, "backrest-pad"): (-0.019, -0.04),
    (200, "backrest-shell"): (-0.03, -0.054),
    (500, "backrest"): (-0.029, -0.046),
}


def chains(points):
    """🔗️ The rear (larger v) and the front chain of a side profile, split at its lowest and highest point along the bone."""
    low = min(range(len(points)), key=lambda i: points[i][0])
    ring = points[low:] + points[:low]
    high = max(range(len(ring)), key=lambda i: ring[i][0])
    first, second = ring[: high + 1], ring[high:] + ring[:1]
    mean = lambda chain: sum(p[1] for p in chain) / len(chain)
    return (first, second) if mean(first) > mean(second) else (second, first)


def clip(ring, limit, keep):
    """✂️ The part of a ring on the kept side of a height along the bone."""
    cut = []
    for a, b in zip(ring, ring[1:] + ring[:1]):
        if keep(a[0]):
            cut.append(a)
        if keep(a[0]) != keep(b[0]):
            t = (limit - a[0]) / (b[0] - a[0])
            cut.append([limit, round(a[1] + t * (b[1] - a[1]), 5)])
    return cut


def band(points, rear_shift, span):
    """📐️ The profile of one band: the rear edge set forward, foot and head cut where the rounded corners of the shell end in that band; the front edge stays, so the band lies inside the old outline."""
    rear, front = chains(points)
    ring = [[p[0], round(p[1] + rear_shift, 5)] for p in rear] + [list(p) for p in front]
    return clip(clip(ring, span[0], lambda u: u >= span[0]), span[1], lambda u: u <= span[1])


for (scale, shape_id), (inner, outer) in SHIFTS.items():
    path, shape = doc.shape(scale, "elevation", shape_id, "backrest")
    half = shape["solid"]["z1"]
    assert shape["solid"]["z0"] == -half and half > BANDS[1]
    parts = [{**shape, "solid": {**shape["solid"], "z0": -BANDS[0], "z1": BANDS[0]}}]
    for name, shift, lo, hi in (("mid", inner, BANDS[0], BANDS[1]), ("edge", outer, BANDS[1], half)):
        for side, z0, z1 in (("left", lo, hi), ("right", -hi, -lo)):
            parts.append({**shape, "id": f"{shape_id}-{name}-{side}", "points": band(shape["points"], shift, SPAN[name]), "solid": {**shape["solid"], "z0": z0, "z1": z1}})
    doc.many(path, parts)

for (scale, shape_id), shifts in LINES.items():
    path, shape = doc.shape(scale, "elevation", shape_id, "backrest")
    half = shape["solid"]["z1"]
    parts = [{**shape, "solid": {**shape["solid"], "z0": -BANDS[0], "z1": BANDS[0]}}]
    for name, shift, lo, hi in (("mid", shifts[0], BANDS[0], BANDS[1]), ("edge", shifts[1], BANDS[1], half)):
        for side, z0, z1 in (("left", lo, hi), ("right", -hi, -lo)):
            parts.append({**shape, "id": f"{shape_id}-{name}-{side}", "points": [[min(p[0], SPAN[name][1]), round(p[1] + shift, 5)] for p in shape["points"]], "solid": {**shape["solid"], "z0": z0, "z1": z1}})
    doc.many(path, parts)

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
