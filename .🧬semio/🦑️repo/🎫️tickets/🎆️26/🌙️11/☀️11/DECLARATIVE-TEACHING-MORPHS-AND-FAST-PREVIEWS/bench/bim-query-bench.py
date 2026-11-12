"""Time BIM model query paths (cold and warm) and record result digests. Usage: python bim-query-bench.py <label> <out.json>

Digests are sha256 over canonical JSON of the query results; the before and after runs must agree digest for digest (model semantics unchanged).
"""
import hashlib
import importlib
import json
import sys
import time

api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
label, out = sys.argv[1], sys.argv[2]
result = {"label": label, "ms": {}, "digests": {}}


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":"), default=repr).encode("utf-8")).hexdigest()


def clock(name, fn, repeat=1):
    print("[DEBUG] start", name, flush=True)
    start = time.perf_counter()
    for _ in range(repeat):
        value = fn()
    took = (time.perf_counter() - start) / repeat * 1000
    result["ms"][name] = round(took, 3)
    result["digests"][name] = digest(value)
    print("[DEBUG] took ms", name, took, flush=True)
    with open(out, "w", encoding="utf-8") as handle:
        json.dump(result, handle, indent=1)
    return value


model = api.load_model()
clock("load_model", api.load_model, 3)
clock("context", lambda: cache.canonical_context(api.geometry_producers()) is not None, 1)
by_class = {}
for item in model["elements"]:
    by_class.setdefault(item.get("class"), []).append(item["tag"])
column = api.role("column_example", model)
clock("quantities_cold", lambda: api.quantities(column))
clock("quantities_warm", lambda: api.quantities(column), 5)
for kind, name in (("IfcFlowTerminal", "service_geometry"), ("IfcBuildingElementProxy", "solid_geometry"), ("IfcWall", "wall_geometry")):
    tags = by_class.get(kind) or []
    if not tags:
        continue
    fn = getattr(api, name)
    clock(name + "_cold", lambda: fn(tags[0]))
    clock(name + "_warm", lambda: fn(tags[0]), 5)
clock("revision_warm", api.geometry_revision, 5)
