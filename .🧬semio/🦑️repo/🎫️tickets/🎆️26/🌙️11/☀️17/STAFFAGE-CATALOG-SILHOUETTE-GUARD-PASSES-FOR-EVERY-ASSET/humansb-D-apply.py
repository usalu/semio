"""🧷 Applies ops files to a humans asset one op at a time (each op re-reads the file, so an op may refer to a shape inserted by an earlier op) through the span edits of humans-ops.py. python3 humansb-D-apply.py <slug> <ops.json> [<ops.json> ...]"""
import importlib.util, json, sys
spec = importlib.util.spec_from_file_location('h', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/humans-ops.py')
h = importlib.util.module_from_spec(spec); spec.loader.exec_module(h)
slug = sys.argv[1]
n = 0
for f in sys.argv[2:]:
    for op in json.load(open(f)):
        h.apply(slug, [op]); n += 1
print("applied", n, "ops to", slug)
