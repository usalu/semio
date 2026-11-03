import sys,json,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
a,sc,view=sys.argv[1:4]
r=load(a)['asset']['representations'][sc]
print(json.dumps(r['pens']),json.dumps(r['hatches']),json.dumps(r['validRange']))
for s in r['geometry'][view]:
    g={k:v for k,v in s.items() if k not in('id','bone','role','z','filled','tone','region','regions','material','kind','hatch')}
    gs=json.dumps(g)
    if len(gs)>130: gs=gs[:130]+'..'
    print(f"{s['id']:16s} {s['bone']:10s} {s['role'][:4]} z{s['z']} t{s['tone']} {s.get('region') or s.get('regions')} {s.get('hatch')} {s.get('penMm')} {s['kind']} {gs}")
