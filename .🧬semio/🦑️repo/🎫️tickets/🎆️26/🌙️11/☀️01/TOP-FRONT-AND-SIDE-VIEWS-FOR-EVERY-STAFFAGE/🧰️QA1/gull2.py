import json,glob,sys,copy
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW')
from atomic_json import write_json_atomic
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/herring-gull/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); reps=d['asset']['representations']
for sc in('100','200'):
    g=reps[sc]['geometry']['elevation']
    i=[k for k,s in enumerate(g) if s['id']=='near-leg'][0]
    n=copy.deepcopy(g[i]); n['id']='far-leg'; n['bone']='shank-f'; n['z']=1.2
    g.insert(i+1,n)
    t=reps[sc]['decisions']['silhouette']
    t['en']+=' The side view also draws the far leg behind the near one, so a landing gull with its legs apart keeps the height of the front view.'
    t['de']+=' Die Seitenansicht zeichnet auch das ferne Bein hinter dem nahen, sodass eine landende Möwe mit gespreizten Beinen die Höhe der Vorderansicht behält.'
write_json_atomic(f,d)
