"""🔎️ Round 3 tram: prints the 2D shapes of a scale and view compactly (`python3 r3-tram-info.py 50 elevation [full]`)."""
import json, glob, sys
A = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/tram/🔣️.json")[0]
a = json.load(open(A))['asset']
sc, view = sys.argv[1], sys.argv[2]
full = len(sys.argv) > 3
rep = a['representations'][sc]
print({k: v for k, v in rep.items() if k not in ('geometry', 'decisions')})
for s in rep['geometry'][view]:
    g = {k: v for k, v in s.items() if k not in ('id', 'bone', 'role', 'kind', 'z', 'filled', 'tone', 'region', 'material')}
    t = json.dumps(g, ensure_ascii=False)
    print(s['id'], s.get('bone'), s['role'], s['kind'], 'z', s.get('z'), 'f' if s.get('filled') else '-', s.get('tone'), s.get('region'), s.get('material'), t if full else t[:230])
