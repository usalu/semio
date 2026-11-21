"""🏗️ Writes the round 5 source of vehicles/city-car (`volumes/vehicles/city-car.json`) from the hand-stated lists of every scale (`r5_city_car_s<N>` where it exists, else round 3). Usage: python3 r5-city-car-build.py [50 100 200 500]"""
import importlib
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
SOURCE = os.path.join(T, "volumes", "vehicles", "city-car.json")
scales = sys.argv[1:] or ["50", "100", "200", "500"]
current = json.load(open(SOURCE, encoding="utf-8"))
for scale in scales:
    name = f"r5_city_car_s{scale}" if importlib.util.find_spec(f"r5_city_car_s{scale}") else f"r3_city_car_s{scale}"
    module = importlib.import_module(name)
    volume = {"status": "authored", "denominator": int(scale), "body": module.body(), "thermal": module.thermal(), "structure": module.structure()}
    volume["decisions"] = module.decisions() if hasattr(module, "decisions") else current["volumes"][scale]["decisions"]
    for list_name in ("body", "thermal", "structure"):
        ids = [s["id"] for s in volume[list_name]]
        doubles = sorted({i for i in ids if ids.count(i) > 1})
        if doubles:
            raise SystemExit(f"{scale} {list_name}: duplicate ids {doubles}")
    current["volumes"][scale] = volume
    print(scale, name, {n: len(volume[n]) for n in ("body", "thermal", "structure")})
with open(SOURCE + ".r5.tmp", "w", encoding="utf-8") as handle:
    json.dump(current, handle, ensure_ascii=False)
os.replace(SOURCE + ".r5.tmp", SOURCE)
