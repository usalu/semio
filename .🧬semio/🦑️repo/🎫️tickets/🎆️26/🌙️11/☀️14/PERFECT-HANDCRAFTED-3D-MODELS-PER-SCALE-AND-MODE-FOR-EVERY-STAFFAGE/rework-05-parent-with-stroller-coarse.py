"""🪚 Coarse scales of `humans/parent-with-stroller` modelled for their own smallest feature (15 / 30 / 75 mm): fist instead of fingers, merged hand skeleton, caster fork as one strut, wheels as discs, tubes at the feature width.

Usage: python3 rework-05-parent-with-stroller-coarse.py   (edits the candidate 🗑️generated/rework-parent-with-stroller/pws-cand/🔣️.json in place; run once after the girdle script)
"""
import copy
import json

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CANDIDATE = T + "/🗑️generated/rework-parent-with-stroller/pws-cand/🔣️.json"
SIDES = ("l", "r")
TUBES = ("push-bar", "rear-leg-l", "rear-leg-r", "front-leg-l", "front-leg-r", "rear-axle", "bumper-bar")
FINGERS = [f"{part}-{s}{n}" for s in SIDES for part, n in (("palm", ""), ("finger", "-1"), ("finger", "-2"), ("finger", "-3"), ("finger", "-4"), ("thumb", ""))]
HAND_BONES = [f"{part}-{s}{n}" for s in SIDES for part, n in (("carpus", ""), ("metacarpal", "-1"), ("metacarpal", "-2"), ("metacarpal", "-3"), ("metacarpal", "-4"), ("phalanges", "-1"), ("phalanges", "-2"), ("phalanges", "-3"), ("phalanges", "-4"), ("thumb-ray", ""))]
FIST = {"200": [(-0.006, 0, 0.028, 0.019), (0.03, 0.004, 0.036, 0.024), (0.07, 0.008, 0.04, 0.03), (0.1, 0.008, 0.04, 0.028), (0.109, 0.008, 0.03, 0.018)], "500": [(-0.006, 0, 0.03, 0.024), (0.03, 0.004, 0.037, 0.027), (0.07, 0.008, 0.04, 0.03), (0.1, 0.008, 0.04, 0.028), (0.109, 0.008, 0.032, 0.024)]}
PLATE = {"200": 0.015, "500": 0.024}
TUBE = {"200": 0.015, "500": 0.0195}


def index(solids):
    return {s["id"]: s for s in solids}


def drop(solids, identifiers):
    return [s for s in solids if s["id"] not in identifiers]


def radius(solid, value):
    if solid["kind"] == "capsule":
        solid["r0"] = solid["r1"] = value
    elif solid["kind"] == "ellipsoid":
        solid["radii"] = [max(r, value) for r in solid["radii"]]
    else:
        for section in solid["sections"]:
            section["rx"], section["ry"] = max(section["rx"], value), max(section["ry"], value)


def fist(palm, scale, suffix):
    template = palm["sections"][0]
    sections = [{**template, "at": [x, y, 0], "rx": rx, "ry": ry} for x, y, rx, ry in FIST[scale]]
    return {**palm, "id": "hand-" + suffix, "sections": sections}


def plate(suffix, scale):
    thickness = PLATE[scale]
    sections = [{"at": [x, y, 0], "rx": rx, "ry": thickness, "exponent": 2.4, "rotation": [0, -0.70711, 0, 0.70711]} for x, y, rx in ((0.004, 0.003, 0.026), (0.06, 0.02, 0.036), (0.1, 0.03, 0.036))]
    return {"id": "hand-bones-" + suffix, "kind": "loft", "bone": "hand-" + suffix, "material": "bone", "role": "body", "tone": 0.93, "capStart": True, "capEnd": True, "sections": sections, "member": "hand-" + suffix, "part": "member"}


def strut(swivel, suffix, value):
    z = swivel["p0"][2]
    return {**swivel, "id": "fork-" + suffix, "p0": [0.826, -0.8, z], "p1": [0.8, -0.9204, z], "r0": value, "r1": value}


def disc(tyre, identifier, profile):
    return {**tyre, "id": identifier, "profile": profile}


def thermal(lists, regions):
    body = index(lists["body"])
    kept = [s for s in lists["thermal"] if "of" not in s or s["of"] in body]
    known = {s.get("of") for s in kept}
    for identifier, region in regions.items():
        if identifier in body and identifier not in known:
            kept.append({"id": "th-" + identifier, "of": identifier, "region": region})
    lists["thermal"] = kept


def carried(lists, merged):
    body, seen, kept = index(lists["body"]), set(), []
    for solid in lists["structure"]:
        target = solid.get("of")
        if target is None or target in body:
            kept.append(solid)
            continue
        target = next(new for old, new in merged if target.startswith(old)) + target[-2:]
        if target not in seen:
            seen.add(target)
            kept.append({**solid, "id": "m-" + target, "of": target})
    lists["structure"] = kept


def stretch(solid, by):
    first, last = solid["sections"][0]["at"], solid["sections"][-1]["at"]
    length = sum((b - a) ** 2 for a, b in zip(first, last)) ** 0.5
    step = [(b - a) / length * by for a, b in zip(first, last)]
    solid["sections"][0]["at"] = [round(a - d, 5) for a, d in zip(first, step)]
    solid["sections"][-1]["at"] = [round(b + d, 5) for b, d in zip(last, step)]


def hands(lists, scale):
    body = index(lists["body"])
    made = [fist(copy.deepcopy(body["palm-" + s]), scale, s) for s in SIDES]
    at = [s["id"] for s in lists["body"]].index("palm-l")
    lists["body"] = drop(lists["body"][:at] + made + lists["body"][at:], FINGERS)
    structure = lists["structure"]
    at = [s["id"] for s in structure].index("carpus-l")
    lists["structure"] = drop(structure[:at] + [plate(s, scale) for s in SIDES] + structure[at:], HAND_BONES)
    for s in SIDES:
        index(lists["structure"])["joint-wrist-" + s]["radii"] = [PLATE[scale], PLATE[scale], 0.024]


def frame(lists, scale):
    body = index(lists["body"])
    for identifier in TUBES:
        radius(body[identifier], TUBE[scale])
    forks = [strut(copy.deepcopy(body["swivel-" + s]), s, TUBE[scale]) for s in SIDES]
    at = [s["id"] for s in lists["body"]].index("fork-crown-l")
    gone = [f"fork-{part}-{s}" for s in SIDES for part in ("crown", "prong-a", "prong-b")]
    lists["body"] = drop(lists["body"][:at] + forks + lists["body"][at:], gone)


def scale50(lists):
    body, structure = index(lists["body"]), index(lists["structure"])
    for s in SIDES:
        z0, z1 = body["fork-prong-b-" + s]["p1"][2], body["fork-prong-a-" + s]["p1"][2]
        axle = {**copy.deepcopy(body["fork-prong-a-" + s]), "id": "axle-front-" + s, "p0": [0.8, -0.9204, z0], "p1": [0.8, -0.9204, z1], "r0": 0.004, "r1": 0.004}
        lists["body"].append(axle)
        lists["structure"].append({**structure["m-fork-prong-a-" + s], "id": "m-axle-front-" + s, "of": "axle-front-" + s})
    thermal(lists, {"axle-front-" + s: "stroller-frame" for s in SIDES})


def scale100(lists):
    body, structure = index(lists["body"]), index(lists["structure"])
    for s in SIDES:
        body["fork-crown-" + s]["size"] = [0.034, 0.016, 0.068]
        radius(body["fork-prong-a-" + s], 0.0075)
        radius(body["fork-prong-b-" + s], 0.0075)
        body["spokes-rear-" + s]["size"] = [0.094, 0.016, 0.016]
        body["spokes-front-" + s]["size"] = [0.036, 0.015, 0.015]
        for name in ("child-tibia-", "child-humerus-", "child-forearm-", "child-foot-bones-", "thumb-ray-"):
            structure[name + s]["slender"] = True
        radius(structure["child-hand-bones-" + s], 0.0075)
        for n in "1234":
            structure[f"metacarpal-{s}-{n}"]["slender"] = True
            structure[f"phalanges-{s}-{n}"]["slender"] = True


def scale200(lists):
    hands(lists, "200")
    frame(lists, "200")
    body = index(lists["body"])
    made = []
    for s in SIDES:
        made.append(disc(copy.deepcopy(body["rim-rear-" + s]), "web-rear-" + s, [[-0.015, 0.02], [-0.015, 0.115], [0.015, 0.115], [0.015, 0.02], [-0.015, 0.02]]))
        made.append(disc(copy.deepcopy(body["rim-front-" + s]), "disc-front-" + s, [[-0.016, 0], [-0.016, 0.058], [0.016, 0.058], [0.016, 0]]))
    lists["body"] = drop(lists["body"], [f"{part}-{s}" for s in SIDES for part in ("spokes-rear", "spokes-front", "rim-front", "hub-front")]) + made
    structure = index(lists["structure"])
    for s in SIDES:
        for name in ("child-femur-", "child-tibia-", "child-humerus-", "child-forearm-"):
            structure[name + s]["slender"] = True
    radius(structure["child-cervical-column"], 0.015)
    radius(structure["child-atlas"], 0.015)
    lists["structure"] = drop(lists["structure"], [f"{part}-{s}" for s in SIDES for part in ("occipital-condyle", "child-foot-bones", "child-hand-bones")])
    carried(lists, (("fork-", "fork"), ("spokes-rear", "web-rear"), ("rim-front", "disc-front"), ("hub-front", "disc-front"), ("spokes-front", "disc-front")))
    thermal(lists, {**{"hand-" + s: "hand-skin" for s in SIDES}, **{"fork-" + s: "stroller-frame" for s in SIDES}, **{f"{part}-{s}": "tyres" for s in SIDES for part in ("web-rear", "disc-front")}})


def scale500(lists):
    hands(lists, "500")
    frame(lists, "500")
    body = index(lists["body"])
    for s in SIDES:
        radius(body["child-arm-" + s], 0.0375)
        radius(body["child-arm-" + s + "-distal"], 0.0375)
        radius(body["child-boot-" + s], 0.0375)
        radius(body["swivel-" + s], 0.0195)
    radius(body["child-exposed-neck"], 0.0375)
    radius(body["handle-grip"], 0.0195)
    made = []
    for s in SIDES:
        made.append(disc(copy.deepcopy(body["tyre-rear-" + s]), "wheel-rear-" + s, [[-0.023, 0], [-0.023, 0.15], [-0.014, 0.158], [0, 0.1592], [0.014, 0.158], [0.023, 0.15], [0.023, 0]]))
        made.append(disc(copy.deepcopy(body["tyre-front-" + s]), "wheel-front-" + s, [[-0.017, 0], [-0.017, 0.073], [-0.009, 0.079], [0, 0.0796], [0.009, 0.079], [0.017, 0.073], [0.017, 0]]))
    gone = [f"{part}-{end}-{s}" for s in SIDES for end in ("rear", "front") for part in ("tyre", "rim", "hub", "spokes")] + ["child-hand-l", "child-hand-r"]
    lists["body"] = drop(lists["body"], gone) + made
    structure = index(lists["structure"])
    radius(structure["joint-neck"], 0.0375)
    radius(structure["child-cervical-column"], 0.0375)
    radius(structure["child-atlas"], 0.0375)
    for s in SIDES:
        structure["child-femur-" + s]["slender"] = True
        for name in ("child-tibia-", "child-humerus-", "child-forearm-"):
            radius(structure[name + s], 0.0375)
    lists["structure"] = drop(lists["structure"], [f"{part}-{s}" for s in SIDES for part in ("occipital-condyle", "child-foot-bones", "child-hand-bones")])
    stretch(body["child-exposed-neck"], 0.015)
    stretch(structure["child-cervical-column"], 0.015)
    carried(lists, (("fork-", "fork"), ("tyre-rear", "wheel-rear"), ("rim-rear", "wheel-rear"), ("hub-rear", "wheel-rear"), ("spokes-rear", "wheel-rear"), ("tyre-front", "wheel-front"), ("rim-front", "wheel-front"), ("hub-front", "wheel-front"), ("spokes-front", "wheel-front")))
    thermal(lists, {**{"hand-" + s: "hand-skin" for s in SIDES}, **{"fork-" + s: "stroller-frame" for s in SIDES}, **{f"wheel-{end}-{s}": "tyres" for s in SIDES for end in ("rear", "front")}})


SLOPE = {
    "50": [(0.197, 0.47), (0.198, 0.497), (0.184, 0.516), (0.156, 0.534), (0.126, 0.552), (0.1, 0.571), (0.084, 0.588), (0.0632, 0.6011), (0.0154, 0.6049)],
    "100": [(0.198, 0.48), (0.19, 0.51), (0.126, 0.552), (0.084, 0.588), (0.0596, 0.6021)],
    "200": [(0.198, 0.485), (0.185, 0.515), (0.1, 0.571), (0.0615, 0.6099)],
    "500": [(0.195, 0.49), (0.1, 0.571), (0.0531, 0.6094)],
}


def front(asset):
    for scale, slope in SLOPE.items():
        shape = next(s for s in asset["representations"][scale]["geometry"]["front"] if s["id"] == "torso")
        points = shape["points"]
        start = next(i for i, p in enumerate(points) if p[1] > 0.44)
        low = [p for p in points[start:] + points[:start] if p[1] <= 0.44]
        shape["points"] = low + [[x, y] for x, y in slope] + [[-x, y] for x, y in reversed(slope)]
    coarse = asset["representations"]["500"]["geometry"]
    for view in ("front", "plan", "elevation"):
        for shape in coarse[view]:
            if shape["id"] in ("upper-arm-l", "upper-arm-r"):
                elbow = {(0.362, 0.041): [0.362, 0.052], (0.3579, -0.04): [0.3579, -0.052], (0.366, 0.0187): [0.366, 0.03]}
                shape["points"] = [elbow.get(tuple(p), p) for p in shape["points"]]
    for view in ("front", "plan"):
        for shape in coarse[view]:
            if shape["id"] in ("upper-arm-l", "upper-arm-r"):
                shape["points"] = [[x, round(r + 0.006, 4) if r else r] for x, r in shape["points"]]
    for solid in asset["volumes"]["500"]["body"]:
        if solid["id"] in ("sleeve-upper-l", "sleeve-upper-r"):
            solid["sections"][-1]["rx"], solid["sections"][-1]["ry"] = 0.05, 0.052
            for section in solid["sections"]:
                section["rx"], section["ry"] = round(section["rx"] + 0.006, 4), round(section["ry"] + 0.006, 4)


document = json.load(open(CANDIDATE, encoding="utf-8"))
front(document["asset"])
model = document["asset"]["volumes"]
if any(s["id"] == "hand-l" for s in model["500"]["body"]):
    raise SystemExit("already applied")
scale50(model["50"])
scale100(model["100"])
scale200(model["200"])
scale500(model["500"])
json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False)
for scale in ("50", "100", "200", "500"):
    print(scale, {name: len(model[scale][name]) for name in ("body", "thermal", "structure")})
