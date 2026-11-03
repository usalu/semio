import json, sys, io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')
base = r'C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🧱️components'
out = r'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\C1'
ids = sys.argv[1:]
for i in ids:
    path = os.path.join(base, i, '🔣️.json')
    raw = open(path, encoding='utf8').read()
    j = json.loads(raw)
    rt = json.dumps(j, ensure_ascii=False, indent=2)
    a = j['asset']
    with open(os.path.join(out, f'dump-{i}.txt'), 'w', encoding='utf8') as f:
        f.write(f'roundtrip-identical {rt == raw or rt + chr(10) == raw}\n')
        f.write('dims %s\n' % a.get('dimensions'))
        f.write('presentation %s\n' % json.dumps(a.get('presentation')))
        for b in a['rig']['bones']:
            if b['id'].startswith('slide-') and b['id'] not in ('slide-0', 'slide-1'):
                continue
            if b['id'].startswith('slat-') and b['id'] not in ('slat-0', 'slat-1'):
                continue
            f.write('bone %s\n' % json.dumps({k: v for k, v in b.items() if k in ('id', 'parent', 'x', 'y', 'angleDeg', 'lengthM', 'joint')}, ensure_ascii=False))
        for p in a.get('poses', []):
            f.write('pose %s\n' % json.dumps(p, ensure_ascii=False)[:600])
        for c in a.get('clips', []):
            f.write('clip %s\n' % json.dumps(c, ensure_ascii=False)[:900])
        for sc, rep in a['representations'].items():
            f.write(f'--- scale {sc} {rep.get("status")}\n')
            if rep.get('status') != 'authored':
                continue
            f.write('pens %s hatches %s\n' % (rep.get('pens'), json.dumps(rep.get('hatches'), ensure_ascii=False)))
            f.write('keys %s\n' % list(rep.keys()))
            for vp, shapes in rep['geometry'].items():
                for s in shapes:
                    f.write('%s %s\n' % (vp, json.dumps({k: v for k, v in s.items() if k != 'title'}, ensure_ascii=False)))
            f.write('decisions %s\n' % json.dumps(rep['decisions'], ensure_ascii=False))
