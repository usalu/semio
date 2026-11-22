"""🪚️ Chisel of author 02: turns hand-chosen anatomical key sections into the solids of a staffage volume source.

Nothing here decides a form: every station, radius and offset is stated by the asset script that calls it. The helpers only
write the solid JSON (sections normal to the bone axis u, `ry` along v, `rx` along w), smooth a loft between the stated key
stations (monotone cubic, so no overshoot), round its ends, mirror a left part to the right, inset a shell and cut a loft.
"""
import copy
import json
import math
import os

Q = [0, -0.70711, 0, 0.70711]
T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
CATEGORY = {"humans": "👤️humans"}


def r5(x):
    """🔢️ Round to 0.01 mm."""
    return round(x + 0.0, 5)


def _pchip(xs, ys):
    """📈️ Monotone cubic interpolant (Fritsch-Carlson) through (xs, ys)."""
    n = len(xs)
    h = [xs[i + 1] - xs[i] for i in range(n - 1)]
    d = [(ys[i + 1] - ys[i]) / h[i] for i in range(n - 1)]
    m = [0.0] * n
    if n == 2:
        m = [d[0], d[0]]
    else:
        m[0], m[-1] = d[0], d[-1]
        for i in range(1, n - 1):
            m[i] = 0.0 if d[i - 1] * d[i] <= 0 else 3 * (h[i - 1] + h[i]) / ((2 * h[i] + h[i - 1]) / d[i - 1] + (h[i] + 2 * h[i - 1]) / d[i])

    def f(x):
        i = max(0, min(n - 2, next((k for k in range(n - 1) if x <= xs[k + 1]), n - 2)))
        t = (x - xs[i]) / h[i]
        return (2 * t**3 - 3 * t**2 + 1) * ys[i] + (t**3 - 2 * t**2 + t) * h[i] * m[i] + (-2 * t**3 + 3 * t**2) * ys[i + 1] + (t**3 - t**2) * h[i] * m[i + 1]

    return f


def stations(keys, sub):
    """🧵️ Key rows (u, cv, cw, rv, rw[, exponent]) smoothed to `sub` steps per key interval."""
    keys = [tuple(k) + ((2,) if len(k) < 6 else ()) for k in keys]
    us = [k[0] for k in keys]
    rev = us[0] > us[-1]
    if rev:
        keys = keys[::-1]
        us = us[::-1]
    fs = [_pchip(us, [k[c] for k in keys]) for c in range(1, 6)]
    out = []
    for i in range(len(us) - 1):
        for s in range(sub):
            u = us[i] + (us[i + 1] - us[i]) * s / sub
            out.append((u,) + tuple(f(u) for f in fs))
    out.append(keys[-1])
    return out[::-1] if rev else out


def _section(row):
    u, cv, cw, rv, rw, e = row
    return {"at": [r5(u), r5(cv), r5(cw)], "rx": r5(rw), "ry": r5(rv), "exponent": round(e, 2), "rotation": Q}


def loft(id, bone, material, tone, keys, sub=3, role="body", round0=0.0, round1=0.0, **extra):
    """🏺️ A loft through the stated key sections; `round0` / `round1` are the lengths of rounded ends beyond the first / last key."""
    rows = stations(keys, sub)
    steps = (0.3, 0.6, 0.82, 0.95) if sub > 1 else (0.5, 0.88)
    direction = 1 if rows[-1][0] >= rows[0][0] else -1
    if round0:
        u, cv, cw, rv, rw, e = rows[0]
        rows = [(u - direction * round0 * s, cv, cw, rv * math.sqrt(1 - s * s), rw * math.sqrt(1 - s * s), e) for s in steps[::-1]] + rows
    if round1:
        u, cv, cw, rv, rw, e = rows[-1]
        rows = rows + [(u + direction * round1 * s, cv, cw, rv * math.sqrt(1 - s * s), rw * math.sqrt(1 - s * s), e) for s in steps]
    return {"id": id, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": [_section(r) for r in rows], "capStart": True, "capEnd": True, **extra}


def keep(rows, scale):
    """🪜️ The stations of a part that a scale keeps: the seventh member of a row is the coarsest scale that still has it."""
    return [r[:6] for r in rows if r[6] >= scale]


def tilted(solid, degrees):
    """📐️ Re-states a loft whose rows were written in a frame turned by `degrees` against its bone (a foot written ahead / up on an oblique foot bone): stations and section planes are turned into the bone frame."""
    a = math.radians(degrees)
    c, s, h = math.cos(a), math.sin(a), 0.70711
    rotation = [r5(h * math.sin(-a / 2)), r5(-h * math.cos(-a / 2)), r5(h * math.sin(-a / 2)), r5(h * math.cos(-a / 2))]
    for sec in solid["sections"]:
        x, y = sec["at"][0], sec["at"][1]
        sec["at"][0], sec["at"][1] = r5(x * c + y * s), r5(-x * s + y * c)
        sec["rotation"] = rotation
    return solid


def shell2(solid, inset_w, inset_v, id, **extra):
    """🧅️ A shell with its own inset across the breadth (w) and the depth (v): a flat sole has less depth to give."""
    s = shell(solid, 0, id, **extra)
    for sec in s["sections"]:
        sec["rx"], sec["ry"] = r5(max(sec["rx"] - inset_w, sec["rx"] * 0.35)), r5(max(sec["ry"] - inset_v, sec["ry"] * 0.35))
    return s


def capsule(id, bone, material, tone, p0, p1, r0, r1=None, role="body", **extra):
    """💊️ A capsule between two stated points."""
    return {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": [r5(c) for c in p0], "p1": [r5(c) for c in p1], "r0": r0, "r1": r0 if r1 is None else r1, **extra}


def ellipsoid(id, bone, material, tone, at, radii, role="body", rotation=None, **extra):
    """🥚️ An ellipsoid of stated radii (u, v, w) at a stated centre."""
    transform = {"translation": [r5(c) for c in at]}
    if rotation:
        transform["rotation"] = rotation
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": list(radii), "transform": transform, **extra}


def box(id, bone, material, tone, at, size, corner, role="body", **extra):
    """📦️ A rounded box of stated size (u, v, w) at a stated centre."""
    return {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": list(size), "cornerRadius": corner, "transform": {"translation": [r5(c) for c in at]}, **extra}


def _swap(text):
    for a, b in (("-l-", "-r-"),):
        if a in text:
            return text.replace(a, b)
    return text[:-2] + "-r" if text.endswith("-l") else text


def mirror(solid):
    """🪞️ The right-hand twin of a left-hand solid: w negated, `-l` ids and bones become `-r`."""
    s = copy.deepcopy(solid)
    s["id"] = _swap(s["id"])
    for key in ("bone", "of", "member"):
        if key in s:
            s[key] = _swap(s[key])
    for sec in s.get("sections", []):
        sec["at"][2] = r5(-sec["at"][2])
    for key in ("p0", "p1"):
        if key in s:
            s[key][2] = r5(-s[key][2])
    if "profile" in s and s.get("plane") == "xy":
        s["from"], s["to"] = r5(-s["to"]), r5(-s["from"])
    t = s.get("transform")
    if t:
        if "translation" in t:
            t["translation"][2] = r5(-t["translation"][2])
        if "rotation" in t:
            x, y, z, w = t["rotation"]
            t["rotation"] = [-x, -y, z, w]
    return s


def both(solid):
    """👐️ A left solid and its mirrored right twin."""
    return [solid, mirror(solid)]


def shell(solid, inset, id, **extra):
    """🧅️ The solid shrunk by `inset` on every radius: the next inner layer of a thermal region."""
    s = copy.deepcopy(solid)
    s["id"] = id
    for sec in s.get("sections", []):
        sec["rx"], sec["ry"] = r5(max(sec["rx"] - inset, sec["rx"] * 0.35)), r5(max(sec["ry"] - inset, sec["ry"] * 0.35))
    if s["kind"] == "box":
        s["size"] = [r5(c - 2 * inset) for c in s["size"]]
        s["cornerRadius"] = r5(max(s["cornerRadius"] - inset, 0.0005))
    if s["kind"] == "capsule":
        s["r0"], s["r1"] = r5(s["r0"] - inset), r5(s["r1"] - inset)
    if s["kind"] == "ellipsoid":
        s["radii"] = [r5(c - inset) for c in s["radii"]]
    s.update(extra)
    return s


def cut(solid, u0, u1, id, **extra):
    """✂️ The part of a loft between the stations u0 and u1 (linear between its sections)."""
    s = copy.deepcopy(solid)
    s["id"] = id
    secs = s["sections"]
    if secs[0]["at"][0] > secs[-1]["at"][0]:
        secs = secs[::-1]

    def at(u):
        for a, b in zip(secs, secs[1:]):
            if a["at"][0] <= u <= b["at"][0]:
                t = (u - a["at"][0]) / ((b["at"][0] - a["at"][0]) or 1)
                mix = lambda p, q: r5(p + (q - p) * t)
                return {"at": [r5(u), mix(a["at"][1], b["at"][1]), mix(a["at"][2], b["at"][2])], "rx": mix(a["rx"], b["rx"]), "ry": mix(a["ry"], b["ry"]), "exponent": round(a["exponent"] + (b["exponent"] - a["exponent"]) * t, 2), "rotation": Q}
        raise ValueError(f"{solid['id']}: station {u} outside")

    s["sections"] = [at(u0)] + [x for x in secs if u0 < x["at"][0] < u1] + [at(u1)]
    s.update(extra)
    return s


def of(id, target, region, **extra):
    """🔗️ A thermal or structural reference to a body solid."""
    return {"id": id, "of": target, "region": region, **extra}


def area(solid):
    """📐️ Lateral area of a loft in square metres (Ramanujan perimeter of each section, trapezoid along the path), for checking a region against its `areaM2`."""
    total = 0.0
    secs = solid["sections"]

    def perimeter(s):
        a, b = s["rx"], s["ry"]
        return math.pi * (3 * (a + b) - math.sqrt((3 * a + b) * (a + 3 * b))) * (1 + 0.035 * (s["exponent"] - 2))

    for a, b in zip(secs, secs[1:]):
        total += (perimeter(a) + perimeter(b)) / 2 * math.dist(a["at"], b["at"])
    return total


def source_path(asset):
    """📁️ The volume source of an asset in the ticket."""
    return os.path.join(T, "volumes", asset + ".json")


def catalog_path(asset):
    """📁️ The catalog document of an asset."""
    category, slug = asset.split("/")
    return os.path.join(CATALOG, CATEGORY[category], slug, "🔣️.json")


def load_source(asset):
    """📖️ The volume source as stored."""
    with open(source_path(asset), encoding="utf-8") as f:
        return json.load(f)


def save_source(asset, source):
    """💾️ Write the volume source (re-read by every tool)."""
    with open(source_path(asset), "w", encoding="utf-8") as f:
        json.dump(source, f, ensure_ascii=False, indent=1)
        f.write("\n")


def load_catalog(asset):
    """📖️ The catalog document, read immediately before a write."""
    with open(catalog_path(asset), encoding="utf-8") as f:
        return json.load(f)


def save_catalog(asset, document, path=None):
    """💾️ Write the catalog document in its stored formatting: two-space JSON with the volumes spliced by `volumes-insert.py` from the ticket source."""
    import subprocess
    path = path or catalog_path(asset)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
        f.write("\n")
    subprocess.run(["python3", os.path.join(T, "volumes-insert.py"), source_path(asset), path], check=True, stdout=subprocess.DEVNULL)
