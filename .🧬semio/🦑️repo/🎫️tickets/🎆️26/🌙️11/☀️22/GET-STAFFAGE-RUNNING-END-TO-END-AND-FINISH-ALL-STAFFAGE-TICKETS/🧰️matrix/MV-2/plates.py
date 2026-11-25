#!/usr/bin/env python3
"""🧮️ Census of the depth plates of plants: per scale and view the shapes whose solid is a thin extrusion (0.1 m or thinner; `mvc-` = a plate of `carry.ts`, `slab` = a redrawn blade of `R4-P/redraw.ts` or any other), their number, points and bones, and the flat fills, the round solids. plates.py <catalog/vegetation dir> <id>…"""
import json, re, sys, collections

root = sys.argv[1]
for asset_id in sys.argv[2:]:
    asset = json.load(open(f"{root}/{asset_id}/🔣️.json"))["asset"]
    print("==", asset_id, "default", asset["presentation"]["defaultPose"], "poses", " ".join(p["id"] for p in asset["poses"]))
    for scale, representation in asset["representations"].items():
        line = []
        for view in ("elevation", "front", "plan"):
            kinds = collections.Counter(); plates = collections.Counter(); points = 0; bones = set(); depth = collections.Counter()
            for shape in (representation.get("geometry") or {}).get(view) or []:
                solid = shape.get("solid")
                kinds[solid.get("kind", "?") if solid else "flat"] += 1
                if solid and shape["id"].startswith("mvc-"):
                    plates["mvc"] += 1; bones.add(shape["bone"]); points += len(json.dumps(solid)) // 18
                elif solid and solid.get("kind") == "extruded":
                    plates[re.split(r"[-\d]", shape["id"])[0]] += 1
                    depth[f"{(solid['z1'] - solid['z0']) * 1000:.0f} mm"] += 1
            line.append(f"{view}: {dict(kinds)} extr {dict(plates.most_common(5))} mvc-bones {len(bones)}" + (f" depths {dict(depth.most_common(3))}" if depth else ""))
        print("  1:" + scale, " | ".join(line))
