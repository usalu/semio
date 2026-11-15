"""Author 61: the willow's plan `bole` ellipse (r 0.9 at every scale) contradicted the elevation and front root flare (half width 1.3 m at the ground, 1:100-1:500 `bole` and 1:50 `root-flare`) by 0.4 m: set to 1.3."""
import importlib.util, json, re
spec = importlib.util.spec_from_file_location("l", "author-47-lib2d.py"); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/weeping-willow/🔣️.json"
t = open(P).read()
for scale in ("50", "100", "200", "500"):
    s, a, b = l.get_shape(t, scale, "plan", "bole")
    seg = t[a:b]
    new = re.sub(r'"rx":\s*0\.9', '"rx": 1.3', re.sub(r'"ry":\s*0\.9', '"ry": 1.3', seg))
    assert new != seg, scale
    t = t[:a] + new + t[b:]
    print(scale, "bole plan r 0.9 -> 1.3")
open(P, "w").write(t)
json.loads(t)
