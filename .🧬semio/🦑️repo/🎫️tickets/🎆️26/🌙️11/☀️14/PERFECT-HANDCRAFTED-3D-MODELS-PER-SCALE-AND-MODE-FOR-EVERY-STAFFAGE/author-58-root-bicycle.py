import re, json, sys
sys.path.insert(0, ".")
from author_43_root import *
slug = "city-bicycle"
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
a, b = view_span(txt, "200", "front")
blk = txt[a:b]
edits = {"wheel-r": [(r'("x": )-0\.025', r"\g<1>-0.03"), (r'("w": )0\.05\b', r"\g<1>0.06")], "frame-column": [(r'("y": )0\.225', r"\g<1>0.285"), (r'("h": )0\.765', r"\g<1>0.705")]}
for sid, subs in edits.items():
    s, e = span_of(blk, sid)
    seg = blk[s:e]
    for pat, rep in subs:
        seg2 = re.sub(pat, rep, seg, count=1)
        assert seg2 != seg, (sid, pat)
        seg = seg2
    blk = blk[:s] + seg + blk[e:]
json.loads(blk)
txt = txt[:a] + blk + txt[b:]
json.loads(txt)
save(slug, txt)
print("ok")
