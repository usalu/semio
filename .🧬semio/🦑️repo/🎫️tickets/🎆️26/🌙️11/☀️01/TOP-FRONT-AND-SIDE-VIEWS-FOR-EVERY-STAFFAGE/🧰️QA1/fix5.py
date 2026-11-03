import json,glob,re
def load(aid):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+aid+'/🔣️.json')[0]
    return f,json.load(open(f,encoding='utf8'))
def save(f,d): open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
f,d=load('herring-gull')
for sc in ('50','100','200'):
    for s in d['asset']['representations'][sc]['geometry']['front']:
        m=re.fullmatch(r'(near|far)-(arm|hand)',s['id'])
        if m:
            side,part=m.groups(); lo,hi=(0.063,0.097) if part=='arm' else (0.065,0.095)
            piv=0.068
            z0,z1=((-hi+piv),(-lo+piv)) if side=='near' else ((lo-piv),(hi-piv))
            s['solid']['z0']=round(z0,4); s['solid']['z1']=round(z1,4)
save(f,d)
f,d=load('horse'); reps=d['asset']['representations']
drop={'50':{'forelock','muzzle','eye-n','eye-f'},'100':{'muzzle'}}
for sc,ids in drop.items():
    g=reps[sc]['geometry']; g['plan']=[s for s in g['plan'] if s['id'] not in ids]
    t=reps[sc]['decisions']['silhouette']
    t['en']+=' In plan the muzzle'+(', forelock and eyes' if sc=='50' else '')+' are left out: a head capsule follows its bone from joint to tip, so short features along it would stretch when the head is lowered; the head capsule carries the plan silhouette.'
    t['de']+=' Im Grundriss entfallen der Maulteil'+(', der Schopf und die Augen' if sc=='50' else '')+': Die Kopfkapsel folgt ihrem Knochen von Gelenk zu Spitze, kurze Merkmale darauf würden sich beim Senken des Kopfes strecken; die Kopfkapsel trägt die Silhouette im Grundriss.'
save(f,d)
