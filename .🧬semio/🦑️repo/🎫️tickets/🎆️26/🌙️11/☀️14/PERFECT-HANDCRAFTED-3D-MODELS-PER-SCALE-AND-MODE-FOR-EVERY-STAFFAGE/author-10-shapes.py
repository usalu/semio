import json,sys
d=json.load(open(f'🗑️generated/author10/{sys.argv[1]}.json'))
scales=[sys.argv[2]] if len(sys.argv)>2 else ['50','100','200','500']
for s in scales:
    v=d['scales'][s]
    if v['status']!='authored': print(s,'unavailable'); continue
    for view in ('elevation','front','plan'):
        print('##',s,view)
        for e in v['views'][view]:
            if not e['rings']: continue
            if e['role'] in ('texture',) and len(sys.argv)<4: continue
            pts=[p for r in e['rings'] for p in r]
            xs=[p[0] for p in pts]; ys=[p[1] for p in pts]
            print(f"  {e['id']:20} {e['role'][:4]} {'C' if e['closed'] else 'o'} {str(e['bone']):12} x {min(xs):.3f}..{max(xs):.3f} y {min(ys):.3f}..{max(ys):.3f} n={len(pts)}")
