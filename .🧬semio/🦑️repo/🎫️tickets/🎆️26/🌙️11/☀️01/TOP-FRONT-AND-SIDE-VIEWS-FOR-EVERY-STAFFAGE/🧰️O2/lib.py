import json, os, math

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"


def path_of(asset_id):
    return os.path.join(ROOT, asset_id, "🔣️.json")


INDENT = {}


def load(asset_id):
    INDENT[asset_id] = indent_of(asset_id)
    with open(path_of(asset_id), encoding="utf-8") as f:
        return json.load(f)


def indent_of(asset_id):
    with open(path_of(asset_id), encoding="utf-8") as f:
        f.readline()
        second = f.readline()
    return len(second) - len(second.lstrip(" "))


def save(asset_id, doc):
    import tempfile
    path = path_of(asset_id)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path), suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline=chr(10)) as f:
            f.write(json.dumps(doc, ensure_ascii=False, indent=INDENT.get(asset_id, 2)) + chr(10))
        os.replace(tmp, path)
    except BaseException:
        if os.path.exists(tmp):
            os.remove(tmp)
        raise


def roundtrip_ok(asset_id):
    with open(path_of(asset_id), encoding="utf-8", newline="") as f:
        raw = f.read()
    return json.dumps(json.loads(raw), ensure_ascii=False, indent=2) + "\n" == raw


def r(v):
    return round(v + 0.0, 5)


def base(id, bone, role, z, tone, filled=True, region=None, regions=None, material=None, pen=None, hatch=None, solid=None):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    if solid is not None:
        s["solid"] = solid
    return s


def rect(x, y, w, h, rad=None, follow=None, **kw):
    s = base(**kw)
    s.update({"kind": "rect", "x": r(x), "y": r(y), "w": r(w), "h": r(h)})
    if rad:
        s["r"] = r(rad)
    if follow is not None:
        s["followTip"] = follow
    return s


def ellipse(cx, cy, rx, ry, **kw):
    s = base(**kw)
    s.update({"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rx), "ry": r(ry)})
    return s


def poly(points, closed=True, follow=None, **kw):
    s = base(**kw)
    s.update({"kind": "poly", "points": [[r(x), r(y)] for x, y in points], "closed": closed})
    if follow is not None:
        s["followTip"] = follow
    return s


def path(d, **kw):
    s = base(**kw)
    s.update({"kind": "path", "d": d})
    return s


def capsule(x0, y0, x1, y1, r0, r1=None, follow=None, **kw):
    s = base(**kw)
    s.update({"kind": "capsule", "x0": r(x0), "y0": r(y0), "x1": r(x1), "y1": r(y1), "r0": r(r0), "r1": r(r1 if r1 is not None else r0)})
    if follow is not None:
        s["followTip"] = follow
    return s


def mirror_y(points):
    return [(x, -y) for x, y in points]


LEADS = {"en": (" Front view:", " Front and plan view:", " Plan view:"), "de": (" Vorderansicht:", " Vorder- und Draufsicht:", " Draufsicht:")}


def extend_decisions(rep, adds):
    for key, (en, de) in adds.items():
        d = rep["decisions"][key]
        for lang, add in (("en", en), ("de", de)):
            text = d[lang]
            cuts = [text.find(lead) for lead in LEADS[lang] if text.find(lead) >= 0]
            if cuts:
                text = text[: min(cuts)]
            assert add.startswith(tuple(lead.strip() for lead in LEADS[lang])), (key, lang, add[:30])
            d[lang] = text.rstrip() + " " + add


def install(doc, scale, front, plan, adds):
    rep = doc["asset"]["representations"][str(scale)]
    assert rep["status"] == "authored"
    rep["geometry"]["front"] = front
    rep["geometry"]["plan"] = plan
    extend_decisions(rep, adds)
    ids = [s["id"] for s in front]
    assert len(ids) == len(set(ids)), ("front ids", scale)
    ids = [s["id"] for s in plan]
    assert len(ids) == len(set(ids)), ("plan ids", scale)


def finish(doc):
    doc["asset"]["presentation"].pop("viewpoints", None)
