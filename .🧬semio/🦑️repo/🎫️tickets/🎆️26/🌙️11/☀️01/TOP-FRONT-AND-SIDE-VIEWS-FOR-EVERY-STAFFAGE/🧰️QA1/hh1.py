import json,glob,copy
f=glob.glob('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json')[0]
d=json.load(open(f,encoding='utf8')); a=d['asset']
Z=0.04
NAMES={'coat-r':{'near':'coat-rn','far':'coat-rf'},'coat-f':{'near':'coat-fn','far':'coat-ff'}}
TITLE={('coat-r','near'):('Rear spine skirt, near side (panniculus flap)','Hinterer Stachelsaum, nahe Seite (Hautmuskel-Lappen)'),
       ('coat-r','far'):('Rear spine skirt, far side (panniculus flap)','Hinterer Stachelsaum, ferne Seite (Hautmuskel-Lappen)'),
       ('coat-f','near'):('Front spine skirt over the forehead, near side','Vorderer Stachelsaum über der Stirn, nahe Seite'),
       ('coat-f','far'):('Front spine skirt over the forehead, far side','Vorderer Stachelsaum über der Stirn, ferne Seite')}
# rig
bones=[]
for b in a['rig']['bones']:
    if b['id'] in NAMES:
        for side in('near','far'):
            n=copy.deepcopy(b); n['id']=NAMES[b['id']][side]; n['z']=-Z if side=='near' else Z
            n['title']={'en':TITLE[(b['id'],side)][0],'de':TITLE[(b['id'],side)][1]}
            n['mass']['kg']=round(b['mass']['kg']/2,4)
            bones.append(n)
    else: bones.append(b)
a['rig']['bones']=bones
# anchors
for an in a['anchors']:
    if an['bone'] in NAMES: an['bone']=NAMES[an['bone']]['near']
# poses
ABD={'rolled':40,'rolling-half':20}
for p in a['poses']:
    nb={}
    for k,v in p['bones'].items():
        if k in NAMES:
            for side in('near','far'):
                o=dict(v)
                if p['id'] in ABD: o['abductionDeg']=ABD[p['id']] if side=='near' else -ABD[p['id']]
                nb[NAMES[k][side]]=o
        else: nb[k]=v
    p['bones']=nb
# geometry
def split(shapes,fn):
    out=[]
    for s in shapes:
        if s['bone'] in NAMES:
            for side in('near','far'):
                out.append(fn(copy.deepcopy(s),s['bone'],side))
        else: out.append(s)
    return out
def elev(s,old,side):
    s['bone']=NAMES[old][side]; s['id']=s['id']+('-near' if side=='near' else '-far')
    if side=='far': s['z']=round(s['z']-3.2,2)
    return s
def front(s,old,side):
    sg=-1 if side=='near' else 1
    s['bone']=NAMES[old][side]; s['id']=s['id']+('-near' if side=='near' else '-far')
    s['cx']=sg*Z; s['rx']=0.032
    return s
def plan(s,old,side):
    sg=-1 if side=='near' else 1
    s['bone']=NAMES[old][side]; s['id']=s['id']+('-near' if side=='near' else '-far')
    s['cy']=sg*Z; s['ry']=0.022
    return s
for sc in('50','100'):
    g=a['representations'][sc]['geometry']
    g['elevation']=split(g['elevation'],elev); g['front']=split(g['front'],front); g['plan']=split(g['plan'],plan)
    t=a['representations'][sc]['decisions']['silhouette']
    t['en']+=' The spine skirts are four bones, a near and a far one at the front and at the rear, 0.04 m beside the symmetry plane; rolling up closes them towards the middle (the near skirt spreads towards the far side and the far skirt towards the near side), and in front and plan each skirt is drawn centred on its own pivot.'
    t['de']+=' Die Stachelsäume sind vier Knochen, je ein naher und ein ferner vorn und hinten, 0,04 m neben der Symmetrieebene; beim Einrollen schließen sie sich zur Mitte hin (der nahe Saum spreizt zur fernen Seite, der ferne zur nahen), und in Vorder- und Draufsicht ist jeder Saum um seinen eigenen Drehpunkt gezeichnet.'
open(f,'w',encoding='utf8',newline='').write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
print('ok')
