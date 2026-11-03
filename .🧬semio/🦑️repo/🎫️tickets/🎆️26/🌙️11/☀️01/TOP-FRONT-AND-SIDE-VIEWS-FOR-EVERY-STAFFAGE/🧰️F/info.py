import sys
from lib import *
n=sys.argv[1]; sc=sys.argv[2]
doc,_=load_original(n); a=doc['asset']
w=rest_world(a)
print('dims',a['dimensions'],'default',a['presentation']['defaultPose'])
for b in a['rig']['bones']:
    o=w[b['id']]; print(f"{b['id']:14s} par={str(b['parent']):12s} o=({o[0]:.3f},{o[1]:.3f}) a={math.degrees(o[2]):.1f} L={b['lengthM']} {b['joint']}")
for p in a['poses']: print('pose',p['id'],json.dumps(p['bones']))
for c in a['clips']: print('clip',c['id'],c['durationS'],[ (k['t'],k['pose']) for k in c['keys']] if 'keys' in c else '')
r=a['representations'][sc]
print('pens',r['pens'],[h['id'] for h in r['hatches']])
for s in r['geometry']['elevation']:
    g={k:v for k,v in s.items() if k not in('id','bone','role','z','filled','tone','region','regions','material','kind','hatch','penMm')}
    print(f"{s['id']} {s['bone']} {s['role'][:3]} z{s['z']} {'F' if s['filled'] else 'o'} t{s['tone']} {s.get('region') or s.get('regions')} {s.get('material')} {s['kind']} {json.dumps(g)}")
