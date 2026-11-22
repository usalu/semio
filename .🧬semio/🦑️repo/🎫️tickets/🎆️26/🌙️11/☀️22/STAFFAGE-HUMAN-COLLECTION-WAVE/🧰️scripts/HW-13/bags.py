"""🛍️ Generator of `humans/person-carrying-shopping-bags`: the archive's 1.78 m man in overcoat and scarf (body, rig and outlines of
`walking-commuter`, the shoulder bag removed) walking home with a full paper shopping bag in each fist.

New: two bag bones hung in the fists (4.5 kg each), the bags per scale (kraft paper body pinched at the top, twisted paper handles, a
baguette and a leek showing), passive regions for the bags and the groceries, arms that hang with the load (little swing, slight
abduction so the bags clear the legs), the bags solved to hang plumb with a small pendulum lag (kit.ts), and the walk cycle and idle
breathing re-keyed. Run: `python3 bags.py [--solve]`.
"""
import copy
import json
import math
import sys

from lib import (GEN, SCALES, add_part, bun, find, load, mass_marker, prism_part, prismify, rect, rest_frames, save, strip_bones, strip_regions, t)
import texts_bags as TX

SLUG = "person-carrying-shopping-bags"
GRIP = (0.082, 0.024)
BAG_LEN = 0.45
BAG_KG = 4.5
SWING = 0.3
LAG = 0.35

def fist_left(a, frames):
    """✊️ The left hand closes into the fist of the right (the source's right fist round its strap): the left finger bones take the mirrored rest of the right ones, the left hand drawings become the mirrored right fist outline as prisms across the fist in every view, and at 1:200 and 1:500 the open left mitten solid becomes the mirrored right fist."""
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    for b in a["rig"]["bones"]:
        if b["id"].endswith("-l") and b["id"].split("-")[0] in ("thumb", "index", "middle", "ring", "little"):
            r = bones[b["id"][:-2] + "-r"]
            for k in ("x", "y", "angleDeg", "lengthM"):
                b[k] = r[k]
            b["z"] = -r.get("z", 0)
            b["abductionDeg"] = -r.get("abductionDeg", 0)
            if "twistDeg" in r:
                b["twistDeg"] = -r["twistDeg"]
    for p in a["poses"]:
        p["bones"] = {k: v for k, v in p["bones"].items() if not (k.endswith("-l") and k.split("-")[0] in ("thumb", "index", "middle", "ring", "little"))}
    for c in a["clips"]:
        for k in c["keys"]:
            if "bones" in k:
                k["bones"] = {b: v for b, v in k["bones"].items() if not (b.endswith("-l") and b.split("-")[0] in ("thumb", "index", "middle", "ring", "little"))}
    span = {"r": (-0.02, 0.055), "l": (-0.055, 0.02)}
    for sc in ("50", "100", "200", "500"):
        geo = a["representations"][sc]["geometry"]
        right = next((x for x in geo["elevation"] if x["id"] == "hand-r"), None)
        outline = right["points"] if right is not None else ell(0.064, 0.0, 0.056, 0.046)
        for v in ("elevation", "front", "plan"):
            for x in geo[v]:
                if x["id"] in ("hand-l", "hand-r"):
                    side = x["id"][-1]
                    oz = frames[x["bone"]]["o"][2]
                    x["kind"] = "poly"
                    x["points"] = [list(q) for q in outline]
                    x["closed"] = True
                    x["solid"] = {"kind": "extruded", "sweep": "w", "z0": round(oz + span[side][0], 4), "z1": round(oz + span[side][1], 4)}
        if sc in ("200", "500"):
            vol = a["volumes"][sc]
            r = next(x for x in vol["body"] if x["id"] == "hand-r")
            l = next(x for x in vol["body"] if x["id"] == "hand-l")
            l.clear()
            l.update(copy.deepcopy(r))
            l["id"], l["bone"] = "hand-l", "hand-l"
            l["transform"]["translation"][2] = -r["transform"]["translation"][2]


def mirror_right_arm(a):
    """🪞️ The source drew its right arm bent up to the strap; hanging with a bag it is the mirror of the free left arm: the front and plan outlines of the right sleeve, forearm, wrist and cuff become the mirrored left ones (lateral coordinate negated)."""
    for sc in ("50", "100", "200", "500"):
        geo = a["representations"][sc]["geometry"]
        for v, k in (("front", 0), ("plan", 1)):
            left = {x["id"]: x for x in geo[v]}
            for x in geo[v]:
                if x["bone"] in ("arm-r", "forearm-r") and x["id"].endswith("-r") and x["id"][:-2] + "-l" in left and x.get("kind") == "poly":
                    src = left[x["id"][:-2] + "-l"]
                    x["points"] = [[(-q[0] if k == 0 else q[0]), (-q[1] if k == 1 else q[1])] for q in src["points"]][::-1]
                    x["closed"] = src.get("closed", True)


def ell(cu, cv, ru, rv, n=12):
    return [[round(cu + ru * math.cos(2 * math.pi * k / n), 4), round(cv + rv * math.sin(2 * math.pi * k / n), 4)] for k in range(n)]


def rig(a):
    bones = a["rig"]["bones"]
    for side, sg in (("l", -1), ("r", 1)):
        bones.append({"id": f"bag-{side}", "parent": f"hand-{side}", "title": TX.BAG_BONE[side], "x": GRIP[0], "y": 0, "z": sg * GRIP[1], "angleDeg": 0, "lengthM": BAG_LEN,
                      "joint": {"type": "revolute", "minDeg": -120, "maxDeg": 120, "twistMinDeg": -180, "twistMaxDeg": 180},
                      "mass": {"kg": BAG_KG, "comX": 0.31, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "ainsworth-2011"}})
        a["anchors"].append({"id": f"grip-{side}", "bone": f"hand-{side}", "x": GRIP[0], "y": 0, "z": sg * GRIP[1], "kind": "attachment", "title": TX.GRIP[side]})


def arm_offsets(swing_l, swing_r):
    """💪️ Arms that hang with the load: a third of the free swing, 9 degrees out from the trunk, elbows nearly straight, fists closed."""
    return {"arm-l": {"angleDeg": round(swing_l, 2), "abductionDeg": 9}, "forearm-l": {"angleDeg": round(6 + max(swing_l, 0) * 0.4, 2)}, "hand-l": {"angleDeg": 2},
            "arm-r": {"angleDeg": round(swing_r, 2), "abductionDeg": -9}, "forearm-r": {"angleDeg": round(6 + max(swing_r, 0) * 0.4, 2)}, "hand-r": {"angleDeg": 2, "abductionDeg": 0}}


def bag_stage(swing_l, swing_r):
    terms = []
    for side, swing in (("l", swing_l), ("r", swing_r)):
        lag = math.radians(-LAG * swing)
        terms.append({"kind": "axis", "bone": f"bag-{side}", "axis": "u", "direction": [math.sin(lag), -math.cos(lag), 0]})
        terms.append({"kind": "axis", "bone": f"bag-{side}", "axis": "v", "direction": [math.cos(lag), math.sin(lag), 0], "weight": 5})
    vars_ = [[f"bag-{s}", k, -170, 170] for s in ("l", "r") for k in ("angleDeg", "abductionDeg", "twistDeg")]
    return {"vars": vars_, "terms": terms, "regularise": 1e-7}


def strip(a):
    strip_bones(a, ["bag"])
    strip_regions(a, ["bag-leather"])
    a["anchors"] = [x for x in a["anchors"] if x["id"] not in ("bag-cog", "strap-grip")]
    a["poses"] = [p for p in a["poses"] if p["id"] != "check-watch"]
    a["clips"] = [c for c in a["clips"] if c["id"] != "glance-at-watch"]
    for p in a["poses"]:
        p["loads"] = []


def swings(a):
    """🎚️ Free swing of the commuter's left arm per pose and key, a third of it kept; the right arm swings in the opposite phase."""
    out = {}
    walk = find(a["clips"], "walk-cycle")
    keys = walk["keys"]
    n = len(keys)
    for i, k in enumerate(keys):
        j = (i + n // 2) % n
        out[("key", i)] = (SWING * k["bones"]["arm-l"]["angleDeg"], SWING * keys[j]["bones"]["arm-l"]["angleDeg"])
    mirror = {"contact-left": "contact-right", "loading-left": "loading-right", "passing-left": "passing-right"}
    mirror.update({v: k for k, v in mirror.items()})
    for p in a["poses"]:
        if p["id"] == "standing":
            out[("pose", "standing")] = (0.0, 0.0)
        else:
            other = find(a["poses"], mirror[p["id"]])
            out[("pose", p["id"])] = (SWING * p["bones"]["arm-l"]["angleDeg"], SWING * other["bones"]["arm-l"]["angleDeg"])
    return out


def solve(a, draft, redo):
    out = f"{GEN}/bags-solved.json"
    sw = swings(a)
    if redo:
        poses = []
        for p in a["poses"]:
            l, r = sw[("pose", p["id"])]
            poses.append({"id": p["id"], "bones": {**p["bones"], **arm_offsets(l, r)}, "contacts": p["contacts"], "stages": [bag_stage(l, r)]})
        walk = find(a["clips"], "walk-cycle")
        for i, k in enumerate(walk["keys"]):
            l, r = sw[("key", i)]
            poses.append({"id": f"key-{i}", "bones": {**k["bones"], **arm_offsets(l, r)}, "contacts": [c["anchor"] for c in walk["contacts"] if c["fromS"] <= k["t"] <= c["toS"]] or ["heel-l"], "stages": [bag_stage(l, r)]})
        with open(f"{GEN}/bags-spec.json", "w") as f:
            json.dump({"poses": poses}, f)
        bun("kit.ts", "solve", draft, f"{GEN}/bags-spec.json", out)
    with open(out) as f:
        return json.load(f)


def thermal(p):
    th = p["thermal"]
    walking = p["id"] != "standing"
    th["activityMet"] = 3.5 if walking else 1.5
    th["breathing"] = "walking-loaded" if walking else "rest-standing"
    th["regions"]["paper-bags"] = {"exposedFraction": 0.9, "sunlitFraction": 0}
    th["regions"]["groceries"] = {"exposedFraction": 0.3, "sunlitFraction": 0}
    th["regions"]["hand-skin"] = {"exposedFraction": 0.7, "sunlitFraction": 0}


# ------------------------------------------------------------------ parts
PAPER = dict(region="paper-bags", material="kraft-paper")
HANDLE = dict(region="paper-bags", material="paper-twist")


def body_profile(scale):
    """🛍️ The side outline (u, w) of the full bag: pinched to the handles at the top, a full bulge below, a flat bottom."""
    if scale == "50":
        return [[0.135, -0.030], [0.150, -0.062], [0.175, -0.082], [0.210, -0.089], [0.330, -0.090], [0.430, -0.087], [0.446, -0.080], [0.450, -0.070], [0.450, 0.070], [0.446, 0.080], [0.430, 0.087], [0.330, 0.090], [0.210, 0.089], [0.175, 0.082], [0.150, 0.062], [0.135, 0.030]]
    if scale == "100":
        return [[0.135, -0.030], [0.165, -0.078], [0.210, -0.089], [0.440, -0.087], [0.450, -0.075], [0.450, 0.075], [0.440, 0.087], [0.210, 0.089], [0.165, 0.078], [0.135, 0.030]]
    if scale == "200":
        return [[0.135, -0.032], [0.190, -0.088], [0.450, -0.088], [0.450, 0.088], [0.190, 0.088], [0.135, 0.032]]
    return [[0.12, -0.075], [0.16, -0.090], [0.45, -0.090], [0.45, 0.090], [0.16, 0.090], [0.12, 0.075]]


def handle_profile(scale):
    """🪢️ One twisted paper handle as an arch strip in the (u, v) plane: from the bag rim at v = ±0.055 up into the fist."""
    if scale in ("200", "500"):
        return [[0.0, -0.012], [0.0, 0.012], [0.13, 0.06], [0.13, -0.06]]
    w = 0.007 if scale in ("50", "100") else 0.012
    outer = [[0.140, -0.058], [0.050, -0.030], [0.006, -0.013], [-0.004, 0.0], [0.006, 0.013], [0.050, 0.030], [0.140, 0.058]]
    inner = [[0.140, 0.058 - w], [0.050 + w * 0.3, 0.030 - w], [0.006 + w, 0.013 - w * 0.6], [w * 0.8, 0.0], [0.006 + w, -0.013 + w * 0.6], [0.050 + w * 0.3, -0.030 + w], [0.140, -0.058 + w]]
    return outer + inner


def octagon(cv, cw, r):
    return [[round(cv + r * math.cos(math.pi / 8 + k * math.pi / 4), 5), round(cw + r * math.sin(math.pi / 8 + k * math.pi / 4), 5)] for k in range(8)]


def add_bags(a, scale, frames):
    for side, sg in (("l", -1), ("r", 1)):
        bone = f"bag-{side}"
        base_z = frames[bone]["o"][2]
        half = {"50": 0.16, "100": 0.16, "200": 0.16, "500": 0.165}[scale]
        tone = 0.58 if scale != "500" else 0.55
        shapes, solid = prism_part(f"bag-body-{side}", bone, [[p[0], p[1]] for p in body_profile(scale)], "v", -half, half, tone=tone, z=6.2 if side == "r" else 0.3, **PAPER)
        if scale == "500" and side == "l":
            del shapes["elevation"]
        add_part(a, scale, shapes, solid, "paper-bags", structure=bone)
        hw = {"50": (0.026, 0.033), "100": (0.025, 0.033), "200": (-0.03, 0.03), "500": (-0.03, 0.03)}[scale]
        handles = [("front", hw)] if scale in ("200", "500") else [("outer", (hw[0], hw[1])), ("inner", (-hw[1], -hw[0]))]
        for name, (w0, w1) in handles:
            shapes, solid = prism_part(f"bag-handle-{name}-{side}", bone, handle_profile(scale), "w", w0, w1, base_z=base_z, tone=0.42, z=6.25 if side == "r" else 0.32, **HANDLE)
            if scale in ("50", "100"):
                solid["slender"] = True
            add_part(a, scale, shapes, solid, "paper-bags", structure=bone)
        if scale == "50":
            shapes, solid = prism_part(f"bag-rim-{side}", bone, [[0.135, -0.031], [0.150, -0.063], [0.162, -0.073], [0.162, 0.073], [0.150, 0.063], [0.135, 0.031]], "v", -0.161, 0.161, tone=0.48, z=6.21 if side == "r" else 0.31, role="detail", **PAPER)
            add_part(a, scale, shapes, solid, "paper-bags")
        food = groceries(side, scale)
        for pid, prof, lo, hi, mat, tone_, role in food:
            shapes, solid = prism_part(pid, bone, prof, "u", lo, hi, tone=tone_, z=6.15 if side == "r" else 0.28, role=role, region="groceries", material=mat)
            add_part(a, scale, shapes, solid, "groceries")
        vol = a["volumes"][scale]
        vol["structure"].append(mass_marker(a, scale, bone, BAG_KG, [0.31, 0, 0], "kraft-paper"))
        if scale in ("50", "100"):
            vol["structure"].append({"id": f"joint-grip-{side}", "kind": "ellipsoid", "bone": bone, "material": "paper-twist", "role": "body", "tone": 0.4, "radii": [0.01, 0.01, 0.01], "member": bone, "part": "joint"})


def groceries(side, scale):
    """🥖️ What shows above the rim: a baguette in the left bag, a leek with its green top in the right bag."""
    if side == "l":
        r = {"50": 0.03, "100": 0.03, "200": 0.032, "500": 0.035}[scale]
        top = {"50": 0.045, "100": 0.05, "200": 0.06, "500": 0.07}[scale]
        out = [(f"baguette-{side}", octagon(0.105, 0.0, r), top, 0.40, "bread-crust", 0.7, "body")]
        if scale == "50":
            out.append((f"baguette-tip-{side}", octagon(0.105, 0.0, 0.022), top - 0.012, top + 0.002, "bread-crust", 0.62, "detail"))
        return out
    r = {"50": 0.02, "100": 0.02, "200": 0.024, "500": 0.03}[scale]
    out = [(f"leek-{side}", octagon(-0.10, 0.01, r), 0.10, 0.42, "leek", 0.82, "body")]
    if scale in ("50", "100"):
        out.append((f"leek-leaves-{side}", rect(-0.135, -0.065, -0.012, 0.03), 0.015, 0.11, "leek", 0.35, "body"))
    elif scale == "200":
        out.append((f"leek-leaves-{side}", rect(-0.135, -0.065, -0.014, 0.034), 0.03, 0.11, "leek", 0.38, "body"))
    return out


# ------------------------------------------------------------------ assembly


def base():
    a = load("walking-commuter")
    a["id"] = SLUG
    a["title"] = TX.TITLE
    a["summary"] = TX.SUMMARY
    a["tags"] = ["human", "adult", "walking", "chores", "shopping", "carrying", "groceries", "load", "winter"]
    strip(a)
    a["materials"] = [m for m in a["materials"] if m["id"] != "leather-bag"] + TX.MATERIALS
    pv = a["provenance"]
    pv["author"] = "Semio staffage human collection worker HW-13"
    pv["created"] = "2026-11-22"
    pv["references"] += TX.REFERENCES
    pv["notes"] = TX.NOTES
    rig(a)
    a["thermal"]["regions"] += TX.REGIONS
    a["thermal"]["respiration"]["patterns"] = [p for p in a["thermal"]["respiration"]["patterns"] if p["id"] != "walking"] + [TX.WALK_BREATH]
    a["thermal"]["assumptions"] = TX.THERMAL_ASSUMPTIONS
    a["structure"]["assumptions"] = TX.STRUCTURE_ASSUMPTIONS
    for c in a["clips"]:
        for tr in c.get("tracks", []):
            if tr["target"] == "activityMet":
                for k in tr["keys"]:
                    k["v"] = 3.5
    return a


def main():
    redo = "--solve" in sys.argv
    a = base()
    draft = save(a, SLUG, f"{GEN}/draft/{SLUG}/🔣️.json")
    frames = rest_frames(draft)
    fist_left(a, frames)
    draft = save(a, SLUG, f"{GEN}/draft/{SLUG}/🔣️.json")
    solved = solve(a, draft, redo)
    for sc in SCALES:
        add_bags(a, sc, frames)
        a["representations"][sc]["decisions"] = TX.DECISIONS[sc]
        a["volumes"][sc]["decisions"] = a["volumes"][sc]["decisions"] + TX.VOLUME_DECISIONS[sc]
    for p in a["poses"]:
        p["bones"] = solved[p["id"]]["bones"]
        p["title"], p["description"] = TX.POSES[p["id"]]
        thermal(p)
    walk = find(a["clips"], "walk-cycle")
    for i, k in enumerate(walk["keys"]):
        k["bones"] = solved[f"key-{i}"]["bones"]
    walk["title"], walk["description"] = TX.CLIPS["walk-cycle"]
    idle = find(a["clips"], "idle-breathing")
    idle["title"], idle["description"] = TX.CLIPS["idle-breathing"]
    for k in idle["keys"]:
        if "bones" in k:
            k["bones"] = {b: v for b, v in k["bones"].items() if not b.startswith(("arm-", "forearm-", "hand-", "bag-"))}
    mirror_right_arm(a)
    prismify(a, frames, ("arm-l", "arm-r", "forearm-l", "forearm-r"))
    a["dimensions"] = dict(TX.DIMENSIONS)
    a["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"]), 3)
    a["presentation"] = {"defaultPose": "standing", "defaultClip": "walk-cycle", "defaultScale": 100}
    worst = max(max(p["residual"]) for p in solved.values())
    print(f"[DEBUG] worst bag residual {worst:.2e}")
    print(save(a, SLUG))


if __name__ == "__main__":
    main()
