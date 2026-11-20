"""🏗️ Writes the round 3 source of vehicles/city-car (`volumes/vehicles/city-car.json`) from the hand-stated lists of every scale. Usage: python3 r3-city-car-build.py [50 100 200 500]"""
import importlib
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "vehicles", "city-car.json")
scales = sys.argv[1:] or ["50", "100", "200", "500"]
current = json.load(open(SOURCE, encoding="utf-8"))
for scale in scales:
    module = importlib.import_module(f"r3_city_car_s{scale}")
    volume = {"status": "authored", "denominator": int(scale), "body": module.body()}
    volume["thermal"] = module.thermal() if hasattr(module, "thermal") else [{"id": "th-hull", "of": "hull", "region": "body-panels"}]
    volume["structure"] = module.structure() if hasattr(module, "structure") else [{"id": "st-hull", "of": "hull", "member": "chassis", "part": "member"}]
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
