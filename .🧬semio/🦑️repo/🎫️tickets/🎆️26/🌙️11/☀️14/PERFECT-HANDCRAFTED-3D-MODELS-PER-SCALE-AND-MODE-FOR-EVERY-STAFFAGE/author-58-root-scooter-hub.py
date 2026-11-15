import re, json, sys
sys.path.insert(0, ".")
from author_43_root import *
slug = "motor-scooter"
txt = load(slug)
def span_of(blk, sid):
    i = blk.index('"id": "%s"' % sid)
    s = blk.rfind("{", 0, i)
    d = 0
    for k in range(s, len(blk)):
        if blk[k] == "{": d += 1
        elif blk[k] == "}":
            d -= 1
            if d == 0: return s, k + 1
for scale, old, new in (("50", "0.045", "0.06"), ("100", "0.055", "0.07")):
    a, b = view_span(txt, scale, "elevation")
    blk = txt[a:b]
    for sid in ("hub-f", "hub-r"):
        s, e = span_of(blk, sid)
        seg = blk[s:e]
        seg2 = re.sub(r'("r[xy]": )%s' % re.escape(old), lambda m: m.group(1) + new, seg)
        assert seg2 != seg, (scale, sid)
        blk = blk[:s] + seg2 + blk[e:]
    json.loads(blk)
    txt = txt[:a] + blk + txt[b:]
json.loads(txt)
save(slug, txt)
print("ok")
