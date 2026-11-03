import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/dairy-cow/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); r=d['asset']['representations']['500']
for v in('elevation','plan'):
    r['geometry'][v]=[s for s in r['geometry'][v] if s['id']!='udder']
t=r['decisions']['silhouette']
t['en']=t['en'].replace('The udder is not drawn in front: the chest and the legs hide it, and what is left of it when the cow lies down is a sliver below the minimum feature.','The udder is left out of all three views at 1:500: at 0.3 mm it is below the minimum feature, and hides behind the chest and legs in front.')
t['de']=t['de'].replace('Das Euter wird in der Vorderansicht nicht gezeichnet: Brust und Beine verdecken es, und was beim Ablegen der Kuh davon übrig bleibt, ist ein Rest unter dem Mindestmerkmal.','Das Euter entfällt bei 1:500 in allen drei Ansichten: Mit 0,3 mm liegt es unter dem Mindestmerkmal und versteckt sich vorn hinter Brust und Beinen.')
write_json_atomic(f,d)
