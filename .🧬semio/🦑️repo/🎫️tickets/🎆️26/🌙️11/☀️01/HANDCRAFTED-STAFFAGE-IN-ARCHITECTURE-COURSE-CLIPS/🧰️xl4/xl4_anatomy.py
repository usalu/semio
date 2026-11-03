"""Memory anatomy of bare manim VMobjects and of drawn shapes. Usage: xl4_anatomy.py"""
import gc
import sys

import numpy as np

import xl4_common as C

visuals = C.load()
manim = visuals._engine


def deep(value, seen):
    if id(value) in seen:
        return 0
    seen.add(id(value))
    size = sys.getsizeof(value)
    if isinstance(value, np.ndarray):
        return size if value.base is None else sys.getsizeof(value)
    if isinstance(value, dict):
        size += sum(deep(k, seen) + deep(v, seen) for k, v in value.items())
    elif isinstance(value, (list, tuple, set)):
        size += sum(deep(v, seen) for v in value)
    elif hasattr(value, "__dict__"):
        size += deep(value.__dict__, seen)
    return size


sample = visuals._shape([np.array([[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]], float)], "#ffffff", 1.0, "#000000", 0.45)
seen = {id(manim.config)}
print("instance", sys.getsizeof(sample), "dict", sys.getsizeof(sample.__dict__))
rows = []
for name, value in sample.__dict__.items():
    rows.append((deep(value, set(seen)), name, type(value).__name__))
for size, name, kind in sorted(rows, reverse=True)[:25]:
    print(f"{size:6d} {name} {kind}")
print("total", sum(r[0] for r in rows))

C.say("before")
gc.collect()
before = C.rss()
shapes = [visuals._shape([np.array([[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]], float) + i * 1e-3], "#ffffff", 1.0, "#000000", 0.45) for i in range(20000)]
gc.collect()
print("bytes per bare quad shape", (C.rss() - before) / 20000)
copies = [s.copy() for s in shapes[:5000]]
gc.collect()
print("bytes per copy", (C.rss() - before) / 20000 - 0, "after copies total")
