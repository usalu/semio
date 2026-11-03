import sys,json
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
a=sys.argv[1]; sc=sys.argv[2] if len(sys.argv)>2 else '100'
d=load(a)['asset']; w=rest(d)
print(json.dumps(d['dimensions']))
for b in d['rig']['bones']:
    o=w[b['id']]; t=tip(w,b['id']); print(f"{b['id']:14s} par={str(b['parent']):12s} o=({o[0]:.3f},{o[1]:.3f}) a={o[2]:.1f} t=({t[0]:.3f},{t[1]:.3f}) L={o[3]}")
print([ (r['id'],r.get('model')) for r in d['thermal']['regions']])
r=d['representations'][sc]
for s in r['geometry']['elevation']:
    g={k:v for k,v in s.items() if k not in('id','bone','role','z','filled','tone','region','regions','material','kind','hatch')}
    gs=json.dumps(g)
    if len(gs)>110: gs=gs[:110]+'..'
    print(f"{s['id']:18s} {s['bone']:11s} {s['role'][:4]} z{s['z']} t{s['tone']} {s.get('region') or s.get('regions')} {s.get('material')} {s['kind']} {gs}")
