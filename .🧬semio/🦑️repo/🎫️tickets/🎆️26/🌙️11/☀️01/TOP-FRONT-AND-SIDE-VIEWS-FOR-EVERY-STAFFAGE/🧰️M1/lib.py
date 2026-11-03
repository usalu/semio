import json, os, re, sys, io
from decimal import Decimal

S = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
EXP = re.compile(r"(?<![\w\"])-?\d+(?:\.\d+)?e[+-]?\d+(?![\w\"])")


def _js(tok):
    x = float(tok.group(0))
    if x == 0:
        return "0"
    if abs(x) >= 1e-6 and abs(x) < 1e21:
        s = format(Decimal(repr(x)), "f")
        return s.rstrip("0").rstrip(".") if "." in s else s
    mant, ex = repr(x).split("e")
    return mant + "e" + ("-" if ex[0] == "-" else "+") + str(int(ex.lstrip("+-")))


def path_of(aid):
    return os.path.join(S, aid, "🔣️.json")


def load(aid):
    raw = open(path_of(aid), "rb").read().decode("utf8")
    indent = len(raw.split("\n")[1]) - len(raw.split("\n")[1].lstrip(" "))
    return json.loads(raw), indent, raw


def dumps(doc, indent):
    return EXP.sub(_js, json.dumps(doc, ensure_ascii=False, indent=indent)) + "\n"


def save(aid, doc, indent):
    import tempfile
    target = path_of(aid)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), suffix=".tmp")
    try:
        with os.fdopen(fd, "wb") as f:
            f.write(dumps(doc, indent).encode("utf8"))
        os.replace(tmp, target)
    except BaseException:
        if os.path.exists(tmp):
            os.remove(tmp)
        raise


def r(v):
    return round(v + 0.0, 5)


def rect(id, bone, role, z, tone, x, y, w, h, region=None, regions=None, material=None, rr=None, penMm=None, followTip=None, hatch=None, filled=True):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "rect", "x": r(x), "y": r(y), "w": r(w), "h": r(h)}
    if rr is not None:
        s["r"] = rr
    return _tail(s, region, regions, material, penMm, followTip, hatch)


def ellipse(id, bone, role, z, tone, cx, cy, rx, ry, region=None, regions=None, material=None, rotationDeg=None, penMm=None, hatch=None, filled=True):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rx), "ry": r(ry)}
    if rotationDeg is not None:
        s["rotationDeg"] = rotationDeg
    return _tail(s, region, regions, material, penMm, None, hatch)


def poly(id, bone, role, z, tone, points, closed=True, region=None, regions=None, material=None, penMm=None, followTip=None, hatch=None, filled=True):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled and closed, "tone": tone, "kind": "poly", "points": [[r(a), r(b)] for a, b in points], "closed": closed}
    return _tail(s, region, regions, material, penMm, followTip, hatch)


def capsule(id, bone, role, z, tone, x0, y0, x1, y1, r0, r1=None, region=None, regions=None, material=None, penMm=None, followTip=None, hatch=None, filled=True):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "capsule", "x0": r(x0), "y0": r(y0), "x1": r(x1), "y1": r(y1), "r0": r(r0), "r1": r(r0 if r1 is None else r1)}
    return _tail(s, region, regions, material, penMm, followTip, hatch)


def path(id, bone, role, z, tone, d, region=None, regions=None, material=None, penMm=None, hatch=None, filled=True):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "path", "d": d}
    return _tail(s, region, regions, material, penMm, None, hatch)


def ex(z0, z1):
    return {"kind": "extruded", "z0": z0, "z1": z1}


def _tail(s, region, regions, material, penMm, followTip, hatch, solid=None):
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if penMm is not None:
        s["penMm"] = penMm
    if followTip is not None:
        s["followTip"] = followTip
    if hatch is not None:
        s["hatch"] = hatch
    if solid is not None:
        s["solid"] = solid
    return s


def mirror_y(shapes, suffix_from="left", suffix_to="right"):
    return shapes


ORIG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "orig")


def helper_bone(id, parent, x, y, angleDeg, lengthM, en, de):
    return {"id": id, "parent": parent, "title": {"en": en, "de": de}, "x": x, "y": y, "angleDeg": angleDeg, "lengthM": lengthM, "joint": {"type": "fixed"}}


def apply(aid, plan, bones=()):
    os.makedirs(ORIG, exist_ok=True)
    backup = os.path.join(ORIG, aid + ".json")
    if not os.path.exists(backup):
        open(backup, "wb").write(open(path_of(aid), "rb").read())
    raw = open(backup, "rb").read().decode("utf8")
    doc = json.loads(raw)
    line = raw.splitlines()[1]
    indent = len(line) - len(line.lstrip(" "))
    a = doc["asset"]
    for sc, rep in a["representations"].items():
        if rep["status"] == "authored" and sc not in plan:
            raise SystemExit("authored scale %s of %s not handled" % (sc, aid))
    for sc, spec in plan.items():
        rep = a["representations"][sc]
        assert rep["status"] == "authored", (aid, sc)
        g = rep["geometry"]
        g["front"] = spec["front"]
        g["plan"] = spec["plan"]
        for k, (en, de) in spec["dec"].items():
            t = rep["decisions"][k]
            t["en"] = t["en"] + " " + en
            t["de"] = t["de"] + " " + de
    a["presentation"].pop("viewpoints", None)
    for b in bones:
        a["rig"]["bones"].append(b)
    save(aid, doc, indent)
    print("saved", aid, {sc: (len(s["front"]), len(s["plan"])) for sc, s in plan.items()})


def X(shape, z0, z1):
    """Make a front/plan shape an extruded solid: its (bone-local) profile spans the lateral range z0..z1 and follows the full bone transform."""
    shape["solid"] = ex(z0, z1)
    return shape
