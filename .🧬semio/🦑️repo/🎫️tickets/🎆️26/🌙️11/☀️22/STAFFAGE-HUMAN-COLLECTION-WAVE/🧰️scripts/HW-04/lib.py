"""🧰️ HW-04 authoring kit: derives a new construction person from the catalog figure `worker-carrying-box` (rig, outlines and volumes of a stocky 1.741 m man in hi-vis vest, hard hat and safety boots), without its carton, at the stature and build of the new person; and states tool parts once per scale as bone-local profiles that become the 2D shapes of all three views (revolved or extruded solids) and the matching 3D solids.

The base figure is read from its catalog place, never copied. Everything a new asset adds is stated in its own generator (`shovel.py`, `jackhammer.py`, `surveyor.py`).
"""
import copy
import json
import math
import os
import subprocess

R = "/Users/ueli/Documents/semio"
S = R + "/🎓️teaching/🏛️architecture/🧍️staffage"
CATALOG = S + "/🗂️catalog/👤️humans"
T = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
HERE = T + "/🧰️scripts/HW-04"
GEN = T + "/🗑️generated/HW-04"
BUN = R + "/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
BASE = "worker-carrying-box"
SCALES = ("50", "100", "200", "500")
BOX_BONES = {"box-l", "box-r"}


def tx(en, de):
    return {"en": en, "de": de}


def load_base():
    return json.load(open(CATALOG + "/" + BASE + "/🔣️.json", encoding="utf-8"))


def r5(v):
    return round(v, 5)


# ---------------------------------------------------------------- strip the carton

def strip_box(a):
    """📦️ Removes the carton: its two half bones, anchor, material, thermal region, shapes and solids (with every `of` reference to them), poses and clips."""
    a["rig"]["bones"] = [b for b in a["rig"]["bones"] if b["id"] not in BOX_BONES]
    a["anchors"] = [x for x in a["anchors"] if x["id"] != "box-base"]
    a["materials"] = [m for m in a["materials"] if m["id"] != "cardboard"]
    a["thermal"]["regions"] = [r for r in a["thermal"]["regions"] if r["id"] != "box-surface"]
    for rep in a["representations"].values():
        for view, shapes in rep["geometry"].items():
            rep["geometry"][view] = [s for s in shapes if s.get("bone") not in BOX_BONES]
    for vol in a["volumes"].values():
        gone = set()
        for lst in ("body", "thermal", "structure"):
            keep = []
            for s in vol[lst]:
                if s.get("bone") in BOX_BONES or s.get("member") in BOX_BONES or s.get("of") in gone or s.get("region") == "box-surface":
                    gone.add(s["id"])
                    continue
                keep.append(s)
            vol[lst] = keep
    a["poses"] = []
    a["clips"] = []


def drop_parts(a, ids):
    """✂️ Removes shapes and body solids by id at every scale, with the thermal and structure solids that refer to them or carry their id."""
    ids = set(ids)
    for rep in a["representations"].values():
        for view, shapes in rep["geometry"].items():
            rep["geometry"][view] = [s for s in shapes if s["id"] not in ids]
    for vol in a["volumes"].values():
        gone = set(i for i in ids)
        for lst in ("body", "thermal", "structure"):
            vol[lst] = [s for s in vol[lst] if not (s["id"] in gone or s.get("of") in gone or s["id"] in {"th-" + i for i in ids} or any(s["id"].startswith("th-" + i + "-") for i in ids))]


# ---------------------------------------------------------------- uniform scale of the figure

VOLUME_LENGTHS = {"size", "radii", "p0", "p1", "r0", "r1", "profile", "holes", "from", "to", "at", "rx", "ry", "cornerRadius", "origin"}


HALF = {"r0", "r1", "rx", "ry", "radii"}


def _keep(v, s, f, half):
    """A cross-section extent scaled by s, but never pushed below the smallest feature f of its scale when it was at or above it."""
    n = v * s
    full = 2 * v if half else v
    if f and s < 1 and full >= f and (2 * n if half else n) < f:
        n = (f / 2 if half else f) * 1.002
    return r5(n)


def _scale_tree(o, s, keys, f=0.0):
    if isinstance(o, dict):
        if f and s < 1 and "from" in o and "to" in o and isinstance(o["from"], (int, float)):
            t = o["to"] - o["from"]
            m = (o["to"] + o["from"]) / 2 * s
            if t >= f and t * s < f:
                o["from"], o["to"] = r5(m - f * 0.501), r5(m + f * 0.501)
                for k, v in list(o.items()):
                    if k not in ("from", "to"):
                        _scale_tree(v, s, keys, f) if not k in keys else o.__setitem__(k, _mul(v, s))
                return
        for k, v in list(o.items()):
            if k in HALF and f:
                o[k] = [_keep(x, s, f, True) for x in v] if isinstance(v, list) else _keep(v, s, f, True)
            elif k == "size" and f:
                o[k] = [_keep(x, s, f, False) for x in v]
            elif k in keys:
                o[k] = _mul(v, s)
            elif k == "transform" and isinstance(v, dict) and "translation" in v:
                v["translation"] = _mul(v["translation"], s)
            elif k == "repeat" and isinstance(v, dict):
                for kk in ("translation", "origin"):
                    if kk in v:
                        v[kk] = _mul(v[kk], s)
            else:
                _scale_tree(v, s, keys, f)
    elif isinstance(o, list):
        for x in o:
            _scale_tree(x, s, keys, f)


def _mul(v, s):
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return r5(v * s)
    if isinstance(v, list):
        return [_mul(x, s) for x in v]
    return v


def scale_figure(a, s):
    """📏️ Scales the whole figure by `s` about the rig root: bones, anchors, every 2D shape and solid placement, every volume solid, thermal lengths and the breathing motion."""
    for b in a["rig"]["bones"]:
        for k in ("x", "y", "z", "lengthM"):
            if k in b:
                b[k] = r5(b[k] * s)
        if "mass" in b:
            for k in ("comX", "comY"):
                if k in b["mass"]:
                    b["mass"][k] = r5(b["mass"][k] * s)
        j = b.get("joint", {})
        for k in ("minM", "maxM"):
            if k in j:
                j[k] = r5(j[k] * s)
    for x in a["anchors"]:
        for k in ("x", "y", "z", "footprintLengthM", "footprintWidthM"):
            if k in x:
                x[k] = r5(x[k] * s)
    for rep in a["representations"].values():
        for shapes in rep["geometry"].values():
            for sh in shapes:
                sh["points"] = [[r5(p[0] * s), r5(p[1] * s)] for p in sh["points"]]
                so = sh.get("solid")
                if so:
                    for k in ("axisX", "axisZ", "z0", "z1"):
                        if k in so:
                            so[k] = r5(so[k] * s)
                    if "zTip" in so:
                        so["zTip"] = [r5(z * s) for z in so["zTip"]]
    for key, vol in a["volumes"].items():
        for lst in ("body", "thermal", "structure"):
            _scale_tree(vol[lst], s, VOLUME_LENGTHS, 0.00015 * int(key))
    for r in a["thermal"]["regions"]:
        if "characteristicLengthM" in r:
            r["characteristicLengthM"] = r5(r["characteristicLengthM"] * s)
    for m in a["thermal"]["respiration"].get("motion", []):
        if "dy" in m:
            m["dy"] = r5(m["dy"] * s)


# ---------------------------------------------------------------- anthropometry

DE_LEVA = {
    "male": {"trunk": 0.4346, "head": 0.0694, "upper-arm": 0.0271, "forearm": 0.0162, "hand": 0.0061, "thigh": 0.1416, "shank": 0.0433, "foot": 0.0137},
    "female": {"trunk": 0.4257, "head": 0.0668, "upper-arm": 0.0255, "forearm": 0.0138, "hand": 0.0056, "thigh": 0.1478, "shank": 0.0481, "foot": 0.0129},
}


def set_masses(a, mass, sex):
    """⚖️ Segment masses after de Leva (1996) for the sex and body mass; trunk split between pelvis and torso and head between neck and head as in the base figure."""
    f = DE_LEVA[sex]
    total = sum(f.values()) + f["upper-arm"] + f["forearm"] + f["hand"] + f["thigh"] + f["shank"] + f["foot"]
    k = mass / total
    seg = {"pelvis": f["trunk"] * 9.383 / 36.507, "torso": f["trunk"] * 27.124 / 36.507, "neck": f["head"] * 0.96 / 5.83, "head": f["head"] * 4.87 / 5.83}
    for side in "lr":
        for part in ("upper-arm", "forearm", "hand", "thigh", "shank", "foot"):
            seg[part + "-" + side] = f[part]
    for b in a["rig"]["bones"]:
        if b["id"] in seg:
            b["mass"]["kg"] = round(seg[b["id"]] * k, 3)
    if sex == "female":
        for b in a["rig"]["bones"]:
            if b.get("mass", {}).get("reference") == "de-leva-1996":
                pass


def dubois(mass, stature_m):
    return 0.007184 * mass ** 0.425 * (stature_m * 100) ** 0.725


def mifflin_w(mass, stature_m, age, sex):
    kcal = 10 * mass + 6.25 * stature_m * 100 - 5 * age + (5 if sex == "male" else -161)
    return kcal * 4184 / 86400


def set_body(a, mass, stature_m, age, sex):
    """🫀️ Thermal body after DuBois (skin area) and Mifflin-St Jeor (basal metabolism); region areas follow the skin area of the base figure."""
    body = a["thermal"]["body"]
    old = body["skinAreaM2"]
    new = round(dubois(mass, stature_m), 3)
    body["massKg"] = mass
    body["skinAreaM2"] = new
    body["basalMetabolismW"] = round(mifflin_w(mass, stature_m, age, sex), 1)
    for r in a["thermal"]["regions"]:
        if r.get("model") == "body":
            r["areaM2"] = round(r["areaM2"] * new / old, 4)
    return new


# ---------------------------------------------------------------- tool parts: one profile, three views, one solid

def ring(points):
    return [[r5(x), r5(y)] for x, y in points]


def capsule_profile(u0, u1, r, n=6):
    """A bar of radius r from u0 to u1 with round ends, as a closed (u, v) outline."""
    pts = []
    for i in range(n + 1):
        t = -math.pi / 2 + math.pi * i / n
        pts.append((u1 + r * math.cos(t), r * math.sin(t)))
    for i in range(n + 1):
        t = math.pi / 2 + math.pi * i / n
        pts.append((u0 + r * math.cos(t), r * math.sin(t)))
    return pts


def rect(u0, u1, v0, v1):
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]


def ellipse(cu, cv, ru, rv, n=16):
    return [(cu + ru * math.cos(2 * math.pi * i / n), cv + rv * math.sin(2 * math.pi * i / n)) for i in range(n)]


def tool_shapes(spec, views=("elevation", "front", "plan")):
    """🔧️ The shapes of one tool part in every view: the same bone-local profile with its solid placement, so the drawing of each view is the projection of the part in every pose."""
    out = {}
    for view in views:
        s = {"id": spec["id"], "bone": spec["bone"], "role": spec.get("role", "body"), "z": spec["z"][view] if isinstance(spec["z"], dict) else spec["z"], "filled": spec.get("filled", True), "tone": spec["tone"], "region": spec["region"], "material": spec["material"], "kind": "poly", "points": ring(spec["profile"]), "closed": True, "solid": copy.deepcopy(spec["solid"])}
        if "penMm" in spec:
            s["penMm"] = spec["penMm"]
        out[view] = s
    return out


def add_shapes(a, scale, shapes_by_view):
    g = a["representations"][scale]["geometry"]
    for view, s in shapes_by_view.items():
        g[view] = [x for x in g[view] if x["id"] != s["id"]] + [s]


def add_solids(a, scale, lst, solids):
    v = a["volumes"][scale][lst]
    ids = {s["id"] for s in solids}
    a["volumes"][scale][lst] = [x for x in v if x["id"] not in ids] + solids


# ---------------------------------------------------------------- poses

def regions_binding(a, over=None, sun=0.0):
    """🌡️ A binding for every thermal region: exposed and sunlit fractions, the base figure's contacts of gloves and boots, overrides per region."""
    over = over or {}
    out = {}
    for r in a["thermal"]["regions"]:
        b = {"exposedFraction": 1, "sunlitFraction": sun}
        out[r["id"]] = b
    for k, v in over.items():
        out[k] = {**out[k], **v}
    return out


def run_ik(spec, name):
    os.makedirs(GEN, exist_ok=True)
    sp = GEN + "/ik-" + name + ".json"
    op = GEN + "/ik-" + name + "-out.json"
    text = json.dumps(spec, indent=1)
    if os.path.exists(sp) and os.path.exists(op) and open(sp).read() == text and os.path.getmtime(op) >= os.path.getmtime(sp):
        return {x["id"]: x for x in json.load(open(op))}
    open(sp, "w").write(text)
    p = subprocess.run([BUN, HERE + "/ik.ts", sp, op], capture_output=True, text=True)
    print(p.stdout.strip())
    if p.returncode:
        raise SystemExit(p.stderr[-3000:])
    return {x["id"]: x for x in json.load(open(op))}


def clean_offsets(bones):
    out = {}
    for b, o in bones.items():
        d = {k: round(v, 3) for k, v in o.items() if k == "angleDeg" or abs(v) > 5e-4}
        if "angleDeg" not in d:
            d["angleDeg"] = 0
        out[b] = d
    return out


def write_asset(doc, slug):
    a = doc["asset"]
    a["id"] = slug
    path = CATALOG + "/" + slug
    os.makedirs(path, exist_ok=True)
    tmp = path + "/🔣️.json.tmp"
    json.dump(doc, open(tmp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    os.replace(tmp, path + "/🔣️.json")
    return path + "/🔣️.json"


def draft_path(slug):
    os.makedirs(GEN + "/draft", exist_ok=True)
    return GEN + "/draft/" + slug + ".json"


def save_draft(doc, slug):
    p = draft_path(slug)
    json.dump(doc, open(p, "w", encoding="utf-8"), ensure_ascii=False)
    return p


# ---------------------------------------------------------------- poses, clips, texts and dimensions

def make_pose(a, pid, title, description, bones, contacts, met, breathing, regions=None, emitters=None, operating=None, external=0, loads=None, sun=0.0):
    return {"id": pid, "title": title, "description": description, "bones": clean_offsets(bones), "ground": "auto", "contacts": contacts,
            "thermal": {"activityMet": met, "externalWorkW": external, "breathing": breathing, "regions": regions_binding(a, regions, sun), "emitters": emitters or {"plume": 0.6}, "operating": operating or {}},
            "loads": loads or []}


def key(t, pose, bones=None, ease="smooth"):
    k = {"t": round(t, 3), "pose": pose, "ease": ease}
    if bones:
        k["bones"] = clean_offsets(bones)
    return k


def span(anchors, t0, t1):
    return [{"anchor": x, "fromS": round(t0, 3), "toS": round(t1, 3)} for x in anchors]


def add_references(a, refs):
    have = {r["id"] for r in a["provenance"]["references"]}
    a["provenance"]["references"] += [r for r in refs if r["id"] not in have]


def bbox_dims(path, pose=None):
    """📦️ Width, height and depth of the 1:50 body volume of the default pose as the silhouette guard measures it."""
    p = subprocess.run([BUN, HERE + "/dims.ts", path] + ([pose] if pose else []), capture_output=True, text=True)
    if p.returncode:
        raise SystemExit(p.stderr[-2000:])
    return json.loads(p.stdout.strip().splitlines()[-1])


MIFFLIN = {"id": "mifflin-1990", "citation": "Mifflin, M. D. et al. (1990). A new predictive equation for resting energy expenditure in healthy individuals. American Journal of Clinical Nutrition 51(2), 241-247."}


def finish_common(doc, X, env, base_met, default_pose, default_clip):
    """🧾️ Texts, provenance, thermal environment, breathing patterns, assumptions and the decisions of every scale from the person's text module `X`."""
    import texts_common as C
    a = doc["asset"]
    a["title"], a["summary"], a["tags"] = X.TITLE, X.SUMMARY, X.TAGS
    a["provenance"].update({"author": "Semio staffage human collection worker HW-04", "method": "handcrafted", "created": "2026-11-22", "notes": X.NOTES})
    add_references(a, X.REFS + [MIFFLIN])
    t = a["thermal"]
    t["environment"] = env
    t["baseMet"] = base_met
    t["body"]["reference"] = "mifflin-1990"
    t["respiration"]["patterns"] = X.PATTERNS
    t["assumptions"] = X.thermal_assumptions(t["body"]["skinAreaM2"])
    a["structure"]["assumptions"] = X.STRUCTURE_ASSUMPTIONS
    a["structure"]["dynamics"] = "quasi-static"
    a["presentation"] = {"defaultPose": default_pose, "defaultClip": default_clip, "defaultScale": 100}
    for sc in SCALES:
        a["representations"][sc]["decisions"] = C.decisions(sc, X.DECISIONS[sc])
        a["volumes"][sc]["decisions"] = C.volume_decisions(a["volumes"][sc]["decisions"], sc, X.VOLUME[sc])


def set_dims(doc, slug, mass_total):
    """📐️ States the dimensions that satisfy the volume guard (one smallest feature per scale) and the drawing guard (max(2 %, 15 mm) per drawing) at once, see FIN-A/fit_dims.py."""
    import sys
    sys.path.insert(0, T + "/🧰️scripts/FIN-A")
    import fit_dims
    path = save_draft(doc, slug)
    dims, defects = fit_dims.fit(path)
    doc["asset"]["dimensions"].update({**dims, "massKg": round(mass_total, 1)})
    print("dims", dims, defects)
    return save_draft(doc, slug)
