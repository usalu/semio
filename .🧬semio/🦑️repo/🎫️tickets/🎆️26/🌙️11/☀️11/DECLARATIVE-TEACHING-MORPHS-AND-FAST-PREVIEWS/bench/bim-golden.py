"""Record sha256 digests of cheap BIM model queries. Usage: python bim-golden.py <out.json>. Run on the unmodified model for the golden file, again after the change."""
import hashlib
import importlib
import json
import sys
import time

api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
out = sys.argv[1]
model = api.load_model()
column = api.role("column_example", model)
by_class = {}
for item in model["elements"]:
    by_class.setdefault(item.get("class"), []).append(item["tag"])
plan = [("quantities", column), ("quantities", "STAIR-PA"), ("quantities", "RIB-CELLAR"), ("quantities", "ROOFLIGHT-1"), ("quantities", by_class["IfcSlab"][0]),
        ("solid_geometry", "ROOFLIGHT-1"), ("solid_geometry", "RIB-CELLAR"), ("solid_geometry", "GL-GARAGE-1"),
        ("service_geometry", by_class["IfcSpaceHeater"][0]), ("service_geometry", by_class["IfcSanitaryTerminal"][0])]
cases = []
for call, arg in plan:
    start = time.perf_counter()
    value = getattr(api, call)(arg)
    digest = hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":"), default=repr).encode("utf-8")).hexdigest()
    cases.append({"id": f"{call}.{arg}", "call": call, "arg": arg, "digest": digest})
    print("[DEBUG]", call, arg, round((time.perf_counter() - start) * 1000), "ms", flush=True)
    with open(out, "w", encoding="utf-8") as handle:
        json.dump({"queries": cases}, handle, indent=2, ensure_ascii=False)
