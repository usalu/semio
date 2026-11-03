"""Deep sizes of the pipeline stages (recipes, restored items, shapes) of a small tower variant. Usage: xl4_pipeline.py [storeys]"""
import gc
import sys

import numpy as np

import xl4_common as C

visuals = C.load()
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 2
fit = C.program_fit()


def deep(value, seen):
    if id(value) in seen:
        return 0
    seen.add(id(value))
    size = sys.getsizeof(value)
    if isinstance(value, np.ndarray):
        return size + (0 if value.base is not None else value.nbytes)
    if isinstance(value, dict):
        size += sum(deep(k, seen) + deep(v, seen) for k, v in value.items())
    elif isinstance(value, (list, tuple, set, frozenset)):
        size += sum(deep(v, seen) for v in value)
    elif callable(value) and hasattr(value, "__closure__"):
        size += sum(deep(c.cell_contents, seen) for c in (value.__closure__ or ())) + deep(getattr(value, "__defaults__", None), seen) + deep(getattr(value, "__kwdefaults__", None), seen)
    elif hasattr(value, "__dict__"):
        size += deep(value.__dict__, seen)
    return size


captured = {}
original = visuals._section_order


def spy(items, view, *args, **kwargs):
    result = original(items, view, *args, **kwargs)
    captured["items"], captured["result"], captured["view"] = items, result, view
    return result


visuals._section_order = spy
original_remember = visuals._remember_order
visuals._remember_order = lambda key, order: captured.setdefault("order", order)
visuals._ORDER_MEMORY_ROWS = 0
draw = visuals.NakaginDrawing()
drawing = draw.tower_axon(**fit, types=False, towers=(0,), where=lambda s, n=storeys: s.tower == 0 and s.storey < n, core=False, podium=False, capital=False)
n = len(captured["result"])
seen = set()
sources = deep(captured["items"], seen)
rows = len(captured["order"])
recipes = deep(captured["order"], set())
restored = deep(captured["result"], set(id(x) for x in captured["items"]))
template = next(iter(visuals._TOWERS.values()))
shapes = [m for m in template.get_family()]
shape_bytes = deep(shapes, set())
print(f"[pipeline] source items={len(captured['items'])} {sources / 1e6:.1f}MB; recipes rows={rows} {recipes / 1e6:.1f}MB ({recipes / rows:.0f}B/row); restored items n={n} {restored / 1e6:.1f}MB ({restored / n:.0f}B/item); shapes={len(shapes)} {shape_bytes / 1e6:.1f}MB ({shape_bytes / len(shapes):.0f}B/shape)")
