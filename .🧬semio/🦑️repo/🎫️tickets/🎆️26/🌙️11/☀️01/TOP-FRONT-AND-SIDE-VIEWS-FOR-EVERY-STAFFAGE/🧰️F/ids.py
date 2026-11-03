import sys
from lib import *
n=sys.argv[1]
doc,_=load_original(n); a=doc['asset']
for sc in sys.argv[2].split(','):
    r=a['representations'][sc]; print('##',sc,r['pens'],[h['id'] for h in r['hatches']])
    for s in r['geometry']['elevation']:
        g={k:v for k,v in s.items() if k not in('id','bone','role','z','filled','tone','region','regions','material','kind','hatch','penMm')}
        print(f"{s['id']} {s['bone']} {s['role'][:3]} z{s['z']} {'F' if s['filled'] else 'o'} {s.get('region') or s.get('regions')} {s['kind']} {json.dumps(g)[:150]}")
