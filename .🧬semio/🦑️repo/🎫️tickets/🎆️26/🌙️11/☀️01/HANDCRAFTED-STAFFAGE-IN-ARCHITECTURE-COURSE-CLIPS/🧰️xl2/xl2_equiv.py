"""Compare a freshly drawn tinted/lit/typed small tower with the cached template recoloured in place, and tower_capsules with the fresh flat tint. Usage: xl2_equiv.py"""
import importlib
import os
import sys
import time

import numpy as np

sys.path.insert(0, os.getcwd())

V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
draw = V.NakaginDrawing()
OPTIONS = dict(towers=(0,), where=lambda s: s.tower == 0 and s.storey < 2, core=False, podium=False, capital=False)
SCALE = 1.0


def signature(shape):
    return (np.round(shape.points, 9).tobytes(), tuple(np.round(shape.get_fill_rgbas().ravel(), 6)), tuple(np.round(shape.get_stroke_rgbas().ravel(), 6)), round(float(shape.get_stroke_width()), 6))


def rows(group):
    return [signature(m) for m in group.get_family() if len(m.points)]


def fresh(**kw):
    padding = V._raster_footprint(SCALE, 1.2 if kw.get("lit") is not None else .45)
    return draw._tower_drawing(at=(0, 0, 0), scale=SCALE, coverage_padding=padding, **OPTIONS, **kw)


began = time.time()
keys = [(slot.building, slot.tower, slot.storey, slot.facade, slot.item) for slot in V._model.slots() if slot.tower == 0 and slot.storey < 2]
pick = set(keys[::7])
paint = {k: "#AA3377" for k in keys[::3]}
cases = {
    "tint": dict(tint=lambda k: paint.get(k)),
    "types": dict(types=True),
    "lit": dict(lit=lambda k: k in pick),
    "lit+tint": dict(lit=lambda k: k in pick, tint=lambda k: paint.get(k)),
    "tint+lit-types": dict(lit=lambda k: k in pick, tint=lambda k: paint.get(k), types=True),
    "plain": dict(),
}
failed = 0
for name, kw in cases.items():
    expected = fresh(**kw)
    got = draw.tower_axon(at=(0, 0, 0), scale=SCALE, **OPTIONS, **kw)
    left, right = rows(expected), rows(got)
    ok = left == right
    if not ok:
        bad = [i for i, (a, b) in enumerate(zip(left, right)) if a != b]
        print(f"[equiv] {name}: counts {len(left)} {len(right)} differing {len(bad)} first {bad[:3]}", flush=True)
        if bad:
            print([[x == y for x, y in zip(left[bad[0]], right[bad[0]])]], left[bad[0]][1:], right[bad[0]][1:], flush=True)
    failed += not ok
    print(f"[equiv] {name}: {'OK' if ok else 'DIFFERENT'} shapes={len(rows(got))} t={time.time() - began:.0f}s cache={len(V._TOWERS)}", flush=True)
    del expected, got

flat = fresh(tint=lambda k: "#2255CC")
sources = draw.tower_capsules(keys[:6], "#2255CC", at=(0, 0, 0), scale=SCALE, **OPTIONS)
ok = all(rows(source) == rows(flat.capsules[k]) for source, k in zip(sources, keys[:6]))
failed += not ok
print(f"[equiv] tower_capsules: {'OK' if ok else 'DIFFERENT'}", flush=True)
print(f"[equiv] failed={failed} t={time.time() - began:.0f}s", flush=True)
