"""Scales the radii of the finger capsules (hand-l-finger-N and -tip) by span edits. python3 humansa-fingers.py <slug> <factor> <scales> [pattern]"""
import importlib.util, json, re, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug, factor, scales = sys.argv[1], float(sys.argv[2]), sys.argv[3].split(",")
pat = re.compile(sys.argv[4] if len(sys.argv) > 4 else r"^hand-[lr]-finger-\d(-tip)?$")
d = s.Doc("👤️humans/" + slug)
n = 0
for sc in scales:
    for it in s.walk(d.root, ["asset", "volumes", sc, "body"])["items"]:
        sid = None
        for k, v in it["pairs"]:
            if k == "id": sid = json.loads(d.t[v["s"]:v["e"]])
        if sid is None or not pat.match(sid): continue
        for k, v in it["pairs"]:
            if k in ("r0", "r1"):
                old = float(d.t[v["s"]:v["e"]]); d.setv(v, round(old * factor, 5)); n += 1
d.commit()
print(slug, "scaled", n, "radii")
