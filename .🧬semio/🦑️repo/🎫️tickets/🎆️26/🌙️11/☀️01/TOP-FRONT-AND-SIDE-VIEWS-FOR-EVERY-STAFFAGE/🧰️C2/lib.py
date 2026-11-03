import json, math, sys
sys.stdout.reconfigure(encoding="utf8")

ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"


def path(aid):
    return ROOT + aid + "/🔣️.json"


ORIG = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️C2/orig/"


def load(aid):
    return json.load(open(ORIG + aid + ".json", encoding="utf8", newline=""))


def save(aid, doc):
    sys.path.insert(0, ORIG.replace("🧰️C2/orig/", "🧰️FW"))
    from atomic_json import write_json_atomic
    write_json_atomic(path(aid), doc)


def r5(v):
    return round(v + 0.0, 5)


def common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen):
    if role in ("detail", "texture"):
        tone = max(tone, 0.35)
    d = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if regions:
        d["regions"] = regions
    elif region:
        d["region"] = region
    if material:
        d["material"] = material
    if hatch:
        d["hatch"] = hatch
    if pen:
        d["penMm"] = pen
    return d


def R(id, bone, role, z, x, y, w, h, tone=0.5, region=None, regions=None, material=None, hatch=None, pen=None, filled=True, r=None, follow=False):
    d = common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)
    d.update({"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(h)})
    if r:
        d["r"] = r
    if follow:
        d["followTip"] = True
    return d


def P(id, bone, role, z, pts, closed=True, tone=0.5, region=None, regions=None, material=None, hatch=None, pen=None, filled=None, follow=False):
    d = common(id, bone, role, z, closed if filled is None else filled, tone, region, regions, material, hatch, pen)
    d.update({"kind": "poly", "points": [[r5(x), r5(y)] for x, y in pts], "closed": closed})
    if follow:
        d["followTip"] = True
    return d


def C(id, bone, role, z, x0, y0, x1, y1, r0, r1=None, tone=0.5, region=None, regions=None, material=None, hatch=None, pen=None, filled=True, follow=False):
    d = common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)
    d.update({"kind": "capsule", "x0": r5(x0), "y0": r5(y0), "x1": r5(x1), "y1": r5(y1), "r0": r5(r0), "r1": r5(r0 if r1 is None else r1)})
    if follow:
        d["followTip"] = True
    return d


def E(id, bone, role, z, cx, cy, rx, ry=None, tone=0.5, region=None, regions=None, material=None, hatch=None, pen=None, filled=True):
    d = common(id, bone, role, z, filled, tone, region, regions, material, hatch, pen)
    d.update({"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(rx), "ry": r5(rx if ry is None else ry)})
    return d


def line(id, bone, role, z, pts, tone=0.3, region=None, material=None, pen=None, follow=False):
    return P(id, bone, role, z, pts, closed=False, tone=tone, region=region, material=material, pen=pen, filled=False, follow=follow)


def circ(cx, cy, r, a0=0, a1=360, n=None):
    n = n or max(2, int(abs(a1 - a0) / 7.5))
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def check_pens(rep, shapes, view):
    pens = rep["pens"]
    ids = set()
    hatches = {h["id"] for h in rep["hatches"]}
    for s in shapes:
        assert s["id"] not in ids, ("dup", view, s["id"])
        ids.add(s["id"])
        assert pens.get(s["role"]) is not None or s.get("penMm"), ("pen", view, s["id"], s["role"])
        if s.get("hatch"):
            assert s["hatch"] in hatches, ("hatch", s["id"])


def add_text(rep, key, en, de):
    d = rep["decisions"][key]
    d["en"] = d["en"].rstrip() + " " + en
    d["de"] = d["de"].rstrip() + " " + de


def install(aid, scale, front, plan, doc, en, de, key="silhouette", extra=()):
    rep = doc["asset"]["representations"][str(scale)]
    g = rep["geometry"]
    el = g["elevation"]
    check_pens(rep, el, "elevation")
    check_pens(rep, front, "front")
    check_pens(rep, plan, "plan")
    new = {"elevation": el, "front": front, "plan": plan}
    for k, v in g.items():
        if k not in new:
            new[k] = v
    rep["geometry"] = new
    add_text(rep, key, en, de)
    for k, e, d in extra:
        add_text(rep, k, e, d)


def finish(aid, doc):
    doc["asset"]["presentation"].pop("viewpoints", None)
    save(aid, doc)
