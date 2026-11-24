"""🔬️ The hard findings of a gate log beside the shapes they concern: `python3 facts.py <gate.log> <asset.json> [--subjects]` prints per bone or anchor the scales, codes, worst deltas and subjects, then the shapes of that bone in the three views at those scales (kind, solid, local extent; for an anchor only with `--anchor-shapes`); `--subjects` prints instead the regex of every subject a hard finding named (for `gate2.ts --only`)."""
import json, re, sys, collections
log, doc = sys.argv[1], sys.argv[2]
a = json.load(open(doc))["asset"]
found = collections.defaultdict(list)
subjects = set()
scale = code = None
for line in open(log):
    m = re.match(r"\S+ 1:(\d+) (HARD|twin|reported) (\S+) ", line)
    if m:
        scale, code = m.group(1), m.group(3) if m.group(2) == "HARD" else None
        continue
    m = re.match(r"    (\S+) ×(\d+) (\d+) mm @ (.*)$", line)
    if m and code:
        found[m.group(1)].append((scale, code, int(m.group(2)), int(m.group(3)), m.group(4)))
        subjects.add(m.group(4))
if "--subjects" in sys.argv:
    print("^(" + "|".join(re.escape(s) for s in sorted(subjects)) + ")$")
    sys.exit()
def ext(s):
    if s["kind"] == "poly":
        xs = [p[0] for p in s["points"]]; ys = [p[1] for p in s["points"]]
        return "x[%.3f,%.3f] y[%.3f,%.3f] n%d%s" % (min(xs), max(xs), min(ys), max(ys), len(xs), "" if s.get("closed") else " open")
    return s["kind"] + " " + " ".join("%s=%s" % (k, v) for k, v in s.items() if k in ("x", "y", "w", "h", "x0", "y0", "x1", "y1", "r", "r0", "r1", "cx", "cy", "rx", "ry", "followTip"))
bones = {b["id"]: b for b in a["rig"]["bones"]}
for key, items in sorted(found.items()):
    print("##", key)
    for it in items: print("   1:%s %s ×%d %d mm @ %s" % it)
    bone = key.split("@")[0]
    if bone not in bones:
        an = next((x for x in a["anchors"] if x["id"] == bone), None)
        if an: print("   anchor", {k: v for k, v in an.items() if k in ("bone", "x", "y", "z", "kind")})
        if "--anchor-shapes" not in sys.argv or not an: continue
        bone = an["bone"]
    b = bones[bone]; print("   bone %s <- %s len %s angle %s abd %s twist %s z %s" % (bone, b.get("parent"), b.get("lengthM"), b.get("angleDeg"), b.get("abductionDeg"), b.get("twistDeg"), b.get("z")))
    kids = [c for c in bones if bones[c].get("parent") == bone]
    for sc in sorted({it[0] for it in items}, key=int):
        for v in ("elevation", "front", "plan"):
            for s in a["representations"][sc]["geometry"][v]:
                if s["bone"] != bone and not (bone != key.split("@")[0] and s["bone"] in kids): continue
                so = s.get("solid")
                sol = "flat" if so is None else so["kind"][:3] + ":" + str(so.get("sweep", so.get("axis", ""))) + " " + " ".join("%s=%s" % (k, w) for k, w in so.items() if k not in ("kind", "sweep", "axis", "axisBone", "mirror"))
                print("   %4s %-4s %-10s %-20s %-6s %s | %s" % (sc, v[:4], s["bone"], s["id"][:20], s.get("role", "")[:6], sol, ext(s)))
