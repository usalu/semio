"""🌡️ Second round, pass 1 of `furniture/three-seat-sofa`: every surface solid of the thermal lists names its layer, `fabric` or `beech-foot` for the feet (1:100, 1:200 and 1:500 left it out, so the cover read as no layer of its region), and the four region blocks of 1:500 carry the cover material instead of foam and plywood.

Usage: python3 rework-three-seat-sofa-r2-thermal-layers.py
"""
import json

PATH = "volumes/furniture/three-seat-sofa.json"
BLOCKS_500 = ("th-base", "th-seat", "th-back-cushions", "th-back-frame")

with open(PATH, encoding="utf-8") as handle:
    source = json.load(handle)
for scale, volume in source["volumes"].items():
    for solid in volume["thermal"]:
        if scale == "500" and solid["id"] in BLOCKS_500:
            solid["material"] = "upholstery-fabric"
        if "layer" not in solid:
            solid["layer"] = "beech-foot" if solid["region"] == "feet" else "fabric"
with open(PATH, "w", encoding="utf-8") as handle:
    json.dump(source, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
