import json, os, re
from decimal import Decimal

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"


def path(aid):
    return os.path.join(ROOT, aid, "🔣️.json")


def load(aid):
    return json.load(open(path(aid), encoding="utf-8"))


def _plain(m):
    return format(Decimal(m.group(0)), "f")


def save(aid, doc):
    doc["asset"]["presentation"].pop("viewpoints", None)
    text = json.dumps(doc, ensure_ascii=False, indent=2)
    text = re.sub(r"(?<![\w.\"])-?\d+(?:\.\d+)?[eE][-+]?\d+(?![\w\"])", _plain, text)
    import tempfile
    target = path(aid)
    fd, temporary = tempfile.mkstemp(dir=os.path.dirname(target), suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="\n") as f:
            f.write(text + "\n")
        os.replace(temporary, target)
    except BaseException:
        if os.path.exists(temporary):
            os.remove(temporary)
        raise


def add_bone(doc, after, bone):
    bones = doc["asset"]["rig"]["bones"]
    if any(b["id"] == bone["id"] for b in bones):
        return
    bones.insert([b["id"] for b in bones].index(after) + 1, bone)


def n(v):
    return round(v, 5)


def _base(id, bone, z, tone, region, material, role, filled, extra):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    return s


def _tail(s, region, material, extra):
    if isinstance(region, (list, tuple)):
        s["regions"] = list(region)
    elif region is not None:
        s["region"] = region
    if material is not None:
        s["material"] = material
    for k, v in extra.items():
        s[k] = v
    return s


def R(id, bone, z, tone, x, y, w, h, region, material, role="body", filled=True, **extra):
    s = _base(id, bone, z, tone, region, material, role, filled, extra)
    s.update(kind="rect", x=n(x), y=n(y), w=n(w), h=n(h))
    if "r" in extra:
        s["r"] = extra.pop("r")
    return _tail(s, region, material, extra)


def E(id, bone, z, tone, cx, cy, rx, ry, region, material, role="body", filled=True, **extra):
    s = _base(id, bone, z, tone, region, material, role, filled, extra)
    s.update(kind="ellipse", cx=n(cx), cy=n(cy), rx=n(rx), ry=n(ry))
    return _tail(s, region, material, extra)


def P(id, bone, z, tone, points, region, material, role="body", filled=True, closed=True, **extra):
    s = _base(id, bone, z, tone, region, material, role, filled, extra)
    s.update(kind="poly", points=[[n(a), n(b)] for a, b in points], closed=closed)
    return _tail(s, region, material, extra)


def C(id, bone, z, tone, x0, y0, x1, y1, r0, r1, region, material, role="body", filled=True, **extra):
    s = _base(id, bone, z, tone, region, material, role, filled, extra)
    s.update(kind="capsule", x0=n(x0), y0=n(y0), x1=n(x1), y1=n(y1), r0=n(r0), r1=n(r1))
    return _tail(s, region, material, extra)


def D(id, bone, z, tone, d, region, material, role="body", filled=True, **extra):
    s = _base(id, bone, z, tone, region, material, role, filled, extra)
    s.update(kind="path", d=d)
    return _tail(s, region, material, extra)


def put(doc, scale, front, plan, en, de):
    rep = doc["asset"]["representations"][str(scale)]
    assert rep["status"] == "authored"
    g = rep["geometry"]
    for v, shapes in (("front", front), ("plan", plan)):
        ids = [s["id"] for s in shapes]
        assert len(ids) == len(set(ids)), (scale, v, "dup ids")
        for s in shapes:
            assert s["bone"] in [b["id"] for b in doc["asset"]["rig"]["bones"]], s["id"]
    rep["geometry"] = {"elevation": g["elevation"], "front": front, "plan": plan}
    sil = rep["decisions"]["silhouette"]
    marker_en = "Front and top views:"
    if marker_en in sil["en"]:
        sil["en"] = sil["en"][: sil["en"].index(" " + marker_en)]
        sil["de"] = sil["de"][: sil["de"].index(" Vorder- und Draufsicht:")]
    sil["en"] += " " + marker_en + " " + en
    sil["de"] += " Vorder- und Draufsicht: " + de
