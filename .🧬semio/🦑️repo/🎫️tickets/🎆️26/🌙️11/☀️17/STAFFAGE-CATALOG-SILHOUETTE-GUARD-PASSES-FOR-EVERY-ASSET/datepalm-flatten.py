"""Date palm: every frond is one flat plate. The volume ribbon (sections in the (u, w) plane of the bone, thickness T along v) and the drawn blade (the same profile as an extruded solid of the same thickness) are generated from one profile."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
T = {"50": 0.00855, "100": 0.01729, "200": 0.0165}
if len(sys.argv) > 1: T = json.loads(sys.argv[1])
F = float(sys.argv[2]) if len(sys.argv) > 2 else 1.0
def num(x):
    s = repr(round(x, 5))
    return s[:-2] if s.endswith(".0") else s
EXT = json.loads(sys.argv[3]) if len(sys.argv) > 3 else {}
d = mv.D("🌳️vegetation/date-palm"); a = d.val("asset"); n = 0
for sc in ("50", "100", "200"):
    t = T[sc]
    for i, s in enumerate(a["volumes"][sc]["body"]):
        if s["kind"] != "loft" or not s["bone"].startswith("fr-"): continue
        for k, sec in enumerate(s["sections"]):
            base = ["asset", "volumes", sc, "body", str(i), "sections", str(k)]
            d.setv(sp.walk(d.root, base + ["ry"]), t)
            for c in (1, 2):
                if sec["at"][c] != 0: d.setv(sp.walk(d.root, base + ["at", str(c)]), 0.0)
        ext = EXT.get(sc, {}).get(s["bone"], 0.0)
        if ext:
            last = len(s["sections"]) - 1
            s["sections"][last]["at"][0] += ext
            d.setv(sp.walk(d.root, ["asset", "volumes", sc, "body", str(i), "sections", str(last), "at", "0"]), s["sections"][last]["at"][0])
        up = [(x["at"][0], x["rx"]) for x in s["sections"]]
        pts = [(u, r) for u, r in up] + [(u, -r) for u, r in reversed(up)]
        path = "M" + "L".join(num(u) + " " + num(w) for u, w in pts) + "Z"
        for view in ("elevation", "front", "plan"):
            sid = s["bone"] + "-blade"
            node = d.shape(sc, view, sid)
            d.set(node, "d", path)
            so = d.field(node, "solid")
            d.set(so, "z0", -t * F); d.set(so, "z1", t * F)
            n += 1
d.commit(); print("fronds", n)
