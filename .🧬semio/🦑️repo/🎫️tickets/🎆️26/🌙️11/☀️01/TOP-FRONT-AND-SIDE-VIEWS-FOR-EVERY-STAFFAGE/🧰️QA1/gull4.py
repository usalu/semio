import json,glob,sys
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/herring-gull/🔣️.json')[0]
d=json.load(open(f,encoding='utf8'))
r=d['asset']['representations']['200']
for s in r['geometry']['elevation']:
    if s['id']=='head': s['points']=[[p[0],round(p[1]*0.88,5)] for p in s['points']]
t=r['decisions']['silhouette']
t['en']+=' The head is drawn 12 % slimmer than the 1:100 blob so the default-pose height stays within the stated height.'
t['de']+=' Der Kopf ist 12 % schlanker gezeichnet, sodass die Höhe der Standardpose innerhalb der angegebenen Höhe bleibt.'
write_json_atomic(f,d)
