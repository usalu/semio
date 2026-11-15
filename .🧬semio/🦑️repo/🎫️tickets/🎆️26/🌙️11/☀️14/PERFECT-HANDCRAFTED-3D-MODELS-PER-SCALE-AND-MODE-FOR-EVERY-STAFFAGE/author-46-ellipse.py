"""Usage: python3 author-46-ellipse.py <slug> <scale> <view> <shape id> key=value ... [--apply] — text patch of numeric fields (cx, cy, rx, ry, ...) of one drawn shape."""
import glob, json, re, sys
slug, scale, view, sid = sys.argv[1:5]
kv = [a.split("=") for a in sys.argv[5:] if "=" in a]
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/%s/🔣️.json" % slug)[0]
t = open(path, encoding="utf-8").read()
rep0 = t.index('"representations"')
for m in re.finditer(r'"geometry": \{', t[rep0:]):
    gs = rep0 + m.end() - 1
    if re.findall(r'"denominator": (\d+)', t[rep0:gs])[-1] != scale:
        continue
    vs = t.index('"%s": [' % view, gs)
    i = t.index('"id": "%s"' % sid, vs)
    end = t.index("}", i)
    seg = t[i:end]
    for k, v in kv:
        seg, n = re.subn(r'"%s": -?[0-9.e-]+' % k, '"%s": %s' % (k, v), seg)
        assert n == 1, k
    out = t[:i] + seg + t[end:]
    json.loads(out)
    print(seg.replace("\n", " ")[:300])
    if "--apply" in sys.argv:
        assert open(path, encoding="utf-8").read() == t
        open(path, "w", encoding="utf-8").write(out)
        print("patched")
    break
