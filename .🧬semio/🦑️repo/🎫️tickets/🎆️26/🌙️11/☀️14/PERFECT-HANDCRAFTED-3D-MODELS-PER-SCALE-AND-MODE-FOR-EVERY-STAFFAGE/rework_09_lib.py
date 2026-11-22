"""🪚 Author 09 chisel: reads a volumes source, replaces or adds hand-stated solids by id in one list of one scale and writes the source back; every number is stated by the calling script."""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))


class Source:
    """📦 The `volumes` source of one asset."""

    def __init__(self, asset):
        self.path = os.path.join(T, "volumes", asset + ".json")
        self.doc = json.load(open(self.path, encoding="utf-8"))
        self.v = self.doc["volumes"]

    def ids(self, scale, name):
        return [s["id"] for s in self.v[str(scale)][name]]

    def get(self, scale, name, id):
        return next((s for s in self.v[str(scale)][name] if s["id"] == id), None)

    def put(self, scale, name, solid, after=None):
        """📌 Replaces the solid of the same id in place, else inserts it after `after` (or at the end)."""
        items = self.v[str(scale)][name]
        for i, s in enumerate(items):
            if s["id"] == solid["id"]:
                items[i] = solid
                return solid
        at = next((i + 1 for i, s in enumerate(items) if s["id"] == after), len(items))
        items.insert(at, solid)
        return solid

    def drop(self, scale, name, *ids):
        self.v[str(scale)][name] = [s for s in self.v[str(scale)][name] if s["id"] not in ids]

    def decide(self, scale, en, de):
        """📝 Adds one decision sentence pair unless it is already there."""
        d = self.v[str(scale)]["decisions"]
        if not any(x["en"] == en for x in d):
            d.append({"en": en, "de": de})

    def save(self):
        for scale, volume in self.v.items():
            for name in ("body", "thermal", "structure"):
                ids = [s["id"] for s in volume.get(name, [])]
                doubles = sorted({i for i in ids if ids.count(i) > 1})
                if doubles:
                    raise SystemExit(f"{scale} {name}: duplicate ids {doubles}")
        with open(self.path + ".rework09.tmp", "w", encoding="utf-8") as handle:
            json.dump(self.doc, handle, ensure_ascii=False)
        os.replace(self.path + ".rework09.tmp", self.path)


def r(values):
    return [round(float(c), 5) for c in values]


def cap(id, bone, p0, p1, r0, r1=None, **fields):
    return {"id": id, "kind": "capsule", "bone": bone, **fields, "p0": r(p0), "p1": r(p1), "r0": r0, "r1": r0 if r1 is None else r1}


def box(id, bone, size, at, corner=None, rotation=None, **fields):
    transform = {"translation": r(at)}
    if rotation is not None:
        transform = {"rotation": r(rotation), "translation": r(at)}
    out = {"id": id, "kind": "box", "bone": bone, **fields, "size": r(size), "transform": transform}
    if corner is not None:
        out["cornerRadius"] = corner
    return out


def ell(id, bone, radii, at, **fields):
    return {"id": id, "kind": "ellipsoid", "bone": bone, **fields, "radii": r(radii), "transform": {"translation": r(at)}}


def rev(id, bone, profile, axis, origin, **fields):
    return {"id": id, "kind": "revolve", "bone": bone, **fields, "profile": [r(p) for p in profile], "axis": axis, "origin": r(origin)}


def loft(id, bone, sections, **fields):
    """🧵 sections: (at, rx, ry, exponent[, rotation])."""
    out = []
    for s in sections:
        item = {"at": r(s[0]), "rx": s[1], "ry": s[2], "exponent": s[3]}
        if len(s) > 4:
            item["rotation"] = r(s[4])
        out.append(item)
    return {"id": id, "kind": "loft", "bone": bone, **fields, "sections": out, "capStart": True, "capEnd": True}


def of(id, target, **fields):
    return {"id": id, "of": target, **fields}


def both(make):
    """🪞 Calls make(suffix, sign) for the left (+w) and the right (-w) side."""
    return [make("l", 1), make("r", -1)]


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def pads(source, scale, contacts, length, width, material="rubber"):
    """🛞 One support pad per contact anchor (id, bone, x, y, z in the bone frame): a slab of one smallest feature standing on the contact plane inside the tyre."""
    t = FEATURE[scale]
    for anchor, bone, x, y, z in contacts:
        source.put(scale, "structure", box(f"support-{anchor}", bone, [length, t, width], [x, y + t / 2, z], material=material, role="detail", tone=0.1, member=bone, part="support", anchor=anchor))


def rename(source, scale, old, new, solid):
    """🏷️ Replaces body solid `old` by `solid` (id `new`) and re-points the thermal and structure references to it."""
    source.put(scale, "body", {**solid, "id": old})
    source.get(scale, "body", old)["id"] = new
    for name in ("thermal", "structure"):
        for s in source.v[str(scale)][name]:
            if s.get("of") == old:
                s["of"] = new
                s["id"] = s["id"].replace(old, new)
