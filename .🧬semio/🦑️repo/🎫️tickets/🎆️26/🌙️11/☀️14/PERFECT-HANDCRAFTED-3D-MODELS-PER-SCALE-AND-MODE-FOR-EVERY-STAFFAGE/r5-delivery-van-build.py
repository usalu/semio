"""🏗️ Writes the round 5 source of vehicles/delivery-van (`volumes/vehicles/delivery-van.json`) from the hand-stated lists of every scale (`r5_delivery_van_s<N>`). Usage: python3 r5-delivery-van-build.py [50 100 200 500]"""
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
    module = importlib.import_module(f"r5_delivery_van_s{scale}")
    volume = {"status": "authored", "denominator": int(scale), "body": module.body(), "thermal": module.thermal(), "structure": module.structure(), "decisions": module.decisions()}
    for name in ("body", "thermal", "structure"):
        ids = [s["id"] for s in volume[name]]
        doubles = sorted({i for i in ids if ids.count(i) > 1})
        if doubles:
            raise SystemExit(f"{scale} {name}: duplicate ids {doubles}")
    current["volumes"][scale] = volume
    print(scale, {name: len(volume[name]) for name in ("body", "thermal", "structure")})
with open(SOURCE + ".r5.tmp", "w", encoding="utf-8") as handle:
    json.dump(current, handle, ensure_ascii=False)
os.replace(SOURCE + ".r5.tmp", SOURCE)
