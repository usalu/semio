import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/herring-gull/🔣️.json')[0]
d=json.load(open(f,encoding='utf8'))
g=d['asset']['representations']['200']['geometry']['elevation']
near=[s for s in g if s['id']=='near-leg'][0]
for s in g:
    if s['id']=='far-leg': s['points']=[list(p) for p in near['points']]
    if s['id']=='head':
        for p in s['points']:
            if abs(p[0])<0.001 and p[1]>0.049: p[1]=0.0466
write_json_atomic(f,d)
