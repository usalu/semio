"""🔬️ Structure of an asset document: `python3 show.py <file.json> [scale]` prints the bones, poses, clips and anchors, and per authored scale and bone the shapes of the three views with their construction (flat, followTip, sweep v, sweep w, revolved)."""
import json, sys, collections
a = json.load(open(sys.argv[1]))["asset"]
only = sys.argv[2] if len(sys.argv) > 2 else None
print(a["id"], a["dimensions"], "default", a["presentation"]["defaultPose"])
print("bones:", " ".join(f'{b["id"]}<{b.get("parent")}' for b in a["rig"]["bones"]))
print("poses:", " ".join(p["id"] if isinstance(p, dict) else p for p in (a["poses"] if isinstance(a["poses"], list) else a["poses"].keys())))
print("clips:", " ".join(c["id"] if isinstance(c, dict) else c for c in (a.get("clips") or [])))
for an in a.get("anchors", []):
    print("  anchor", an["id"], an["kind"], an["bone"], an.get("x"), an.get("y"), an.get("z"))
def kind(s):
    so = s.get("solid")
    base = s["kind"] + ("+tip" if s.get("followTip") else "")
    if so is None: return base
    if so["kind"] == "extruded": return f'{base}:sw{so.get("sweep","?")}'
    return f'{base}:{so["kind"][:3]}'
for key, rep in a["representations"].items():
    if rep.get("status") != "authored" or (only and key != only): continue
    bones = collections.OrderedDict()
    for view in ("elevation", "front", "plan"):
        for s in rep["geometry"][view]:
            if s.get("role") == "ground": continue
            bones.setdefault(s["bone"], {"elevation": [], "front": [], "plan": []})[view].append(f'{s["id"]}({kind(s)})')
    print(f"== 1:{key}", {v: len(rep["geometry"][v]) for v in ("elevation", "front", "plan")})
    for bone, views in bones.items():
        print(f"  {bone}: E[{' '.join(views['elevation'])}] F[{' '.join(views['front'])}] P[{' '.join(views['plan'])}]"[:700])
