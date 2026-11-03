import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/dairy-cow/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); r=d['asset']['representations']['500']
for s in r['geometry']['front']:
    if s['id']=='udder': s.pop('followTip',None); s.pop('penMm',None)
t=r['decisions']['silhouette']
t['en']=t['en'].replace('In front the udder uses a 0.2 mm pen so that the small udder of a lying cow stays above the minimum feature.','In front the udder is a rigid block on its bone (it no longer squashes when the cow lies down), so it stays above the minimum feature.')
t['de']=t['de'].replace('In der Vorderansicht zeichnet das Euter mit 0,2 mm Feder, damit das kleine Euter einer liegenden Kuh über dem Mindestmerkmal bleibt.','In der Vorderansicht ist das Euter ein starrer Block auf seinem Knochen (es staucht sich beim Ablegen nicht mehr), sodass es über dem Mindestmerkmal bleibt.')
write_json_atomic(f,d)
