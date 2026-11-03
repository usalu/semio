import json,glob
def load(aid):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+aid+'/🔣️.json')[0]
    return f,json.load(open(f,encoding='utf8'))
def save(f,d): open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
f,d=load('hedgehog'); rep=d['asset']['representations']['100']
for s in rep['geometry']['front']:
    if s['id']=='dome': s['cy']=0.015; s['ry']=0.048
    if s['id'] in('near-fore','far-fore'):
        for p in s['points']:
            if p[1]==-0.095: p[1]=-0.082
    if s['id'] in('near-hind','far-hind'):
        for p in s['points']:
            if p[1]==-0.105: p[1]=-0.092
t=rep['decisions']['silhouette']
t['en']=t['en'].replace('The front dome is drawn lower (0.036 m half height) so the front height matches the 0.128 m of the side view.','The front legs are drawn 13 mm shorter than at 1:50 so the front height matches the side view.')
t['de']=t['de'].replace('Die Kuppel der Vorderansicht ist niedriger gezeichnet (0,036 m halbe Höhe), damit die Vorderhöhe den 0,128 m der Seitenansicht entspricht.','Die Beine der Vorderansicht sind 13 mm kürzer gezeichnet als bei 1:50, damit die Vorderhöhe der Seitenansicht entspricht.')
save(f,d)
f,d=load('herring-gull')
for sc in ('50','100','200'):
    for s in d['asset']['representations'][sc]['geometry']['plan']:
        if s['id'] in('near-hand','far-hand'): s['x1']=-0.235
save(f,d)
