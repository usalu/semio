"""📋 Ops entry: shape <id> of scale B takes the geometry (kind, path or points, solid) of the same id at scale A, keeping its own other fields. python3 humansb-A2-copyshape.py <asset> <view> <id> <fromScale> <toScale>"""
import json, sys
slug, view, sid, a, b = sys.argv[1:6]
d = json.load(open('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/%s/🔣️.json' % slug))['asset']
src = [x for x in d['representations'][a]['geometry'][view] if x['id'] == sid][0]
dst = [x for x in d['representations'][b]['geometry'][view] if x['id'] == sid][0]
geo = ("kind", "points", "closed", "d", "solid", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r")
n = {k: v for k, v in src.items() if k in geo}
for k, v in dst.items():
    if k not in geo: n[k] = v
print(json.dumps([{"op": "shape", "scale": b, "view": view, "id": sid, "shape": n}]))
