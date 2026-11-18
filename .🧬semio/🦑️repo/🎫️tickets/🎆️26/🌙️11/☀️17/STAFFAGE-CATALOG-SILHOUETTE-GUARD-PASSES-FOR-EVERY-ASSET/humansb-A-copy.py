"""📋 Ops that add shapes of one scale to another (after the last shape). python3 humansb-A-copy.py <asset> <fromScale> <toScale> <view> <id,id,...> <out.json>"""
import json, sys
a, A, B, view, ids, out = sys.argv[1:7]
d = json.load(open(f'/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/{a}/🔣️.json'))['asset']
src = {s['id']: s for s in d['representations'][A]['geometry'][view]}
last = d['representations'][B]['geometry'][view][-1]['id']
ops = []
for i in ids.split(','):
    ops.append({"op": "shape-add", "scale": B, "view": view, "after": last, "shape": src[i]}); last = i
json.dump(ops, open(out, 'w'))
