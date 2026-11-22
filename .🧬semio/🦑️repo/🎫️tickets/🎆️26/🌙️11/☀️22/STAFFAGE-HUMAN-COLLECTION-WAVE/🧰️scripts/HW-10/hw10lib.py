"""🧰️ HW-10 generator helpers: load a base human, scale its anthropometry, strip or rename parts, add props, write the document.

The shipped JSON is produced only by the per-asset generators in this folder; see ../../📋️brief.md."""
import copy, json, math, os, subprocess

R = "/Users/ueli/Documents/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
CATALOG = f"{S}/🗂️catalog/👤️humans"
T = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
H = f"{T}/🧰️scripts/HW-10"
O = f"{T}/🗑️generated/HW-10"
BUN = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
SCALES = ["50", "100", "200", "500"]
VIEWS = ["elevation", "front", "plan"]
LISTS = ["body", "thermal", "structure"]


def load(slug):
    with open(f"{CATALOG}/{slug}/🔣️.json") as f:
        return json.load(f)


def write(doc, path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")


def rnd(v, d=5):
    return round(v, d)


def text(en, de):
    return {"en": en, "de": de}


def scale_shape(s, k):
    for key in ("x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1"):
        if key in s:
            s[key] = rnd(s[key] * k)
    if "points" in s:
        s["points"] = [[rnd(p[0] * k), rnd(p[1] * k)] for p in s["points"]]
    sol = s.get("solid")
    if sol:
        for key in ("axisX", "axisZ", "z0", "z1"):
            if key in sol:
                sol[key] = rnd(sol[key] * k)
        if "zTip" in sol:
            sol["zTip"] = [rnd(v * k) for v in sol["zTip"]]
    assert s["kind"] in ("poly", "capsule", "rect", "ellipse"), s["kind"]


def scale_solid(s, k):
    for key in ("cornerRadius", "r0", "r1", "from", "to"):
        if key in s:
            s[key] = rnd(s[key] * k, 6)
    for key in ("size", "radii", "p0", "p1", "origin"):
        if key in s:
            s[key] = [rnd(v * k, 6) for v in s[key]]
    if "profile" in s:
        s["profile"] = [[rnd(v * k, 6) for v in p] for p in s["profile"]]
    if "holes" in s:
        s["holes"] = [[[rnd(v * k, 6) for v in p] for p in h] for h in s["holes"]]
    for sec in s.get("sections", []):
        sec["at"] = [rnd(v * k, 6) for v in sec["at"]]
        sec["rx"] = rnd(sec["rx"] * k, 6)
        sec["ry"] = rnd(sec["ry"] * k, 6)
    tr = s.get("transform")
    if tr and "translation" in tr:
        tr["translation"] = [rnd(v * k, 6) for v in tr["translation"]]
    rp = s.get("repeat")
    if rp:
        for key in ("translation", "origin"):
            if key in rp:
                rp[key] = [rnd(v * k, 6) for v in rp[key]]


def scale_offsets(bones, k):
    for o in bones.values():
        for key in ("dx", "dy"):
            if key in o:
                o[key] = rnd(o[key] * k)


def scale_asset(a, k, mass_kg):
    """Scales every length of the figure by k and every segment mass so that the masses sum to mass_kg."""
    total = sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b)
    m = mass_kg / total
    for b in a["rig"]["bones"]:
        for key in ("x", "y", "z", "lengthM"):
            if key in b:
                b[key] = rnd(b[key] * k)
        if "mass" in b:
            b["mass"]["kg"] = rnd(b["mass"]["kg"] * m, 4)
            for key in ("comX", "comY"):
                if key in b["mass"]:
                    b["mass"][key] = rnd(b["mass"][key] * k)
    for an in a["anchors"]:
        for key in ("x", "y", "z", "footprintLengthM", "footprintWidthM"):
            if key in an:
                an[key] = rnd(an[key] * k)
    for r in a["representations"].values():
        if r["status"] != "authored":
            continue
        for v in VIEWS:
            for s in r["geometry"].get(v, []):
                scale_shape(s, k)
        for sec in (r.get("sections") or {}).values():
            sec["plane"] = rnd(sec["plane"] * k)
    for v in a["volumes"].values():
        if v["status"] != "authored":
            continue
        for lst in LISTS:
            for s in v[lst]:
                scale_solid(s, k)
    for p in a["poses"]:
        scale_offsets(p["bones"], k)
        if "root" in p:
            for key in ("x", "y"):
                if key in p["root"]:
                    p["root"][key] = rnd(p["root"][key] * k)
    for c in a["clips"]:
        for key in c["keys"]:
            scale_offsets(key.get("bones", {}), k)
        if "advanceM" in c:
            c["advanceM"] = rnd(c["advanceM"] * k)
    t = a["thermal"]
    for reg in t["regions"]:
        reg["areaM2"] = rnd(reg["areaM2"] * k * k, 4)
        reg["characteristicLengthM"] = rnd(reg["characteristicLengthM"] * k, 4)
    if "minimumAirPassageM" in t:
        t["minimumAirPassageM"] = rnd(t["minimumAirPassageM"] * k)
    for mo in t.get("respiration", {}).get("motion", []):
        mo["dy"] = rnd(mo["dy"] * k)
    d = a["dimensions"]
    for key in ("heightM", "widthM", "depthM"):
        d[key] = rnd(d[key] * k, 3)
    d["massKg"] = rnd(mass_kg, 2)


def drop(a, bone_ids=(), regions=(), materials=(), shape_pred=None, solid_pred=None):
    """Removes bones (and every shape, solid, anchor, pose and clip offset on them), regions, materials and the shapes/solids a predicate selects."""
    bone_ids, regions, materials = set(bone_ids), set(regions), set(materials)
    a["rig"]["bones"] = [b for b in a["rig"]["bones"] if b["id"] not in bone_ids]
    a["anchors"] = [x for x in a["anchors"] if x["bone"] not in bone_ids]
    keep_anchor = {x["id"] for x in a["anchors"]}

    def shape_out(s):
        return s["bone"] in bone_ids or s.get("region") in regions or (s.get("solid", {}).get("axisBone") in bone_ids) or (shape_pred and shape_pred(s))

    for r in a["representations"].values():
        if r["status"] != "authored":
            continue
        for v in VIEWS:
            shapes = [s for s in r["geometry"].get(v, []) if not shape_out(s)]
            for s in shapes:
                if "regions" in s:
                    s["regions"] = [x for x in s["regions"] if x not in regions]
            r["geometry"][v] = shapes
    for v in a["volumes"].values():
        if v["status"] != "authored":
            continue
        gone = set()
        for lst in LISTS:
            out = []
            for s in v[lst]:
                if s.get("bone") in bone_ids or s.get("material") in materials or s.get("region") in regions or s.get("of") in gone or (solid_pred and solid_pred(lst, s)) or (s.get("member") in bone_ids):
                    gone.add(s["id"])
                    continue
                if "weights" in s and any(w["bone"] in bone_ids for w in s["weights"]):
                    raise ValueError(f"weighted solid {s['id']} on dropped bone")
                if "members" in s:
                    s["members"] = [x for x in s["members"] if x not in bone_ids]
                    if not s["members"]:
                        del s["members"]
                out.append(s)
            v[lst] = out
    for p in a["poses"]:
        p["bones"] = {b: o for b, o in p["bones"].items() if b not in bone_ids}
        p["contacts"] = [c for c in p["contacts"] if c in keep_anchor]
        p["loads"] = [l for l in p["loads"] if l.get("anchor") in keep_anchor]
    for c in a["clips"]:
        for key in c["keys"]:
            if "bones" in key:
                key["bones"] = {b: o for b, o in key["bones"].items() if b not in bone_ids}
                if not key["bones"]:
                    del key["bones"]
    t = a["thermal"]
    t["regions"] = [x for x in t["regions"] if x["id"] not in regions]
    a["materials"] = [m for m in a["materials"] if m["id"] not in materials]


def shapes(a, scale, view):
    return a["representations"][scale]["geometry"][view]


def solids(a, scale, lst):
    return a["volumes"][scale][lst]


def bone(a, bid):
    return next(b for b in a["rig"]["bones"] if b["id"] == bid)


def pose(a, pid):
    return next(p for p in a["poses"] if p["id"] == pid)


def region_binding(a, exposed=None, overrides=None):
    out = {}
    for reg in a["thermal"]["regions"]:
        out[reg["id"]] = {"exposedFraction": (exposed or {}).get(reg["id"], 1), "sunlitFraction": 0}
    for rid, b in (overrides or {}).items():
        out[rid].update(b)
    return out


def run(cmd, timeout=900):
    p = subprocess.run(cmd, cwd=f"{S}/📦️packages/🟦️typescript", capture_output=True, text=True, timeout=timeout)
    return p.returncode, p.stdout + p.stderr


def probe(path, spec, bones=None):
    code, out = run([BUN, f"{H}/probe.ts", path, spec] + ([",".join(bones)] if bones else []))
    if code:
        raise RuntimeError(out)
    return json.loads(out.strip().splitlines()[-1])


def fit(spec):
    path = f"{O}/[DEBUG]-fit-spec-{os.getpid()}.json"
    write(spec, path)
    code, out = run([BUN, f"{H}/fit.ts", path])
    if code:
        raise RuntimeError(out)
    return json.loads(out.strip().splitlines()[-1])


def revolve_profile_shape(sid, bone_id, profile, tone, region, z, role="body", **extra):
    """A half profile [axial u, radial v] closed through the axis, drawn as a revolved solid about the bone axis (every view)."""
    pts = [[rnd(u), rnd(r)] for u, r in profile]
    s = {"id": sid, "bone": bone_id, "role": role, "z": z, "filled": True, "tone": tone}
    if isinstance(region, list):
        s["regions"] = region
    else:
        s["region"] = region
    s.update({"kind": "poly", "points": pts, "closed": True, "solid": {"kind": "revolved", "axisBone": bone_id, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}})
    s.update(extra)
    return s


def rename_region(a, old, new):
    for r in a["representations"].values():
        if r["status"] != "authored":
            continue
        for v in VIEWS:
            for s in r["geometry"].get(v, []):
                if s.get("region") == old:
                    s["region"] = new
                if "regions" in s:
                    s["regions"] = [new if x == old else x for x in s["regions"]]
    for v in a["volumes"].values():
        if v["status"] != "authored":
            continue
        for lst in LISTS:
            for s in v[lst]:
                if s.get("region") == old:
                    s["region"] = new
    for reg in a["thermal"]["regions"]:
        if reg["id"] == old:
            reg["id"] = new
    for e in a["thermal"]["emitters"]:
        if e.get("region") == old:
            e["region"] = new
    for p in a["poses"]:
        regs = p["thermal"]["regions"]
        if old in regs:
            regs[new] = regs.pop(old)


def rename_material(a, old, new):
    for m in a["materials"]:
        if m["id"] == old:
            m["id"] = new
    for r in a["representations"].values():
        if r["status"] != "authored":
            continue
        for v in VIEWS:
            for s in r["geometry"].get(v, []):
                if s.get("material") == old:
                    s["material"] = new
    for v in a["volumes"].values():
        if v["status"] != "authored":
            continue
        for lst in LISTS:
            for s in v[lst]:
                if s.get("material") == old:
                    s["material"] = new
    for reg in a["thermal"]["regions"]:
        if reg["material"] == old:
            reg["material"] = new
        for layer in reg["layers"]:
            if layer["material"] == old:
                layer["material"] = new


def rename_bone(a, old, new):
    if a["rig"]["root"] == old:
        a["rig"]["root"] = new
    for b in a["rig"]["bones"]:
        if b["id"] == old:
            b["id"] = new
        if b.get("parent") == old:
            b["parent"] = new
    for an in a["anchors"]:
        if an["bone"] == old:
            an["bone"] = new
    for r in a["representations"].values():
        if r["status"] != "authored":
            continue
        for v in VIEWS:
            for s in r["geometry"].get(v, []):
                if s["bone"] == old:
                    s["bone"] = new
                if s.get("solid", {}).get("axisBone") == old:
                    s["solid"]["axisBone"] = new
    for v in a["volumes"].values():
        if v["status"] != "authored":
            continue
        for lst in LISTS:
            for s in v[lst]:
                if s.get("bone") == old:
                    s["bone"] = new
                if s.get("member") == old:
                    s["member"] = new
                if "members" in s:
                    s["members"] = [new if x == old else x for x in s["members"]]
                for w in s.get("weights", []):
                    if w["bone"] == old:
                        w["bone"] = new
    for p in a["poses"]:
        if old in p["bones"]:
            p["bones"][new] = p["bones"].pop(old)
    for c in a["clips"]:
        for key in c["keys"]:
            if old in key.get("bones", {}):
                key["bones"][new] = key["bones"].pop(old)
    for mo in a["thermal"].get("respiration", {}).get("motion", []):
        if mo["bone"] == old:
            mo["bone"] = new
    st = a["structure"]
    st["supports"] = [new if x == old else x for x in st["supports"]]


def rest_pose(a, pid="rest"):
    p = copy.deepcopy(a["poses"][0])
    p["id"] = pid
    p["bones"] = {}
    p["key"] = True
    a["poses"].append(p)
    return p


def unit(v):
    n = math.sqrt(sum(x * x for x in v))
    return [x / n for x in v]


def frame_point(f, local):
    return [f["o"][k] + local[0] * f["u"][k] + local[1] * f["v"][k] + local[2] * f["w"][k] for k in range(3)]


def fit_pose(a, pid, arms, targets, rest=(), restarts=0, regular=2e-6, root_y=0.0):
    """Fits the listed pose fields (and rest fields) of pose `pid` in place; the current pose values are the start."""
    pose_ = pose(a, pid)
    vars_ = [{"scope": "pose", "bone": b, "field": f, "init": pose_["bones"].get(b, {}).get(f, 0), "lo": lo, "hi": hi} for b, f, lo, hi in arms]
    vars_ += [{"scope": "rest", "bone": b, "field": f, "init": bone(a, b).get(f, 0), "lo": lo, "hi": hi} for b, f, lo, hi in rest]
    res = fit({"asset": copy.deepcopy(a), "pose": pid, "vars": vars_, "targets": targets, "regular": regular, "restarts": restarts, "rootY": root_y})
    for v in res["values"]:
        if v["scope"] == "rest":
            bone(a, v["bone"])[v["field"]] = v["value"]
        else:
            pose_["bones"].setdefault(v["bone"], {})[v["field"]] = v["value"]
    print(pid, "cost", round(res["cost"], 6), [(t["kind"], t["bone"], t.get("world"), t["got"]) for t in res["report"]], flush=True)
    return res


def probe_asset(a, spec, tag):
    path = f"{O}/[DEBUG]-probe-{tag}.json"
    write({"version": "semio.staffage/v2", "asset": a}, path)
    return probe(path, spec)


ARM_R = [("arm-r", "angleDeg", -60, 170), ("arm-r", "abductionDeg", -80, 30), ("arm-r", "twistDeg", -90, 90), ("forearm-r", "angleDeg", 0, 150), ("forearm-r", "twistDeg", -90, 90), ("hand-r", "angleDeg", -60, 60), ("hand-r", "abductionDeg", -35, 35), ("hand-r", "twistDeg", -35, 35)]
ARM_L = [(b.replace("-r", "-l"), f, (-hi if f != "angleDeg" else lo), (-lo if f != "angleDeg" else hi)) for b, f, lo, hi in ARM_R]


def rename_layer(a, region, old, new):
    for reg in a["thermal"]["regions"]:
        if reg["id"] == region:
            for layer in reg["layers"]:
                if layer["id"] == old:
                    layer["id"] = new
    for v in a["volumes"].values():
        if v["status"] != "authored":
            continue
        for s in v["thermal"]:
            if s.get("region") == region and s.get("layer") == old:
                s["layer"] = new


def fit_dimensions(a, path):
    """Sets heightM, widthM and depthM to the middle of the default-pose extents of all scales and views, as the validator measures them."""
    code, out = run([BUN, f"{H}/extent.ts", path])
    ext = json.loads(out.strip().splitlines()[-1])
    mid = lambda keys: round((max(ext[s][k] for s in ext for k in keys) + min(ext[s][k] for s in ext for k in keys)) / 2, 3)
    a["dimensions"].update({"heightM": mid(["height", "frontV"]), "widthM": mid(["width", "planH"]), "depthM": mid(["frontH", "planV"]),
                            "massKg": round(sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b), 2)})
    print("dimensions", a["dimensions"], json.dumps(ext))


def issues_of(path):
    """Runs validate.ts on a draft and returns (code, path, message) of every issue."""
    p = subprocess.run([BUN, f"{H}/validate.ts", path], cwd=f"{S}/📦️packages/🟦️typescript", capture_output=True, text=True, timeout=900, env={**os.environ, "LIMIT": "5000"})
    out = []
    for line in p.stdout.splitlines():
        if line.startswith("   "):
            code, rest = line.strip().split(" ", 1)
            loc, msg = rest.split(": ", 1) if ": " in rest else ("", rest)
            out.append((code, loc, msg))
    return out


def restore_floor(a, path, factor):
    """Thickens the cross-sections of solids that scaling pushed just below the smallest feature of their scale back to it (floor solids of the template)."""
    import re
    fixed = 0
    for code, loc, msg in issues_of(path):
        m = re.match(r"volumes\.(\d+)\.(body|thermal|structure)\[(\d+)\]", loc)
        if code != "volumes-feature" or not m or "thin, below" not in msg:
            continue
        s = a["volumes"][m.group(1)][m.group(2)][int(m.group(3))]
        for key in ("r0", "r1"):
            if key in s:
                s[key] = rnd(s[key] * factor, 6)
        if "radii" in s:
            s["radii"] = [rnd(v * factor, 6) for v in s["radii"]]
        for sec in s.get("sections", []):
            sec["rx"], sec["ry"] = rnd(sec["rx"] * factor, 6), rnd(sec["ry"] * factor, 6)
        if "size" in s:
            s["size"] = [rnd(v * factor, 6) for v in s["size"]]
        fixed += 1
    return fixed
