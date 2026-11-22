"""🔍 Writes close-up candidates of the street lamp (column top with bracket and luminaire, column foot) for `r3-sheet.ts --file`.

Usage: python3 rework-street-lamp-zoom.py <scale>
The column solids are cut to the shown stretch (top: above 5.05 m of the pole bone, lowered by 5 m; foot: below 1.25 m with the bracket bones shrunk onto the stub) so the sheet zooms in; the solids themselves are unchanged.
"""
import copy
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/street-lamp/🔣️.json"
scale = sys.argv[1]
document = json.load(open(CATALOG))
document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes/objects/street-lamp.json")))["volumes"]


def cut(profile, low, high):
    rows = profile
    for bound, keep in ((low, lambda u: u >= low), (high, lambda u: u <= high)):
        clipped = []
        for index, (u, r) in enumerate(rows):
            pu, pr = rows[index - 1]
            if keep(u) != keep(pu):
                clipped.append([bound, pr + (r - pr) * (bound - pu) / (u - pu)])
            if keep(u):
                clipped.append([u, r])
        rows = clipped
    return rows


for part, low, high in (("top", 5.05, 9.0), ("foot", -1.0, 1.25)):
    candidate = copy.deepcopy(document)
    volumes = candidate["asset"]["volumes"][scale]
    body = {solid["id"]: solid for solid in volumes["body"]}
    for name in ("body", "thermal", "structure"):
        kept = []
        for solid in volumes[name]:
            source = solid if "kind" in solid else body[solid["of"]]
            bone = source["bone"]
            if bone == "pole":
                if source["kind"] != "revolve" or source.get("axis") != [1, 0, 0]:
                    reach = json.dumps(source)
                    if (part == "top") != ("5." in reach):
                        continue
                else:
                    us = [p[0] for p in source["profile"]]
                    if max(us) <= low or min(us) >= high:
                        continue
                    if "kind" in solid:
                        solid["profile"] = cut(solid["profile"], low, high)
                    else:
                        body[solid["of"]]["profile"] = cut(source["profile"], low, high)
            elif (part == "top") != (bone in ("arm", "head")):
                continue
            kept.append(solid)
        volumes[name] = kept
    bones = {bone["id"]: bone for bone in candidate["asset"]["rig"]["bones"]}
    if part == "top":
        shift = 5.0
        bones["pole"]["lengthM"] -= shift
        bones["arm"]["x"] -= shift
        for name in ("body", "thermal", "structure"):
            for solid in volumes[name]:
                if solid.get("bone") == "pole" and solid.get("kind") == "revolve":
                    solid["profile"] = [[u - shift, r] for u, r in solid["profile"]]
    else:
        bones["pole"]["lengthM"] = high
        bones["arm"].update(x=high - 0.1, lengthM=0.05)
        bones["head"].update(x=0.02, lengthM=0.02)
    folder = os.path.join(T, "🗑️generated", "rework-street-lamp", f"{part}{scale}")
    os.makedirs(folder, exist_ok=True)
    json.dump(candidate, open(os.path.join(folder, "🔣️.json"), "w"), ensure_ascii=False)
    print(os.path.join(folder, "🔣️.json"))
