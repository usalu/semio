import json, math, os, shutil
BASE = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"

def path(a): return os.path.join(BASE, a, "🔣️.json")
def load(a): return json.load(open(path(a), encoding="utf-8"))
def save(a, doc):
    import sys
    sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧰️FW"))
    from atomic_json import write_json_atomic
    write_json_atomic(path(a), doc)

def r5(v): return round(v, 5)

def S(id, bone, role, z, tone, kind, region=None, material=None, regions=None, filled=True, **g):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    for k, v in g.items():
        if k == "points": s[k] = [[r5(x), r5(y)] for x, y in v]
        elif isinstance(v, float): s[k] = r5(v)
        else: s[k] = v
    if kind == "poly": s["closed"] = g.get("closed", True) if "closed" in g else True
    if region: s["region"] = region
    if regions: s["regions"] = regions
    if material: s["material"] = material
    return s

def rect(id, bone, x, y, w, h, role="body", z=3, tone=0.5, filled=True, **kw):
    f = kw.pop("followTip", None)
    s = S(id, bone, role, z, tone, "rect", filled=filled, x=x, y=y, w=w, h=h, **kw)
    if f: s["followTip"] = True
    return s

def poly(id, bone, pts, role="body", z=3, tone=0.5, closed=True, filled=True, **kw):
    f = kw.pop("followTip", None)
    s = S(id, bone, role, z, tone, "poly", filled=filled and closed, points=pts, **kw)
    s["closed"] = closed
    if f: s["followTip"] = True
    return s

def ell(id, bone, cx, cy, rx, ry, role="body", z=3, tone=0.5, filled=True, **kw):
    return S(id, bone, role, z, tone, "ellipse", filled=filled, cx=cx, cy=cy, rx=rx, ry=ry, **kw)

def cap(id, bone, x0, y0, x1, y1, r0, r1, role="body", z=3, tone=0.5, filled=True, follow=False, **kw):
    s = S(id, bone, role, z, tone, "capsule", filled=filled, x0=x0, y0=y0, x1=x1, y1=y1, r0=r0, r1=r1, **kw)
    if follow: s["followTip"] = True
    return s

def pth(id, bone, d, role="body", z=3, tone=0.5, filled=True, **kw):
    return S(id, bone, role, z, tone, "path", filled=filled, d=d, **kw)

def ensure_backup(a):
    orig = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "O1", "orig")
    os.makedirs(orig, exist_ok=True)
    backup = os.path.join(orig, a + ".json")
    if not os.path.exists(backup):
        shutil.copyfile(path(a), backup)
    return backup


def apply(a, views, bones=(), drop=(), strip=None):
    """views: {scale: (front, plan, {decision-key: (en, de)})}"""
    orig = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "O1", "orig")
    os.makedirs(orig, exist_ok=True)
    backup = os.path.join(orig, a + ".json")
    if not os.path.exists(backup):
        shutil.copyfile(path(a), backup)
    doc = json.load(open(backup, encoding="utf-8"))
    asset = doc["asset"]
    asset["rig"]["bones"] = [b for b in asset["rig"]["bones"] if b["id"] not in drop]
    asset["rig"]["bones"].extend(bones)
    pres = asset.get("presentation", {})
    pres.pop("viewpoints", None)
    for sc, (front, plan, extra) in views.items():
        rep = asset["representations"][str(sc)]
        assert rep["status"] == "authored", (a, sc)
        g = rep["geometry"]
        rep["geometry"] = {"elevation": g["elevation"], "front": front, "plan": plan}
        dec = rep["decisions"]
        for k, (en, de) in (strip or {}).items():
            dec[k]["en"] = dec[k]["en"].replace(en, "through the lid bone itself.")
            dec[k]["de"] = dec[k]["de"].replace(de, "über den Deckelknochen selbst.")
        for k, (en, de) in extra.items():
            if en in dec[k]["en"]:
                continue
            dec[k]["en"] = dec[k]["en"].rstrip() + " " + en
            dec[k]["de"] = dec[k]["de"].rstrip() + " " + de
    save(a, doc)


def orig_shape(a, scale, vp, sid):
    backup = ensure_backup(a)
    doc = json.load(open(backup, encoding="utf-8"))
    for s in doc["asset"]["representations"][str(scale)]["geometry"][vp]:
        if s["id"] == sid: return s
    raise KeyError(sid)


def shift_path(d, dx=0.0, dy=0.0):
    import re
    toks = re.findall(r"[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?", d)
    out, i, nums = [], 0, []
    cmd = None
    res = []
    for t in toks:
        if t.isalpha():
            cmd = t; res.append(t); nums = []
        else:
            nums.append(float(t))
            res.append(None)
            res[-1] = (len(nums) - 1) % 2
            res[-1] = ("n", float(t), (len(nums) - 1) % 2)
    parts = []
    for r in res:
        if isinstance(r, str): parts.append(r)
        else:
            v = r[1] + (dx if r[2] == 0 else dy)
            parts.append(f"{v:.5f}".rstrip("0").rstrip("."))
    return " ".join(parts)


def helper_bone(id, parent, x, y, angle, length, en, de):
    return {"id": id, "parent": parent, "title": {"en": en, "de": de}, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": {"type": "fixed"}}
