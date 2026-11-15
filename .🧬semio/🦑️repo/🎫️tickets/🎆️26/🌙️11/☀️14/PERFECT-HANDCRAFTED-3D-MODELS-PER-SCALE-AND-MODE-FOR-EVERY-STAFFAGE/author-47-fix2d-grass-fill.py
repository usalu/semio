"""Author 47: miscanthus 2D defects. (1) closed body outlines drawn `filled: false` (culm tops and plugs at 1:50, culm axes at 1:100, blade tips and plumes at 1:200) become filled; (2) open detail strokes whose `penMm` (1.1-1.99 mm = 0.1-0.4 m of real width) overrides the scale's pen table (nodes at 1:50, culm axes at 1:200) lose the override."""
import importlib.util, json, re
spec = importlib.util.spec_from_file_location("l", "author-47-lib2d.py"); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
P = l.path("vegetation", "miscanthus-grass")
t = open(P).read()
done = []
def edit(scale, view, sid, fill=False, nopen=False):
    global t
    s, a, b = l.get_shape(t, scale, view, sid)
    seg = t[a:b]
    new = seg
    if fill:
        new = re.sub(r'"filled":\s*false', '"filled": true', new)
    if nopen:
        new = re.sub(r',\s*"penMm":\s*[0-9.]+', '', new)
        new = re.sub(r'"penMm":\s*[0-9.]+\s*,\s*', '', new)
    if new != seg:
        t = t[:a] + new + t[b:]
        done.append((scale, view, sid))
for n in (1, 2, 3, 4):
    edit("50", "elevation", "culm-%db" % n, fill=True)
    edit("50", "elevation", "culm-%da-plug1" % n, fill=True)
    edit("50", "elevation", "culm-%db-plug0" % n, fill=True)
    edit("50", "elevation", "culm-%d-nodes" % n, nopen=True)
    edit("100", "elevation", "culm-%db-axis" % n, fill=True)
    edit("200", "elevation", "plume-%d" % n, fill=True)
    edit("200", "elevation", "culm-%da-axis" % n, nopen=True)
for b in ("l1", "l2", "l3", "r1", "r2", "r3"):
    edit("200", "elevation", "bl-%sb-0" % b, fill=True)
open(P, "w").write(t)
print(len(done), "edits")
d = json.loads(t)
