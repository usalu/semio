"""🧷 Applies an ops file one op at a time (each op reparses the file, so an op can follow an element added by the previous one). python3 humansb-C-apply.py <slug> <ops.json>"""
import importlib.util, json, sys
spec = importlib.util.spec_from_file_location('o', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/humans-ops.py')
o = importlib.util.module_from_spec(spec); spec.loader.exec_module(o)
slug = sys.argv[1]
for op in json.load(open(sys.argv[2])): o.apply(slug, [op])
print("applied")
