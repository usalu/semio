"""Author 47: miscanthus plan capsules (1:100 x-blades 'b', 1:200 z-blades 'b') end one tip radius beyond their blade (rounded cap), so the plan is 20-50 mm longer than the elevation/front drawings of the same blade (C11 whole-asset extents); the capsule end point moves back by the tip radius so that the cap ends where the blade ends."""
import importlib.util, json, math, re
spec = importlib.util.spec_from_file_location("l", "author-47-lib2d.py"); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
P = l.path("vegetation", "miscanthus-grass")
t = open(P).read()
def fix(scale, sid):
    global t
    s, a, b = l.get_shape(t, scale, "plan", sid)
    dx, dy = s["x1"] - s["x0"], s["y1"] - s["y0"]
    L = math.hypot(dx, dy)
    k = s["r1"] / L
    nx, ny = round(s["x1"] - dx * k, 4), round(s["y1"] - dy * k, 4)
    seg = t[a:b]
    seg = re.sub(r'"x1":\s*[-0-9.e]+', '"x1": %s' % nx, seg)
    seg = re.sub(r'"y1":\s*[-0-9.e]+', '"y1": %s' % ny, seg)
    t = t[:a] + seg + t[b:]
for side in "lr":
    for n in "123":
        for scale, k in (("100", "1"), ("200", "2")):
            try:
                fix(scale, "%s%s-%s-b" % (side, n, k))
            except ValueError:
                pass
open(P, "w").write(t)
print("done")
