import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
d = mv.D("🌳️vegetation/scots-pine")
node = d.solid("500", "crown")
last = node["pairs"][-1][1]
ind = d.t[d.t.rfind("\n", 0, node["s"]) + 1:node["s"]]
d.rep({"s": last["e"], "e": last["e"]}, ',\n' + ind + '  "transform": {"scale": [1, 1, 0.974]}')
for sc in ("100", "200"):
    node = d.solid(sc, "bole-upper")
    d.setv(sp.walk(node, ["sections", "0", "at", "0"]), -0.02)
d.commit()
