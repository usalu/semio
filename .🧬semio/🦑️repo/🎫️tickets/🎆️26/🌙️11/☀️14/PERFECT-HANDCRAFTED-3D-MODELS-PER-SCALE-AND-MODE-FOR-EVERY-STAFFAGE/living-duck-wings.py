"""🦆 Articulate the mallard elbow, wrist and feather tracts in their folded positions."""
import copy
import json
import math
from pathlib import Path
ticket=Path(__file__).parent
catalog=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/mallard-duck/*.json'))
source=ticket/'volumes/animals/mallard-duck.json'
fixture=json.loads((ticket/'living-duck-wings.json').read_text())
def difference(p,q):return [x-y for x,y in zip(p,q)]
def unit(v):return [x/math.hypot(*v) for x in v]
def dot(u,v):return sum(x*y for x,y in zip(u,v))
def rounded(x):return round(x,10)
def orient(direction):
    u=unit(direction)
    a=math.atan2(u[1],u[0]);b=math.asin(u[2])
    basis=[u,[-math.sin(a),math.cos(a),0],[-math.cos(a)*math.sin(b),-math.sin(a)*math.sin(b),math.cos(b)]]
    return (rounded(math.degrees(a)),rounded(math.degrees(b)),basis)
def joint(name,parent,title,at,direction,mass,centre):
    a,b,frame=orient(direction)
    return dict(id=name,parent=parent,title=dict(en=title[0],de=title[1]),x=rounded(at[0]),y=rounded(at[1]),z=rounded(at[2]),angleDeg=a,abductionDeg=b,lengthM=rounded(math.hypot(*direction)),joint=dict(type='revolute',minDeg=-80,maxDeg=80),mass=dict(kg=mass,comX=centre,comY=0,distribution='point',source='authored-estimate',reference='cramp-simmons-1977')),frame
def weight(id,d,side):
    if 'primar' in id:return [dict(bone='hand-'+side,weight=1)]
    if 'speculum' in id:return [dict(bone='forearm-'+side,weight=1)]
    if 'scapular' in id:return []
    if 'wing' in id:
        fraction=.45 if d=='50' else .52 if d=='100' else .5
        return [dict(bone='wing-'+side,weight=1-fraction),dict(bone='forearm-'+side,weight=fraction)]
    return []
def volumes(vs):
    for scale,v in vs.items():
        for side,word,sign in fixture['side']:
            carrier='wing-'+side;fore='forearm-'+side;hand='hand-'+side
            for s in v['body']+v['thermal']:
                if s.get('bone')!=carrier:continue
                w=weight(s['id'],scale,side)
                if w:s['weights']=w
            if scale in ('200','500'):
                wing=next(s for s in v['body'] if s['id']==word+'-wing')
                quills=copy.deepcopy(wing); rows=wing['sections']
                overlap=copy.deepcopy(rows[2])
                overlap['at']=[round(.28*x+.72*y,7) for x,y in zip(rows[2]['at'],rows[3]['at'])]
                overlap['rx']=round(rows[3]['rx']*.62,7);overlap['ry']=round(rows[3]['ry']*.64,7)
                quills.update(id=word+'-primaries',sections=[overlap,*rows[3:]],weights=[dict(bone=hand,weight=1)])
                inner=copy.deepcopy(rows[3]); inner['at']=[round(.86*x+.14*y,7) for x,y in zip(rows[3]['at'],rows[4]['at'])];inner['rx']=round(inner['rx']*.58,7);inner['ry']=round(inner['ry']*.58,7)
                wing['sections']=[*rows[:4],inner]
                v['body'].append(quills);v['thermal'].append(dict(id='th-'+quills['id'],of=quills['id'],region='wing-plumage'))
            if scale=='500':
                old=next(s for s in v['structure'] if s['id']==word+'-wing-bones')
                pts=[q['at'] for q in old['sections']];i=v['structure'].index(old)
                new=[]
                for j,(label,bone) in enumerate([('humerus',carrier),('forearm-bones',fore),('manus',hand)]):
                    part=dict(id=word+'-'+label,kind='capsule',bone=carrier,material='bone',role='body',tone=.88,p0=pts[j],p1=pts[j+1],r0=[.00185,.0016,.00135][j],r1=[.00165,.00145,.00105][j],slender=True,member=bone,part='member')
                    if j:part['weights']=[dict(bone=bone,weight=1)]
                    new.append(part)
                v['structure'][i:i+1]=new
            else:
                for suffix,bone in [('forearm-bones',fore),('manus',hand)]:
                    s=next(x for x in v['structure'] if x['id']==word+'-'+suffix)
                    s.update(member=bone,weights=[dict(bone=bone,weight=1)])
        v['decisions'].append(dict(en='The paired elbow and wrist pivots follow the folded ulna and carpometacarpus. The speculum follows the forearm and the primaries follow the hand.',de='Die beidseitigen Ellbogen- und Handgelenke folgen der gefalteten Elle und dem Carpometacarpus. Der Flügelspiegel folgt dem Unterarm und die Handschwingen folgen der Hand.'))
def rig(asset):
    bones=asset['rig']['bones']
    if any(s['id']=='forearm-n' for s in bones):raise ValueError('The mallard already has articulated wings.')
    pose=next(p for p in asset['poses'] if p['id']=='wing-stretch')
    for side,word,sign in fixture['side']:
        shoulder='wing-'+side;fore='forearm-'+side;hand='hand-'+side
        near=side=='n';ad='near' if near else 'far';de='Naher' if near else 'Ferner'
        elbow,wrist,tip=([v[0],v[1],v[2]*sign] for v in [fixture['shoulderFrameElbow'],fixture['shoulderFrameWrist'],fixture['shoulderFrameManusTip']])
        arm,basis=joint(fore,shoulder,(ad.capitalize()+' radius and ulna',de+' Unterarm mit Elle und Speiche'),elbow,difference(wrist,elbow),.020,.036)
        manus,_=joint(hand,fore,(ad.capitalize()+' carpometacarpus and primary fan',de+' Carpometacarpus mit Handschwingen'),[arm['lengthM'],0,0],[dot(ax,difference(tip,wrist)) for ax in basis],.015,.074)
        parent=next(b for b in bones if b['id']==shoulder);i=bones.index(parent)+1;bones[i:i]=[arm,manus]
        fcom=[elbow[i]+.036*basis[0][i] for i in range(3)];uc=unit(difference(tip,wrist));hcom=[wrist[i]+.074*uc[i] for i in range(3)]
        parent['mass'].update(kg=.040,comX=rounded((.075*.09-.020*fcom[0]-.015*hcom[0])/.040),comY=rounded((-.020*fcom[1]-.015*hcom[1])/.040),distribution='point')
        pose['bones'][fore]=dict(angleDeg=-12 if near else -14,abductionDeg=sign*(18 if near else 20))
        pose['bones'][hand]=dict(angleDeg=8 if near else 9,abductionDeg=sign*(12 if near else 13))
        for k,r in asset['representations'].items():
            for shapes in r.get('geometry',{}).values():
                if not isinstance(shapes,list):continue
                for s in shapes:
                    if s.get('bone')!=shoulder or s.get('solid',{}).get('kind')!='extruded':continue
                    w=weight(s['id'],k,side)
                    if k=='200' and 'wing' in s['id']:w=[dict(bone=shoulder,weight=.32),dict(bone=fore,weight=.38),dict(bone=hand,weight=.30)]
                    if w:s['solid']['weights']=w
    pose['description']=dict(en='Standing with the wings raised after bathing. Each elbow opens the secondary fan and each wrist extends the primaries beyond it.',de='Stehend mit angehobenen Flügeln nach dem Baden. Beide Ellbogen öffnen die Armschwingenfächer und die Handgelenke strecken die Handschwingen darüber hinaus.')
def revise(path):
    raw=path.read_text();doc=json.loads(raw);a=doc.get('asset',doc);volumes(a['volumes'])
    if 'asset' in doc:rig(a)
    keys=('rig','poses','representations','volumes') if 'asset' in doc else ('volumes',)
    edits=[]
    for key in keys:
        mark=raw.index('"'+key+'":');start=mark+len(key)+3
        while raw[start].isspace():start+=1
        _,length=json.JSONDecoder().raw_decode(raw[start:]);value=json.dumps(a[key],ensure_ascii=False,separators=(',',':'))
        edits.append((start,start+length,value))
    for start,end,value in sorted(edits,reverse=True):raw=raw[:start]+value+raw[end:]
    if (json.loads(raw).get('asset',json.loads(raw)))!=a:raise ValueError(path)
    text=path.read_text()
    if json.loads(text).get('asset',json.loads(text)).get('volumes')==a['volumes']:raise ValueError('Unexpected concurrent edit')
    return text,raw
edits=[(p,*revise(p)) for p in (catalog,source)]
for p,previous,new in edits:
    if p.read_text()!=previous:raise RuntimeError(p)
    p.write_text(new)
