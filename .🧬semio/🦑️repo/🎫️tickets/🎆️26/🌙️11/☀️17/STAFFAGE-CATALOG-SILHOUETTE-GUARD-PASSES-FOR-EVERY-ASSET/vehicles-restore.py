"""Restores the representations block of an asset from HEAD: python3 vehicles-restore.py <category/asset>"""
import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
sp = importlib.import_module("sil-span")
d = m.V(sys.argv[1])
h = sp.head_text(sys.argv[1]); hr = sp.parse(h)
vn = sp.walk(d.root, ["asset", "representations"]); hn = sp.walk(hr, ["asset", "representations"])
new = d.t[:vn["s"]] + h[hn["s"]:hn["e"]] + d.t[vn["e"]:]
json.loads(new)
open(d.path, "w", encoding="utf8").write(new)
