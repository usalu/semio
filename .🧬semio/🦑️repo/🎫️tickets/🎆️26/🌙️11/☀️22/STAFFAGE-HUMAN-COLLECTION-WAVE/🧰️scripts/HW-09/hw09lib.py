"""🧩️ HW-09 composition helpers: take a bone subtree of an existing staffage asset (rig, anchors, shapes per scale and view,
volume solids per scale and list, thermal regions, materials, hatches) so a new handcrafted person can be assembled from the
closest existing figure and an existing vehicle, placed, renamed and anthropometrically scaled.

See /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/README.md (Front and plan views, Solid placements, Volume).
"""
import copy
import json
import re
from pathlib import Path

R = Path("/Users/ueli/Documents/semio")
S = R / "🎓️teaching/🏛️architecture/🧍️staffage"
CAT = S / "🗂️catalog"
T = R / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
SCALES = ["50", "100", "200", "500"]
VIEWS = ["elevation", "front", "plan"]
LISTS = ["body", "thermal", "structure"]


def load(category, slug):
    """📥️ The asset of `🗂️catalog/<category>/<slug>/🔣️.json`."""
    return json.loads((CAT / category / slug / "🔣️.json").read_text())["asset"]


def write(slug, asset):
    """💾️ Writes the asset document of a human with the catalog's formatting (2 spaces, UTF-8, trailing newline)."""
    folder = CAT / "👤️humans" / slug
    folder.mkdir(parents=True, exist_ok=True)
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (folder / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")


def subtree(asset, roots):
    """🌳️ The ids of `roots` and every descendant bone."""
    children = {}
    for b in asset["rig"]["bones"]:
        children.setdefault(b["parent"], []).append(b["id"])
    out, stack = [], list(roots)
    while stack:
        b = stack.pop(0)
        out.append(b)
        stack.extend(children.get(b, []))
    return out


def solid_bones(solid):
    """🦴️ Bones a volume solid depends on (its bone and its weights)."""
    out = {solid["bone"]} if "bone" in solid else set()
    out |= {w["bone"] for w in solid.get("weights", [])}
    return out


def take(asset, bones, scales=SCALES):
    """✂️ Everything of `asset` bound to `bones`: rig bones, anchors, shapes per scale/view, solids per scale/list, the regions,
    materials and hatches those use. Thermal/structure `of` references follow their body solid."""
    bones = list(bones)
    keep = set(bones)
    part = {"bones": [copy.deepcopy(b) for b in asset["rig"]["bones"] if b["id"] in keep]}
    part["anchors"] = [copy.deepcopy(a) for a in asset["anchors"] if a["bone"] in keep]
    part["shapes"], part["solids"], part["hatches"], part["pens"] = {}, {}, {}, {}
    for s in scales:
        rep = asset["representations"][s]
        part["shapes"][s] = {}
        if rep["status"] == "authored":
            part["pens"][s] = rep["pens"]
            for v in VIEWS:
                part["shapes"][s][v] = [copy.deepcopy(x) for x in rep["geometry"].get(v, []) if x["bone"] in keep]
            used = {x.get("hatch") for v in VIEWS for x in part["shapes"][s][v]}
            part["hatches"][s] = [copy.deepcopy(h) for h in rep.get("hatches", []) if h["id"] in used]
        vol = asset["volumes"][s]
        part["solids"][s] = {}
        if vol["status"] == "authored":
            body = [copy.deepcopy(x) for x in vol["body"] if solid_bones(x) <= keep]
            ids = {x["id"] for x in body}
            part["solids"][s]["body"] = body
            for lst in ["thermal", "structure"]:
                out = []
                for x in vol[lst]:
                    if "of" in x:
                        if x["of"] not in ids:
                            continue
                    elif not solid_bones(x) <= keep:
                        continue
                    if lst == "structure":
                        members = [x["member"]] + x.get("members", [])
                        if not set(members) <= keep:
                            continue
                    out.append(copy.deepcopy(x))
                part["solids"][s][lst] = out
    regions = set()
    materials = set()
    for s in scales:
        for v in VIEWS:
            for x in part["shapes"][s].get(v, []):
                regions |= {x["region"]} if "region" in x else set()
                regions |= set(x.get("regions", []))
                materials |= {x["material"]} if "material" in x else set()
        for lst in LISTS:
            for x in part["solids"][s].get(lst, []):
                regions |= {x["region"]} if "region" in x else set()
                materials |= {x["material"]} if "material" in x else set()
    part["regions"] = [copy.deepcopy(r) for r in asset["thermal"]["regions"] if r["id"] in regions]
    for r in part["regions"]:
        materials.add(r["material"])
        materials |= {l["material"] for l in r.get("layers", []) if "material" in l}
    for b in part["bones"]:
        if "section" in b:
            materials.add(b["section"]["material"])
    part["materials"] = [copy.deepcopy(m) for m in asset["materials"] if m["id"] in materials]
    return part


def shift_rest(part, dx=0.0, dy=0.0, dz=0.0):
    """↔️ Moves the rest placement of a part by a world translation (its root bone is moved by the caller): front shapes without a
    solid are world (lateral, up), plan shapes world (ahead, lateral); extruded `w` solids name absolute rest lateral positions."""
    for s, views in part["shapes"].items():
        for v, shapes in views.items():
            for x in shapes:
                sol = x.get("solid")
                if sol is not None:
                    if sol["kind"] == "extruded" and sol["sweep"] == "w" and dz != 0:
                        sol["z0"] = round(sol["z0"] + dz, 6)
                        sol["z1"] = round(sol["z1"] + dz, 6)
                        if "zTip" in sol:
                            sol["zTip"] = [round(sol["zTip"][0] + dz, 6), round(sol["zTip"][1] + dz, 6)]
                    continue
                if v == "elevation" or x["kind"] == "member":
                    continue
                ax, ay = (dz, dy) if v == "front" else (dx, dz)
                translate_shape(x, ax, ay)


def translate_shape(x, ax, ay):
    """➕️ Translates a planar shape by (ax, ay) in its own drawing coordinates."""
    k = x["kind"]
    if k == "poly":
        x["points"] = [[round(p[0] + ax, 6), round(p[1] + ay, 6)] for p in x["points"]]
    elif k == "ellipse":
        x["cx"], x["cy"] = round(x["cx"] + ax, 6), round(x["cy"] + ay, 6)
    elif k == "rect":
        x["x"], x["y"] = round(x["x"] + ax, 6), round(x["y"] + ay, 6)
    elif k == "capsule":
        for a, b in (("x0", "y0"), ("x1", "y1")):
            x[a], x[b] = round(x[a] + ax, 6), round(x[b] + ay, 6)
    elif k == "path":
        x["d"] = map_path(x["d"], lambda px, py: (px + ax, py + ay), lambda r: r)
    else:
        raise ValueError(f"cannot translate shape kind {k}")


NUM = r"-?\d*\.?\d+(?:e-?\d+)?"


def map_path(d, point, length):
    """🛤️ Maps an SVG-like path: absolute commands M L C Q A Z (and H V) with `point` for coordinates and `length` for radii."""
    tokens = re.findall(r"[A-Za-z]|" + NUM, d)
    out, i, cmd = [], 0, None
    while i < len(tokens):
        t = tokens[i]
        if re.match(r"[A-Za-z]", t):
            cmd = t
            out.append(t)
            i += 1
            continue
        if cmd in "MLCQST":
            px, py = point(float(tokens[i]), float(tokens[i + 1]))
            out += [f"{px:.6g}", f"{py:.6g}"]
            i += 2
        elif cmd == "A":
            rx, ry = length(float(tokens[i])), length(float(tokens[i + 1]))
            px, py = point(float(tokens[i + 5]), float(tokens[i + 6]))
            out += [f"{rx:.6g}", f"{ry:.6g}", tokens[i + 2], tokens[i + 3], tokens[i + 4], f"{px:.6g}", f"{py:.6g}"]
            i += 7
        else:
            raise ValueError(f"unsupported path command {cmd} in {d[:60]}")
    return " ".join(out)


def rename(part, mapping, kinds):
    """🏷️ Renames ids inside a part. `kinds` ⊆ {shape, solid, region, material, hatch, anchor, bone}."""
    m = mapping
    if "bone" in kinds:
        for b in part["bones"]:
            b["id"] = m.get(b["id"], b["id"])
            if b["parent"] is not None:
                b["parent"] = m.get(b["parent"], b["parent"])
        for a in part["anchors"]:
            a["bone"] = m.get(a["bone"], a["bone"])
    for s in part["shapes"]:
        for v in part["shapes"][s]:
            for x in part["shapes"][s][v]:
                if "shape" in kinds:
                    x["id"] = m.get(x["id"], x["id"])
                if "bone" in kinds:
                    x["bone"] = m.get(x["bone"], x["bone"])
                    if x.get("solid", {}).get("axisBone"):
                        x["solid"]["axisBone"] = m.get(x["solid"]["axisBone"], x["solid"]["axisBone"])
                    for e in ("from", "to"):
                        if e in x and "bone" in x[e]:
                            x[e]["bone"] = m.get(x[e]["bone"], x[e]["bone"])
                if "region" in kinds:
                    if "region" in x:
                        x["region"] = m.get(x["region"], x["region"])
                    if "regions" in x:
                        x["regions"] = [m.get(r, r) for r in x["regions"]]
                if "material" in kinds and "material" in x:
                    x["material"] = m.get(x["material"], x["material"])
                if "hatch" in kinds and "hatch" in x:
                    x["hatch"] = m.get(x["hatch"], x["hatch"])
        if "hatch" in kinds:
            for h in part["hatches"].get(s, []):
                h["id"] = m.get(h["id"], h["id"])
    for s in part["solids"]:
        for lst in part["solids"][s]:
            for x in part["solids"][s][lst]:
                if "solid" in kinds:
                    x["id"] = m.get(x["id"], x["id"])
                    if "of" in x:
                        x["of"] = m.get(x["of"], x["of"])
                if "bone" in kinds:
                    if "bone" in x:
                        x["bone"] = m.get(x["bone"], x["bone"])
                    for w in x.get("weights", []):
                        w["bone"] = m.get(w["bone"], w["bone"])
                    if "member" in x:
                        x["member"] = m.get(x["member"], x["member"])
                    if "members" in x:
                        x["members"] = [m.get(b, b) for b in x["members"]]
                if "anchor" in kinds and "anchor" in x:
                    x["anchor"] = m.get(x["anchor"], x["anchor"])
                if "region" in kinds and "region" in x:
                    x["region"] = m.get(x["region"], x["region"])
                if "material" in kinds and "material" in x:
                    x["material"] = m.get(x["material"], x["material"])
    if "region" in kinds:
        for r in part["regions"]:
            r["id"] = m.get(r["id"], r["id"])
    if "material" in kinds:
        for mat in part["materials"]:
            mat["id"] = m.get(mat["id"], mat["id"])
        for r in part["regions"]:
            r["material"] = m.get(r["material"], r["material"])
            for l in r.get("layers", []):
                if "material" in l:
                    l["material"] = m.get(l["material"], l["material"])
        for b in part["bones"]:
            if "section" in b:
                b["section"]["material"] = m.get(b["section"]["material"], b["section"]["material"])
    if "anchor" in kinds:
        for a in part["anchors"]:
            a["id"] = m.get(a["id"], a["id"])


def r6(v):
    return round(v, 6)


def scale_shape(x, k):
    """📏️ Scales a planar shape (bone-local or world rest coordinates) and its solid placement by k."""
    kind = x["kind"]
    if kind == "poly":
        x["points"] = [[r6(p[0] * k), r6(p[1] * k)] for p in x["points"]]
    elif kind == "ellipse":
        for f in ("cx", "cy", "rx", "ry"):
            x[f] = r6(x[f] * k)
    elif kind == "rect":
        for f in ("x", "y", "w", "h", "r"):
            if f in x:
                x[f] = r6(x[f] * k)
    elif kind == "capsule":
        for f in ("x0", "y0", "x1", "y1", "r0", "r1"):
            x[f] = r6(x[f] * k)
    elif kind == "path":
        x["d"] = map_path(x["d"], lambda px, py: (px * k, py * k), lambda r: r * k)
    elif kind == "member":
        for e in ("from", "to"):
            for f in ("x", "y", "z"):
                x[e][f] = r6(x[e][f] * k)
        sec = x["section"]
        for f in ("w", "h", "d"):
            if f in sec:
                sec[f] = r6(sec[f] * k)
    sol = x.get("solid")
    if sol:
        if sol["kind"] == "extruded":
            sol["z0"], sol["z1"] = r6(sol["z0"] * k), r6(sol["z1"] * k)
            if "zTip" in sol:
                sol["zTip"] = [r6(sol["zTip"][0] * k), r6(sol["zTip"][1] * k)]
        else:
            sol["axisX"] = r6(sol["axisX"] * k)
            if "axisZ" in sol:
                sol["axisZ"] = r6(sol["axisZ"] * k)


def scale_solid(x, k):
    """📏️ Scales a volume solid (bone-local metres) by k; `of` references are untouched."""
    if "of" in x:
        return
    kind = x["kind"]
    sc = lambda v: [r6(c * k) for c in v]
    if kind == "box":
        x["size"] = sc(x["size"])
        if "cornerRadius" in x:
            x["cornerRadius"] = r6(x["cornerRadius"] * k)
    elif kind == "ellipsoid":
        x["radii"] = sc(x["radii"])
    elif kind == "capsule":
        x["p0"], x["p1"], x["r0"], x["r1"] = sc(x["p0"]), sc(x["p1"]), r6(x["r0"] * k), r6(x["r1"] * k)
    elif kind == "revolve":
        x["profile"] = [sc(p) for p in x["profile"]]
        if "origin" in x:
            x["origin"] = sc(x["origin"])
    elif kind == "extrude":
        x["profile"] = [sc(p) for p in x["profile"]]
        if "holes" in x:
            x["holes"] = [[sc(p) for p in h] for h in x["holes"]]
        x["from"], x["to"] = r6(x["from"] * k), r6(x["to"] * k)
    elif kind == "loft":
        for sec in x["sections"]:
            sec["at"] = sc(sec["at"])
            sec["rx"], sec["ry"] = r6(sec["rx"] * k), r6(sec["ry"] * k)
    elif kind == "polyhedron":
        x["vertices"] = [sc(p) for p in x["vertices"]]
    else:
        raise ValueError(kind)
    tr = x.get("transform")
    if tr and "translation" in tr:
        tr["translation"] = sc(tr["translation"])
    rp = x.get("repeat")
    if rp:
        for f in ("translation", "origin"):
            if f in rp:
                rp[f] = sc(rp[f])


def scale_part(part, k, mass_k):
    """🧍️ Anthropometric scaling of a body part: lengths by k, segment masses by `mass_k`, anchors, shapes, solids, region areas by k²."""
    for b in part["bones"]:
        for f in ("x", "y", "z", "lengthM"):
            if f in b:
                b[f] = r6(b[f] * k)
        if "mass" in b:
            b["mass"]["kg"] = round(b["mass"]["kg"] * mass_k, 4)
            b["mass"]["comX"] = r6(b["mass"]["comX"] * k)
            b["mass"]["comY"] = r6(b["mass"]["comY"] * k)
        j = b["joint"]
        for f in ("minM", "maxM"):
            if f in j:
                j[f] = r6(j[f] * k)
    for a in part["anchors"]:
        for f in ("x", "y", "z", "footprintLengthM", "footprintWidthM"):
            if f in a:
                a[f] = r6(a[f] * k)
    for s in part["shapes"]:
        for v in part["shapes"][s]:
            for x in part["shapes"][s][v]:
                scale_shape(x, k)
    for s in part["solids"]:
        for lst in part["solids"][s]:
            for x in part["solids"][s][lst]:
                scale_solid(x, k)
    for r in part["regions"]:
        r["areaM2"] = round(r["areaM2"] * k * k, 4)
        r["characteristicLengthM"] = round(r["characteristicLengthM"] * k, 4)


def merge_refs(*lists):
    """📚️ Union of reference lists by id (first citation wins)."""
    out, seen = [], set()
    for lst in lists:
        for r in lst:
            if r["id"] not in seen:
                seen.add(r["id"])
                out.append(copy.deepcopy(r))
    return out


def uniq(items, key="id"):
    out, seen = [], set()
    for x in items:
        if x[key] in seen:
            continue
        seen.add(x[key])
        out.append(x)
    return out


def T_(en, de):
    return {"en": en, "de": de}
