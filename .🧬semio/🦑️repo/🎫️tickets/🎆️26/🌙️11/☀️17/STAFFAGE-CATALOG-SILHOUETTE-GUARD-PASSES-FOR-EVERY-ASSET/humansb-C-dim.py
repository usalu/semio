"""📏 Sets one value of an asset in place. python3 humansb-C-dim.py <slug> <json-path comma-separated> <value>"""
import importlib.util, json, sys
spec = importlib.util.spec_from_file_location('s', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/sil-span.py')
s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
d = s.Doc(sys.argv[1]); n = d.at(*sys.argv[2].split(",")); print("was", d.t[n["s"]:n["e"]])
d.setv(n, json.loads(sys.argv[3])); d.commit()
