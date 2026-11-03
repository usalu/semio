"""Time the exact-occlusion phases of the first whole-tower build with the built-in progress hook. Usage: xl2_progress.py"""
import importlib
import os
import sys
import time

sys.path.insert(0, os.getcwd())
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
draw = V.NakaginDrawing()
began = time.time()
view = V.make_view("axon", (0, 0, 0), 1, (0, 0, 0))
capsules, cores, capitals = {}, {}, {}
items = draw._tower_items(lit=None, tint=None, types=False, capital=True, podium=True, where=None, core=True, ground=False, segments=2,
                          capsules=capsules, cores=cores, capitals=capitals, base=V.CAPSULE, towers=(0, 1), storeys=None, view=view, coverage_padding=V._raster_footprint(1, .45))
print(f"[prog] items={len(items)} built in {time.time() - began:.0f}s", flush=True)
state = {"phase": None, "stamp": time.time()}


def progress(row):
    if row["phase"] != state["phase"] or time.time() - state["stamp"] > 120:
        print(f"[prog] t={time.time() - began:.0f}s {row['phase']} {row['completed']}/{row['total']}", flush=True)
        state.update(phase=row["phase"], stamp=time.time())


v = V.make_view("axon", (0, 0, 0), 1, (V.np.vstack([i.verts for i in items]).min(axis=0) + V.np.vstack([i.verts for i in items]).max(axis=0)) / 2)
order = V._section_order(items, v, progress=progress)
print(f"[prog] done t={time.time() - began:.0f}s order={len(order)}", flush=True)
