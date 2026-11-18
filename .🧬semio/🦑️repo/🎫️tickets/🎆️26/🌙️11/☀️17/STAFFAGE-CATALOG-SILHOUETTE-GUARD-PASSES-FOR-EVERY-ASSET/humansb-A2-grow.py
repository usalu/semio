"""📐 Ops entry that adds a constant to every radius of a profile shape. python3 humansb-A2-grow.py <scale> <view> <shapeId> <delta> <asset>"""
import json, sys
sc, view, sid, dl, slug = sys.argv[1:6]
d = json.load(open('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/%s/🔣️.json' % slug))['asset']
s = [x for x in d['representations'][sc]['geometry'][view] if x['id'] == sid][0]
pts = [[a, r if (i == 0 or i == len(s['points']) - 1 or r < 0.003) else round(r + float(dl), 4)] for i, (a, r) in enumerate(s['points'])]
print(json.dumps([{"op": "shape-set", "scale": sc, "view": view, "id": sid, "set": {"points": pts}}]))
