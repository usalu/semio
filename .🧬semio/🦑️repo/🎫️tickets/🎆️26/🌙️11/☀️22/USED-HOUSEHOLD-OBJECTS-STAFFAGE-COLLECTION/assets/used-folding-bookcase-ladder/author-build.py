"""🏗️ The one authoritative build of `used-folding-bookcase-ladder`: rig, anchors, the four scales (2D and 3D from the same parts), poses, clips, thermal and structural models; writes `🔣️.json` next to this file.

`python3 author-build.py [--report]` (`--report` prints masses, areas and the reference measurements).

@see ./author-geometry.py — the hand-chosen numbers and the kinematics
@see ./author-parts.py — the parts per scale
@see ./author-text.py — every bilingual text"""
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


g = _load("geometry", "author-geometry.py")
P = _load("parts", "author-parts.py")
X = _load("text", "author-text.py")
T = X.T
SCALES = [50, 100, 200, 500]
REGION_OF_BONE_MASS = {"ladder", "rear"} | {f"shelf-{i + 1}" for i in range(5)}


def masses():
    """⚖️ Mass and bone-local centre of mass of every loaded bone from the 1:50 solids (section x length x density); blanket and wall anchor carry none."""
    acc = {}
    for part in P.parts(50):
        if not part["solid3d"] or part["bone"] not in REGION_OF_BONE_MASS:
            continue
        vol, (cu, cv, _) = P.volume_of(part)
        m = vol * g.DENSITY[part["material"]]
        a = acc.setdefault(part["bone"], [0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * cu
        a[2] += m * cv
    return {b: (a[0], a[1] / a[0], a[2] / a[0]) for b, a in acc.items()}


MASS = masses()


def com_world(offsets):
    """🎯️ World centre of mass of the shelf (all loaded bones) for bone offsets."""
    f = g.frames(offsets)
    m = sx = sy = 0.0
    for bone, (kg, cu, cv) in MASS.items():
        x, y = g.to_world(f[bone], cu, cv)
        m += kg
        sx += kg * x
        sy += kg * y
    return m, sx / m, sy / m


CARRIED = g.carried_offsets()
_, COM_CX, COM_CY = com_world(CARRIED)
GRIP_MID = (COM_CX - g.CARRY_FOOT[0]) / math.cos(math.radians(g.CARRY_ANGLE))
GRIP_U = (round(GRIP_MID - 0.30, 4), round(GRIP_MID + 0.30, 4))
RAIL_ZC = round((g.RAIL_W[0] + g.RAIL_W[1]) / 2, 4)


def bones():
    """🦴️ The rig bones with titles, masses and member sections."""
    titles = X.BONE_TITLES
    sections = {
        "ladder": {"areaM2": 2 * 0.044 * 0.028, "secondMomentM4": 2 * 0.028 * 0.044 ** 3 / 12, "extremeFibreM": 0.022, "material": "pine-aged", "title": T("Two pine rails 44 x 28 mm", "Zwei Kiefernholme 44 x 28 mm")},
        "rear": {"areaM2": 2 * 0.034 * 0.028, "secondMomentM4": 2 * 0.028 * 0.034 ** 3 / 12, "extremeFibreM": 0.017, "material": "pine-aged", "title": T("Two pine rear rails 34 x 28 mm", "Zwei Kiefern-Hinterholme 34 x 28 mm")},
    }
    for i in range(5):
        sections[f"shelf-{i + 1}"] = {"areaM2": 0.536 * 0.018, "secondMomentM4": 0.536 * 0.018 ** 3 / 12, "extremeFibreM": 0.009, "material": "pine-board", "title": T("Pine panel 536 x 18 mm", "Kiefernplatte 536 x 18 mm")}
    out = []
    for b in g.BONES:
        bone = {"id": b["id"], "parent": b["parent"], "title": titles[b["id"]], "x": round(b["x"], 6), "y": round(b["y"], 6), "angleDeg": round(b["angle"], 4), "lengthM": round(b["length"], 6), "joint": b["joint"]}
        if b["id"] in MASS:
            kg, cu, cv = MASS[b["id"]]
            bone["mass"] = {"kg": round(kg, 3), "comX": round(cu, 4), "comY": round(cv, 4), "distribution": "point", "source": "authored-estimate", "reference": "din-68364-2003"}
        if b["id"] in sections:
            s = dict(sections[b["id"]])
            s["areaM2"] = round(s["areaM2"], 7)
            s["secondMomentM4"] = float(f"{s['secondMomentM4']:.4e}")
            bone["section"] = s
        out.append(bone)
    return out


def anchors():
    """📍️ Feet, the wall contacts of the folded package, the hand grips, the wall fixing and the pull point."""
    t = X.ANCHOR_TITLES
    top = g.PIVOT_Y - g.REAR_TOP
    vb = -(g.PIVOT_X - g.REAR_X[0])
    vm = -(g.PIVOT_X - (g.REAR_X[0] + g.REAR_X[1]) / 2)
    out = []
    for side, z in (("l", RAIL_ZC), ("r", -RAIL_ZC)):
        out.append({"id": f"foot-front-{side}", "bone": "ladder", "x": 0.0, "y": 0.0, "z": z, "kind": "contact", "title": t[f"foot-front-{side}"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.0446, "footprintWidthM": 0.028, "frictionCoefficient": 0.4})
    for side, z in (("l", RAIL_ZC), ("r", -RAIL_ZC)):
        out.append({"id": f"foot-rear-{side}", "bone": "rear", "x": g.PIVOT_Y, "y": round(vm, 5), "z": z, "kind": "contact", "title": t[f"foot-rear-{side}"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.034, "footprintWidthM": 0.028, "frictionCoefficient": 0.4})
    for side, z in (("l", RAIL_ZC), ("r", -RAIL_ZC)):
        out.append({"id": f"rest-wall-{side}", "bone": "rear", "x": round(top, 5), "y": round(vb, 5), "z": z, "kind": "contact", "title": t[f"rest-wall-{side}"], "support": "contact", "normalDeg": 0, "footprintLengthM": 0.01, "footprintWidthM": 0.028, "frictionCoefficient": 0.4})
    out.append({"id": "grip-r", "bone": "ladder", "x": GRIP_U[0], "y": 0.0, "z": -RAIL_ZC, "kind": "contact", "title": t["grip-r"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.028, "frictionCoefficient": 0.8})
    out.append({"id": "grip-l", "bone": "ladder", "x": GRIP_U[1], "y": 0.0, "z": RAIL_ZC, "kind": "contact", "title": t["grip-l"], "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.028, "frictionCoefficient": 0.8})
    out.append({"id": "wall-fixing", "bone": "wall-anchor", "x": 0.03, "y": 0.0, "z": 0.258, "kind": "support", "title": t["wall-fixing"], "support": "roller", "normalDeg": 0})
    out.append({"id": "pull-point", "bone": "shelf-5", "x": -g.SHELF_FRONT, "y": -g.SHELF_T / 2, "z": 0.258, "kind": "load", "title": t["pull-point"]})
    return out


def shape(part, view, order):
    """✏️ One drawn shape of a part in one viewpoint: the bone-local profile, in front and plan carried as a lateral extrusion of its span."""
    s = {"id": part["id"], "bone": part["bone"], "role": part["role"], "z": order, "filled": part["filled"], "tone": part["tone"], "penMm": part["pen"]}
    if part["region"]:
        s["region"] = part["region"]
    if part["filled"]:
        s["material"] = part["material"]
    s["kind"] = "poly"
    s["points"] = part["profile"]
    s["closed"] = True
    if view != "elevation":
        s["solid"] = {"kind": "extruded", "sweep": "w", "z0": round(part["z"][0], 5), "z1": round(part["z"][1], 5)}
    return s


def order_key(part, view):
    """🗂️ Paint order at rest: nearer to the viewer later (elevation from the right, front from ahead, plan from above)."""
    f = g.REST[part["bone"]]
    pts = [g.to_world(f, u, v) for u, v in part["profile"]]
    if view == "elevation":
        return -part["z"][0]
    if view == "front":
        return max(x for x, _ in pts)
    return max(y for _, y in pts)


def extra_2d(scale):
    """🧵️ Drawing-only texture of the 1:50 plan: the quilting seams of the moving blanket."""
    if scale != 50:
        return {"elevation": [], "front": [], "plan": []}
    seams = []
    for k, x in enumerate([0.10, 0.20, 0.30, 0.40]):
        seams.append({"id": f"blanket-seam-{k + 1}", "bone": "blanket", "role": "texture", "z": 50, "filled": False, "tone": 0.2, "penMm": 0.13, "region": "blanket", "kind": "poly", "points": [[round(g.BLANKET_X + x, 4), -0.43], [round(g.BLANKET_X + x, 4), 0.43]], "closed": False})
    return {"elevation": [], "front": [], "plan": seams}


def representation(scale):
    """🗺️ The authored representation of one scale."""
    geometry = {}
    plist = P.parts(scale)
    for view in ("elevation", "front", "plan"):
        chosen = [p for p in plist if view in p["views"]]
        ranked = sorted(chosen, key=lambda p: order_key(p, view))
        geometry[view] = [shape(p, view, round(1 + k * 0.1, 2)) for k, p in enumerate(ranked)] + extra_2d(scale)[view]
    band = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": band[0], "maxDenominator": band[1]}, "pens": pens, "hatches": [], "geometry": geometry, "decisions": X.REP_DECISIONS[scale]}


def solid(part):
    """🧊️ The body solid of a part: its profile pushed laterally over its span."""
    s = {"id": part["id"], "kind": "extrude", "bone": part["bone"], "material": part["material"], "role": "detail" if part["role"] == "detail" else "body", "tone": part["tone"], "profile": part["profile"], "plane": "xy", "from": round(part["z"][0], 5), "to": round(part["z"][1], 5)}
    for flag, on in part["flags"].items():
        if on:
            s[flag] = True
    return s


def rod(id_, bone, u, v, z0, z1, r, member, part, material="steel-zinc"):
    """🔩️ A round rod along the lateral axis: a hinge axis or a pin line of the structural list."""
    return {"id": id_, "kind": "extrude", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "profile": P.polygon_circle(u, v, r, 12), "plane": "xy", "from": z0, "to": z1, "slender": True, "member": member, "part": part}


def volume(scale):
    """🧊️ The handcrafted volume of one scale: body, thermal references, structural idealisation."""
    plist = [p for p in P.parts(scale) if p["solid3d"]]
    body = [solid(p) for p in plist]
    thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in plist if p["region"] and p["material"] != "steel-zinc"]
    structure = []
    for p in plist:
        if p["id"].startswith("front-rail"):
            structure.append({"id": f"m-{p['id']}", "of": p["id"], "member": "ladder", "part": "member"})
        elif p["id"].startswith("rear-rail"):
            structure.append({"id": f"m-{p['id']}", "of": p["id"], "member": "rear", "part": "member"})
        elif p["id"].startswith("shelf-"):
            structure.append({"id": f"m-{p['id']}", "of": p["id"], "member": p["bone"], "part": "member"})
    r = {50: 0.004, 100: 0.005, 200: 0.006, 500: 0.0075}[scale]
    structure.append(rod("j-hinge", "ladder", g.LADDER_LEN, 0.0, -0.30, 0.30, r, "rear", "joint"))
    for i in range(5):
        structure.append(rod(f"j-shelf-{i + 1}", f"shelf-{i + 1}", 0.0, 0.0, -0.286, 0.286, r, f"shelf-{i + 1}", "joint"))
    rm = round(P.FEATURE[scale] * 0.8 + 0.006, 4)
    for b in ("ladder", "rear"):
        kg, cu, cv = MASS[b]
        structure.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": "pine-aged", "role": "detail", "tone": 0.3, "radii": [rm, rm, rm], "transform": {"translation": [round(cu, 4), round(cv, 4), 0.0]}, "member": b, "part": "mass"})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": X.VOL_DECISIONS[scale]}


def region_areas():
    """🌡️ Outer area of every thermal region at 1:50 (prism surfaces less the faces two solids of one region share)."""
    plist = [p for p in P.parts(50) if p["solid3d"] and p["region"] and p["material"] != "steel-zinc"]
    out = {}
    for p in plist:
        out[p["region"]] = out.get(p["region"], 0.0) + P.outer_area(p)
    cleat_face = 0.034 * g.CLEAT_H
    out["rear-rails"] -= 10 * 2 * cleat_face
    chip = next(p for p in plist if p["id"] == "shelf-2-chip")
    out["shelves"] -= 2 * g.poly_area_centroid(chip["profile"])[0]
    return out


AREAS = region_areas()


def heat_capacity(region):
    """♨️ Areal heat capacity of a region: its solid volume over its outer area times density and specific heat."""
    mat = {m["id"]: m for m in X.MATERIALS}
    vol = cap = 0.0
    for p in P.parts(50):
        if p["solid3d"] and p["region"] == region and p["material"] != "steel-zinc":
            v, _ = P.volume_of(p)
            vol += v
            cap += v * mat[p["material"]]["densityKgM3"] * mat[p["material"]]["specificHeatJKgK"]
    return round(cap / AREAS[region])


def thermal():
    """🌡️ The passive thermal model: four regions of aged pine (with the beech cleats and the pine anchor block), pine panel and felt."""
    regions = []
    for spec in X.REGIONS:
        r = dict(spec)
        r["areaM2"] = round(AREAS[r["id"]], 3)
        r["heatCapacityJM2K"] = heat_capacity(r["id"])
        regions.append(r)
    return {"environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.15, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions, "emitters": [], "assumptions": X.THERMAL_ASSUMPTIONS}


def binding(kind):
    """🌬️ The region binding of a pose family."""
    table = {
        "stored": {"front-rails": (1.0, 0.0), "rear-rails": (1.0, 0.0), "shelves": (1.0, 0.0), "blanket": (1.0, 0.0)},
        "carried": {"front-rails": (0.9, 0.0), "rear-rails": (1.0, 0.0), "shelves": (1.0, 0.0), "blanket": (1.0, 0.0)},
        "in-use": {"front-rails": (1.0, 0.15), "rear-rails": (1.0, 0.0), "shelves": (0.55, 0.2), "blanket": (1.0, 0.05)},
        "empty": {"front-rails": (1.0, 0.15), "rear-rails": (1.0, 0.0), "shelves": (1.0, 0.25), "blanket": (1.0, 0.05)},
    }[kind]
    return {"regions": {k: {"exposedFraction": a, "sunlitFraction": b} for k, (a, b) in table.items()}, "emitters": {}}


SHELF_LOADS_KG = [22.0, 15.0, 8.5, 5.0, 1.5]


def shelf_loads():
    """📚️ Books and potted plants on the five shelves as uniformly distributed loads along each shelf (world newtons per metre)."""
    out = []
    for i, kg in enumerate(SHELF_LOADS_KG):
        length = g.shelf_end_u(i)
        w = kg * g.G / length
        out.append({"id": f"load-shelf-{i + 1}", "kind": "imposed", "title": X.LOAD_TITLES[i], "distributed": {"bone": f"shelf-{i + 1}", "fromM": 0.0, "toM": round(length, 4), "wxNPerM": 0.0, "wyNPerM": round(-w, 2)}, "fxN": 0.0, "fyN": 0.0, "source": "authored-estimate", "reference": "en-1991-1-1-2002"})
    return out


def round_offsets(offsets):
    """🔢️ Offsets rounded for the document."""
    return {b: {k: round(v, 4) for k, v in o.items()} for b, o in offsets.items()}


FOLDED_HOME = g.folded_home_offsets()


def poses():
    """📥️ Stored, carried, in use, opened empty, the pull test and the key poses of the clips."""
    feet = ["foot-front-l", "foot-front-r"]
    rear_feet = ["foot-rear-l", "foot-rear-r"]
    d = X.POSE_TEXT
    return [
        {"id": "stored", "title": d["stored"][0], "description": d["stored"][1], "bones": round_offsets(g.STORED), "ground": "authored", "contacts": feet + ["rest-wall-l", "rest-wall-r"], "thermal": binding("stored"), "loads": []},
        {"id": "carried", "title": d["carried"][0], "description": d["carried"][1], "bones": round_offsets(CARRIED), "ground": "authored", "contacts": ["grip-r", "grip-l"], "thermal": binding("carried"), "loads": []},
        {"id": "in-use", "title": d["in-use"][0], "description": d["in-use"][1], "bones": {}, "ground": "authored", "contacts": feet + rear_feet + ["wall-fixing"], "thermal": binding("in-use"), "loads": shelf_loads()},
        {"id": "opened-empty", "title": d["opened-empty"][0], "description": d["opened-empty"][1], "bones": {}, "ground": "authored", "contacts": feet + rear_feet + ["wall-fixing"], "thermal": binding("empty"), "loads": []},
        {"id": "pull-test", "title": d["pull-test"][0], "description": d["pull-test"][1], "bones": {}, "ground": "authored", "contacts": feet + rear_feet + ["wall-fixing"], "thermal": binding("empty"), "loads": [{"id": "pull-forward", "kind": "live", "title": T("Forward pull of 50 N at the left end of the top shelf", "Zug von 50 N nach vorn am linken Ende des obersten Bodens"), "anchor": "pull-point", "fxN": 50.0, "fyN": 0.0, "source": "authored-estimate", "reference": "en-14749-2016"}]},
        {"id": "folded-home", "key": True, "title": d["folded-home"][0], "description": d["folded-home"][1], "bones": round_offsets(FOLDED_HOME), "ground": "authored", "contacts": feet + ["rest-wall-l", "rest-wall-r"], "thermal": binding("empty"), "loads": []},
    ]


def lerp_offsets(a, b, t):
    """🎚️ Linear blend of two offset sets."""
    out = {}
    for bone in set(a) | set(b):
        keys = set(a.get(bone, {})) | set(b.get(bone, {}))
        out[bone] = {k: round(a.get(bone, {}).get(k, 0.0) * (1 - t) + b.get(bone, {}).get(k, 0.0) * t, 4) for k in keys}
    return out


def clips():
    """🎬️ `carry` (stored to carried) and `unfold` (folded at home to open, the shelves dropping one by one)."""
    lifted = {k: dict(v) for k, v in g.STORED.items()}
    lifted["lift"] = {"dy": 0.07}
    lifted["slide"] = {"dx": round(g.STORED_SLIDE - 0.02, 4)}
    lifted["ladder"] = {"angleDeg": -4.0}
    carry = {
        "id": "carry", "title": X.CLIP_TEXT["carry"][0], "description": X.CLIP_TEXT["carry"][1], "durationS": 4.0, "fps": 25, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0.0, "pose": "stored", "ease": "smooth"}, {"t": 0.8, "pose": "stored", "ease": "smooth"}, {"t": 1.6, "pose": "stored", "ease": "smooth", "bones": round_offsets(lifted)}, {"t": 4.0, "pose": "carried", "ease": "smooth"}],
        "contacts": [{"anchor": "foot-front-l", "fromS": 0.0, "toS": 1.0}, {"anchor": "foot-front-r", "fromS": 0.0, "toS": 1.0}, {"anchor": "rest-wall-l", "fromS": 0.0, "toS": 0.5}, {"anchor": "rest-wall-r", "fromS": 0.0, "toS": 0.5}, {"anchor": "grip-r", "fromS": 0.4, "toS": 4.0}, {"anchor": "grip-l", "fromS": 0.4, "toS": 4.0}],
        "phases": [{"id": "grip", "fromS": 0.0, "toS": 0.8, "title": T("Gripping the folded shelf", "Das zusammengeklappte Regal greifen")}, {"id": "lift-off", "fromS": 0.8, "toS": 1.6, "title": T("Lifting it off the blanket", "Von der Decke heben")}, {"id": "tilt-and-carry", "fromS": 1.6, "toS": 4.0, "title": T("Tilting it to carry", "Zum Tragen kippen")}],
        "events": [{"t": 0.4, "kind": "contact-on", "title": T("Both hands on the rails", "Beide Hände an den Holmen"), "target": "grip-r"}, {"t": 0.5, "kind": "contact-off", "title": T("Off the depot wall", "Weg von der Lagerwand"), "target": "rest-wall-l"}, {"t": 1.0, "kind": "contact-off", "title": T("Feet leave the blanket; blanket and anchor block stay behind", "Füße verlassen die Decke; Decke und Ankerklotz bleiben liegen"), "target": "foot-front-l"}],
        "tracks": [],
    }
    keys = [{"t": 0.0, "pose": "folded-home", "ease": "smooth"}]
    open_rear = {k: dict(v) for k, v in FOLDED_HOME.items()}
    open_rear["rear"] = {"angleDeg": 0.0}
    open_rear["slide"] = {"dx": 0.0}
    keys.append({"t": 1.5, "pose": "folded-home", "ease": "smooth", "bones": round_offsets(open_rear)})
    state = {k: dict(v) for k, v in open_rear.items()}
    t = 1.5
    for i in [4, 3, 2, 1, 0]:
        t = round(t + 0.8, 2)
        state[f"shelf-{i + 1}"] = {"angleDeg": 0.0}
        keys.append({"t": t, "pose": "folded-home", "ease": "smooth", "bones": round_offsets(state)})
    keys.append({"t": 6.0, "pose": "opened-empty", "ease": "smooth"})
    unfold = {
        "id": "unfold", "title": X.CLIP_TEXT["unfold"][0], "description": X.CLIP_TEXT["unfold"][1], "durationS": 6.0, "fps": 25, "loop": False, "rootMotion": "none", "keys": keys,
        "contacts": [{"anchor": "foot-front-l", "fromS": 0.0, "toS": 6.0}, {"anchor": "foot-front-r", "fromS": 0.0, "toS": 6.0}, {"anchor": "rest-wall-l", "fromS": 0.0, "toS": 1.4}, {"anchor": "rest-wall-r", "fromS": 0.0, "toS": 1.4}, {"anchor": "foot-rear-l", "fromS": 1.4, "toS": 6.0}, {"anchor": "foot-rear-r", "fromS": 1.4, "toS": 6.0}, {"anchor": "wall-fixing", "fromS": 1.4, "toS": 6.0}],
        "phases": [{"id": "open-rear-frame", "fromS": 0.0, "toS": 1.5, "title": T("Swinging the rear frame out to the wall", "Hinterrahmen zur Wand ausschwenken")}, {"id": "drop-shelves", "fromS": 1.5, "toS": 5.5, "title": T("Folding the shelves down from the top", "Böden von oben nach unten herunterklappen")}, {"id": "settle", "fromS": 5.5, "toS": 6.0, "title": T("Open and fixed to the wall", "Offen und an der Wand gesichert")}],
        "events": [{"t": 1.4, "kind": "contact-on", "title": T("Rear feet on the floor, rear rails screwed to the anchor block", "Hinterfüße am Boden, Hinterholme am Ankerklotz verschraubt"), "target": "wall-fixing"}] + [{"t": round(1.5 + 0.8 * (k + 1), 2), "kind": "motion", "title": T(f"Shelf {5 - k} rests on its cleats", f"Boden {5 - k} liegt auf seinen Leisten"), "target": f"shelf-{5 - k}"} for k in range(5)],
        "tracks": [],
    }
    return [carry, unfold]


def asset():
    """🏺️ The whole asset document."""
    total = round(sum(m[0] for m in MASS.values()), 2)
    reps = {f"{s}": representation(s) for s in SCALES}
    vols = {f"{s}": volume(s) for s in SCALES}
    a = {
        "id": "used-folding-bookcase-ladder", "category": "furniture", "title": T("Used Ladder Shelf", "Gebrauchtes Leiterregal"), "summary": X.SUMMARY,
        "tags": ["used", "household", "second-hand", "living", "furniture", "shelf", "ladder-shelf", "folding", "bookcase", "pine", "solid-wood", "storage", "interior", "flat-clearance", "anti-tip"],
        "provenance": X.PROVENANCE, "dimensions": DIMENSIONS, "materials": X.MATERIALS, "rig": {"kind": "mechanical", "root": "ground", "bones": bones()}, "anchors": anchors(),
        "representations": reps, "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal(),
        "structure": {"gravityMS2": g.G, "dynamics": "quasi-static", "supports": [], "assumptions": X.STRUCTURE_ASSUMPTIONS, "clinical": False},
        "volumes": vols, "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored",
    }
    a["thermal"]["regions"][0]["ratedPowerW"] = 0
    a["thermal"]["regions"][0]["ratedPowerBasis"] = "passive"
    assert abs(total - DIMENSIONS["massKg"]) < 0.051, (total, DIMENSIONS["massKg"])
    return a


def stored_extent():
    """📦️ World extent of every 1:50 body solid in the stored pose (x ahead, y up, z lateral)."""
    f = g.frames(g.STORED)
    xs, ys, zs = [], [], []
    for p in P.parts(50):
        if not p["solid3d"] or p["flags"].get("texture"):
            continue
        for u, v in p["profile"]:
            x, y = g.to_world(f[p["bone"]], u, v)
            xs.append(x)
            ys.append(y)
        zs += list(p["z"])
    return (min(xs), max(xs)), (min(ys), max(ys)), (min(zs), max(zs))


EXT = stored_extent()
DIMENSIONS = {"heightM": round(EXT[1][1] - EXT[1][0], 3), "widthM": round(EXT[0][1] - EXT[0][0], 3), "depthM": round(EXT[2][1] - EXT[2][0], 3), "massKg": round(sum(m[0] for m in MASS.values()), 2)}


if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    if "--report" in sys.argv:
        print("theta", round(g.THETA, 3), "ladder", round(g.LADDER_LEN, 4))
        for b, (kg, cu, cv) in MASS.items():
            print("mass", b, round(kg, 3), round(cu, 4), round(cv, 4))
        print("total", DIMENSIONS)
        print("extent stored", EXT)
        print("areas", {k: round(v, 4) for k, v in AREAS.items()})
        print("grip u", GRIP_U, "com carried", round(COM_CX, 4), round(COM_CY, 4))
        for i in range(5):
            print("shelf", i + 1, "top", g.SHELF_TOPS[i], "depth", round(g.shelf_depth(i), 4), "pin u", round(g.shelf_pin_u(i), 4))
        f = g.frames({})
        print("in-use COM", com_world({}))
