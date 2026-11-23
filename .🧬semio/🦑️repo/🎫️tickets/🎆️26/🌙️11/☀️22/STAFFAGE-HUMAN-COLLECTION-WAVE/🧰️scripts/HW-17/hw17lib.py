"""🧰️ Shared helpers of worker HW-17: load a catalog human, scale it as a rigid similarity about the ground origin, move rig subtrees, write the asset.

The similarity scale is the only geometric derivation applied wholesale: every length of rig, anchors, the 2D shapes of all scales and views,
the 3D volumes, the thermal areas and lengths is multiplied by `s` (areas by s^2), so drawing and volume keep their exact agreement while the
person gets a new stature. Everything else (poses, clips, texts, materials, regions) is authored per asset in the asset generators.
See `../../📋️brief.md`, README of the staffage archive (Volume, Front and plan views)."""
import copy
import json
import math
import os
import re

R = "/Users/ueli/Documents/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
HUMANS = f"{S}/🗂️catalog/👤️humans"
T = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
GEN = f"{T}/🗑️generated/HW-17"
BUN = f"{R}/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
SCRIPTS = f"{T}/🧰️scripts/HW-17"


def tx(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def load(slug):
    """📥️ The asset document of a catalog human."""
    with open(f"{HUMANS}/{slug}/🔣️.json", encoding="utf8") as f:
        return json.load(f)


def write(slug, doc):
    """💾️ Writes the asset document into its own catalog folder (2-space JSON, UTF-8, trailing newline)."""
    os.makedirs(f"{HUMANS}/{slug}", exist_ok=True)
    with open(f"{HUMANS}/{slug}/🔣️.json", "w", encoding="utf8") as f:
        f.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")


def rnd(x, n=6):
    """🔢️ Rounds a coordinate to n decimals (micrometres) and normalises -0."""
    v = round(x, n)
    return 0 if v == 0 else v


SHAPE_KEYS = ("x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r")
SOLID_KEYS = ("axisX", "axisZ", "z0", "z1")
VOLUME_SCALARS = ("r0", "r1", "from", "to", "cornerRadius")
VOLUME_VECTORS = ("size", "radii", "p0", "p1", "origin")


def _scale_path(d, s):
    """〰️ Scales the numbers of an SVG path (arc flags and rotation kept)."""
    out, cmd, idx = [], None, 0
    for tok in re.findall(r"[MLHVCSQTAZmlhvcsqtaz]|-?\d*\.?\d+(?:e-?\d+)?", d):
        if tok.isalpha():
            cmd, idx = tok, 0
            out.append(tok)
            continue
        v = float(tok)
        if cmd in "Aa":
            k = idx % 7
            if k in (2, 3, 4):
                out.append(tok)
                idx += 1
                continue
        out.append(repr(rnd(v * s)))
        idx += 1
    return " ".join(out)


def scale_shape(sh, s):
    """✏️ Scales one authored shape (all primitives, solids and members)."""
    for k in SHAPE_KEYS:
        if k in sh and isinstance(sh[k], (int, float)):
            sh[k] = rnd(sh[k] * s)
    if "points" in sh:
        sh["points"] = [[rnd(p[0] * s), rnd(p[1] * s)] for p in sh["points"]]
    if "d" in sh:
        sh["d"] = _scale_path(sh["d"], s)
    if sh.get("kind") == "member":
        for end in ("from", "to"):
            for k in ("x", "y", "z"):
                sh[end][k] = rnd(sh[end][k] * s)
        sec = sh["section"]
        for k in ("w", "h", "d"):
            if k in sec:
                sec[k] = rnd(sec[k] * s)
    so = sh.get("solid")
    if so:
        for k in SOLID_KEYS:
            if k in so:
                so[k] = rnd(so[k] * s)
        if "zTip" in so:
            so["zTip"] = [rnd(z * s) for z in so["zTip"]]


def scale_solid(so, s):
    """🧊️ Scales one volume solid in its bone frame."""
    if "of" in so and "kind" not in so:
        return
    if "emitter" in so.get("tags", []):
        tr = so.get("transform")
        if tr and "translation" in tr:
            tr["translation"] = [rnd(c * s) for c in tr["translation"]]
        return
    for k in VOLUME_SCALARS:
        if k in so:
            so[k] = rnd(so[k] * s)
    for k in VOLUME_VECTORS:
        if k in so:
            so[k] = [rnd(c * s) for c in so[k]]
    for k in ("profile",):
        if k in so:
            so[k] = [[rnd(c * s) for c in p] for p in so[k]]
    if "holes" in so:
        so["holes"] = [[[rnd(c * s) for c in p] for p in ring] for ring in so["holes"]]
    if "vertices" in so:
        so["vertices"] = [[rnd(c * s) for c in p] for p in so["vertices"]]
    for sec in so.get("sections", []):
        sec["at"] = [rnd(c * s) for c in sec["at"]]
        sec["rx"] = rnd(sec["rx"] * s)
        sec["ry"] = rnd(sec["ry"] * s)
    tr = so.get("transform")
    if tr and "translation" in tr:
        tr["translation"] = [rnd(c * s) for c in tr["translation"]]
    rp = so.get("repeat")
    if rp:
        for k in ("translation", "origin"):
            if k in rp:
                rp[k] = [rnd(c * s) for c in rp[k]]


def scale_asset(a, s, mass_factor):
    """📐️ Applies the similarity `s` about the ground origin and multiplies every segment mass by `mass_factor`."""
    for b in a["rig"]["bones"]:
        for k in ("x", "y", "z", "lengthM"):
            if k in b:
                b[k] = rnd(b[k] * s)
        m = b.get("mass")
        if m:
            m["kg"] = rnd(m["kg"] * mass_factor, 4)
            m["comX"] = rnd(m["comX"] * s)
            m["comY"] = rnd(m["comY"] * s)
        sec = b.get("section")
        if sec:
            sec["areaM2"] = rnd(sec["areaM2"] * s * s, 9)
            sec["secondMomentM4"] = sec["secondMomentM4"] * s ** 4
            sec["extremeFibreM"] = rnd(sec["extremeFibreM"] * s)
        j = b["joint"]
        for k in ("minM", "maxM"):
            if k in j:
                j[k] = rnd(j[k] * s)
    for an in a["anchors"]:
        for k in ("x", "y", "z", "footprintLengthM", "footprintWidthM"):
            if k in an:
                an[k] = rnd(an[k] * s)
    for rep in a["representations"].values():
        if rep["status"] != "authored":
            continue
        for view in rep["geometry"].values():
            for sh in view:
                scale_shape(sh, s)
        for sec in (rep.get("sections") or {}).values():
            sec["plane"] = rnd(sec["plane"] * s)
    for sym in a.get("symbols", []):
        for view in (sym.get("geometry") or {}).values():
            for sh in view:
                scale_shape(sh, s)
    for vol in a["volumes"].values():
        if vol["status"] != "authored":
            continue
        for lst in ("body", "thermal", "structure"):
            for so in vol.get(lst, []):
                scale_solid(so, s)
    t = a["thermal"]
    for reg in t["regions"]:
        reg["areaM2"] = rnd(reg["areaM2"] * s * s, 4)
        reg["characteristicLengthM"] = rnd(reg["characteristicLengthM"] * s, 4)
        for lay in reg.get("layers", []):
            if "thicknessM" in lay:
                lay["thicknessM"] = rnd(lay["thicknessM"] * s, 6)
    if "minimumAirPassageM" in t:
        t["minimumAirPassageM"] = rnd(t["minimumAirPassageM"] * s)
    for em in t["emitters"]:
        if em["kind"] == "plume":
            em["diameterM"] = rnd(em["diameterM"] * s, 4)
    for resp in [t.get("respiration")] + [p.get("respiration") for p in t.get("passengers", [])]:
        for mo in (resp or {}).get("motion", []):
            if "dy" in mo:
                mo["dy"] = rnd(mo["dy"] * s, 5)
            if "dx" in mo:
                mo["dx"] = rnd(mo["dx"] * s, 5)
    for p in a["poses"]:
        for off in p["bones"].values():
            for k in ("dx", "dy"):
                if k in off:
                    off[k] = rnd(off[k] * s)
        if "root" in p:
            for k in ("x", "y"):
                if k in p["root"]:
                    p["root"][k] = rnd(p["root"][k] * s)
    for c in a["clips"]:
        if "advanceM" in c:
            c["advanceM"] = rnd(c["advanceM"] * s, 4)
        for key in c["keys"]:
            for off in (key.get("bones") or {}).values():
                for k in ("dx", "dy"):
                    if k in off:
                        off[k] = rnd(off[k] * s)
    d = a["dimensions"]
    for k in ("heightM", "widthM", "depthM"):
        d[k] = rnd(d[k] * s, 4)


def frames(path, poses=()):
    """🦴️ Rest and posed world frames of an asset file through the library (frames.ts)."""
    import subprocess
    out = subprocess.run([BUN, f"{SCRIPTS}/frames.ts", path, *poses], capture_output=True, text=True, cwd=SCRIPTS)
    if out.returncode != 0:
        raise RuntimeError(out.stderr[-3000:])
    return json.loads(out.stdout)


def mass_sum(a):
    """⚖️ Sum of the segment masses of the rig."""
    return sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b)


def dubois(mass_kg, height_m):
    """🧍️ Skin area after DuBois and DuBois (1916), m^2."""
    return 0.007184 * mass_kg ** 0.425 * (height_m * 100) ** 0.725


def shapes_of(a, bones):
    """🔎️ Every (scale, view, shape) bound to one of `bones`."""
    for key, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        for view, lst in rep["geometry"].items():
            for sh in lst:
                if sh["bone"] in bones:
                    yield key, view, sh


def subtree(a, root):
    """🌳️ The ids of `root` and all its descendants."""
    kids = {}
    for b in a["rig"]["bones"]:
        kids.setdefault(b["parent"], []).append(b["id"])
    out, stack = [], [root]
    while stack:
        n = stack.pop()
        out.append(n)
        stack.extend(kids.get(n, []))
    return out


def shift_rest(a, root, d):
    """🚚️ Moves the rest placement of the subtree `root` by the world translation d = (dx ahead, dy up, dz left) without turning it.

    Front shapes (lateral, up) and plan shapes (ahead, lateral) of the subtree live in rest world coordinates and move by the projected
    translation; extruded solids sweeping `w` move their rest lateral span; elevation shapes and volume solids live in bone frames and stay.
    The caller re-places the subtree root bone in its parent (`x`, `y`, `z`, `angleDeg`) so that its rest frame moves by exactly d."""
    ids = set(subtree(a, root))
    dx, dy, dz = d
    for key, view, sh in shapes_of(a, ids):
        if view == "elevation":
            so = sh.get("solid")
            if so and so.get("kind") == "extruded" and so.get("sweep") == "w":
                so["z0"] = rnd(so["z0"] + dz)
                so["z1"] = rnd(so["z1"] + dz)
                if "zTip" in so:
                    so["zTip"] = [rnd(z + dz) for z in so["zTip"]]
            continue
        ox, oy = (dz, dy) if view == "front" else (dx, dz)
        so = sh.get("solid")
        if so and so.get("kind") == "extruded" and so.get("sweep") == "w":
            so["z0"] = rnd(so["z0"] + dz)
            so["z1"] = rnd(so["z1"] + dz)
            if "zTip" in so:
                so["zTip"] = [rnd(z + dz) for z in so["zTip"]]
        if so:
            continue
        if sh["kind"] == "poly":
            sh["points"] = [[rnd(p[0] + ox), rnd(p[1] + oy)] for p in sh["points"]]
        elif sh["kind"] == "capsule":
            for kx, ky in (("x0", "y0"), ("x1", "y1")):
                sh[kx] = rnd(sh[kx] + ox)
                sh[ky] = rnd(sh[ky] + oy)
        elif sh["kind"] == "ellipse":
            sh["cx"] = rnd(sh["cx"] + ox)
            sh["cy"] = rnd(sh["cy"] + oy)
        elif sh["kind"] == "rect":
            sh["x"] = rnd(sh["x"] + ox)
            sh["y"] = rnd(sh["y"] + oy)
        elif sh["kind"] == "member":
            pass
        else:
            raise ValueError(f"shift_rest: unsupported {sh['kind']} {sh['id']}")


def numeric_paths(o, p=""):
    """🧾️ Every numeric leaf path of a JSON value (for the untouched-key audit of the scaler)."""
    if isinstance(o, dict):
        for k, v in o.items():
            yield from numeric_paths(v, f"{p}.{k}")
    elif isinstance(o, list):
        for i, v in enumerate(o):
            yield from numeric_paths(v, f"{p}[{i}]")
    elif isinstance(o, (int, float)) and not isinstance(o, bool):
        yield p, o


def shape_extents(slug, pose, scale, view, filt):
    """📏️ Drawn ('D') and volume ('V') extents per id in one view through shape_ext.ts: {(kind, id): (x0, x1, y0, y1)}."""
    import subprocess
    out = subprocess.run([BUN, f"{SCRIPTS}/shape_ext.ts", slug, pose, str(scale), view, filt], capture_output=True, text=True, cwd=SCRIPTS)
    if out.returncode != 0:
        raise RuntimeError(out.stderr[-3000:])
    res = {}
    for line in out.stdout.splitlines():
        p = line.split()
        if len(p) >= 8:
            res.setdefault((p[0], p[1]), tuple(float(v) for v in (p[3], p[4], p[6], p[7])))
    return res


def fit_lateral(slug, pose, scale, view, drawn_id, volume_id, field):
    """🎯️ Lateral offset (m) by which a drawn front or plan shape stands beside its volume solid in a pose (centre to centre); `field` 'x' for front, 'y' for plan."""
    e = shape_extents(slug, pose, scale, view, drawn_id.rsplit("-", 1)[0] if drawn_id == volume_id else drawn_id)
    if volume_id != drawn_id:
        e.update(shape_extents(slug, pose, scale, view, volume_id))
    d, v = e[("D", drawn_id)], e[("V", volume_id)]
    i = 0 if field == "x" else 2
    return ((d[i] + d[i + 1]) - (v[i] + v[i + 1])) / 2


def volume_dims(slug, pose, scale=50):
    """📦️ Width (ahead), height and depth (lateral) of the volume of one scale in a pose, from the side elevation and the front."""
    el = [v for k, v in shape_extents(slug, pose, scale, "elevation", "").items() if k[0] == "V"]
    fr = [v for k, v in shape_extents(slug, pose, scale, "front", "").items() if k[0] == "V"]
    return (max(v[1] for v in el) - min(v[0] for v in el), max(v[3] for v in el) - min(v[2] for v in el), max(v[1] for v in fr) - min(v[0] for v in fr))


def fit_arms(slug, pose, ids):
    """🎯️ Moves the front (extruded profile w, or revolved axisZ as `standing-adult` has it since 2026-11-22; the sense of the axis is measured, not assumed) and plan (revolved axisZ) drawings of the upper arms onto their volume in the default pose, per scale.

    `ids` maps a scale key to (drawn id pattern with {s}, volume id pattern with {s})."""
    doc = load(slug)
    a = doc["asset"]
    flips = []
    for key, (did, vid) in ids.items():
        for side in ("l", "r"):
            dfront = fit_lateral(slug, pose, key, "front", did.format(s=side), vid.format(s=side), "x")
            dplan = fit_lateral(slug, pose, key, "plan", did.format(s=side), vid.format(s=side), "y")
            for view, delta in (("front", dfront), ("plan", dplan)):
                for sh in a["representations"][key]["geometry"][view]:
                    if sh["id"] != did.format(s=side):
                        continue
                    so = sh["solid"]
                    if view == "front" and so["kind"] == "extruded" and so["sweep"] == "v":
                        sh["points"] = [[p[0], rnd(p[1] - delta)] for p in sh["points"]]
                    elif view == "plan" and so["kind"] == "revolved":
                        so["axisZ"] = rnd(so.get("axisZ", 0) - delta)
                    elif view == "front" and so["kind"] == "revolved":
                        flips.append((key, did.format(s=side), vid.format(s=side), delta))
                        so["axisZ"] = rnd(so.get("axisZ", 0) - delta)
                    else:
                        raise ValueError(f"fit_arms: unexpected solid of {sh['id']} in {view}")
            print(f"[DEBUG] fit {slug} 1:{key} {side}: front {dfront * 1000:.1f} mm, plan {dplan * 1000:.1f} mm")
    write(slug, doc)
    turned = False
    for key, did, vid, delta in flips:
        left = fit_lateral(slug, pose, key, "front", did, vid, "x")
        if abs(left) > abs(delta) + 1e-4:
            for sh in a["representations"][key]["geometry"]["front"]:
                if sh["id"] == did:
                    sh["solid"]["axisZ"] = rnd(sh["solid"]["axisZ"] + 2 * delta)
                    turned = True
        print(f"[DEBUG] fit {slug} 1:{key} {did}: front revolved axis moved by {delta * 1000:.1f} mm, {left * 1000:.1f} mm left{' (turned)' if abs(left) > abs(delta) + 1e-4 else ''}")
    if turned:
        write(slug, doc)


def solve_pose(doc, base_pose, make, residual, x0, h=0.25, iters=8, tol=2e-4):
    """🧮️ Gauss-Newton on pose parameters through the library's own kinematics: `make(x)` returns bone offsets laid over `base_pose`,
    `residual(state)` a list of metres to drive to zero (state as frames.ts prints one pose). One frames.ts call per iteration."""
    import copy as _c
    x = list(x0)
    for _ in range(iters):
        d = _c.deepcopy(doc)
        a = d["asset"]
        base = next(p for p in a["poses"] if p["id"] == base_pose)
        trial = [x] + [[x[j] + (h if j == i else 0) for j in range(len(x))] for i in range(len(x))]
        ids = []
        for k, xx in enumerate(trial):
            p = _c.deepcopy(base)
            p["id"] = f"solve-{k}"
            p["bones"] = {**p["bones"], **make(xx)}
            p.pop("key", None)
            a["poses"].append(p)
            ids.append(p["id"])
        path = f"{GEN}/solve.json"
        with open(path, "w") as f:
            json.dump(d, f)
        fr = frames(path, ids)
        r0 = residual(fr[ids[0]])
        if max(abs(v) for v in r0) < tol:
            return x, r0
        J = [[(residual(fr[ids[i + 1]])[m] - r0[m]) / h for i in range(len(x))] for m in range(len(r0))]
        n = len(x)
        A = [[sum(J[m][i] * J[m][j] for m in range(len(r0))) + (1e-9 if i == j else 0) for j in range(n)] for i in range(n)]
        b = [-sum(J[m][i] * r0[m] for m in range(len(r0))) for i in range(n)]
        for i in range(n):
            piv = max(range(i, n), key=lambda r: abs(A[r][i]))
            A[i], A[piv] = A[piv], A[i]
            b[i], b[piv] = b[piv], b[i]
            for r in range(i + 1, n):
                f_ = A[r][i] / A[i][i]
                for c in range(i, n):
                    A[r][c] -= f_ * A[i][c]
                b[r] -= f_ * b[i]
        dx = [0.0] * n
        for i in reversed(range(n)):
            dx[i] = (b[i] - sum(A[i][c] * dx[c] for c in range(i + 1, n))) / A[i][i]
        x = [x[i] + dx[i] for i in range(n)]
    return x, r0


def grid_pose(doc, base_pose, make, residual, grid):
    """🔎️ Evaluates every parameter tuple of `grid` in one frames.ts call and returns them sorted by the residual norm."""
    import copy as _c
    d = _c.deepcopy(doc)
    a = d["asset"]
    base = next(p for p in a["poses"] if p["id"] == base_pose)
    ids = []
    for k, xx in enumerate(grid):
        p = _c.deepcopy(base)
        p["id"] = f"grid-{k}"
        p["bones"] = {**p["bones"], **make(xx)}
        p.pop("key", None)
        a["poses"].append(p)
        ids.append(p["id"])
    path = f"{GEN}/grid.json"
    with open(path, "w") as f:
        json.dump(d, f)
    fr = frames(path, ids)
    out = [(sum(v * v for v in residual(fr[i])) ** 0.5, xx) for i, xx in zip(ids, grid)]
    return sorted(out)


def solid_w_span(vol, bone):
    """↔️ The lateral span (bone w) of the body solids of one bone in one scale's volume, bone-local metres."""
    lo, hi = float("inf"), float("-inf")
    for so in vol["body"]:
        if so.get("bone") != bone or "kind" not in so:
            continue
        t = (so.get("transform") or {}).get("translation", [0, 0, 0])
        k = so["kind"]
        if k == "capsule":
            for p, r in ((so["p0"], so["r0"]), (so["p1"], so["r1"])):
                lo, hi = min(lo, t[2] + p[2] - r), max(hi, t[2] + p[2] + r)
        elif k == "ellipsoid":
            r = max(so["radii"])
            lo, hi = min(lo, t[2] - r), max(hi, t[2] + r)
        elif k == "box":
            r = max(so["size"]) / 2
            lo, hi = min(lo, t[2] - r), max(hi, t[2] + r)
        elif k == "loft":
            for s in so["sections"]:
                r = max(s["rx"], s["ry"])
                lo, hi = min(lo, t[2] + s["at"][2] - r), max(hi, t[2] + s["at"][2] + r)
        elif k == "revolve":
            r = max(abs(p[1]) for p in so["profile"]) * so.get("lateralScale", 1)
            o = so.get("origin", [0, 0, 0])
            lo, hi = min(lo, t[2] + o[2] - r), max(hi, t[2] + o[2] + r)
        elif k == "extrude":
            pl = so.get("plane", "xy")
            if pl == "xy":
                lo, hi = min(lo, t[2] + so["from"]), max(hi, t[2] + so["to"])
            else:
                ws = [p[1] if pl == "yz" else p[0] for p in so["profile"]]
                lo, hi = min(lo, t[2] + min(ws)), max(hi, t[2] + max(ws))
    return (lo, hi) if lo < hi else None


def extrude_views(a, bones, rest):
    """🧱️ Re-authors the drawings of `bones` as solids: every filled elevation shape of such a bone becomes an extrusion across the bone's true
    lateral span (from that scale's body solids, at its rest lateral position), and the same profiles with the same solids replace the flat front
    and plan shapes of the bone, so all three views follow the bone through any spread, twist or reparenting. Unfilled strokes stay elevation lines."""
    for key, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        vol = a["volumes"][key]
        g = rep["geometry"]
        for b in bones:
            span = solid_w_span(vol, b)
            if span is None:
                continue
            oz = rest[b]["o"][2]
            sol = {"kind": "extruded", "sweep": "w", "z0": rnd(oz + span[0]), "z1": rnd(oz + span[1])}
            elev = [s for s in g["elevation"] if s["bone"] == b]
            for s in elev:
                if s["filled"] and "solid" not in s:
                    s["solid"] = dict(sol)
            for view in ("front", "plan"):
                keep = [s for s in g[view] if s["bone"] != b or "solid" in s]
                have = {s["id"] for s in keep if s["bone"] == b}
                add = []
                for s in elev:
                    if s["filled"] and s["role"] == "body" and s["id"] not in have:
                        c = copy.deepcopy(s)
                        add.append(c)
                g[view] = keep + add


def fit_spans(slug, pose, items, rounds=2):
    """🎯️ Narrows and centres the lateral span (z0..z1) of extruded-w drawings so their front extent matches their volume solid in the default pose.
    `items` lists (scale key, drawn id, volume id)."""
    for _ in range(rounds):
        doc = load(slug)
        a = doc["asset"]
        for key, did, vid in items:
            e = shape_extents(slug, pose, key, "front", did)
            e.update(shape_extents(slug, pose, key, "front", vid))
            if ("D", did) not in e or ("V", vid) not in e:
                continue
            d, v = e[("D", did)], e[("V", vid)]
            f = (v[1] - v[0]) / (d[1] - d[0])
            shift = ((v[0] + v[1]) - (d[0] + d[1])) / 2
            for view in ("elevation", "front", "plan"):
                for sh in a["representations"][key]["geometry"][view]:
                    so = sh.get("solid")
                    if sh["id"] == did and so and so["kind"] == "extruded" and so["sweep"] == "w":
                        c, h = (so["z0"] + so["z1"]) / 2 + shift, (so["z1"] - so["z0"]) / 2 * f
                        so["z0"], so["z1"] = rnd(c - h), rnd(c + h)
            print(f"[DEBUG] span {slug} 1:{key} {did}: x{f:.3f} shift {shift * 1000:.1f} mm")
        write(slug, doc)
