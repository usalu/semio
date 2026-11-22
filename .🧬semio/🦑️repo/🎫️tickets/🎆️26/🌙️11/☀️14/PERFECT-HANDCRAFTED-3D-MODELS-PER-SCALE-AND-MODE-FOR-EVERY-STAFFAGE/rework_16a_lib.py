"""🧰 Chisel of author 16a: solid and 2D-shape constructors, and the exact member-wise write of a catalog asset (rig, poses, 2D drawings) next to its volumes source. Every number stays in the calling rework-16a-<slug>-model.py script."""
import copy
import glob
import json
import math
import os
import subprocess
import sys

from rework_16_lib import T, _INSERT, box, revolve, extrude, capsule, thermal, structural, base

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
UP = [-0.5, -0.5, -0.5, 0.5]
AHEAD = [0.5, 0.5, 0.5, 0.5]
FACES = ["+u", "-u", "+v", "-v", "+w", "-w"]


def r5(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value, 5)


def pts(points):
    """🔢 Rounds a point list."""
    return [[r5(a), r5(b)] for a, b in points]


def vloft(ident, bone, material, stations, exponent, role="body", tone=0.5, caps=True, at=(0.0, 0.0), **more):
    """🏺 A shell or body lofted upwards: stations (height, half extent across w, half extent along u)."""
    sections = [{"at": [at[0], r5(y), at[1]], "rx": r5(hw), "ry": r5(hu), "exponent": exponent, "rotation": UP} for y, hw, hu in stations]
    return base(ident, "loft", bone, material, role, tone, sections=sections, capStart=caps, capEnd=caps, **more)


def xloft(ident, bone, material, stations, exponent, role="body", tone=0.5, caps=True, at=(0.0, 0.0), **more):
    """🧈 A part lofted along the bone: stations (u, half height along v, half width across w)."""
    sections = [{"at": [r5(u), at[0], at[1]], "rx": r5(hv), "ry": r5(hw), "exponent": exponent, "rotation": AHEAD} for u, hv, hw in stations]
    return base(ident, "loft", bone, material, role, tone, sections=sections, capStart=caps, capEnd=caps, **more)


def wloft(ident, bone, material, stations, exponent, role="body", tone=0.5, caps=True, **more):
    """🧈 A part lofted across the bone (along w): stations (centre u, centre v, w, half extent along u, half extent along v)."""
    sections = [{"at": [r5(u), r5(v), r5(w)], "rx": r5(hu), "ry": r5(hv), "exponent": exponent} for u, v, w, hu, hv in stations]
    return base(ident, "loft", bone, material, role, tone, sections=sections, capStart=caps, capEnd=caps, **more)


def ellipsoid(ident, bone, material, radii, at, role="detail", tone=0.5, **more):
    """🥚 An ellipsoid centred at `at`."""
    return base(ident, "ellipsoid", bone, material, role, tone, radii=radii, transform={"translation": at}, **more)


def superellipse(ra, rb, exponent, count):
    """⭕ Outline |a/ra|^e + |b/rb|^e = 1 as `count` points, counter-clockwise from (ra, 0)."""
    out = []
    for k in range(count):
        t = 2 * math.pi * k / count
        c, s = math.cos(t), math.sin(t)
        out.append([r5(ra * math.copysign(abs(c) ** (2 / exponent), c)), r5(rb * math.copysign(abs(s) ** (2 / exponent), s))])
    return out


def arc(cx, cy, radius, a0, a1, steps):
    """🌙 Points of an arc from a0 to a1 degrees, both ends included."""
    return [[r5(cx + radius * math.cos(math.radians(a0 + (a1 - a0) * k / steps))), r5(cy + radius * math.sin(math.radians(a0 + (a1 - a0) * k / steps)))] for k in range(steps + 1)]


def rrect(a0, b0, a1, b1, radius, steps=3):
    """▢ Rounded rectangle, counter-clockwise."""
    return arc(a1 - radius, b0 + radius, radius, -90, 0, steps) + arc(a1 - radius, b1 - radius, radius, 0, 90, steps) + arc(a0 + radius, b1 - radius, radius, 90, 180, steps) + arc(a0 + radius, b0 + radius, radius, 180, 270, steps)


def of(ident, body_id, **tags):
    """🔗 A thermal or structural reference to a body solid."""
    return {"id": ident, "of": body_id, **tags}


def shape(ident, bone, role, z, tone, region, material=None, filled=True, **geometry):
    """✏️ One 2D shape; `region` may be a list of regions."""
    out = {"id": ident, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out["regions" if isinstance(region, list) else "region"] = region
    if material:
        out["material"] = material
    out.update(geometry)
    return out


def poly(points, closed=True):
    """✏️ Polygon geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect(x, y, w, h, r=None):
    """✏️ Rectangle geometry from its lower left corner."""
    out = {"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(h)}
    if r:
        out["r"] = r
    return out


def ellipse(cx, cy, rx, ry):
    """✏️ Ellipse geometry."""
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(rx), "ry": r5(ry)}


def mirror_outline(right):
    """🪞 Closes a right-hand outline (bottom to top) with its mirror image."""
    return [[a, b] for a, b in right] + [[-a, b] for a, b in reversed(right)]


def catalog_path(slug):
    """📍 The catalog file of an asset."""
    return glob.glob(f"{CATALOG}/*/{slug}/*.json")[0]


def source_path(slug):
    """📍 The volumes source of an appliance."""
    return f"{T}/volumes/appliances/{slug}.json"


def save_source(slug, volumes):
    """💾 Writes the volumes source."""
    with open(source_path(slug), "w", encoding="utf-8") as handle:
        handle.write(_INSERT.dump({"volumes": volumes}, 0) + "\n")


def candidate(slug, mutate, volumes):
    """🧪 Writes the catalog document with the root changes and the new volumes to the candidate file of r3-validate --file."""
    document = json.load(open(catalog_path(slug), encoding="utf-8"))
    mutate(document["asset"])
    document["asset"]["volumes"] = volumes
    folder = f"{T}/🗑️generated/r3-{slug}"
    os.makedirs(folder, exist_ok=True)
    path = f"{folder}/🔣️.json"
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False)
    return path


def publish(slug, mutate):
    """📤 Re-reads the catalog file, replaces only the members `mutate` changed and splices the volumes source."""
    path = catalog_path(slug)
    text = open(path, encoding="utf-8").read()
    asset = json.loads(text)["asset"]
    changed = copy.deepcopy(asset)
    mutate(changed)
    result = text
    for key, value in changed.items():
        if key == "volumes" or asset.get(key) == value:
            continue
        start, end = _INSERT.member_range(result, "    ", key)
        result = result[:start] + "    " + json.dumps(key) + ": " + _INSERT.dump(value, 2) + result[end:]
    check = json.loads(result)["asset"]
    if {k: v for k, v in check.items() if k != "volumes"} != {k: v for k, v in changed.items() if k != "volumes"}:
        raise SystemExit("root patch did not round-trip")
    if open(path, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while patching; retry")
    if result != text:
        open(path, "w", encoding="utf-8").write(result)
    print(subprocess.run([sys.executable, f"{T}/volumes-insert.py", source_path(slug), path], capture_output=True, text=True).stdout.strip())


def run(slug, mutate, volumes):
    """🚦 Writes source and candidate; with --publish also the catalog file."""
    save_source(slug, volumes)
    print(candidate(slug, mutate, volumes))
    if "--publish" in sys.argv:
        publish(slug, mutate)


def tangents(path):
    """🧭 Tangent angle (radians) at every point of a centre line in the (u, v) plane, by central difference."""
    out = []
    for k in range(len(path)):
        a, b = path[max(k - 1, 0)], path[min(k + 1, len(path) - 1)]
        out.append(math.atan2(b[1] - a[1], b[0] - a[0]))
    return out


def sweep(ident, bone, material, stations, exponent, role="body", tone=0.5, caps=True, at_w=0.0, **more):
    """🥨 A bar lofted along a centre line in the (u, v) plane: stations (u, v, half thickness in the plane, half width across w)."""
    sections = []
    for (u, v, hp, hw), a in zip(stations, tangents(stations)):
        s, c = math.sin(a / 2), math.cos(a / 2)
        q = [0.5 * (c - s), 0.5 * (c + s), 0.5 * (c + s), 0.5 * (c - s)]
        sections.append({"at": [r5(u), r5(v), at_w], "rx": r5(hp), "ry": r5(hw), "exponent": exponent, "rotation": [round(x, 6) for x in q]})
    return base(ident, "loft", bone, material, role, tone, sections=sections, capStart=caps, capEnd=caps, **more)


def edges(stations):
    """📏 The two edges of a swept bar in its plane: centre plus and minus the half thickness along the normal."""
    plus, minus = [], []
    for (u, v, hp, _), a in zip(stations, tangents(stations)):
        plus.append([r5(u - hp * math.sin(a)), r5(v + hp * math.cos(a))])
        minus.append([r5(u + hp * math.sin(a)), r5(v - hp * math.cos(a))])
    return plus, minus


def band(stations):
    """📏 Closed side outline of a swept bar."""
    plus, minus = edges(stations)
    return plus + minus[::-1]


def moved(points, du, dv):
    """↔️ Shifts stations or points in the plane."""
    return [[p[0] + du, p[1] + dv, *p[2:]] for p in points]
