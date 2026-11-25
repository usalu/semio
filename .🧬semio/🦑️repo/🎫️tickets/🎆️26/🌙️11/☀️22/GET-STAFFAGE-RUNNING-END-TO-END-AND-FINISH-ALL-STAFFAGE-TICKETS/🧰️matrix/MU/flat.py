"""🫓️ The flat (no solid, no member, no loft) front and plan shapes on the bones that a pose or clip turns by 5 degrees or more, or below them: `python3 flat.py <id> [bone…]`."""
import json, glob, sys
S='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'
a=json.load(open(glob.glob(f'{S}/*/{sys.argv[1]}/🔣️.json')[0]))['asset']
turn=set(sys.argv[2:])
if not turn:
    for o in [p.get('bones') or {} for p in a['poses']]+[k.get('bones') or {} for c in a['clips'] for k in c['keys']]:
        for b,v in o.items():
            if any(abs(v.get(k) or 0)>=5 for k in('angleDeg','abductionDeg','twistDeg')): turn.add(b)
par={b['id']:b.get('parent') for b in a['rig']['bones']}
def under(b):
    while b:
        if b in turn: return b
        b=par.get(b)
for k,r in a['representations'].items():
    for v in('front','plan'):
        for s in (r.get('geometry') or {v:[]})[v]:
            if under(s['bone']) and 'solid' not in s and s['kind'] not in('member','loft'):
                print(k,v,s['id'],s['bone'],'under',under(s['bone']),s['kind'],'fill' if s.get('filled') else 'line',('closed' if s.get('closed',True) else 'open'))
for b in sorted(turn): print(b,'parent',par[b])
