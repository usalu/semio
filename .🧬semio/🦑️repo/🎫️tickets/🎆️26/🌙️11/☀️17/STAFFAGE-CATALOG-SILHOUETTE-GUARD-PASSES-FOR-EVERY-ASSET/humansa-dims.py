"""Sets one declared dimension of a humans asset by span edit. python3 humansa-dims.py <slug> <heightM|widthM|depthM> <value>"""
import importlib.util, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug, key, value = sys.argv[1], sys.argv[2], float(sys.argv[3])
d = s.Doc("👤️humans/" + slug)
n = d.at("asset", "dimensions", key)
print(slug, key, d.t[n["s"]:n["e"]], "->", value)
d.setv(n, value)
d.commit()
