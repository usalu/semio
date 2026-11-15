import json, sys, glob
slug = sys.argv[1]
scales = sys.argv[2].split(',') if len(sys.argv) > 2 else ['50']
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*appliances/%s/*.json" % slug)[0]
a = json.load(open(f))["asset"]
def c(x): return json.dumps(x, ensure_ascii=False, separators=(',', ':'))
print("DIM", c(a['dimensions']), "defaultPose", a['presentation'].get('defaultPose'))
print("MATERIALS", [m['id'] for m in a['materials']])
for b in a['rig']['bones']:
    print("BONE", c({k: v for k, v in b.items() if k not in ('title',)}))
for an in a['anchors']:
    print("ANCHOR", c({k: v for k, v in an.items() if k != 'title'}))
t = a['thermal']
for r in t['regions']:
    print("REGION", c({k: r[k] for k in ('id', 'kind', 'areaM2', 'geometry', 'hidden', 'model') if k in r}), "layers", c([(l['id'], l.get('thicknessM'), l.get('material')) for l in r.get('layers', [])]))
for e in t.get('emitters', []): print("EMITTER", c({k: e[k] for k in e if k not in ('title', 'source', 'reference')}))
print("STRUCT", c({k: a['structure'][k] for k in ('supports',)}))
print("POSES", [p['id'] for p in a['poses']])
for s in scales:
    r = a['representations'][s]
    print("=== SCALE", s, r.get('status'))
    for v, shapes in r.get('geometry', {}).items():
        print("--", v)
        for x in shapes:
            d = {k: v2 for k, v2 in x.items() if k not in ('tone', 'bone', 'material', 'filled', 'z', 'closed', 'strokeMm')}
            print("  ", c(d)[:330])
