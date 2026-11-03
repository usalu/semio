import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
NEAR=[[-0.065,0.02],[-0.063,-0.06],[-0.05,-0.075],[-0.03,-0.075],[-0.017,-0.06],[-0.015,0.02]]
for sc in('50','100'):
    g=a['representations'][sc]['geometry']['front']
    for s in g:
        if s['id'].startswith('skirt-'):
            far=s['id'].endswith('far')
            pts=[[-x,y] for x,y in reversed(NEAR)] if far else NEAR
            for k in('cx','cy','rx','ry'): s.pop(k,None)
            s['kind']='poly'; s['points']=[list(p) for p in pts]; s['closed']=True; s['followTip']=True
write_json_atomic(f,d)
