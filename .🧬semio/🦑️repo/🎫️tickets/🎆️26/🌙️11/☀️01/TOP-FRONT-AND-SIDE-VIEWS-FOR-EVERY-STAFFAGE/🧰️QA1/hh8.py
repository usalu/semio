import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
r=a['representations']['100']
r['geometry']['front']=[s for s in r['geometry']['front'] if s['id']!='neck']
t=r['decisions']['silhouette']
t['en']+=' The neck is not drawn in front: the spiny dome and the head hide it in every pose, and a hidden sliver would only be a thin stroke.'
t['de']+=' Der Hals wird in der Vorderansicht nicht gezeichnet: Stachelkuppel und Kopf verdecken ihn in jeder Pose, und ein verdeckter Rest wäre nur ein dünner Strich.'
write_json_atomic(f,d)
