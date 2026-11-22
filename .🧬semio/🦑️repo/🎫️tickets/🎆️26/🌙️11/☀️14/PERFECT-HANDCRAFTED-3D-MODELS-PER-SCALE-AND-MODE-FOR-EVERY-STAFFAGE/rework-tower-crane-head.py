"""🗼 Tower crane, second round pass 1: the tower head (cat head) gets its real height.

The round-2 head was one 7.4 m pyramid whose apex stood 3.46 m above the jib heel pin (pendant slope 8.6 deg).
A 55 m hammerhead jib is held from an apex about 7 m above the heel (pendant slope 15 to 17 deg). The lower
pyramid up to the tie level at 5.4 m (above the slewing ring) is kept, because its legs carry the jib heel
pin, the jib rails and the counter-jib chords; above it stands a slender spire to the apex at 12.0 m with
three more braced panels. Pendants and head sheave move to the new apex.

Usage: python3 rework-tower-crane-head.py            writes the candidate document
       python3 rework-tower-crane-head.py --apply    writes the volumes source and the catalog asset
"""
import copy
import json
import math
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "machinery", "tower-crane.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-tower-crane-head", "🔣️.json")

APEX_Y = 12.0
APEX_Z = 0.12
KINK_Y = 5.4
KINK_X = 0.51892
KINK_Z = 0.4573
LEVELS = [5.4, 7.0, 8.6, 10.2]
PENDANT_ENDS = {"pendant-fore-1": (22.48, 5.21), "pendant-fore-2": (40.55, 5.21), "pendant-back": (-16.47, 5.09)}
CORNERS = {1: (1, 1), 2: (1, -1), 3: (-1, 1), 4: (-1, -1)}
FACES = ("z+", "z-", "x+", "x-")
DECISION = {
    "en": " Second round: the tower head is a 4.2 m pyramid that takes the jib heel and the counter-jib, carrying a slender braced spire to the apex 12.0 m above the slewing ring (6.86 m above the jib heel pin), so the pendants rise at 17 and 10 degrees as on a real 55 m hammerhead crane; the spire has ties at 7.0, 8.6 and 10.2 m and one diagonal per face and panel.",
    "de": " Zweite Runde: Die Turmspitze ist eine 4,2 m hohe Pyramide, die Auslegerfuß und Gegenausleger aufnimmt und eine schlanke, ausgefachte Spitze bis zum Scheitel 12,0 m über dem Drehkranz trägt (6,86 m über dem Auslegerbolzen), sodass die Abspannungen wie bei einem echten 55-m-Obendreher mit 17 und 10 Grad ansteigen; die Spitze hat Riegel auf 7,0, 8,6 und 10,2 m und je Feld und Seite eine Diagonale.",
}


def half(y):
    """📐 Half widths (x, z) of the spire at the height y above the slewing ring."""
    t = (APEX_Y - y) / (APEX_Y - KINK_Y)
    return KINK_X * t, APEX_Z + (KINK_Z - APEX_Z) * t


def node(y, sx, sz):
    hx, hz = half(y)
    return (sx * hx, y, sz * hz)


def rnd(v):
    return round(v, 5)


def bar(template, sid, a, b):
    """🦯 A round bar from a to b with the section of the template solid."""
    s = copy.deepcopy(template)
    d = [b[i] - a[i] for i in range(3)]
    n = math.sqrt(sum(c * c for c in d))
    rad = template["profile"][0][1]
    s.update(id=sid, origin=[rnd(c) for c in a], axis=[rnd(c / n) for c in d], profile=[[0, rad], [rnd(n), rad]])
    s.pop("repeat", None)
    return s


def spire_bars():
    """🧵 The hand-set members of the spire: (family, id, a, b, face)."""
    out = []
    for i, (sx, sz) in CORNERS.items():
        out.append(("leg", f"tower-head-leg-{i}", (sx * 1.2, 1.2, sz * 0.9), (sx * KINK_X, KINK_Y, sz * KINK_Z), ("x+" if sx > 0 else "x-", "z+" if sz > 0 else "z-")))
        lo = node(KINK_Y, sx, sz)
        up = (0.0, APEX_Y, sz * APEX_Z)
        d = [up[k] - lo[k] for k in range(3)]
        n = math.sqrt(sum(c * c for c in d))
        start = tuple(lo[k] - d[k] / n * 0.03 for k in range(3))
        out.append(("spire", f"tower-head-spire-{i}", start, up, ("x+" if sx > 0 else "x-", "z+" if sz > 0 else "z-")))
    for lv, y in zip((4, 5, 6), LEVELS[1:]):
        hx, hz = half(y)
        out.append(("tie-x", f"tower-head-tie-x-{lv}-1", (-hx, y, hz), (hx, y, hz), ("z+",)))
        out.append(("tie-x", f"tower-head-tie-x-{lv}-2", (-hx, y, -hz), (hx, y, -hz), ("z-",)))
        out.append(("tie-z", f"tower-head-tie-z-{lv}-1", (hx, y, -hz), (hx, y, hz), ("x+",)))
        out.append(("tie-z", f"tower-head-tie-z-{lv}-2", (-hx, y, -hz), (-hx, y, hz), ("x-",)))
    n = 13
    for j in range(3):
        y0, y1 = LEVELS[j], (LEVELS[j + 1] if j + 1 < len(LEVELS) else APEX_Y)
        flip = 1 if j % 2 == 0 else -1
        for f in FACES:
            if f[0] == "z":
                sz = 1 if f == "z+" else -1
                a, b = node(y0, -flip, sz), node(y1, flip, sz)
            else:
                sx = 1 if f == "x+" else -1
                a, b = node(y0, sx, -flip), node(y1, sx, flip)
            out.append(("diagonal", f"tower-head-diagonal-{n}", a, b, (f,)))
            n += 1
    return out


OLD_SOLID = re.compile(r"^tower-head-(leg-\d+|tie-[xz]-4|diagonal-1[3-6])$")
OLD_REF = re.compile(r"^(th|member)-tower-head-(leg-\d+|tie-[xz]-4(-\d)?|diagonal-1[3-6])$")
OLD_SHAPE = re.compile(r"^tower-head-(leg-\d+|tie-[xz]-4|diagonal-1[3-6])(-\d)?(-stroke|-hull)?$")


def template_of(body, family):
    probe = {"leg": "tower-head-leg-1", "spire": "tower-head-leg-1", "tie-x": "tower-head-tie-x-4", "tie-z": "tower-head-tie-z-4", "diagonal": "tower-head-diagonal-13"}[family]
    return next((s for s in body if s["id"] == probe), None)


def edit_volumes(vols):
    for sc, v in vols.items():
        body = v["body"]
        tpl = {f: template_of(body, f) for f in ("leg", "spire", "tie-x", "tie-z", "diagonal")}
        at = next(i for i, s in enumerate(body) if OLD_SOLID.match(s["id"]))
        v["body"] = body = [s for s in body if not OLD_SOLID.match(s["id"])]
        new = [bar(tpl[fam], sid, a, b) for fam, sid, a, b, _ in spire_bars() if tpl[fam] is not None]
        body[at:at] = new
        for s in body:
            if s["id"] == "head-sheave":
                s["origin"] = [0, APEX_Y, s["origin"][2]]
            if s["id"] in PENDANT_ENDS:
                ex, ey = PENDANT_ENDS[s["id"]]
                n = math.hypot(ex, ey - APEX_Y)
                rad = s["profile"][0][1]
                s.update(origin=[0, APEX_Y, 0], axis=[rnd(ex / n), rnd((ey - APEX_Y) / n), 0], profile=[[0, rad], [rnd(n), rad]])
        th = v["thermal"]
        at = next((i for i, s in enumerate(th) if OLD_REF.match(s["id"])), None)
        if at is None:
            th.extend({"id": "th-" + s["id"], "of": s["id"], "region": "lattice"} for s in body if s["id"].startswith("tower-head-"))
        else:
            v["thermal"] = th = [s for s in th if not OLD_REF.match(s["id"])]
            th[at:at] = [{"id": "th-" + s["id"], "of": s["id"], "region": "lattice"} for s in new]
        st = v["structure"]
        at = next(i for i, s in enumerate(st) if OLD_REF.match(s["id"]))
        v["structure"] = st = [s for s in st if not OLD_REF.match(s["id"])]
        st[at:at] = [{"id": "member-" + s["id"], "of": s["id"], "member": "slewing", "part": "member"} for s in new]
        for lang in ("en", "de"):
            if DECISION[lang].strip() not in v["decisions"][0][lang]:
                v["decisions"][0][lang] = v["decisions"][0][lang].rstrip() + DECISION[lang]
    return vols


def first(shapes, pattern):
    rx = re.compile(pattern)
    return next((s for s in shapes if s.get("bone") == "slewing" and rx.match(s["id"])), None)


def member_shape(tpl, sid, a, b):
    s = copy.deepcopy(tpl)
    s.update(id=sid)
    s["from"] = dict(zip("xyz", [round(c, 4) for c in a]))
    s["to"] = dict(zip("xyz", [round(c, 4) for c in b]))
    return s


def plan_shape(tpl, sid, a, b, rad):
    s = copy.deepcopy(tpl)
    if tpl["kind"] == "member":
        return member_shape(tpl, sid, a, b)
    if tpl.get("filled"):
        dx, dz = b[0] - a[0], b[2] - a[2]
        n = math.hypot(dx, dz) or 1.0
        ox, oz = -dz / n * rad, dx / n * rad
        pts = [(a[0] + ox, a[2] + oz), (b[0] + ox, b[2] + oz), (b[0] - ox, b[2] - oz), (a[0] - ox, a[2] - oz)]
        s.update(id=sid, d="M " + " L ".join(f"{round(x, 4)} {round(z, 4)}" for x, z in pts) + " Z")
    else:
        s.update(id=sid + "-stroke", d=f"M {round(a[0], 4)} {round(a[2], 4)} L {round(b[0], 4)} {round(b[2], 4)}")
    return s


PROBES = {"leg": r"^tower-head-leg-\d", "spire": r"^tower-head-leg-\d", "tie-x": r"^tower-head-tie-x-\d", "tie-z": r"^tower-head-tie-z-\d", "diagonal": r"^tower-head-diagonal-\d"}
RADII = {"leg": 0.05, "spire": 0.05, "tie-x": 0.03, "tie-z": 0.03, "diagonal": 0.03}
HIDDEN = {"elevation": "z-", "front": "x-"}


def edit_views(asset, vols):
    ties = {sc for sc, v in vols.items() if any(s["id"].startswith("tower-head-tie-x") for s in v["body"])}
    for sc, rep in asset["representations"].items():
        coarse = sc == "500"
        for view, shapes in rep["geometry"].items():
            tpl = {f: first(shapes, p) for f, p in PROBES.items()}
            if view == "plan" and not coarse:
                for f in ("tie-x", "tie-z"):
                    tpl[f] = (first(shapes, r"^tower-head-tie-[xz]-\d-stroke") or tpl["diagonal"]) if sc in ties else None
            old = [i for i, s in enumerate(shapes) if s.get("bone") == "slewing" and OLD_SHAPE.match(s["id"])]
            at = old[0] if old else len(shapes)
            kept = [s for i, s in enumerate(shapes) if i not in set(old)]
            new = []
            for fam, sid, a, b, faces in spire_bars():
                if tpl[fam] is None:
                    continue
                if not coarse and HIDDEN.get(view) in faces:
                    continue
                if view == "plan":
                    new.append(plan_shape(tpl[fam], sid, a, b, RADII[fam]))
                else:
                    suffix = "" if coarse or re.search(r"-\d-\d$", sid) else "-1"
                    new.append(member_shape(tpl[fam], sid + suffix, a, b))
            kept[at:at] = new
            for s in kept:
                if s.get("bone") != "slewing":
                    continue
                if s["id"] == "head-sheave" and "cy" in s:
                    s["cy"] = APEX_Y
                if s["id"] in PENDANT_ENDS and s["kind"] == "member":
                    s["from"]["y"] = APEX_Y
            rep["geometry"][view] = kept
        dec = rep["decisions"]
        key = "silhouette" if "silhouette" in dec else next(iter(dec))
        for lang in ("en", "de"):
            if DECISION[lang].strip() not in dec[key][lang]:
                dec[key][lang] = dec[key][lang].rstrip() + DECISION[lang]


def edit_roots(asset):
    asset["dimensions"]["heightM"] = 52.25
    for b in asset["rig"]["bones"]:
        if b["id"] == "slewing":
            b["lengthM"] = APEX_Y
    for view, shapes in asset["symbols"][0]["geometry"].items():
        for s in shapes:
            if s["id"] in ("elevation-tower-head", "front-tower-head"):
                s["h"] = APEX_Y - 1.2
            if s["id"] in ("elevation-pendant-fore", "elevation-pendant-back"):
                s["points"][0] = [0, 40 + APEX_Y]


def main():
    apply = "--apply" in sys.argv
    doc = json.load(open(CATALOG, encoding="utf-8"))
    src = json.load(open(SOURCE, encoding="utf-8"))
    if any(s["id"] == "tower-head-spire-1" for s in src["volumes"]["50"]["body"]):
        print("already applied")
        return
    edit_volumes(src["volumes"])
    edit_views(doc["asset"], src["volumes"])
    edit_roots(doc["asset"])
    doc["asset"]["volumes"] = src["volumes"]
    if apply:
        json.dump(src, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        json.dump(doc, open(CATALOG, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(doc, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for sc, v in src["volumes"].items():
        print(sc, {k: len(v[k]) for k in ("body", "thermal", "structure")}, {k: len(g) for k, g in doc["asset"]["representations"][sc]["geometry"].items()})


if __name__ == "__main__":
    main()
