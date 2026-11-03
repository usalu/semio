import json, sys, io, os, glob
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')
root = r'C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog'
aid, scale = sys.argv[1], sys.argv[2]
views = sys.argv[3].split(',') if len(sys.argv) > 3 else ['elevation', 'front', 'plan']
p = glob.glob(os.path.join(root, '*', aid, '🔣️.json'))[0]
a = json.load(open(p, encoding='utf8'))['asset']
rep = a['representations'][scale]
print('pens', rep.get('pens'), 'hatches', rep.get('hatches'))
for vp in views:
    for s in rep['geometry'].get(vp, []):
        print(vp, json.dumps({k: v for k, v in s.items() if k != 'title'}, ensure_ascii=False))
if '--dec' in sys.argv:
    print(json.dumps(rep['decisions'], ensure_ascii=False, indent=1))
