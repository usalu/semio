"""🗺️ Writes the search plan of `vol.ts` for a walker from its untouched backup: `python3 volplan.py <asset.json> <out-prefix>` lists, per authored scale, every flat front and plan shape on a limb bone (thigh, shank, foot, toe, arm, forearm, hand) with the makes to try (limbs: bodies of revolution, the elliptic pair and the prism of the front outline; shoes and hands: the three prisms), the volume solids of a shoe part named as the drawing groups them, and writes one plan per scale `<out-prefix>.<scale>.json`."""
import json, re, sys
asset = json.load(open(sys.argv[1]))["asset"]
limb = re.compile(r"^(thigh|shank|foot|toe|arm|forearm|hand)-[lr]$")
for scale, rep in asset["representations"].items():
    if rep.get("status") != "authored": continue
    solids = {s["id"] for s in asset["volumes"][scale]["body"]}
    plan = []
    for view in ("front", "plan"):
        for shape in rep["geometry"][view]:
            bone = shape.get("bone", "")
            if not limb.match(bone) or shape.get("solid") or shape.get("followTip"): continue
            sid, side = shape["id"], bone[-1]
            entry = {"scale": scale, "view": view, "id": sid}
            if bone.startswith(("foot", "toe")):
                group = {"shoe-sole": ["shoe-sole", "shoe-heel"], "shoe-rear": ["shoe-rear", "shoe-heel", "shoe-sole"] if view == "plan" else ["shoe-rear"], "shoe-toe": ["shoe-toe", "shoe-toe-sole"] if view == "plan" else ["shoe-toe"], "shoe-toe-sole": ["shoe-toe-sole"], "shoe": ["shoe"]}.get(sid[:-2])
                if group is None: continue
                entry["solids"] = [f"{g}-{side}" for g in group if f"{g}-{side}" in solids]
                entry["try"] = ["keep", "sweep-w", "sweep-v", "sweep-u"]
            elif bone.startswith("hand"):
                entry["try"] = ["keep", "sweep-w", "sweep-v", "sweep-u", "revolve-m"]
            else:
                entry["try"] = ["keep", "sweep-v", "revolve-m", "revolve-w", "revolve-v"] if view == "front" else ["keep", "revolve-m", "revolve-w", "revolve-v", "ellipse", "sweep-v"]
            if "solids" in entry and not entry["solids"]: continue
            if "solids" not in entry and sid not in solids: continue
            plan.append(entry)
    open(f"{sys.argv[2]}.{scale}.json", "w").write("[\n" + ",\n".join("  " + json.dumps(e) for e in plan) + "\n]\n")
    print(scale, len(plan), "entries")
