"""🧮 Spec of the shapes whose id equals a volume solid id (body role, filled, not texture, kind poly/path/ellipse/rect, no capsule). python3 humansb-A-autospec.py <asset> <scale> <view> [exclude,ids]"""
import json, sys
a, sc, view = sys.argv[1:4]
ex = set(sys.argv[4].split(",")) if len(sys.argv) > 4 else set()
d = json.load(open(f'/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/{a}/🔣️.json'))['asset']
ids = {s['id'] for s in d['volumes'][sc]['body']}
out = []
for s in d['representations'][sc]['geometry'][view]:
    if s['id'] in ids and s['role'] in ('body',) and s.get('filled', True) and s['id'] not in ex and s['kind'] in ('poly', 'path', 'ellipse', 'rect'):
        out.append(f"{s['id']}:{s['id']}")
print(",".join(out))
