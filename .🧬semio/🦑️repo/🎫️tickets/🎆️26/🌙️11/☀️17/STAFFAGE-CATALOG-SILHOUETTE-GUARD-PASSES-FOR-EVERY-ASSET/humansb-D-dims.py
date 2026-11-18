"""📏 Sets one declared dimension value in place (span edit). python3 humansb-D-dims.py <slug> <heightM|widthM|depthM> <value>"""
import importlib.util, sys
spec = importlib.util.spec_from_file_location('s', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
d = s.Doc(sys.argv[1])
d.setv(d.at("asset", "dimensions", sys.argv[2]), float(sys.argv[3]))
d.commit()
