"""Memory cost of one drawn shape. Usage: xl3_mem.py"""
import importlib
import os
import sys
import tracemalloc

import numpy as np

sys.path.insert(0, os.getcwd())
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
poly = [np.array([[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]], float)]
tracemalloc.start()
before = tracemalloc.get_traced_memory()[0]
shapes = [V._shape(poly, "#808080", 1, "#404040", .45) for _ in range(2000)]
after = tracemalloc.get_traced_memory()[0]
print(f"[mem] per shape {(after - before) / 2000:.0f} B", flush=True)
shape = shapes[0]
print({k: (getattr(v, "nbytes", None) or sys.getsizeof(v)) for k, v in shape.__dict__.items()}, flush=True)
before = tracemalloc.get_traced_memory()[0]
copies = [s.copy() for s in shapes[:500]]
print(f"[mem] per copy {(tracemalloc.get_traced_memory()[0] - before) / 500:.0f} B", flush=True)
snapshot = tracemalloc.take_snapshot()
for stat in snapshot.statistics("lineno")[:8]:
    print(stat, flush=True)
