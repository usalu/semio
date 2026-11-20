"""🏗️ Writes the round 3 source of vehicles/delivery-van (`volumes/vehicles/delivery-van.json`) from the hand-stated lists of every scale. Usage: python3 r3-delivery-van-build.py [50 100 200 500]"""
import importlib
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
SOURCE = os.path.join(T, "volumes", "vehicles", "delivery-van.json")
scales = sys.argv[1:] or ["50", "100", "200", "500"]
current = json.load(open(SOURCE, encoding="utf-8"))
for scale in scales:
    module = importlib.import_module(f"r3_delivery_van_s{scale}")
    volume = {"status": "authored", "denominator": int(scale), "body": module.body()}
    volume["thermal"] = module.thermal() if hasattr(module, "thermal") else current["volumes"][scale]["thermal"]
    volume["structure"] = module.structure() if hasattr(module, "structure") else current["volumes"][scale]["structure"]
    volume["decisions"] = module.decisions() if hasattr(module, "decisions") else current["volumes"][scale]["decisions"]
    for name in ("body", "thermal", "structure"):
        ids = [s["id"] for s in volume[name]]
        doubles = sorted({i for i in ids if ids.count(i) > 1})
        if doubles:
            raise SystemExit(f"{scale} {name}: duplicate ids {doubles}")
    current["volumes"][scale] = volume
    print(scale, {name: len(volume[name]) for name in ("body", "thermal", "structure")})
with open(SOURCE + ".r3.tmp", "w", encoding="utf-8") as handle:
    json.dump(current, handle, ensure_ascii=False)
os.replace(SOURCE + ".r3.tmp", SOURCE)
