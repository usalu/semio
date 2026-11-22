"""🍽️ HW-02 generator library: derives handcrafted hospitality figures from the archive's generic standing adult.

The base asset (snapshot `base-standing-adult.json.gz` of `🗂️catalog/👤️humans/standing-adult`) is scaled uniformly to the stature of
the person, its segment masses are re-derived from de Leva (1996) for the sex of the person, clothing regions are re-labelled, the hood
is removed, and garments and carried objects are added as parts whose 2D shapes (bone-local solids: revolved or extruded) and 3D solids
come from one profile, so the views and the volume agree by construction at every pose.

See ../../📋️brief.md and the report ../../📝️HW-02.md.
"""
import copy, gzip, json, math, os, subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
R = os.path.abspath(os.path.join(HERE, *[".."] * 9))
S = os.path.join(R, "🎓️teaching", "🏛️architecture", "🧍️staffage")
HUMANS = os.path.join(S, "🗂️catalog", "👤️humans")
GEN = os.path.abspath(os.path.join(HERE, "..", "..", "🗑️generated", "HW-02"))
BUN = os.path.join(R, ".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
SCALES = ["50", "100", "200", "500"]
VIEWS = ["elevation", "front", "plan"]


def T(en, de):
    return {"en": en, "de": de}


def load_base(name="standing-adult"):
    with gzip.open(os.path.join(HERE, f"base-{name}.json.gz"), "rt") as f:
        return json.load(f)


def r5(x):
    return round(x, 5)


# ---------------------------------------------------------------- uniform scaling
PT_KEYS = {"points", "profile", "vertices"}
NUM_KEYS = {"x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r", "z0", "z1", "axisX", "axisZ",
            "from", "to", "cornerRadius", "plane"}
VEC_KEYS = {"size", "radii", "p0", "p1", "at", "origin", "translation", "zTip"}


def _scale_geom(o, k, keep_radii=False):
    if isinstance(o, list):
        for v in o:
            _scale_geom(v, k, keep_radii)
        return
    if not isinstance(o, dict):
        return
    emitter = "emitter" in (o.get("tags") or [])
    for key, v in list(o.items()):
        if key in PT_KEYS and isinstance(v, list):
            o[key] = [[r5(c * k) for c in p] for p in v]
        elif key in ("holes",):
            o[key] = [[[r5(c * k) for c in p] for p in ring] for ring in v]
        elif key in VEC_KEYS and isinstance(v, list) and all(isinstance(c, (int, float)) for c in v):
            if key == "radii" and emitter:
                continue
            o[key] = [r5(c * k) for c in v]
        elif key in NUM_KEYS and isinstance(v, (int, float)) and not isinstance(v, bool):
            o[key] = r5(v * k)
        elif key == "transform":
            if "translation" in v:
                v["translation"] = [r5(c * k) for c in v["translation"]]
        elif key == "repeat":
            for kk in ("translation", "origin"):
                if kk in v:
                    v[kk] = [r5(c * k) for c in v[kk]]
        elif key in ("sections",) and isinstance(v, list):
            for s in v:
                if "at" in s:
                    s["at"] = [r5(c * k) for c in s["at"]]
                for kk in ("rx", "ry"):
                    if kk in s:
                        s[kk] = r5(s[kk] * k)
        elif key in ("solid", "geometry", "from_", "to_"):
            _scale_geom(v, k)
        elif isinstance(v, (dict, list)) and key not in ("title", "description", "decisions", "pens", "hatches", "validRange", "weights", "rotation"):
            _scale_geom(v, k)


def scale_asset(a, k):
    """📏️ Scales every length of the asset by k (rig, anchors, shapes, solids, areas by k², clips' advance), not masses or physiology."""
    for b in a["rig"]["bones"]:
        for key in ("x", "y", "z", "lengthM"):
            if key in b:
                b[key] = r5(b[key] * k)
        m = b.get("mass")
        if m:
            for key in ("comX", "comY"):
                if key in m:
                    m[key] = r5(m[key] * k)
    for an in a["anchors"]:
        for key in ("x", "y", "z", "footprintLengthM", "footprintWidthM"):
            if key in an:
                an[key] = r5(an[key] * k)
    for sc in SCALES:
        rep = a["representations"][sc]
        if rep["status"] == "authored":
            for view in VIEWS:
                for s in rep["geometry"][view]:
                    _scale_geom(s, k)
        vol = a["volumes"][sc]
        if vol["status"] == "authored":
            for lst in ("body", "thermal", "structure"):
                for s in vol[lst]:
                    if "of" in s:
                        continue
                    _scale_geom(s, k)
    for key in ("heightM", "widthM", "depthM"):
        a["dimensions"][key] = r5(a["dimensions"][key] * k)
    for reg in a["thermal"]["regions"]:
        reg["areaM2"] = round(reg["areaM2"] * k * k, 4)
        if "characteristicLengthM" in reg:
            reg["characteristicLengthM"] = round(reg["characteristicLengthM"] * k, 3)
    for e in a["thermal"]["emitters"]:
        if e["kind"] == "plume":
            e["diameterM"] = round(e["diameterM"] * k, 3)
    for c in a["clips"]:
        if "advanceM" in c:
            c["advanceM"] = r5(c["advanceM"] * k)
    for m in a["thermal"]["respiration"].get("motion", []):
        if "dy" in m:
            m["dy"] = r5(m["dy"] * k)


# ---------------------------------------------------------------- masses (de Leva 1996, Table 4 male / Table 5 female)
DE_LEVA = {
    "male": {"head": 0.0694, "upper-trunk": 0.1596, "mid-trunk": 0.1633, "lower-trunk": 0.1117, "upper-arm": 0.0271, "forearm": 0.0162, "hand": 0.0061, "thigh": 0.1416, "shank": 0.0433, "foot": 0.0137},
    "female": {"head": 0.0668, "upper-trunk": 0.1545, "mid-trunk": 0.1465, "lower-trunk": 0.1247, "upper-arm": 0.0255, "forearm": 0.0138, "hand": 0.0056, "thigh": 0.1478, "shank": 0.0481, "foot": 0.0129},
}


def segment_of(bone_id, bones):
    """🧩️ The de Leva segment a body bone belongs to (fingers and thumb with the hand, toe with the foot, neck with the head)."""
    b = bone_id
    if b in ("head", "neck"):
        return "head"
    if b == "torso":
        return "upper-trunk"
    if b == "lumbar":
        return "mid-trunk"
    if b == "pelvis":
        return "lower-trunk"
    for pre, seg in (("arm-", "upper-arm"), ("forearm-", "forearm"), ("thigh-", "thigh"), ("shank-", "shank"), ("foot-", "foot"), ("toe-", "foot"), ("hand-", "hand")):
        if b.startswith(pre):
            return seg
    if any(b.startswith(f) for f in ("thumb-", "index-", "middle-", "ring-", "little-")):
        return "hand"
    return None


def set_masses(a, sex, body_kg, source_ref="de-leva-1996"):
    """⚖️ Distributes `body_kg` over the body bones by de Leva's segment fractions of `sex` (two sides each), keeping the base's split within a segment (hand and fingers, foot and toe)."""
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    groups = {}
    for b in a["rig"]["bones"]:
        seg = segment_of(b["id"], bones)
        if seg and b.get("mass"):
            side = "l" if b["id"].endswith("-l") else "r" if b["id"].endswith("-r") else "c"
            groups.setdefault((seg, side), []).append(b)
    for (seg, side), bs in groups.items():
        total = sum(x["mass"]["kg"] for x in bs)
        target = DE_LEVA[sex][seg] * body_kg
        for x in bs:
            x["mass"]["kg"] = round(target * x["mass"]["kg"] / total, 4)
            x["mass"]["source"] = "literature"
            x["mass"]["reference"] = source_ref


def body_mass(a):
    return sum(b["mass"]["kg"] for b in a["rig"]["bones"] if b.get("mass") and segment_of(b["id"], None))


# ---------------------------------------------------------------- editing helpers
def shapes(a, sc, view):
    return a["representations"][sc]["geometry"][view]


def remove_shapes(a, pred):
    """✂️ Removes every 2D shape (any scale, any view) for which pred(scale, view, shape) holds."""
    for sc in SCALES:
        rep = a["representations"][sc]
        if rep["status"] != "authored":
            continue
        for view in VIEWS:
            rep["geometry"][view] = [s for s in rep["geometry"][view] if not pred(sc, view, s)]


def remove_solids(a, pred):
    """✂️ Removes body solids for which pred(scale, solid) holds, with every thermal or structural solid that references them."""
    for sc in SCALES:
        vol = a["volumes"][sc]
        if vol["status"] != "authored":
            continue
        gone = {s["id"] for s in vol["body"] if pred(sc, s)}
        vol["body"] = [s for s in vol["body"] if s["id"] not in gone]
        for lst in ("thermal", "structure"):
            vol[lst] = [s for s in vol[lst] if s.get("of") not in gone and not ("of" not in s and pred(sc, s))]


def solid(a, sc, lst, sid):
    for s in a["volumes"][sc][lst]:
        if s["id"] == sid:
            return s
    return None


def body_solid_of(a, sc, s):
    return solid(a, sc, "body", s["of"]) if "of" in s else s


def set_region(a, old, new, bone_pred=lambda b: True, material=None):
    """🏷️ Re-binds 2D shapes and thermal solids of region `old` on bones matching bone_pred to region `new` (thermal solids by the bone of the body solid they reference); optionally sets the body material."""
    for sc in SCALES:
        rep = a["representations"][sc]
        if rep["status"] == "authored":
            for view in VIEWS:
                for s in rep["geometry"][view]:
                    if not bone_pred(s["bone"]):
                        continue
                    if s.get("region") == old:
                        s["region"] = new
                    if "regions" in s:
                        s["regions"] = sorted(set(new if r == old else r for r in s["regions"]))
        vol = a["volumes"][sc]
        if vol["status"] != "authored":
            continue
        for s in vol["thermal"]:
            b = body_solid_of(a, sc, s)
            bone = b["bone"] if b else s.get("bone")
            if s["region"] == old and bone_pred(bone):
                s["region"] = new
                if material and b is not None and "of" in s:
                    b["material"] = material
                if material and "of" not in s and s.get("layer") is None:
                    s["material"] = material


def set_material(a, old, new):
    """🧵️ Renames a material everywhere (materials list, shapes, solids, regions, layers)."""
    def walk(o):
        if isinstance(o, dict):
            for k2, v in o.items():
                if k2 == "material" and v == old:
                    o[k2] = new
                else:
                    walk(v)
        elif isinstance(o, list):
            for v in o:
                walk(v)
    walk(a["representations"]); walk(a["volumes"]); walk(a["thermal"])


def drop_unused_layers_regions(a, region_ids):
    a["thermal"]["regions"] = [r for r in a["thermal"]["regions"] if r["id"] in region_ids]


def add_bone(a, bone, after=None):
    bones = a["rig"]["bones"]
    i = next((n for n, b in enumerate(bones) if b["id"] == after), len(bones) - 1) + 1 if after else len(bones)
    bones.insert(i, bone)


def bone(a, bid):
    return next(b for b in a["rig"]["bones"] if b["id"] == bid)


def write_asset(doc, slug):
    path = os.path.join(HUMANS, slug, "🔣️.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    return path


def write_tmp(doc, name):
    os.makedirs(GEN, exist_ok=True)
    path = os.path.join(GEN, name)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False)
    return path


def bun(script, *args, env=None):
    e = dict(os.environ, **(env or {}))
    return subprocess.run([BUN, os.path.join(HERE, script), *args], capture_output=True, text=True, env=e, cwd=HERE)


def frames(path, query="rest"):
    """🦴️ World frames {bone: {o, u, v, w, tip}} of the asset file in a pose, 'clip@t' or 'rest' (all offsets zero)."""
    out = bun("fk.ts", path, query, env={"JSON": "1"})
    if out.returncode != 0:
        raise RuntimeError(out.stderr[-3000:])
    return json.loads(out.stdout.strip().splitlines()[-1])


# ---------------------------------------------------------------- parts: one profile -> 2D shapes of every view + 3D solid
def revolve_part(a, sc, pid, bone_id, profile, material, region, tone, z, role="body", views=VIEWS, hatch=None, structure=True, member=None, volume=True, draw=True, slender=None, extra=None, regions=None):
    """🏺️ An axisymmetric part about the u axis of its bone: `profile` [[u, r], ...] from the axis out and back to the axis (r ≥ 0).
    Draws it in every view as a revolved solid shape and adds the matching `revolve` body solid with its thermal (and member) reference."""
    pts = [[r5(u), r5(r)] for u, r in profile]
    if draw:
        for view in views:
            s = {"id": pid, "bone": bone_id, "role": role, "z": z, "filled": True, "tone": tone, "region": region, "kind": "poly",
                 "points": pts, "closed": True, "solid": {"kind": "revolved", "axisBone": bone_id, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}}
            if hatch:
                s["hatch"] = hatch
            if regions:
                s.pop("region")
                s["regions"] = regions
            shapes(a, sc, view).append(s)
    if volume:
        prof = pts
        vs = {"id": pid, "kind": "revolve", "bone": bone_id, "material": material, "role": role, "tone": tone, "profile": prof, "axis": [1, 0, 0], "origin": [0, 0, 0]}
        if slender:
            vs["slender"] = True
        vs.update(extra or {})
        add_volume_part(a, sc, vs, region, structure, member or bone_id)


def extrude_part(a, sc, pid, bone_id, profile, w0, w1, rest_z, material, region, tone, z, role="body", views=VIEWS, hatch=None, structure=True, member=None, sheet=False, volume=True, draw=True, weights=None):
    """🧱️ A prismatic part: `profile` [[u, v], ...] in the bone's (u, v) plane swept laterally from bone-local w0 to w1 (the 2D solid states
    the rest asset z = rest_z + w); drawn in every view as an extruded solid shape, with the matching `extrude` body solid."""
    pts = [[r5(u), r5(v)] for u, v in profile]
    if draw:
        for view in views:
            s = {"id": pid, "bone": bone_id, "role": role, "z": z, "filled": True, "tone": tone, "region": region, "kind": "poly",
                 "points": pts, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r5(rest_z + w0), "z1": r5(rest_z + w1), **({"weights": weights} if weights else {})}}
            if hatch:
                s["hatch"] = hatch
            shapes(a, sc, view).append(s)
    if volume:
        vs = {"id": pid, "kind": "extrude", "bone": bone_id, "material": material, "role": role, "tone": tone, "profile": pts, "plane": "xy", "from": r5(w0), "to": r5(w1)}
        if sheet:
            vs["sheet"] = True
        if weights:
            vs["weights"] = weights
        add_volume_part(a, sc, vs, region, structure, member or bone_id)


def add_volume_part(a, sc, vs, region, structure=True, member=None):
    vol = a["volumes"][sc]
    vol["body"].append(vs)
    vol["thermal"].append({"id": f"th-{vs['id']}", "of": vs["id"], "region": region})
    if structure:
        vol["structure"].append({"id": f"st-{vs['id']}", "of": vs["id"], "member": member or vs["bone"], "part": "member"})


def circle_profile(u0, u1, r_at, n=0):
    """〰️ A closed revolve half-profile from the axis at u0, along r_at(u) samples [(u, r)...], back to the axis at u1."""
    return [[u0, 0.0], *r_at, [u1, 0.0]]


def rounded_rect(x0, y0, x1, y1, r, n=3):
    pts = []
    for cx, cy, a0 in ((x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180), (x1 - r, y0 + r, 270)):
        for i in range(n + 1):
            t = math.radians(a0 + 90 * i / n)
            pts.append([r5(cx + r * math.cos(t)), r5(cy + r * math.sin(t))])
    return pts


def quat_axis(axis, deg):
    t = math.radians(deg) / 2
    n = math.sqrt(sum(c * c for c in axis))
    return [r5(axis[0] / n * math.sin(t)), r5(axis[1] / n * math.sin(t)), r5(axis[2] / n * math.sin(t)), r5(math.cos(t))]


# ---------------------------------------------------------------- physiology
def dubois(m, h):
    return 0.007184 * m ** 0.425 * (h * 100) ** 0.725


def mifflin_w(sex, m, h, age):
    kcal = 10 * m + 625 * h - 5 * age + (5 if sex == "male" else -161)
    return kcal * 4184 / 86400


def pose_thermal(regions, emitters, exposed=None, sunlit=0, contact_regions=None, met=1.2, breathing="quiet-standing", operating=None):
    exposed = exposed or {}
    out = {"activityMet": met, "breathing": breathing, "regions": {}, "emitters": dict(emitters), "operating": dict(operating or {})}
    for r in regions:
        e = {"exposedFraction": exposed.get(r, 1), "sunlitFraction": sunlit}
        if contact_regions and r in contact_regions:
            e["contact"] = contact_regions[r]
        out["regions"][r] = e
    return out


def measure_dims(doc, name="t-dims.json"):
    """📐️ The default-pose extents of the drawings (height, forward width, lateral depth) over every scale and view, read from the validator with
    deliberately wrong dimensions; returns the mid-range per axis and the spread, so `dimensions` states what every drawing shows."""
    import re
    d = copy.deepcopy(doc)
    d["asset"]["dimensions"].update({"heightM": 9, "widthM": 9, "depthM": 9})
    path = write_tmp(d, name)
    out = bun("validate.ts", path, env={"LIMIT": "400"}).stdout
    vals = {"h": [], "w": [], "d": []}
    for m in re.finditer(r"default-pose (height|width|lateral width|forward length|lateral depth) ([\d.]+) m", out):
        vals[{"height": "h", "width": "w", "lateral width": "d", "forward length": "w", "lateral depth": "d"}[m.group(1)]].append(float(m.group(2)))
    res = {}
    for key, v in vals.items():
        if not v:
            raise RuntimeError(out[-3000:])
        res[key] = (round((min(v) + max(v)) / 2, 3), round(max(v) - min(v), 3))
    return res


HOODIE_ONLY = ("hood", "hood-roll", "hood-roll-l", "hood-roll-r", "hood-fold", "kangaroo-pocket", "pocket", "drawstring-0", "drawstring-1")


def strip_hoodie(a):
    """🧥️ Removes what makes the base garment a hooded sweatshirt (hood, rolled hood edge, hood fold, kangaroo pocket, drawstrings) from every scale and view and from every volume list; at 1:500 the hood was already merged into the trunk silhouette and stays as the collar mass."""
    remove_shapes(a, lambda sc, view, s: s["id"] in HOODIE_ONLY)
    remove_solids(a, lambda sc, s: s["id"] in HOODIE_ONLY or s["id"] in tuple(f"th-{x}" for x in HOODIE_ONLY))


def rename_ids(a, fn):
    """🔤️ Renames shape and solid ids (and the `of` references) by fn(id) -> id."""
    for sc in SCALES:
        rep = a["representations"][sc]
        if rep["status"] == "authored":
            for view in VIEWS:
                for s in rep["geometry"][view]:
                    s["id"] = fn(s["id"])
        vol = a["volumes"][sc]
        if vol["status"] == "authored":
            for lst in ("body", "thermal", "structure"):
                for s in vol[lst]:
                    s["id"] = fn(s["id"])
                    if "of" in s:
                        s["of"] = fn(s["of"])


def retone(a, pred, tone, drop_hatch=False, hatch=None):
    """🎨️ Sets the tone of 2D shapes (and body solids) matching pred(shape) at every scale."""
    for sc in SCALES:
        rep = a["representations"][sc]
        if rep["status"] == "authored":
            for view in VIEWS:
                for s in rep["geometry"][view]:
                    if pred(s):
                        s["tone"] = tone
                        if drop_hatch:
                            s.pop("hatch", None)
                        if hatch:
                            s["hatch"] = hatch
        vol = a["volumes"][sc]
        if vol["status"] == "authored":
            for s in vol["body"]:
                if pred(s):
                    s["tone"] = tone


def advance_of(doc, clip_id):
    """🚶️ The planted travel of one cycle of a clip as the validator measures it."""
    import re
    path = write_tmp(doc, "t-adv.json")
    out = bun("validate.ts", path, env={"LIMIT": "400"}).stdout
    m = re.search(rf"advanceM [\d.e-]+ m differs from the planted travel ([\d.-]+) m", out)
    return float(m.group(1)) if m else None


def revolve_limbs(a, bones):
    """🦾️ Draws the given limb bones in the side and front elevation with the revolved solids of their plan shapes (the same body of
    revolution about the bone axis that the plan already uses), so a raised, twisted or spread limb reads true in every view."""
    for sc in SCALES:
        if a["representations"][sc]["status"] != "authored":
            continue
        plan = [s for s in shapes(a, sc, "plan") if s["bone"] in bones and (s.get("solid") or {}).get("kind") == "revolved"]
        for view in ("elevation", "front"):
            lst = shapes(a, sc, view)
            old = {s["id"]: s["z"] for s in lst if s["bone"] in bones}
            base_z = {b: min([s["z"] for s in lst if s["bone"] == b] or [0]) for b in bones}
            lst[:] = [s for s in lst if s["bone"] not in bones]
            for n, s in enumerate(plan):
                c = copy.deepcopy(s)
                c["z"] = old.get(s["id"], base_z[s["bone"]] + 0.001 * (n + 1))
                lst.append(c)
