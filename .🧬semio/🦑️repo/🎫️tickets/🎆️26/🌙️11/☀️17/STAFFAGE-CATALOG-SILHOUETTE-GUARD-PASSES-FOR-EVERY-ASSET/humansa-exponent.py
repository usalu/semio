"""Sets the exponent of every loft section of the given solids by span edits. python3 humansa-exponent.py <slug> <solidId,...> <value> <scales>"""
import importlib.util, json, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug, ids, value, scales = sys.argv[1], set(sys.argv[2].split(",")), float(sys.argv[3]), sys.argv[4].split(",")
d = s.Doc("👤️humans/" + slug)
n = 0
for sc in scales:
    for it in s.walk(d.root, ["asset", "volumes", sc, "body"])["items"]:
        sid = json.loads(d.t[next(v for k, v in it["pairs"] if k == "id")["s"]:next(v for k, v in it["pairs"] if k == "id")["e"]])
        if sid not in ids: continue
        for k, v in it["pairs"]:
            if k == "sections":
                for sec in v["items"]:
                    ex = next(vv for kk, vv in sec["pairs"] if kk == "exponent")
                    d.setv(ex, value); n += 1
d.commit()
print(slug, "exponents set", n)
