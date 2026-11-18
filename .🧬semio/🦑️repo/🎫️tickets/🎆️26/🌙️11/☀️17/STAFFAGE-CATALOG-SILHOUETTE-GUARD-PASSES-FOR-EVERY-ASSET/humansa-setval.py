"""Sets one value of a volume solid by span edit. python3 humansa-setval.py <slug> <scale,scale> <solidId> <path e.g. transform.translation.1> <value>"""
import importlib.util, json, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug, scales, sid, path, value = sys.argv[1], sys.argv[2].split(","), sys.argv[3], sys.argv[4].split("."), float(sys.argv[5])
d = s.Doc("👤️humans/" + slug)
for sc in scales:
    for it in s.walk(d.root, ["asset", "volumes", sc, "body"])["items"]:
        if json.loads(d.t[next(v for k, v in it["pairs"] if k == "id")["s"]:next(v for k, v in it["pairs"] if k == "id")["e"]]) != sid: continue
        n = s.walk(it, path)
        print(sc, sid, ".".join(path), d.t[n["s"]:n["e"]], "->", value)
        d.setv(n, value)
d.commit()
