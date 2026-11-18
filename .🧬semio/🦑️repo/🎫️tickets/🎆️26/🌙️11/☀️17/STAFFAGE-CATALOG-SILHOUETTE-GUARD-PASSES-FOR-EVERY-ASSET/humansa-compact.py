"""Collapses the numeric arrays of the volumes block that the in-place dump expanded onto several lines back to the one-line style of the file. python3 humansa-compact.py <slug>"""
import importlib.util, json, re, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
d = s.Doc("👤️humans/" + sys.argv[1])
vn = s.walk(d.root, ["asset", "volumes"])
block = d.t[vn["s"]:vn["e"]]
num = r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?"
pat = re.compile(r"\[\s*(" + num + r"(?:\s*,\s*" + num + r")*)\s*\]")
new = pat.sub(lambda m: "[" + ", ".join(x.strip() for x in m.group(1).split(",")) + "]", block)
if new != block:
    t = d.t[:vn["s"]] + new + d.t[vn["e"]:]
    json.loads(t)
    open(d.path, "w", encoding="utf8").write(t)
    print(sys.argv[1], "collapsed", len(block) - len(new), "chars")
else: print(sys.argv[1], "unchanged")
