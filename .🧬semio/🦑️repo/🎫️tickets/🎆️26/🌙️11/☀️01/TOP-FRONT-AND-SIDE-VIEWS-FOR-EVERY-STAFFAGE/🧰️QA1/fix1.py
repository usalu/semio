import json,glob
def load(aid):
    f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+aid+'/🔣️.json')[0]
    return f,json.load(open(f,encoding='utf8'))
def save(f,d):
    open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
def note(rep,en,de):
    s=rep['decisions']['silhouette']; s['en']+=' '+en; s['de']+=' '+de
# cat
f,d=load('domestic-cat'); r=d['asset']['representations']['100']['geometry']
for v in ('front','plan'):
    drop={'near-eye','far-eye'}|({'nose'} if v=='front' else set())
    r[v]=[s for s in r[v] if s['id'] not in drop]
note(d['asset']['representations']['100'],'At 1:100 the front and plan views leave out the eyes (below 0.1 mm) and, in front, the nose; the 1:50 sheets draw them.','Bei 1:100 lassen Vorder- und Draufsicht die Augen (unter 0,1 mm) und in der Vorderansicht die Nase weg; die 1:50-Blätter zeichnen sie.')
save(f,d)
# gull
f,d=load('herring-gull'); rep=d['asset']['representations']['200']; g=rep['geometry']
g['front']=[s for s in g['front'] if s['id']!='bill']
for s in g['front']:
    if s['id']=='near-leg': s['points']=[[-0.058,-0.102],[-0.058,-0.258],[-0.024,-0.258],[-0.026,-0.102]]
    if s['id']=='far-leg': s['points']=[[0.026,-0.102],[0.024,-0.258],[0.058,-0.258],[0.058,-0.102]]
for s in g['plan']:
    if s['id'] in('near-leg','far-leg'): s['r0']=0.02; s['r1']=0.036
note(rep,'At 1:200 the front view drops the bill (the head covers it) and both views draw the legs about a third thicker than true so they stay readable.','Bei 1:200 lässt die Vorderansicht den Schnabel weg (der Kopf verdeckt ihn) und beide Ansichten zeichnen die Beine etwa ein Drittel dicker als wahr, damit sie lesbar bleiben.')
save(f,d)
# hedgehog
f,d=load('hedgehog'); rep=d['asset']['representations']['100']
for s in rep['geometry']['front']:
    if s['id']=='dome': s['cy']=0.002; s['ry']=0.036
note(rep,'The front dome is drawn lower (0.036 m half height) so the front height matches the 0.128 m of the side view.','Die Kuppel der Vorderansicht ist niedriger gezeichnet (0,036 m halbe Höhe), damit die Vorderhöhe den 0,128 m der Seitenansicht entspricht.')
save(f,d)
