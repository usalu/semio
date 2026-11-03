import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
def load(a):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+a+'/🔣️.json')[0]; return f,json.load(open(f,encoding='utf8'))
f,d=load('dairy-cow'); r=d['asset']['representations']['500']
r['geometry']['front']=[s for s in r['geometry']['front'] if s['id']!='udder']
t=r['decisions']['silhouette']
t['en']=t['en'].replace('In front the udder is a rigid block on its bone (it no longer squashes when the cow lies down), so it stays above the minimum feature.','The udder is not drawn in front: the chest and the legs hide it, and what is left of it when the cow lies down is a sliver below the minimum feature.')
t['de']=t['de'].replace('In der Vorderansicht ist das Euter ein starrer Block auf seinem Knochen (es staucht sich beim Ablegen nicht mehr), sodass es über dem Mindestmerkmal bleibt.','Das Euter wird in der Vorderansicht nicht gezeichnet: Brust und Beine verdecken es, und was beim Ablegen der Kuh davon übrig bleibt, ist ein Rest unter dem Mindestmerkmal.')
write_json_atomic(f,d)
f,d=load('domestic-cat')
for sc in('50','100'):
    for s in d['asset']['representations'][sc]['geometry']['front']:
        if s['id']=='tail-tip': s['z']=2.81
write_json_atomic(f,d)
f,d=load('herring-gull'); r=d['asset']['representations']['200']
for s in r['geometry']['elevation']:
    if s['id']=='far-leg': s['points']=[[-0.03,0],[0.1,0],[0.1389,0.0521],[0.0942,0.1088]]
write_json_atomic(f,d)
