"""🗒️ Lists what an asset document draws: `python3 dump.py <file.json> [scale] [bone-regex]` prints the rig (bone, parent, length), the anchors, the poses and clips with the bones they turn, and per authored scale and bone the shapes of the three views with kind, solid and local extent."""
import json, re, sys
a = json.load(open(sys.argv[1]))["asset"]
only = sys.argv[2] if len(sys.argv) > 2 and sys.argv[2] != "-" else None
rx = re.compile(sys.argv[3]) if len(sys.argv) > 3 else None
print("id", a["id"], "dims", a["dimensions"])
if rx is None:
    for b in a["rig"]["bones"]:
        print("  bone", b["id"], "<-", b.get("parent"), "len", b.get("lengthM"), {k: v for k, v in b.items() if k not in ("id", "parent", "lengthM", "name", "label")})
    for an in a.get("anchors", []):
        print("  anchor", {k: v for k, v in an.items() if k not in ("label", "name", "description")})
    for kind in ("poses", "clips"):
        items = a.get(kind, [])
        items = items if isinstance(items, list) else [dict(v, id=k) for k, v in items.items()]
        for p in items:
            s = json.dumps(p)
            print("  ", kind[:-1], p.get("id"), "bones:", sorted(set(re.findall(r'"([a-z0-9-]+)": \{"(?:angleDeg|abductionDeg|twistDeg)', s))), len(s))
def ext(s):
    if s["kind"] == "poly":
        xs = [p[0] for p in s["points"]]; ys = [p[1] for p in s["points"]]
        return "x[%.3f,%.3f] y[%.3f,%.3f] n%d%s" % (min(xs), max(xs), min(ys), max(ys), len(xs), "" if s.get("closed") else " open")
    return " ".join("%s=%s" % (k, v) for k, v in s.items() if k in ("x", "y", "w", "h", "x0", "y0", "x1", "y1", "r", "r0", "r1", "cx", "cy", "rx", "ry", "followTip"))
for scale, rep in a["representations"].items():
    if rep.get("status") != "authored" or (only and scale != only): continue
    print("== 1:%s" % scale, rep.get("validRange"), {v: len(rep["geometry"][v]) for v in ("elevation", "front", "plan")})
    bones = []
    for v in ("elevation", "front", "plan"):
        for s in rep["geometry"][v]:
            if s["bone"] not in bones: bones.append(s["bone"])
    for b in bones:
        if rx and not rx.search(b): continue
        for v in ("elevation", "front", "plan"):
            for s in rep["geometry"][v]:
                if s["bone"] != b: continue
                so = s.get("solid")
                sol = "flat" if so is None else so["kind"] + ":" + str(so.get("sweep", so.get("axis", ""))) + " " + " ".join("%s=%s" % (k, w) for k, w in so.items() if k not in ("kind", "sweep", "axis", "axisBone"))
                print("  %-12s %-9s %-22s %-5s %-6s %s | %s" % (b, v, s["id"], s["kind"], s.get("role", ""), sol, ext(s)))
