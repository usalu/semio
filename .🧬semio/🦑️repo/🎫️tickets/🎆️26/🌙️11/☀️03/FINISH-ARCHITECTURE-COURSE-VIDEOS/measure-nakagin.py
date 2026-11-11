"""🔬 Count conservative contiguous owner and paint runs in a cached painter order."""
import importlib
import json
import sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
module = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
view = module.make_view("axon", (0, 0, 0), 1, (0, 0, 0))
drawer = module.NakaginDrawing()
items = drawer._tower_items(lit=None, tint=None, types=False, capital=True, podium=True, where=None, core=True, ground=False, segments=2, capsules={}, cores={}, capitals={}, base=module.CAPSULE, towers=(0, 1), storeys=None, view=view, coverage_padding=0)
basis, towards = module._basis(view)
digest = module.hashlib.blake2b(digest_size=20)
digest.update(module.np.asarray(towards, float).tobytes())
digest.update(module.np.asarray(basis, float).tobytes())
for item in items:
    digest.update(module.np.asarray([item.tier], module.np.int64).tobytes())
    digest.update(module.np.asarray([len(item.verts), *(len(p) for p in item.polygons)], module.np.int64).tobytes())
    digest.update(module.np.asarray(item.verts, float).tobytes())
    digest.update(module.np.asarray(item.normal if item.normal is not None else [0, 0, 0], float).tobytes())
path = module._order_path(digest.digest())
order = json.loads(path.read_text(encoding="utf-8"))["order"]
keys = []
for item in items:
    shape = item.draw(view)
    keys.append((id(item.paint_owner), item.part_key, item.material, item.opacity, tuple(shape.fill_rgbas.flatten()), tuple(shape.stroke_rgbas.flatten()), float(shape.get_stroke_width())))
last = None
runs = 0
for row in order:
    source = row if isinstance(row, int) else row["source"]
    if source >= len(keys):
        raise ValueError(f"Cache source {source} exceeds {len(keys)} generated items")
    key = (keys[source], isinstance(row, dict))
    if key != last:
        runs += 1
    last = key
print(json.dumps({"cache": str(path), "items": len(items), "entries": len(order), "runs": runs, "reduction": len(order)-runs}))
