import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/herring-gull/🔣️.json')[0]
d=json.load(open(f,encoding='utf8'))
for s in d['asset']['representations']['200']['geometry']['elevation']:
    if s['id']=='far-leg':
        a=[0.1389,0.0521]; b=[0.0863,0.1188]; k=0.75
        s['points']=[[-0.03,0],[0.1,0],a,[round(a[0]+k*(b[0]-a[0]),4),round(a[1]+k*(b[1]-a[1]),4)]]
write_json_atomic(f,d)
