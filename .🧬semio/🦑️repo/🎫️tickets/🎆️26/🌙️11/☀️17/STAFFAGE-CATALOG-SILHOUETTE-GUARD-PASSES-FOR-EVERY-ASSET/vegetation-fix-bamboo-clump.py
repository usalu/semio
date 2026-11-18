"""Bamboo: the plan ellipse that marks the top of a culm was drawn about 0.1 m (1:100) beside the culm of the volume; its centre goes to the end of the culm line in the plan and its radius is the 8 mm tip radius of the volume culm (it was 0.04 m).  Runs after the rewrites (the culm line is the traced centre line of the slender volume culm)."""
import sys, json, re, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
d = mv.D("🌳️vegetation/bamboo-clump"); a = d.val("asset"); n = 0
for sc in ("50", "100", "200"):
    g = a["representations"][sc]["geometry"]["plan"]
    lines = {s["id"]: s for s in g if s["id"].endswith("-c-line")}
    for t in [s for s in g if s["id"].endswith("-top") and s["kind"] == "ellipse"]:
        l = lines.get(t["id"].replace("-top", "-c-line"))
        if l is None: continue
        nums = [float(x) for x in re.findall(r"-?\d+\.?\d*", l["d"])]
        end = (nums[-2], nums[-1])
        node = d.shape(sc, "plan", t["id"])
        d.setv(sp.walk(node, ["cx"]), round(end[0], 4)); d.setv(sp.walk(node, ["cy"]), round(end[1], 4)); d.setv(sp.walk(node, ["rx"]), 0.008); d.setv(sp.walk(node, ["ry"]), 0.008); n += 1
d.commit(); print("culm tops moved", n)
