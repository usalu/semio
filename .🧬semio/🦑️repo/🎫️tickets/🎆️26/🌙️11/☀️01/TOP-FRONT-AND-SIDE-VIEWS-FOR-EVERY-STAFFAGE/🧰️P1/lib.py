import json, os, shutil

CATALOG = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🔌️appliances"
TICKET = r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
ORIG = os.path.join(TICKET, "🗑️generated", "P1", "orig")


def rn(v):
    return round(v, 5) + 0.0


def _tail(d, region, material, regions, follow):
    if follow:
        d["followTip"] = True
    if regions:
        d["regions"] = regions
    elif region:
        d["region"] = region
    if material:
        d["material"] = material
    return d


def _head(id, bone, role, z, filled, tone):
    return {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}


def rect(id, bone, x, y, w, h, z, tone, region=None, material=None, r=None, role="body", filled=True, regions=None, follow=False):
    d = _head(id, bone, role, z, filled, tone)
    d.update({"kind": "rect", "x": rn(x), "y": rn(y), "w": rn(w), "h": rn(h)})
    if r:
        d["r"] = rn(r)
    return _tail(d, region, material, regions, follow)


def rc(id, bone, cx, cy, w, h, z, tone, **k):
    return rect(id, bone, cx - w / 2, cy - h / 2, w, h, z, tone, **k)


def poly(id, bone, pts, z, tone, region=None, material=None, role="body", filled=True, closed=True, regions=None, follow=False):
    d = _head(id, bone, role, z, filled, tone)
    d.update({"kind": "poly", "points": [[rn(a), rn(b)] for a, b in pts], "closed": closed})
    return _tail(d, region, material, regions, follow)


def line(id, bone, pts, z, tone, region=None, material=None, role="detail", regions=None):
    return poly(id, bone, pts, z, tone, region=region, material=material, role=role, filled=False, closed=False, regions=regions)


def ell(id, bone, cx, cy, rx, ry, z, tone, region=None, material=None, role="body", filled=True, regions=None):
    d = _head(id, bone, role, z, filled, tone)
    d.update({"kind": "ellipse", "cx": rn(cx), "cy": rn(cy), "rx": rn(rx), "ry": rn(ry)})
    return _tail(d, region, material, regions, False)


def cap(id, bone, x0, y0, x1, y1, r0, r1, z, tone, region=None, material=None, role="body", filled=True, regions=None, follow=False):
    d = _head(id, bone, role, z, filled, tone)
    d.update({"kind": "capsule", "x0": rn(x0), "y0": rn(y0), "x1": rn(x1), "y1": rn(y1), "r0": rn(r0), "r1": rn(r1)})
    return _tail(d, region, material, regions, follow)


def path(id, bone, d_, z, tone, region=None, material=None, role="body", filled=True, regions=None):
    d = _head(id, bone, role, z, filled, tone)
    d.update({"kind": "path", "d": d_})
    return _tail(d, region, material, regions, False)


def load(asset):
    return json.load(open(os.path.join(CATALOG, asset, "🔣️.json"), encoding="utf-8"))


def save(asset, doc):
    import sys
    sys.path.insert(0, os.path.join(TICKET, "🧰️FW"))
    from atomic_json import write_json_atomic
    write_json_atomic(os.path.join(CATALOG, asset, "🔣️.json"), doc)


def backup(asset):
    os.makedirs(ORIG, exist_ok=True)
    dst = os.path.join(ORIG, asset + ".json")
    if not os.path.exists(dst):
        shutil.copyfile(os.path.join(CATALOG, asset, "🔣️.json"), dst)
    return json.load(open(dst, encoding="utf-8"))


def apply(asset, spec, text):
    orig = backup(asset)
    doc = load(asset)
    a = doc["asset"]
    a["presentation"].pop("viewpoints", None)
    for scale, views in spec.items():
        rep = a["representations"][str(scale)]
        old = rep["geometry"]
        rep["geometry"] = {"elevation": old["elevation"], "plan": views.get("plan", old.get("plan")), "front": views["front"]}
        base = orig["asset"]["representations"][str(scale)]["decisions"]
        for field, add in text[scale].items():
            for lang in ("en", "de"):
                cur = base[field][lang]
                rep["decisions"][field][lang] = cur.rstrip() + " " + add[lang]
    save(asset, doc)
