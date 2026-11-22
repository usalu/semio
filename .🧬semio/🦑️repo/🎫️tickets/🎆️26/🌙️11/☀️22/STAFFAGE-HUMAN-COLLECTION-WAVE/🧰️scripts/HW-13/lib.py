"""🧺️ HW-13 shared generator helpers: load a source human, strip parts by bone or region, add handcrafted parts (bones, 2D shapes with their solids, volumes), round numbers and write the asset document.

Every new carried part is authored once as prisms and solids of revolution in the bone-local frame of its bone; the 2D shape of every view is
the same profile with the matching `solid` placement (extruded sweep or revolved), so drawings and volumes agree in every pose.
See ../../📋️brief.md and 🎓️teaching/🏛️architecture/🧍️staffage/README.md ("Solid placements", "Volume").
"""
import copy
import json
import math
import os
import subprocess

R = "/Users/ueli/Documents/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
HUMANS = f"{S}/🗂️catalog/👤️humans"
T = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
HERE = f"{T}/🧰️scripts/HW-13"
GEN = f"{T}/🗑️generated/HW-13"
BUN = f"{R}/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
SCALES = ["50", "100", "200", "500"]
VIEWS = ["elevation", "front", "plan"]
LISTS = ["body", "thermal", "structure"]


def t(en, de):
    return {"en": en, "de": de}


def load(slug):
    with open(f"{HUMANS}/{slug}/🔣️.json", encoding="utf-8") as f:
        return json.load(f)["asset"]


def rounded(x, d=5):
    if isinstance(x, float):
        r = round(x, d)
        return 0.0 if r == 0 else r
    if isinstance(x, list):
        return [rounded(v, d) for v in x]
    if isinstance(x, dict):
        return {k: rounded(v, d) for k, v in x.items()}
    return x


def save(asset, slug, path=None):
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": rounded(asset)}
    path = path or f"{HUMANS}/{slug}/🔣️.json"
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write("\n")
    return path


def bun(*args, cwd=HERE):
    out = subprocess.run([BUN, *args], cwd=cwd, capture_output=True, text=True)
    if out.returncode != 0:
        raise RuntimeError(out.stderr[-4000:])
    return out.stdout


def rest_frames(path):
    """🦴️ Rest frames of every bone of a draft asset file, from the library's rig solve (kit.ts frames)."""
    out = f"{GEN}/frames-{os.path.basename(os.path.dirname(path)) or 'draft'}.json"
    bun("kit.ts", "frames", path, "-", out)
    with open(out) as f:
        return json.load(f)["-"]["frames"]


# ---------------------------------------------------------------- stripping


def descendants(asset, roots):
    out = set(roots)
    grew = True
    while grew:
        grew = False
        for b in asset["rig"]["bones"]:
            if b["parent"] in out and b["id"] not in out:
                out.add(b["id"])
                grew = True
    return out


def strip_bones(asset, roots):
    """✂️ Removes bones with their subtree and everything bound to them: shapes, solids, anchors, pose and clip offsets, contacts and loads on their anchors."""
    gone = descendants(asset, roots)
    asset["rig"]["bones"] = [b for b in asset["rig"]["bones"] if b["id"] not in gone]
    lost_anchors = {a["id"] for a in asset["anchors"] if a["bone"] in gone}
    asset["anchors"] = [a for a in asset["anchors"] if a["bone"] not in gone]
    for rep in asset["representations"].values():
        if rep.get("status") != "authored":
            continue
        for v in VIEWS:
            rep["geometry"][v] = [s for s in rep["geometry"].get(v, []) if s["bone"] not in gone and s.get("solid", {}).get("axisBone") not in gone]
    for vol in asset["volumes"].values():
        if vol.get("status") != "authored":
            continue
        strip_solids(vol, lambda s: s.get("bone") in gone or s.get("member") in gone or s.get("anchor") in lost_anchors or any(w["bone"] in gone for w in s.get("weights", [])))
        for s in vol["structure"]:
            if "members" in s:
                s["members"] = [m for m in s["members"] if m not in gone]
                if not s["members"]:
                    del s["members"]
    for p in asset["poses"]:
        p["bones"] = {k: v for k, v in p["bones"].items() if k not in gone}
        p["contacts"] = [c for c in p["contacts"] if c not in lost_anchors]
        p["loads"] = [l for l in p.get("loads", []) if l.get("anchor") not in lost_anchors]
    for c in asset["clips"]:
        for k in c["keys"]:
            if "bones" in k:
                k["bones"] = {b: v for b, v in k["bones"].items() if b not in gone}
        c["contacts"] = [x for x in c["contacts"] if x["anchor"] not in lost_anchors]
        c["events"] = [e for e in c.get("events", []) if e.get("target") not in lost_anchors]
    asset["structure"]["supports"] = [s for s in asset["structure"]["supports"] if s not in lost_anchors]
    return gone


def strip_solids(vol, drop):
    """🧹️ Drops solids matching `drop` from all lists of a volume, and every `of` reference to a dropped body solid."""
    gone = {s["id"] for s in vol["body"] if drop(s)}
    for lst in LISTS:
        keep = []
        for s in vol[lst]:
            if drop(s) or s.get("of") in gone:
                gone.add(s["id"])
                continue
            keep.append(s)
        vol[lst] = keep
    return gone


def strip_regions(asset, ids):
    """🌡️ Removes thermal regions with their bindings, shapes and thermal solids."""
    ids = set(ids)
    asset["thermal"]["regions"] = [r for r in asset["thermal"]["regions"] if r["id"] not in ids]
    for p in asset["poses"]:
        p["thermal"]["regions"] = {k: v for k, v in p["thermal"]["regions"].items() if k not in ids}
    for c in asset["clips"]:
        c["tracks"] = [tr for tr in c.get("tracks", []) if not any(tr["target"].startswith(f"region:{r}:") for r in ids)]
    for rep in asset["representations"].values():
        if rep.get("status") != "authored":
            continue
        for v in VIEWS:
            rep["geometry"][v] = [s for s in rep["geometry"].get(v, []) if s.get("region") not in ids]
            for s in rep["geometry"][v]:
                if "regions" in s:
                    s["regions"] = [r for r in s["regions"] if r not in ids]
    for vol in asset["volumes"].values():
        if vol.get("status") == "authored":
            vol["thermal"] = [s for s in vol["thermal"] if s.get("region") not in ids]


def strip_shapes(asset, ids, scales=SCALES):
    ids = set(ids)
    for sc in scales:
        rep = asset["representations"][sc]
        for v in VIEWS:
            rep["geometry"][v] = [s for s in rep["geometry"][v] if s["id"] not in ids]


# ---------------------------------------------------------------- parts


def revolved_part(pid, bone, profile, *, region, material, tone, z, role="body", pen=None, slender=False, views="efp", axis_x=0.0):
    """🔄️ A solid of revolution about the bone axis u: `profile` is the half outline [(u, r)] from the axis back to the axis.
    Returns (shapes per view, body solid)."""
    pts = [[u, r] for u, r in profile]
    shape = {"id": pid, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "region": region, "material": material, "kind": "poly", "points": pts, "closed": True,
             "solid": {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": axis_x, "axisZ": 0, "mirror": True}}
    if pen is not None:
        shape["penMm"] = pen
    shapes = {v: dict(copy.deepcopy(shape), z=z) for v in VIEWS if v[0] in views}
    solid = {"id": pid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[u, max(r, 0.0)] for u, r in profile], "axis": [1, 0, 0], "origin": [0, 0, 0]}
    if slender:
        solid["slender"] = True
    return shapes, solid


def prism_part(pid, bone, profile, sweep, lo, hi, *, region, material, tone, z, role="body", pen=None, base_z=0.0, sheet=False, slender=False, views="efp", hatch=None):
    """🧱️ A prism: `profile` in the bone-local plane of the sweep (`u`: (v, w); `w`: (u, v); `v`: (u, w)) swept from `lo` to `hi` along the sweep axis.
    For `w` the 2D span is stated in rest-pose world z, so `base_z` is the rest z of the bone origin. Returns (shapes per view, body solid)."""
    shape = {"id": pid, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "region": region, "material": material, "kind": "poly", "points": [list(p) for p in profile], "closed": True,
             "solid": {"kind": "extruded", "sweep": sweep, "z0": lo + (base_z if sweep == "w" else 0.0), "z1": hi + (base_z if sweep == "w" else 0.0)}}
    if pen is not None:
        shape["penMm"] = pen
    if hatch is not None:
        shape["hatch"] = hatch
    shapes = {v: copy.deepcopy(shape) for v in VIEWS if v[0] in views}
    if sweep == "u":
        plane, prof = "yz", [list(p) for p in profile]
    elif sweep == "w":
        plane, prof = "xy", [list(p) for p in profile]
    else:
        plane, prof = "zx", [[p[1], p[0]] for p in profile]
    if signed_area(prof) < 0:
        prof = prof[::-1]
    solid = {"id": pid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": prof, "plane": plane, "from": lo, "to": hi}
    if sheet:
        solid["sheet"] = True
    if slender:
        solid["slender"] = True
    return shapes, solid


def signed_area(p):
    return sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p))) / 2


def rect(a0, a1, b0, b1, r=0.0, n=3):
    """▭️ A rectangle [a0, a1] x [b0, b1] with optional rounded corners (n segments per corner), counter-clockwise."""
    if r <= 0:
        return [[a0, b0], [a1, b0], [a1, b1], [a0, b1]]
    pts = []
    for cx, cy, start in ((a1 - r, b0 + r, -90), (a1 - r, b1 - r, 0), (a0 + r, b1 - r, 90), (a0 + r, b0 + r, 180)):
        for i in range(n + 1):
            ang = math.radians(start + 90 * i / n)
            pts.append([round(cx + r * math.cos(ang), 5), round(cy + r * math.sin(ang), 5)])
    return pts


def add_part(asset, scale, shapes, solid, region, *, structure=None):
    """➕️ Adds the 2D shapes of a part to one scale and its body solid with the thermal reference (and an optional structure member reference)."""
    rep = asset["representations"][scale]
    for v, s in shapes.items():
        rep["geometry"][v].append(s)
    vol = asset["volumes"][scale]
    vol["body"].append(solid)
    vol["thermal"].append({"id": f"th-{solid['id']}", "of": solid["id"], "region": region})
    if structure is not None:
        vol["structure"].append({"id": f"member-{solid['id']}", "of": solid["id"], "member": structure, "part": "member"})


def mass_radius(asset, scale, kg):
    """⚖️ Radius of a mass marker of `kg` at a scale, in the proportion of the asset's own pelvis marker (r ∝ m^(1/3))."""
    vol = asset["volumes"][scale]
    masses = {b["id"]: b["mass"]["kg"] for b in asset["rig"]["bones"] if "mass" in b}
    ref = next((s for s in vol["structure"] if s.get("part") == "mass" and s["member"] == "pelvis" and s.get("kind") == "ellipsoid"), None) or next(s for s in vol["structure"] if s.get("part") == "mass" and s.get("kind") == "ellipsoid" and s["member"] in masses)
    pelvis = masses[ref["member"]]
    feature = 0.15 * int(scale) / 1000
    return round(max(ref["radii"][0] * (kg / pelvis) ** (1 / 3), feature / 2 * 1.02), 5)


def mass_marker(asset, scale, bone, kg, at, material, tone=0.2):
    return {"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": tone, "radii": [mass_radius(asset, scale, kg)] * 3, "transform": {"translation": list(at)}, "member": bone, "part": "mass"}


def quat_z(deg):
    a = math.radians(deg) / 2
    return [0, 0, round(math.sin(a), 5), round(math.cos(a), 5)]


def find(lst, pid):
    return next(x for x in lst if x["id"] == pid)


ARM_BONES = ("arm-", "upper-arm-", "forearm-")


def unify_arm_views(a, scales=SCALES):
    """🔁️ Front shapes of the arm segments take the solid of revolution of their plan twins, so front and plan read the same lateral reach when an arm swings across the body."""
    for sc in scales:
        geo = a["representations"][sc]["geometry"]
        plan = {s["id"]: s for s in geo["plan"]}
        for s in geo["front"]:
            p = plan.get(s["id"])
            if p is not None and s["bone"].startswith(ARM_BONES) and (p.get("solid") or {}).get("kind") == "revolved":
                s["points"] = copy.deepcopy(p["points"])
                s["solid"] = copy.deepcopy(p["solid"])
                s["kind"] = "poly"
                s["closed"] = True


def prism_plan_trunk(a, frames, bones=("torso",), scales=SCALES, extra=None, lift=None):
    """🧱️ Flat plan shapes of the trunk become prisms swept along the trunk axis (their rest outline as (v, w), the height range of their elevation twin), so a forward lean shows in the plan as it does in the side view."""
    for sc in scales:
        geo = a["representations"][sc]["geometry"]
        elev = {s["id"]: s for s in geo["elevation"]}
        for s in geo["plan"]:
            if s["bone"] not in bones or s.get("solid") or s.get("kind") != "poly" or not s.get("filled"):
                continue
            f = frames[s["bone"]]
            twin = elev.get(s["id"])
            if twin is None or twin.get("kind") != "poly":
                continue
            us = [p[0] for p in twin["points"]] + [p[0] for e in (extra or {}).get(s["id"], []) if e in elev for p in elev[e]["points"]]
            floor = min(us) + (lift or {}).get(sc, {}).get(s["id"], 0.0)
            us = [max(u, floor) for u in us]
            loc = []
            for x, z in s["points"]:
                d = [x - f["o"][0], 0.0, z - f["o"][2]]
                loc.append([round(sum(d[k] * f["v"][k] for k in range(3)), 5), round(sum(d[k] * f["w"][k] for k in range(3)), 5)])
            s["points"] = loc
            s["solid"] = {"kind": "extruded", "sweep": "u", "z0": round(min(us), 4), "z1": round(max(us), 4)}


def prismify(a, frames, bones, scales=SCALES):
    """🧱️ Flat shapes of limb bones become prisms in every view, so a limb that a pose turns out of its rest plane is drawn right: the side outline (u, v) swept across the lateral range of its front twin, the front outline (as u, w) swept through the depth of its side twin, the plan outline (as v, w) swept along the length of its side twin."""
    def local(f, p):
        d = [p[k] - f["o"][k] for k in range(3)]
        return [sum(d[k] * f[ax][k] for k in range(3)) for ax in ("u", "v", "w")]
    for sc in scales:
        geo = a["representations"][sc]["geometry"]
        twins = {v: {s["id"]: s for s in geo[v] if s.get("kind") == "poly"} for v in VIEWS}
        for v in VIEWS:
            for s in geo[v]:
                if s["bone"] not in bones or s.get("solid") or s.get("kind") != "poly" or not s.get("filled"):
                    continue
                f = frames[s["bone"]]
                el, fr = twins["elevation"].get(s["id"]), twins["front"].get(s["id"])
                if v == "elevation":
                    if fr is None:
                        continue
                    ws = [local(f, [f["o"][0], y, z])[2] for z, y in fr["points"]]
                    s["solid"] = {"kind": "extruded", "sweep": "w", "z0": round(f["o"][2] + min(ws), 4), "z1": round(f["o"][2] + max(ws), 4)}
                elif v == "front":
                    if el is None:
                        continue
                    vs = [p[1] for p in el["points"]]
                    s["points"] = [[round(c[0], 5), round(c[2], 5)] for c in (local(f, [f["o"][0], y, z]) for z, y in s["points"])]
                    s["solid"] = {"kind": "extruded", "sweep": "v", "z0": round(min(vs), 4), "z1": round(max(vs), 4)}
                else:
                    if el is None:
                        continue
                    us = [p[0] for p in el["points"]]
                    s["points"] = [[round(c[1], 5), round(c[2], 5)] for c in (local(f, [x, f["o"][1], z]) for x, z in s["points"])]
                    s["solid"] = {"kind": "extruded", "sweep": "u", "z0": round(min(us), 4), "z1": round(max(us), 4)}
