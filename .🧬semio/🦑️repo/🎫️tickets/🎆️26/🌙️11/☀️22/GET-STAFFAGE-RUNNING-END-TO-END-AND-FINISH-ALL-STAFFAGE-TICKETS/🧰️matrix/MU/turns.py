"""🔄️ Static survey: the bones of each asset that a pose, a clip key or the root turns by 5 degrees or more, with the count of flat (no solid, no member, no loft) front and plan shapes on them or below per scale: `python3 turns.py <id>…`."""
import json, glob, sys
S='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'
for i in sys.argv[1:]:
    a=json.load(open(glob.glob(f'{S}/*/{i}/🔣️.json')[0]))['asset']
    turn={}
    def add(b,o):
        for k in('angleDeg','abductionDeg','twistDeg'):
            v=o.get(k)
            if isinstance(v,(int,float)): turn.setdefault(b,{}).setdefault(k,[]).append(v)
    for p in a['poses']:
        if p.get('root') and abs(p['root'].get('angleDeg',0) or 0)>0: add('ROOT',p['root'])
        for b,o in (p.get('bones') or {}).items(): add(b,o)
    for c in a['clips']:
        for k in c['keys']:
            for b,o in (k.get('bones') or {}).items(): add(b,o)
        for t in (c.get('tracks') or []):
            if t.get('channel') in('angleDeg','abductionDeg','twistDeg') or t.get('property') in('angleDeg','abductionDeg','twistDeg'): turn.setdefault(t.get('bone'),{}).setdefault('track',[]).append(99)
    big={b:{k:(min(v+[0]),max(v+[0])) for k,v in t.items() if max(v+[0])-min(v+[0])>=5} for b,t in turn.items()}
    big={b:t for b,t in big.items() if t}
    par={b['id']:b.get('parent') for b in a['rig']['bones']}
    def under(b):
        while b:
            if b in big: return True
            b=par.get(b)
        return 'ROOT' in big
    flat={k:sum(1 for v in('front','plan') for s in r['geometry'][v] if under(s['bone']) and 'solid' not in s and s['kind'] not in('member','loft')) for k,r in a['representations'].items() if 'geometry' in r}
    print(i, flat, {b:t for b,t in list(big.items())[:6]}, len(big))
