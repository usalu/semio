"""🛠️ HW-05 shared tools: engine calls (dims, validation issues), the feature floor of thin solids, masses, skin areas and region edits.

@see ./lib.py
"""
import json
import math
import re
import subprocess

from lib import BUN, GENERATED, HUMANS, SCALES, SCRIPTS, VIEWS, r, save, save_path


def bun(script, *args):
    out = subprocess.run([BUN, str(SCRIPTS / script), *map(str, args)], capture_output=True, text=True, cwd=SCRIPTS)
    if out.returncode != 0:
        raise RuntimeError(out.stderr[-3000:])
    return out.stdout


def asset_path(slug):
    return HUMANS / slug / "🔣️.json"


def issues(slug):
    return [json.loads(line) for line in bun("issues.ts", asset_path(slug)).splitlines() if line.startswith("{")]


def body_dims(slug):
    return json.loads(bun("dims.ts", asset_path(slug)).strip().splitlines()[-1])


def dubois(mass, height):
    return 0.007184 * mass ** 0.425 * (height * 100) ** 0.725


def solid_index(doc):
    out = {}
    for sc, vol in doc["asset"]["volumes"].items():
        if vol.get("status") != "authored":
            continue
        for lst in ("body", "thermal", "structure"):
            for i, s in enumerate(vol[lst]):
                out[(sc, lst, i)] = s
    return out


def _grow(s, k):
    """Grows the thin cross-section of a solid by k (its long extent stays)."""
    kind = s["kind"]
    if kind == "capsule":
        s["r0"], s["r1"] = r(s["r0"] * k), r(s["r1"] * k)
    elif kind == "ellipsoid":
        i = min(range(3), key=lambda j: s["radii"][j])
        s["radii"][i] = r(s["radii"][i] * k)
    elif kind == "box":
        i = min(range(3), key=lambda j: s["size"][j])
        s["size"][i] = r(s["size"][i] * k)
    elif kind == "loft" and (max(sec["at"][0] for sec in s["sections"]) - min(sec["at"][0] for sec in s["sections"]) + 2 * s["sections"][0]["rx"] * 0) < 2 * min(min(sec["rx"] for sec in s["sections"]), min(sec["ry"] for sec in s["sections"])) and len({sec["at"][1] for sec in s["sections"]}) >= 1 and abs(max(sec["at"][0] for sec in s["sections"]) - min(sec["at"][0] for sec in s["sections"])) > 1e-6 and (max(sec["at"][0] for sec in s["sections"]) - min(sec["at"][0] for sec in s["sections"])) < 2 * max(max(sec["rx"] for sec in s["sections"]), max(sec["ry"] for sec in s["sections"])):
        us = [sec["at"][0] for sec in s["sections"]]
        mid = (max(us) + min(us)) / 2
        for sec in s["sections"]:
            sec["at"][0] = r(mid + (sec["at"][0] - mid) * k)
    elif kind == "loft":
        ex_rx = max(sec["rx"] for sec in s["sections"])
        ex_ry = max(sec["ry"] for sec in s["sections"])
        key = "rx" if ex_rx <= ex_ry else "ry"
        for sec in s["sections"]:
            sec[key] = r(sec[key] * k)
        if min(ex_rx, ex_ry) * k * 1.0 > 0 and abs(ex_rx - ex_ry) < 1e-9:
            for sec in s["sections"]:
                sec["ry"] = r(sec["ry"] * k)
    elif kind == "extrude":
        xs = [p[0] for p in s["profile"]]
        ys = [p[1] for p in s["profile"]]
        ext = [max(xs) - min(xs), max(ys) - min(ys), abs(s["to"] - s["from"])]
        i = min(range(3), key=lambda j: ext[j])
        if i == 2:
            mid = (s["to"] + s["from"]) / 2
            half = (s["to"] - s["from"]) / 2 * k
            s["from"], s["to"] = r(mid - half), r(mid + half)
        else:
            cx = (max(xs) + min(xs)) / 2 if i == 0 else (max(ys) + min(ys)) / 2
            s["profile"] = [[r(cx + (p[0] - cx) * k), p[1]] if i == 0 else [p[0], r(cx + (p[1] - cx) * k)] for p in s["profile"]]
    elif kind == "revolve":
        s["profile"] = [[p[0], r(p[1] * k)] for p in s["profile"]]
    else:
        raise ValueError(kind)


SLENDER = re.compile(r'Solid "([^"]+)" is declared slender but')


def unslender(doc, found):
    n = 0
    for i in found:
        m = SLENDER.search(i["message"]) if i["code"] == "volumes-slender" else None
        if not m:
            continue
        sc = i["path"].split(".")[1] if i.get("path", "").startswith("volumes.") else None
        for key, vol in doc["asset"]["volumes"].items():
            if vol.get("status") != "authored" or (sc and key != sc):
                continue
            for lst in ("body", "thermal", "structure"):
                for s in vol[lst]:
                    if s.get("id") == m.group(1) and s.get("slender"):
                        del s["slender"]
                        n += 1
    return n


STUCK = {}
FEATURE = re.compile(r'Solid "([^"]+)" of (\w+) is ([\d.]+) mm thin, below ([\d.]+) mm')


def floor_features(doc, found):
    """Applies one round of `volumes-feature` repairs from validation issues; returns the number of solids grown."""
    n = 0
    vols = doc["asset"]["volumes"]
    for i in found:
        if i["code"] != "volumes-feature":
            continue
        m = FEATURE.search(i["message"])
        if not m:
            continue
        sid, lst, thin, limit = m.group(1), m.group(2), float(m.group(3)), float(m.group(4))
        sc = i["path"].split(".")[1]
        solid = next(s for s in vols[sc][lst] if s["id"] == sid)
        if thin <= 0:
            continue
        key = (sc, lst, sid)
        STUCK[key] = STUCK.get(key, 0) + 1
        if STUCK[key] > 2 and solid["kind"] == "loft":
            for sec in solid["sections"]:
                sec["rx"], sec["ry"] = r(sec["rx"] * limit / thin * 1.02), r(sec["ry"] * limit / thin * 1.02)
        else:
            _grow(solid, limit / thin * 1.01)
        n += 1
    return n


def write(doc, slug):
    save(doc, slug)


def set_dims(doc, slug, scale="50"):
    save(doc, slug)
    dims = body_dims(slug)[scale]
    d = doc["asset"]["dimensions"]
    d["widthM"], d["heightM"], d["depthM"] = r(dims[0], 3), r(dims[1], 3), r(dims[2], 3)
    save(doc, slug)
    return dims


def feature_loop(doc, slug, rounds=8):
    for k in range(rounds):
        save(doc, slug)
        found = issues(slug)
        n = floor_features(doc, found) + unslender(doc, found)
        print(f"[DEBUG] feature round {k}: {len(found)} issues, grew {n}")
        if n == 0:
            return found
    save(doc, slug)
    return issues(slug)


def summarize(found, width=220):
    from collections import Counter
    c = Counter(i["code"] for i in found)
    print(dict(c))
    for i in found[:40]:
        print("  ", i["code"], i["message"][:width])


def shapes(doc):
    for sc, rep in doc["asset"]["representations"].items():
        if rep.get("status") != "authored":
            continue
        for view in VIEWS:
            for s in rep["geometry"][view]:
                yield sc, view, s


def solids(doc, lists=("body", "thermal", "structure")):
    for sc, vol in doc["asset"]["volumes"].items():
        if vol.get("status") != "authored":
            continue
        for lst in lists:
            for s in vol[lst]:
                yield sc, lst, s


def rename_region(doc, old, new):
    a = doc["asset"]
    for _, _, s in shapes(doc):
        if s.get("region") == old:
            s["region"] = new
        if "regions" in s:
            s["regions"] = list(dict.fromkeys(new if x == old else x for x in s["regions"]))
    for reg in a["thermal"]["regions"]:
        if reg["id"] == old:
            reg["id"] = new
    for _, _, s in solids(doc, ("thermal",)):
        if s.get("region") == old:
            s["region"] = new
    for e in a["thermal"]["emitters"]:
        if e.get("region") == old:
            e["region"] = new


def rename_material(doc, old, new):
    a = doc["asset"]
    for m in a["materials"]:
        if m["id"] == old:
            m["id"] = new
    for _, _, s in shapes(doc):
        if s.get("material") == old:
            s["material"] = new
    for _, _, s in solids(doc):
        if s.get("material") == old:
            s["material"] = new
    for reg in a["thermal"]["regions"]:
        if reg.get("material") == old:
            reg["material"] = new
        for lay in reg.get("layers", []):
            if lay.get("material") == old:
                lay["material"] = new


def rename_layer(doc, region, old, new):
    for reg in doc["asset"]["thermal"]["regions"]:
        if reg["id"] == region:
            for lay in reg["layers"]:
                if lay["id"] == old:
                    lay["id"] = new
    for _, _, s in solids(doc, ("thermal",)):
        if s.get("region") == region and s.get("layer") == old:
            s["layer"] = new


def drop_shapes(doc, pred):
    for rep in doc["asset"]["representations"].values():
        if rep.get("status") != "authored":
            continue
        for view in VIEWS:
            rep["geometry"][view] = [s for s in rep["geometry"][view] if not pred(s)]


def tone_of(doc, scale, view, sid):
    for s in doc["asset"]["representations"][scale]["geometry"][view]:
        if s["id"] == sid:
            return s["tone"]
    return None


def replace_text(o, pairs):
    """Replaces phrases in every {en, de} text of an object tree; pairs = [(en_old, en_new, de_old, de_new)]."""
    if isinstance(o, dict):
        if set(o.keys()) == {"en", "de"} and isinstance(o["en"], str):
            for eo, en, do, dn in pairs:
                if eo:
                    o["en"] = o["en"].replace(eo, en)
                if do:
                    o["de"] = o["de"].replace(do, dn)
            return
        for v in o.values():
            replace_text(v, pairs)
    elif isinstance(o, list):
        for v in o:
            replace_text(v, pairs)


def texts(o, path=""):
    if isinstance(o, dict):
        if set(o.keys()) == {"en", "de"} and isinstance(o["en"], str):
            yield path, o
            return
        for k, v in o.items():
            yield from texts(v, f"{path}.{k}")
    elif isinstance(o, list):
        for i, v in enumerate(o):
            yield from texts(v, f"{path}[{i}]")
