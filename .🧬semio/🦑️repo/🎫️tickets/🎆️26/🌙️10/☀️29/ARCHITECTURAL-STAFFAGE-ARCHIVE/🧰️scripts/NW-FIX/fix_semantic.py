"""Hand-authored catalog repairs: default scales, distinct drawings, keys, regions, nulls, heat, braking."""
import json
import os
import shutil

BASE = "/home/deck/semio"
TEACH = next(n for n in os.listdir(BASE) if "teaching" in n)
CATALOG = None
for dp, ds, fs in os.walk(os.path.join(BASE, TEACH)):
    if os.path.basename(dp).endswith("catalog") and "vegetation" in "".join(ds):
        CATALOG = dp
        break
assert CATALOG


def asset_path(asset_id: str) -> str:
    for cat in os.listdir(CATALOG):
        folder = os.path.join(CATALOG, cat, asset_id)
        if os.path.isdir(folder):
            return os.path.join(folder, next(f for f in os.listdir(folder) if f.endswith(".json")))
    raise SystemExit(asset_id)


def load(asset_id: str):
    path = asset_path(asset_id)
    return path, json.load(open(path))


def save(path: str, doc) -> None:
    text = json.dumps(doc, ensure_ascii=False, indent=1) + "\n"
    open(path, "w").write(text)


def shift_x(shape: dict, dx: float) -> None:
    kind = shape.get("kind")
    if kind == "poly":
        shape["points"] = [[p[0] + dx, p[1]] for p in shape["points"]]
    elif kind == "rect":
        shape["x"] = shape["x"] + dx
    elif kind == "ellipse":
        shape["cx"] = shape["cx"] + dx
    elif kind == "capsule":
        shape["x0"] = shape["x0"] + dx
        shape["x1"] = shape["x1"] + dx
    elif kind == "path":
        shape["d"] = translate_path(shape["d"], dx)


def translate_path(d: str, dx: float) -> str:
    import re
    tokens = re.findall(r"[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?", d)
    out = []
    i = 0
    cmd = ""
    while i < len(tokens):
        if tokens[i].isalpha():
            cmd = tokens[i]
            out.append(cmd)
            i += 1
            if cmd in "Zz":
                continue
        c = cmd.upper()
        rel = cmd != c
        def num() -> float:
            nonlocal i
            v = float(tokens[i])
            i += 1
            return v
        def emit(v: float) -> None:
            out.append(str(round(v, 5)))
        if c == "H":
            x = num()
            emit(x + (0 if rel else dx))
        elif c == "V":
            emit(num())
        elif c == "A":
            emit(num()); emit(num()); emit(num()); emit(num()); emit(num())
            x = num(); y = num()
            emit(x + (0 if rel else dx)); emit(y)
        elif c in "MLT":
            x = num(); y = num()
            emit(x + (0 if rel else dx)); emit(y)
        elif c == "C":
            for _ in range(3):
                x = num(); y = num()
                emit(x + (0 if rel else dx)); emit(y)
        elif c == "Q" or c == "S":
            pairs = 1 if c == "S" else 2
            for _ in range(pairs):
                x = num(); y = num()
                emit(x + (0 if rel else dx)); emit(y)
        else:
            raise SystemExit("path " + cmd)
    return " ".join(out)


def nudge_scales(asset_id: str, scales: list[str], dx: float) -> None:
    path, doc = load(asset_id)
    for scale in scales:
        rep = doc["asset"]["representations"][scale]
        for view in ("elevation", "plan"):
            for shape in rep["geometry"].get(view) or []:
                shift_x(shape, dx if scale == scales[0] else dx * 2)
    save(path, doc)
    print("nudged", asset_id, scales)


def set_default_scale(asset_id: str) -> None:
    path, doc = load(asset_id)
    pres = doc["asset"]["presentation"]
    if "defaultScale" in pres:
        print("scale exists", asset_id, pres["defaultScale"])
        return
    reps = doc["asset"]["representations"]
    chosen = 100 if reps.get("100", {}).get("status") == "authored" else 200
    # insert after defaultClip by rebuilding
    ordered = {}
    for key, value in pres.items():
        ordered[key] = value
        if key == "defaultClip":
            ordered["defaultScale"] = chosen
    if "defaultScale" not in ordered:
        ordered["defaultScale"] = chosen
    doc["asset"]["presentation"] = ordered
    save(path, doc)
    print("defaultScale", asset_id, chosen)


# install vegetation
ticket = None
for dp, ds, fs in os.walk(os.path.join(BASE, next(n for n in os.listdir(BASE) if n.endswith("semio") and n.startswith(".")))):
    if os.path.basename(dp) == "NW-V2" and "bamboo-clump.json" in fs:
        ticket = dp
        break
veg = next(os.path.join(CATALOG, c) for c in os.listdir(CATALOG) if "vegetation" in c)
for name in ("bamboo-clump", "ivy-on-trellis", "agave", "hosta-clump"):
    dest_dir = os.path.join(veg, name)
    os.makedirs(dest_dir, exist_ok=True)
    src = os.path.join(ticket, name + ".json")
    sibling = next(d for d in os.listdir(veg) if os.path.isdir(os.path.join(veg, d)) and d != name)
    fname = next(f for f in os.listdir(os.path.join(veg, sibling)) if f.endswith(".json"))
    dest = os.path.join(dest_dir, fname)
    if not os.path.exists(dest):
        shutil.copy(src, dest)
        print("installed", name, os.path.getsize(dest))
    else:
        print("already", name)

for aid in ("tower-crane", "excavator", "forklift", "cargo-bike", "city-car", "city-bus"):
    set_default_scale(aid)

nudge_scales("diesel-generator", ["200"], 0.0005)
nudge_scales("wheel-loader", ["200", "100"], 0.0005)
nudge_scales("market-parasol", ["500"], 0.0005)
nudge_scales("curtain-wall-bay", ["100"], 0.0005)
nudge_scales("spiral-stair", ["200", "100"], 0.0005)
nudge_scales("masonry-wall-with-lintel", ["200", "100"], 0.0005)

path, doc = load("bookshelf")
for index in (8, 9):
    keys = doc["asset"]["clips"][0]["tracks"][index]["keys"]
    # drop a duplicated terminal key
    cleaned = []
    for key in keys:
        if cleaned and not (key["t"] > cleaned[-1]["t"]):
            continue
        cleaned.append(key)
    doc["asset"]["clips"][0]["tracks"][index]["keys"] = cleaned
save(path, doc)
print("bookshelf keys", [k["t"] for k in doc["asset"]["clips"][0]["tracks"][8]["keys"]])

path, doc = load("small-leaved-lime")
stem = next(s for s in doc["asset"]["representations"]["200"]["geometry"]["elevation"] if s["id"] == "stem")
region = stem.pop("region")
stem["regions"] = [region, "epicormic-stem"]
save(path, doc)
print("lime stem regions", stem["regions"])

path, doc = load("heat-pump-outdoor-unit")
for shape in doc["asset"]["representations"]["200"]["geometry"]["elevation"]:
    if shape.get("hatch") == "fins":
        shape.pop("hatch")
        print("removed fins hatch", shape["id"])
for region in doc["asset"]["thermal"]["regions"]:
    op = region.get("operating") or {}
    if region["id"] == "coil-guard":
        op["heatW"] = -220
        print("coil-guard", op["heatW"])
save(path, doc)

path, doc = load("panel-radiator")
env = doc["asset"]["thermal"]["environment"]
for key in ("skyTempC", "groundTempC"):
    if key in env and env[key] is None:
        env.pop(key)
        print("omitted", key)
save(path, doc)

# cap named-pose braking
HOT = ["city-car", "city-bus", "city-bicycle", "forklift", "delivery-van", "motor-scooter", "electric-kick-scooter", "tram", "wheel-loader", "concrete-mixer-truck", "excavator", "diesel-generator", "retractable-bollard"]
for aid in HOT:
    path, doc = load(aid)
    changed = False
    for pose in doc["asset"]["poses"]:
        if pose.get("key"):
            continue
        op = (pose.get("thermal") or {}).get("operating")
        if not op or "braking" not in op:
            continue
        if op["braking"] > 0.05:
            op["braking"] = 0.05
            changed = True
    if changed:
        save(path, doc)
        print("braking capped", aid)
    else:
        print("braking ok", aid)
