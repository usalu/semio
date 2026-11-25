"""🚶️ The ops of a person whose flat front and plan limbs were drawn for the stride of the default pose (nurse-walking, office-worker-carrying-laptop, patient-with-iv-pole): `python3 walker-ops.py <asset.json> [--no-arm-plan 50,100] > ops/<id>.json`. For every authored scale and every closed body shape of a limb bone (thigh, shank, arm, forearm, hand) that has an elevation twin: `front-sweep` (the front outline swept along the bone normal over the thickness of the elevation outline) and, where a plan twin exists, `plan-revolve` with `"radius": "front"` and `"rebuild"` (a body of revolution of the width the front shows; the limb volumes are lofts of elliptic section, and the mean of width and thickness misses the plan silhouette); `--no-arm-plan` leaves the plan of arms and hands as it is at the scales named (where the silhouette guard refuses the revolved arm). For every such shape of a foot or toe bone `extrude` in front and plan (the elevation outline over the width of the shoe). Shapes that are already revolved are left alone."""
import json, re, sys
a = json.load(open(sys.argv[1]))["asset"]
LIMB = re.compile(r"^(thigh|shank|arm|forearm|upper-arm|hand)-[lr]$")
FOOT = re.compile(r"^(foot|toe)-[lr]$")
ops = []
SKIP = set((sys.argv[sys.argv.index("--no-arm-plan") + 1] if "--no-arm-plan" in sys.argv else "").split(","))
for scale, rep in a["representations"].items():
    if rep.get("status") != "authored": continue
    geo = rep["geometry"]
    def twin(view, s): return next((t for t in geo[view] if t["id"] == s["id"] and t["bone"] == s["bone"]), None)
    for s in geo["elevation"]:
        if s.get("role") == "ground" or s["kind"] != "poly" or not s.get("closed") or (s.get("solid") or {}).get("kind") == "revolved": continue
        front, plan = twin("front", s), twin("plan", s)
        if LIMB.match(s["bone"]):
            if front is None: continue
            kind = front.get("solid")
            if kind is not None and not (kind["kind"] == "extruded" and kind.get("sweep") == "v"): continue
            ops.append({"op": "front-sweep", "scale": scale, "id": s["id"]})
            arm = not re.match("thigh|shank", s["bone"])
            if plan is not None and not (arm and scale in SKIP): ops.append({"op": "plan-revolve", "scale": scale, "id": s["id"], "radius": "front", "rebuild": True})
        elif FOOT.match(s["bone"]):
            for view, t in (("front", front), ("plan", plan)):
                if t is not None: ops.append({"op": "extrude", "scale": scale, "id": s["id"], "view": view})
print("[\n" + ",\n".join("  " + json.dumps(o) for o in ops) + "\n]")
