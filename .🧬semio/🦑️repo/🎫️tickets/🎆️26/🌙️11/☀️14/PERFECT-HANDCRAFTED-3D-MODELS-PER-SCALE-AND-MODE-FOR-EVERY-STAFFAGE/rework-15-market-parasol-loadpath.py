"""⛱️ Rework 15 market parasol: closes the structural load path from the mast into the base. The foot hardware (base plate, clevis flange and cheeks or socket, mast lug) carries the mast and is now part of the structure list, and the mast masses sit against the tube wall instead of floating in the bore.

Usage: python3 rework-15-market-parasol-loadpath.py [--apply]
"""
import sys

from rework_15_kit import Kit, SCALES, rnd

kit = Kit("objects/market-parasol")
FOOT = {"50": [("base-plate", "base"), ("clevis-flange", "base"), ("clevis-cheek-near", "base"), ("clevis-cheek-far", "base"), ("mast-lug", "mast")],
        "100": [("base-plate", "base"), ("base-socket", "base")], "200": [("base-socket", "base")]}
BORE = {"50": {"mass-mast": 0.0225, "mass-mast-upper": 0.019}, "100": {"mass-mast": 0.0205, "mass-mast-upper": 0.0205}}
for scale in SCALES:
    structure = kit.solids(scale, "structure")
    for id, member in FOOT.get(scale, []):
        if kit.solid(scale, id) is not None and not any(s.get("of") == id for s in structure):
            structure.append({"id": "st-" + id, "of": id, "member": member, "part": "member"})
    for s in structure:
        if s["id"] in BORE.get(scale, {}):
            s["radii"] = [s["radii"][0], BORE[scale][s["id"]], BORE[scale][s["id"]]]
    for name in ("body", "thermal", "structure"):
        kit.volumes[scale][name][:] = [rnd(s, 8) for s in kit.volumes[scale][name]]
kit.finish(sys.argv)
