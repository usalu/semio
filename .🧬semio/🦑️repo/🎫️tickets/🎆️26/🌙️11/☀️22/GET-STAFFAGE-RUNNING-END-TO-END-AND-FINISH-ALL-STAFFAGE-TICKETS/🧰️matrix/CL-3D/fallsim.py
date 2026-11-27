"""🪂️ Simulates the staged roundness levels from the asset files alone, with the independent oracle `stage/budget.py`: the level of the model (all scales together within the triangle budget), then per coarser scale the level lowered until no list of the scale has more nominal triangles than the same list one scale finer. Prints the assets below the full level and what stays over. fallsim.py <modelTriangles> [ids…]"""
import glob, json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "stage", "budget.py"), encoding="utf-8").read())
CATALOG = os.path.join(HERE, *[".."] * 9, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
RULE = {"paperFeatureMm": 0.15, "chordPerFeature": 0.02, "largestPerChord": 3200, "roundSegments": 24, "minSegments": 12, "maxSegments": {"20": 96, "50": 64, "100": 48, "200": 40, "500": 32}, "fullLevel": 64, "modelTriangles": int(sys.argv[1])}
counts = {"model": 0, "falling": 0, "over": 0, "floor": 0}
for path in sorted(glob.glob(os.path.join(CATALOG, "*", "*", "🔣️.json"))):
    name = os.path.basename(os.path.dirname(path))
    if len(sys.argv) > 2 and name not in sys.argv[2:]:
        continue
    asset = json.load(open(path, encoding="utf-8"))["asset"]
    volumes = {k: v for k, v in asset["volumes"].items() if v.get("status") == "authored"}
    result = budget({"largestM": max(asset["dimensions"][k] for k in ("heightM", "widthM", "depthM")), "volumes": volumes}, RULE)
    levels = [result["levels"][k] for k in sorted(volumes, key=int)]
    total = sum(result["triangles"].values())
    falling = any(level < result["model"] for level in levels)
    counts["model"] += result["model"] < 64
    counts["falling"] += falling
    counts["over"] += total > RULE["modelTriangles"]
    counts["floor"] += min(levels) < 24
    if result["model"] < 64 or falling or len(sys.argv) > 2:
        print(f"{asset['category']:11} {name:36} model {result['model']:2} levels {levels} caps {[result['segments'][k] for k in sorted(volumes, key=int)]} triangles {result['fullTriangles']:8} -> {total:7}{' OVER' if total > RULE['modelTriangles'] else ''}")
print(counts)
