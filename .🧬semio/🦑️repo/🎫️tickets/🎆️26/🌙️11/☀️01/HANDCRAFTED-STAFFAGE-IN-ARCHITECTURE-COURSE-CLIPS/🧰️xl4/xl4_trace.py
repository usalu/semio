"""Live bytes (tracemalloc) per face of the memory template, a deep-copy clone and a thaw. Usage: xl4_trace.py [storeys]"""
import gc
import sys
import tracemalloc

import numpy as np

import xl4_common as C

visuals = C.load()
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 2
fit = C.program_fit()
where = lambda s, n=storeys: s.tower == 0 and s.storey < n
options = dict(towers=(0,), where=where, core=False, podium=False, capital=False)
draw = visuals.NakaginDrawing()
visuals._FROZEN_SHAPES = 10**9
draw.tower_axon(**fit, types=False, **options)
gc.collect()
tracemalloc.start()
base = tracemalloc.get_traced_memory()[0]
template = next(iter(visuals._TOWERS.values()))
count = sum(1 for m in template.get_family() if isinstance(m, visuals._Facet))
clone = template.copy()
gc.collect()
copied = tracemalloc.get_traced_memory()[0] - base
print(f"[trace] faces={count} deepcopy clone {copied / count:.0f} B/face", flush=True)
del clone
gc.collect()
frozen = visuals._Frozen(template)
gc.collect()
base = tracemalloc.get_traced_memory()[0]
drawing = frozen.thaw(fit["scale"], np.asarray(fit["at"], float))
gc.collect()
thawed = tracemalloc.get_traced_memory()[0] - base
print(f"[trace] thaw {thawed / count:.0f} B/face; skeleton {len(frozen.skeleton) / 1e6:.1f}MB offsets {frozen.offsets.nbytes / 1e6:.1f}MB", flush=True)
sample = next(m for m in drawing.get_family() if isinstance(m, visuals._Facet) and hasattr(m, "world_vertices"))
rows = []
seen = set()


def deep(value):
    if id(value) in seen:
        return 0
    seen.add(id(value))
    size = sys.getsizeof(value)
    if isinstance(value, np.ndarray):
        return size + (0 if value.base is not None else value.nbytes)
    if isinstance(value, (list, tuple)):
        return size + sum(deep(v) for v in value)
    return size


for name, value in sample.__dict__.items():
    rows.append((deep(value), name))
print("[trace] sample", sys.getsizeof(sample), sys.getsizeof(sample.__dict__), sorted(rows, reverse=True))


def breakdown(root, label):
    seen_local, totals = set(), {}

    def size(value):
        if id(value) in seen_local:
            return 0
        seen_local.add(id(value))
        total = sys.getsizeof(value)
        if isinstance(value, np.ndarray):
            return total + value.nbytes
        if isinstance(value, (list, tuple)):
            return total + sum(size(v) for v in value)
        return total

    faces = [m for m in root.get_family() if isinstance(m, visuals._Facet)]
    for face in faces:
        totals["__dict__"] = totals.get("__dict__", 0) + sys.getsizeof(face.__dict__) + sys.getsizeof(face)
        for name, value in face.__dict__.items():
            totals[name] = totals.get(name, 0) + size(value)
    print(f"[trace] {label}: " + ", ".join(f"{k}={v / len(faces):.0f}" for k, v in sorted(totals.items(), key=lambda kv: -kv[1])[:9]), f"total={sum(totals.values()) / len(faces):.0f}", flush=True)


breakdown(drawing, "thaw")
breakdown(template.copy(), "deepcopy")
breakdown(template, "template")
