"""🪚 Shared chisel of author 11: reads and writes the volumes source and the catalog document of one machinery asset and offers terse constructors for hand-stated solids.

Every number comes from the calling `rework-11-<slug>-*.py`; nothing here generates geometry.
"""
import glob
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
SCALES = ("50", "100", "200", "500")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def catalog_path(slug):
    """📍 The catalog document of a machinery asset."""
    return glob.glob(os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "*", slug, "🔣️.json"))[0]


def source_path(slug):
    """📍 The volumes source of a machinery asset."""
    return os.path.join(T, "volumes", "machinery", slug + ".json")


def candidate_path(slug):
    """📍 Where the candidate document for `r3-validate --file` goes."""
    folder = os.path.join(T, "🗑️generated", "r3-" + slug)
    os.makedirs(folder, exist_ok=True)
    return os.path.join(folder, "🔣️.json")


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth=0):
    """🖨️ The layout of `volumes-insert.py`: one member per line, number rows inline."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def load_catalog(slug):
    """📖 The catalog document and its text as read."""
    text = open(catalog_path(slug), encoding="utf-8").read()
    return json.loads(text), text


def save(slug, document, volumes, text_before, publish):
    """💾 Writes the volumes source and the candidate document; with `publish` also the catalog file (refused when it changed since it was read)."""
    open(source_path(slug), "w", encoding="utf-8").write(dump({"volumes": volumes}) + "\n")
    document["asset"]["volumes"] = volumes
    out = dump(document) + "\n"
    open(candidate_path(slug), "w", encoding="utf-8").write(out)
    if publish:
        if open(catalog_path(slug), encoding="utf-8").read() != text_before:
            raise SystemExit("catalog file changed since it was read; run again")
        open(catalog_path(slug), "w", encoding="utf-8").write(out)
        print("catalog written:", catalog_path(slug))
    else:
        print("candidate written:", candidate_path(slug))


def r(x):
    return round(x, 5)


def rr(values):
    return [r(v) for v in values]


def quat(axis, deg):
    """🧭 Unit quaternion [x, y, z, w] of a turn of `deg` about `axis`."""
    n = math.sqrt(sum(a * a for a in axis))
    s = math.sin(math.radians(deg) / 2) / n
    return rr([axis[0] * s, axis[1] * s, axis[2] * s, math.cos(math.radians(deg) / 2)])


def base(id, kind, bone, material, tone, role="body", **more):
    solid = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    solid.update({k: v for k, v in more.items() if v is not None})
    return solid


def box(id, bone, material, tone, size, at, role="body", corner=None, rot=None, repeat=None, **more):
    """📦 A box of `size` [u, v, w] centred at `at`."""
    transform = {"translation": rr(at)}
    if rot:
        transform["rotation"] = rot
    return base(id, "box", bone, material, tone, role, size=rr(size), cornerRadius=corner, transform=transform, repeat=repeat, **more)


def ell(id, bone, material, tone, radii, at, role="body", rot=None, **more):
    """🥚 An ellipsoid of `radii` centred at `at`."""
    transform = {"translation": rr(at)}
    if rot:
        transform["rotation"] = rot
    return base(id, "ellipsoid", bone, material, tone, role, radii=rr(radii), transform=transform, **more)


def cap(id, bone, material, tone, p0, p1, r0, r1=None, role="body", repeat=None, **more):
    """💊 A capsule from `p0` to `p1`."""
    return base(id, "capsule", bone, material, tone, role, p0=rr(p0), p1=rr(p1), r0=r0, r1=r0 if r1 is None else r1, repeat=repeat, **more)


def rev(id, bone, material, tone, profile, axis, origin, role="body", repeat=None, **more):
    """🏺 A revolve of `profile` [axial, radial] about `axis` through `origin`."""
    return base(id, "revolve", bone, material, tone, role, profile=[rr(p) for p in profile], axis=axis, origin=rr(origin), repeat=repeat, **more)


def ext(id, bone, material, tone, profile, plane, a, b, role="body", holes=None, repeat=None, **more):
    """🧇 An extrude of `profile` in `plane` from `a` to `b`."""
    solid = base(id, "extrude", bone, material, tone, role, profile=[rr(p) for p in profile])
    if holes:
        solid["holes"] = [[rr(p) for p in h] for h in holes]
    solid.update({"plane": plane, "from": r(a), "to": r(b)})
    if repeat:
        solid["repeat"] = repeat
    solid.update({k: v for k, v in more.items() if v is not None})
    return solid


def loft(id, bone, material, tone, sections, role="body", repeat=None, **more):
    """🧵 A loft through `sections` of (at, rx, ry, exponent[, rotation])."""
    out = []
    for s in sections:
        sec = {"at": rr(s[0]), "rx": r(s[1]), "ry": r(s[2]), "exponent": s[3] if len(s) > 3 else 2}
        if len(s) > 4 and s[4] is not None:
            sec["rotation"] = s[4]
        out.append(sec)
    return base(id, "loft", bone, material, tone, role, sections=out, repeat=repeat, **more)


def ring(cx, cy, radius, n, start=0.0):
    """⭕ `n` points of a circle, counter-clockwise from `start` degrees."""
    return [[r(cx + radius * math.cos(math.radians(start + 360 * k / n))), r(cy + radius * math.sin(math.radians(start + 360 * k / n)))] for k in range(n)]


def torus(axial, radial, tube, n=10):
    """🍩 Closed revolve profile of a round bead at (`axial`, `radial`)."""
    pts = [[r(axial + tube * math.cos(2 * math.pi * k / n)), r(radial + tube * math.sin(2 * math.pi * k / n))] for k in range(n)]
    return pts + [pts[0]]


def rep(count, translation=None, rotation=None, axis=None, origin=None):
    """🔁 A `repeat` member."""
    out = {"count": count}
    if translation:
        out["translation"] = rr(translation)
    if rotation is not None:
        out.update({"rotationRad": r(rotation), "rotationAxis": axis, "origin": rr(origin)})
    return out


def th(solid_or_id, region, layer=None, tags=None, own=None):
    """🌡️ A thermal entry: a reference to a body solid, or with `own` a solid of its own."""
    if own is not None:
        out = dict(own)
        out["region"] = region
    else:
        out = {"id": "th-" + solid_or_id, "of": solid_or_id, "region": region}
    if layer:
        out["layer"] = layer
    if tags:
        out["tags"] = tags
    return out


def st(id, member, part, of=None, own=None, anchor=None):
    """🏗️ A structural entry: a reference to a body solid, or with `own` a solid of its own."""
    if own is not None:
        out = dict(own)
        out["id"] = id
    else:
        out = {"id": id, "of": of}
    out.update({"member": member, "part": part})
    if anchor:
        out["anchor"] = anchor
    return out


def text(en, de):
    return {"en": en, "de": de}


def find(items, id):
    """🔎 The entry with that id."""
    return next(i for i in items if i["id"] == id)


def pristine(slug):
    """🗄️ The catalog asset as it was before this rework (`T/rework-11-pristine/<slug>.json`), the source of the solids and shapes that are kept as authored."""
    return json.load(open(os.path.join(T, "rework-11-pristine", slug + ".json"), encoding="utf-8"))["asset"]


def kept(asset, scale, list_name, *ids, **changes):
    """🧷 Copies of the named solids of the pristine asset, in the order asked, with `changes` applied to each."""
    out = []
    for id in ids:
        solid = json.loads(json.dumps(find(asset["volumes"][scale][list_name], id)))
        solid.update(changes)
        out.append(solid)
    return out
