"""✏️ Draws the cylinder feet that the 1:200 and 1:500 models now carry (bucket cylinder foot on the stick at 1:200; both feet and the boom rod boss at 1:500) into elevation, front and plan, each directly above the body shape of its bone, with the profile of the solid. Usage: python3 rework-excavator-2d.py <in asset json> <out asset json>"""
import copy, json, sys
src, out = sys.argv[1], sys.argv[2]
doc = json.load(open(src))
R = doc['asset']['representations']
PLAN = (('200', '100', ['bucket-cyl-foot']), ('500', '200', ['stick-cyl-foot', 'boom-rod-boss']), ('500', '100', ['bucket-cyl-foot']))
for target, source, ids in PLAN:
    for view in ('elevation', 'front', 'plan'):
        shapes = R[target]['geometry'][view]
        for i in ids:
            if any(s['id'] == i for s in shapes):
                continue
            s = copy.deepcopy(next(x for x in R[source]['geometry'][view] if x['id'] == i))
            body = next(n for n, x in enumerate(shapes) if x['id'] == s['bone'] + '-body')
            zs = sorted(x['z'] for x in shapes)
            zb = shapes[body]['z']
            above = next((z for z in zs if z > zb), zb + 1)
            s['z'] = round((zb + above) / 2, 3) if above - zb > 1e-6 else zb
            s['role'], s['tone'] = 'body', 0.5 if i != 'boom-rod-boss' else 0.4
            s.pop('penMm', None)
            shapes.insert(body + 1, s)
            print('[DEBUG]', target, view, i, 'z', s['z'], json.dumps(s)[:260])
json.dump(doc, open(out, 'w'), ensure_ascii=False, indent=1)
