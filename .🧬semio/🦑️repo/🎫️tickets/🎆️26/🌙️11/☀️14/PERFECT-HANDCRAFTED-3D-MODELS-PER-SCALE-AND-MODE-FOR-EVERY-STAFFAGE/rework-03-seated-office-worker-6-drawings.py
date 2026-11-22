"""✏️ Stage 6 of the seated office worker: the 2D drawings follow the restated solids. Per scale and view (elevation first, then front and plan) the outline of every part is ruled with `r3-project2d.ts` and written over the shape of the same id; parts that are new get a shape of their own with the tone, region and material of their solid; the revolved arm profiles are restated from the loft sections; strokes and shapes without a solid (mouth) stay.

Run: `python3 rework-03-seated-office-worker-6-drawings.py [scale ...]` (writes the catalog file view by view, re-read before every write).
"""
import subprocess
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
REGION = {"cotton-shirt": "shirt", "pa-plastic": "chair-plastic", "steel-painted": "chair-steel", "desk-steel": "desk"}
NEW = {"shirt-waist": "shirt", "chair-armrest-sleeve-l": "chair-armrest-post-l", "chair-armrest-sleeve-r": "chair-armrest-post-r", "chair-armrest-bracket-l": "chair-armrest-post-l", "chair-armrest-bracket-r": "chair-armrest-post-r",
       "chair-backrest-slider": "chair-backrest-pad", "chair-backrest-strut": "chair-backrest-pad", "chair-lever": "chair-paddle", "chair-knob": "chair-mechanism", "desk-rail-front": "desk-rail-back",
       "chair-backrest-pivot": "chair-backrest-pad", "chair-mechanism-arm": "chair-mechanism", "chair-mechanism": "chair-seat-cushion",
       **{f"chair-stem-{k}": f"chair-wheel-{k}" for k in ("front", "side-a", "back-a", "back-b", "side-b")}}
scales = sys.argv[1:] or ["50", "100", "200", "500"]


def arm_profile(solid):
    secs = solid["sections"]
    ax = sum(s["at"][1] for s in secs) / len(secs)
    az = sum(s["at"][2] for s in secs) / len(secs)
    pts = [[secs[0]["at"][0], round(ax, 4)]] + [[s["at"][0], round(ax + (s["rx"] + s["ry"]) / 2, 4)] for s in secs] + [[secs[-1]["at"][0], round(ax, 4)]]
    return pts, round(ax, 4), round(az, 4)


for scale in scales:
    body = by_id(read(source_path(A))["volumes"][scale]["body"])
    for view in ("elevation", "front", "plan"):
        doc = read(catalog_path(A))
        shapes = doc["asset"]["representations"][scale]["geometry"][view]
        have = {s["id"] for s in shapes}
        for id, sibling in NEW.items():
            if id in body and id not in have and sibling in have:
                ref = next(s for s in shapes if s["id"] == sibling)
                solid = body[id]
                seed = [[0, 0], [0.01, 0], [0, 0.01]]
                new = {"id": id, "bone": solid["bone"], "role": "body", "z": round(ref["z"] + (0.04 if id != "shirt-waist" else -0.04), 2), "filled": True, "tone": solid["tone"],
                       "region": ref["region"] if solid["material"] not in REGION else (REGION[solid["material"]] if solid["material"] != "steel-painted" or not solid["bone"].startswith("desk") else "desk"),
                       "material": solid["material"], "kind": "poly", "points": seed, "closed": True}
                shapes.insert(shapes.index(ref) + (1 if id != "shirt-waist" else 0), new)
        for s in shapes:
            solid = body.get(s["id"])
            if solid and s.get("solid", {}).get("kind") == "revolved" and s["id"].startswith(("sleeve", "forearm")):
                s["points"], s["solid"]["axisX"], s["solid"]["axisZ"] = arm_profile(solid)
            if solid and s["material"] != solid["material"]:
                s.update({"material": solid["material"], "tone": solid["tone"], "region": REGION.get(solid["material"], s["region"])})
        write(catalog_path(A), doc)
        run = subprocess.run([bun(), "r3-project2d.ts", A, "--scale", scale, "--view", view, "--candidate", "--quiet"], capture_output=True, text=True, cwd=T)
        assert run.returncode == 0, run.stderr[-600:]
        cand = read(f"{T}/🗑️generated/r3-project2d/{A}.s{scale}.{view}/🔣️.json")["asset"]["representations"][scale]["geometry"][view]
        ruled = {}
        for s in cand:
            ruled.setdefault(s["id"], []).append(s)
        doc = read(catalog_path(A))
        shapes = doc["asset"]["representations"][scale]["geometry"][view]
        out, kept, replaced = [], 0, 0
        for s in shapes:
            if "solid" in s or s["id"] not in ruled or s.get("filled") is False:
                out.append(s)
                kept += 1
                continue
            new = ruled[s["id"]]
            if any(n.get("points") == s.get("points") for n in new) and len(new) == 1 and s["id"] not in body:
                out.append(s)
                kept += 1
                continue
            for k, n in enumerate(new):
                n = dict(n)
                if k:
                    n["id"] = f"{s['id']}-{k + 1}"
                out.append(n)
            replaced += 1
        doc["asset"]["representations"][scale]["geometry"][view] = out
        write(catalog_path(A), doc)
        print(scale, view, "shapes", len(out), "ruled", replaced, "kept", kept, "new ids", [s["id"] for s in out if s["id"].startswith("r3-new")])
