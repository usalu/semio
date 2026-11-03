import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
for sc in('50','100'):
    g=a['representations'][sc]['geometry']
    g['elevation']=[s for s in g['elevation'] if not s['id'].endswith('-far')]
    for s in g['front']:
        if s['id'].startswith('skirt-'):
            near=s['id'].endswith('near')
            s['points']=[[x,(-0.105 if y==-0.075 else (-0.09 if y==-0.06 else y))] for x,y in s['points']]
    t=a['representations'][sc]['decisions']['silhouette']
    t['en']+=' The side view draws only the near skirt of each end; the far skirt lies behind it and is drawn in front and plan.'
    t['de']+=' Die Seitenansicht zeichnet nur den nahen Saum jedes Endes; der ferne liegt dahinter und ist in Vorder- und Draufsicht gezeichnet.'
write_json_atomic(f,d)
