"""🪚 Shared chisel of author 16: loads and saves a volumes source and offers the solid constructors the per-asset rework scripts state their hand-chosen numbers with."""
import json
import os

import importlib.util

T = os.path.dirname(os.path.abspath(__file__))
_SPEC = importlib.util.spec_from_file_location("volumes_insert", f"{T}/volumes-insert.py")
_INSERT = importlib.util.module_from_spec(_SPEC)
_ARGV = __import__("sys").argv
__import__("sys").argv = ["volumes-insert.py"]
try:
    _SPEC.loader.exec_module(_INSERT)
except SystemExit:
    pass
__import__("sys").argv = _ARGV
FACES = ["+u", "-u", "+v", "-v", "+w", "-w"]


def load(slug):
    """📖 Reads the volumes source of an appliance."""
    with open(f"{T}/volumes/appliances/{slug}.json", encoding="utf-8") as handle:
        return json.load(handle)


def save(slug, document):
    """💾 Writes the volumes source of an appliance."""
    with open(f"{T}/volumes/appliances/{slug}.json", "w", encoding="utf-8") as handle:
        handle.write(_INSERT.dump(document, 0) + "\n")


def put(items, solid, after=None, before=None):
    """📌 Replaces the solid of the same id or inserts it after or before a named neighbour, else appends it."""
    for index, item in enumerate(items):
        if item["id"] == solid["id"]:
            items[index] = solid
            return
    anchor = after or before
    for index, item in enumerate(items):
        if item["id"] == anchor:
            items.insert(index + (1 if after else 0), solid)
            return
    items.append(solid)


def drop(items, *ids):
    """🧹 Removes the solids with the given ids."""
    items[:] = [item for item in items if item["id"] not in ids]


def get(items, ident):
    """🔎 Returns the solid with the given id."""
    return next(item for item in items if item["id"] == ident)


def base(ident, kind, bone, material, role, tone, **more):
    """🧱 Common members of every solid."""
    return {"id": ident, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **more}


def box(ident, bone, material, size, at, role="body", tone=0.5, radius=None, **more):
    """📦 A box of size [u, v, w] centred at `at`."""
    solid = base(ident, "box", bone, material, role, tone, size=size)
    if radius:
        solid["cornerRadius"] = radius
    solid["transform"] = {"translation": at}
    solid.update(more)
    return solid


def revolve(ident, bone, material, profile, axis, origin, role="body", tone=0.5, **more):
    """🏺 A turned part: profile [axial, radial] about `axis` through `origin`."""
    return base(ident, "revolve", bone, material, role, tone, profile=profile, axis=axis, origin=origin, **more)


def extrude(ident, bone, material, profile, plane, start, end, role="body", tone=0.5, **more):
    """🧇 A prismatic part: profile in `plane`, swept from `start` to `end`."""
    return base(ident, "extrude", bone, material, role, tone, profile=profile, plane=plane, **{"from": start, "to": end}, **more)


def capsule(ident, bone, material, p0, p1, r0, r1=None, role="body", tone=0.5, **more):
    """💊 A round bar from p0 to p1."""
    return base(ident, "capsule", bone, material, role, tone, p0=p0, p1=p1, r0=r0, r1=r0 if r1 is None else r1, **more)


def thermal(solid, region, layer=None, inner=False):
    """🌡️ Tags a solid for the thermal list; `inner` marks a part inside the body that shows every face in a section."""
    solid = dict(solid)
    solid["region"] = region
    if layer:
        solid["layer"] = layer
    if inner:
        solid["sectionFaces"] = FACES
    return solid


def structural(solid, member, part="member", anchor=None):
    """🏗️ Tags a solid for the structure list."""
    solid = dict(solid)
    solid["member"] = member
    solid["part"] = part
    if anchor:
        solid["anchor"] = anchor
    return solid


def decide(volume, en, de):
    """📝 Appends a decision once."""
    if not any(item["en"] == en for item in volume["decisions"]):
        volume["decisions"].append({"en": en, "de": de})
