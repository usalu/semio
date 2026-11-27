"""🎚️ Simulates the budget cap of the roundness rule from the asset files alone (the triangle counts of `estimate.py`): the caps of the four scales fall together in steps (step k of 64 down to 24: cap = max(24, floor(k * standard / 64)); then every cap j of 23 down to 12) until the unique authored triangles of all scales stay within M and every list of every scale within L, then a coarser list that has more triangles than the same list one scale finer lowers the cap of its scale. Prints the assets below the standard caps. capsim.py <M> <L> [ids…]"""
import glob, json, os, sys

source = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "estimate.py"), encoding="utf-8").read().split("rows = []")[0]
ns = {"__file__": os.path.abspath(__file__)}
exec(source, ns)
M, L = int(sys.argv[1]), int(sys.argv[2])
STANDARD = {50: 64, 100: 48, 200: 40, 500: 32}
STEPS = [{s: max(24, (k * c) // 64) for s, c in STANDARD.items()} for k in range(64, 23, -1)] + [{s: j for s in STANDARD} for j in range(23, 11, -1)]
touched = budget = falling = 0
for path in sorted(glob.glob(os.path.join(ns["CATALOG"], "*", "*", "🔣️.json"))):
    name = os.path.basename(os.path.dirname(path))
    if len(sys.argv) > 3 and name not in sys.argv[3:]:
        continue
    asset = json.load(open(path, encoding="utf-8"))["asset"]
    largest = max(asset["dimensions"][k] for k in ("heightM", "widthM", "depthM"))
    memo = {}

    def lists(scale, cap):
        if (scale, cap) not in memo:
            volume = asset["volumes"][str(scale)]
            quality = ns["quality_of"](largest, scale)
            quality["max"] = min(quality["max"], cap)
            body = {s["id"]: s for s in volume["body"]}
            own = sum(ns["triangles"](s, quality) for mode in ("body", "thermal", "structure") for s in volume[mode] if "kind" in s)
            memo[(scale, cap)] = (own, {mode: sum(ns["triangles"](s if "kind" in s else body[s["of"]], quality) for s in volume[mode]) for mode in ("body", "thermal", "structure")})
        return memo[(scale, cap)]

    def over(caps):
        return sum(lists(s, c)[0] for s, c in caps.items()) > M or any(n > L for s, c in caps.items() for n in lists(s, c)[1].values())

    step = 0
    while step < len(STEPS) - 1 and over(STEPS[step]):
        step += 1
    caps = dict(STEPS[step])
    fell = []
    scales = sorted(STANDARD)
    for finer, scale in zip(scales, scales[1:]):
        while caps[scale] > 12 and any(lists(scale, caps[scale])[1][m] > lists(finer, caps[finer])[1][m] for m in ("body", "thermal", "structure")):
            caps[scale] -= 1
        if caps[scale] < STEPS[step][scale]:
            fell.append(scale)
    left = [f"{m} 1:{scale}" for finer, scale in zip(scales, scales[1:]) for m in ("body", "thermal", "structure") if lists(scale, caps[scale])[1][m] > lists(finer, caps[finer])[1][m]]
    if step > 0 or fell or len(sys.argv) > 3:
        touched += 1
        budget += step > 0
        falling += bool(fell)
        before = sum(lists(s, c)[0] for s, c in STANDARD.items())
        after = sum(lists(s, c)[0] for s, c in caps.items())
        print(f"{asset['category']:11} {name:34} caps {list(caps.values())} unique {before:8} -> {after:7} 1:50 {list(lists(50, 64)[1].values())} -> {list(lists(50, caps[50])[1].values())}{' falling ' + str(fell) if fell else ''}{' OVER BUDGET' if over(caps) else ''}{' STILL GROWS ' + ', '.join(left) if left else ''}")
print(touched, "assets below the standard caps:", budget, "by the budget,", falling, "by the falling rule")
