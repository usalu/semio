import json, sys, io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')
base = r'C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🧱️components'
detail = '--detail' in sys.argv
ids = [a for a in sys.argv[1:] if not a.startswith('--')]
for i in ids:
    a = json.load(open(os.path.join(base, i, '🔣️.json'), encoding='utf8'))['asset']
    print('==', i, 'dims', a.get('dimensions'))
    print(' presentation', json.dumps(a.get('presentation')))
    print(' bones', [(b['id'], b.get('parent'), b.get('x'), b.get('y'), b.get('angleDeg'), b.get('lengthM'), (b.get('joint') or {}).get('type')) for b in a['rig']['bones']])
    print(' poses', [p.get('id') for p in a.get('poses', [])] if isinstance(a.get('poses'), list) else a.get('poses'))
    print(' clips', [c.get('id') for c in a.get('clips', [])] if isinstance(a.get('clips'), list) else a.get('clips'))
    for sc, rep in a['representations'].items():
        g = rep.get('geometry', {})
        print(' rep', sc, rep.get('status'), {k: len(v) for k, v in g.items()})
        if detail and sc == detail_scale if False else detail:
            for vp, shapes in g.items():
                for s in shapes:
                    print('   ', vp, json.dumps({k: v for k, v in s.items() if k not in ('title',)}, ensure_ascii=False)[:400])
