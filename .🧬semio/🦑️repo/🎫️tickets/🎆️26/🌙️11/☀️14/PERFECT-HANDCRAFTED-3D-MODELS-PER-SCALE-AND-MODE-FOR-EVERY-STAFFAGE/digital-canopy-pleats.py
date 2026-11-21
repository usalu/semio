"""🌂 Cut three radial accordion pleats into each gore and solve their sewn crease directions."""
from pathlib import Path
from copy import deepcopy
import json
from math import sin, cos, asin, acos, radians, degrees

T=Path(__file__).resolve().parent
CAT=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/market-parasol/🔣️.json'))
SIDES=['right','b','c','d','left','f','g','h']
FACETS=6
LENGTH=1.3476

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def angle(a,b):return acos(max(-1,min(1,dot(a,b))))
def ray(bone,offset):
    t=radians(bone['angleDeg']+offset['angleDeg']);p=radians(offset['abductionDeg'])
    return [cos(t)*cos(p),sin(t)*cos(p),sin(p)]
def identifier(side,i):return f'pleat-{side}-{i+1}'
def part(side,i):return side if i==0 else f'{side}-{i+1}'
def replace_gore(items,templates,prefix,build):
    out=[]
    for old in items:
        if old['id'].startswith(prefix) and old['id'][len(prefix):] in SIDES:
            side=old['id'][len(prefix):]
            out.extend(build(old,side,i,templates[side][i]) for i in range(FACETS))
        else:out.append(old)
    return out

def author(a):
    if any(b['id'].startswith('pleat-') for b in a['rig']['bones']):return a
    bones=a['rig']['bones'];ribs={s:next(b for b in bones if b['id']=='rib-'+s) for s in SIDES}
    poses={p['id']:p for p in a['poses']};sources=[p['bones'] for p in a['poses']]
    keyOffsets=[]
    for clip in a['clips']:
        for key in clip['keys']:
            if key.get('bones'):
                full=deepcopy(poses[key['pose']]['bones']);full.update(key['bones'])
                sources.append(full);keyOffsets.append((key,full))
    rays=[{s:ray(ribs[s],q['rib-'+s])for s in SIDES} for q in sources]
    steps={s:(max(angle(row[s],row[SIDES[(j+1)%8]]) for row in rays)+radians(.06))/FACETS for j,s in enumerate(SIDES)}
    a['rig']['radialFolds']=[{'left':'rib-'+s,'right':'rib-'+SIDES[(j+1)%8],'facets':[identifier(s,i) for i in range(FACETS)],'cutAngleDeg':round(degrees(steps[s])*FACETS,7)} for j,s in enumerate(SIDES)]
    newBones=[b for b in bones if not b['id'].startswith('fold-')]
    for j,s in enumerate(SIDES):
        for prefix in ['rib','stretch']:
            item=next(b for b in newBones if b['id']==prefix+'-'+s)
            oldAngle=item['angleDeg'];bearing='pivot-'+prefix+'-'+s
            az=11+45*j
            newBones.append(dict(id=bearing,parent=item['parent'],title={'en':f'{prefix.title()} clevis at {az}°','de':f'Gabelgelenk bei {az}°'},x=0,y=0,angleDeg=0,abductionDeg=(-az+180)%360-180,lengthM=.01,joint={'type':'fixed'}))
            restPitch=round(degrees(asin(ray({'angleDeg':oldAngle},sources[0][item['id']])[1])),6)
            item.update(parent=bearing,angleDeg=restPitch,joint={'type':'revolute','minDeg':-90,'maxDeg':90})
            for q in sources:
                axis=ray({'angleDeg':oldAngle},q[item['id']])
                q[item['id']]={'angleDeg':round(degrees(asin(axis[1]))-restPitch,6)}
        for i in range(FACETS):
            newBones.append(dict(id=identifier(s,i),parent='crown',title={'en':f'Gore {j+1}, pleat face {i+1}','de':f'Bahn {j+1}, Faltenfläche {i+1}'},x=0,y=.00825,angleDeg=0,lengthM=.05,joint=dict(type='revolute',minDeg=-180,maxDeg=180,twistMinDeg=-180,twistMaxDeg=180)))
    a['rig']['bones']=newBones
    for key,full in keyOffsets:key['bones'].update({k:v for k,v in full.items() if k.startswith(('rib-','stretch-'))})
    for q in [p['bones'] for p in a['poses']]+[k['bones'] for k,_ in keyOffsets]:
        for k in list(q):
            if k.startswith('fold-'):del q[k]
    def profile(step,i,scale):
        full=step*FACETS
        def at(j):
            phi=step*j;f=(j/FACETS)
            sag=.012*(1-abs(2*f-1)) if scale=='500' else .012*4*f*(1-f)
            r=LENGTH*cos(full/2)/cos(phi-full/2)-sag
            return [round(-r*sin(phi-i*step),8),round(r*cos(phi-i*step),8)]
        intervals={'50':3,'100':2,'200':1,'500':1}[scale]
        return [[0,0]]+[at(i+j/intervals) for j in range(intervals+1)]
    for d,v in a['volumes'].items():
        templates={s:[profile(steps[s],i,d)for i in range(FACETS)]for s in SIDES}
        def panel(p,s,i,profile):
            out=deepcopy(p);prefix=p['id'][:-len(s)];out.update(id=prefix+part(s,i),bone=identifier(s,i),profile=profile)
            out.pop('weights',None)
            out['from']=round(out['from']-.00825,7);out['to']=round(out['to']-.00825,7)
            return out
        for key,prefix in [('body','panel-'),('thermal','th-panel-'),('thermal','th-underside-')]:
            v[key]=replace_gore(v[key],templates,prefix,panel)
        def paper(p,s,i,profile):
            out=deepcopy(p);out.update(id='panel-'+part(s,i),bone=identifier(s,i),points=[[x,z]for z,x in profile])
            out['solid'].update(z0=-.00025,z1=.00025);out['solid'].pop('weights',None)
            return out
        if d in a['representations']:
            geo=a['representations'][d]['geometry']
            for view,items in geo.items():geo[view]=replace_gore(items,templates,'panel-',paper)
        for note in v['decisions']:
            for lang in note:
                note[lang]=note[lang].replace('Both thermal faces follow the same cloth and fold bones.','Each gore forms three radial pleats. Its six facets follow the sewn seams and keep their cut area in the furled pose. Both exchange faces follow that cloth.').replace('Beide thermischen Oberflächen folgen demselben Stoff und denselben Faltknochen.','Jede Bahn bildet drei radiale Falten. Ihre sechs Flächen folgen den Nähten und behalten beim Schließen den Zuschnitt. Beide Wärmeübergangsflächen folgen dem Stoff.')
    return a
if __name__=='__main__':
    doc=json.loads(CAT.read_text());author(doc['asset']);(T/'🗑️generated/digital-pleat-candidate.json').write_text(json.dumps(doc,ensure_ascii=False))
