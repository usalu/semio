"""🍳 Handcraft the stainless envelope, duct collar and centrifugal blower of the wall-mounted hood."""
from pathlib import Path
from copy import deepcopy
import json
import math
from runpy import run_path
import sys
T=Path(__file__).resolve().parent
M=run_path(str(T/'r3-kitchen-extractor-hood-model.py'));K=M['k']
C,CH,CT,W='canopy','chimney','chimney-top','blower-wheel'
FAN=(.105,.135,.08)
SS,G=M['SS'],M['GALV'];TEX=lambda en,de:{'en':en,'de':de}
SOURCE=T/'volumes/appliances/kitchen-extractor-hood.json'
CAT=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/kitchen-extractor-hood/🔣️.json'))
def circle(r,n,cy=0,cz=0):
    return [[round(cy+r*math.cos(math.tau*i/n),7),round(cz+r*math.sin(math.tau*i/n),7)] for i in range(n)]

def rotor(d):
    n={50:48,100:32,200:20,500:12}[d]
    def shaft(sid,bone,profile,origin):
        return K.rev(sid,bone,G,'body',.43,profile,origin,[1,0,0])
    back=shaft('wheel-backplate',W,[[.015,.004],[.015,.012],[.026,.017],[.0327,.03],[.0327,.067],[.034,.067],[.034,.015],[.044,.012],[.044,.004],[.015,.004]],[0,0,0])
    ring=shaft('wheel-inlet-ring',W,[[-.031,.052],[-.034,.050],[-.034,.067],[-.0325,.067],[-.0325,.053],[-.031,.052]],[0,0,0])
    hub=shaft('blower-motor',C,[[.015,0],[.015,.004],[.050,.004],[.050,.028],[.054,.039],[.085,.039],[.091,.032],[.094,.016],[.094,0]],FAN)
    scroll=[]
    inner=[]
    for i in range(n+1):
        f=i/n;t=math.pi-.5-f*(math.tau-.5);r=.071+.059*f
        scroll.append([r*math.sin(t),r*math.cos(t)])
        inner.append([(r-.001)*math.sin(t),(r-.001)*math.cos(t)])
    ys=.062;outer=[[ys,-.065]];contour=outer+scroll+[[ys,-.130]]
    wall=K.ext('blower-scroll',C,G,'body',.49,'yz',[(y+.135,z+.08) for y,z in (scroll+[[ys,-.130],[ys,-.129]]+list(reversed(inner)))],.065,.145)
    backcover=K.ext('blower-back',C,G,'body',.47,'yz',[(y+.135,z+.08) for y,z in contour],.144,.145,holes=[circle(.005,n,.135,.08)])
    inlet=K.ext('blower-inlet',C,G,'body',.47,'yz',[(y+.135,z+.08) for y,z in contour],.065,.066,holes=[circle(.052,n,.135,.08)])
    lower=[(.065,-.050),(.145,-.050),(.145,.015),(.065,.015)]
    vertices=[];triangles=[]
    for level in (0,1,2,3):
        for i in range(n):
            side=i//(n//4);f=(i%(n//4))/(n//4)
            angle=math.radians(225)+math.tau*i/n
            if level in (1,2):
                r=.075 if level==1 else .0744
                point=[.085+r*math.cos(angle),.2454,r*math.sin(angle)]
            else:
                rect=lower
                x=(1-f)*rect[side][0]+f*rect[(side+1)%4][0];z=(1-f)*rect[side][1]+f*rect[(side+1)%4][1]
                if level==3:
                    x=.105+(x-.105)*.985;z=-.0175+(z+.0175)*(.0638/.065)
                point=[x,.197,z]
            vertices.append([round(v,7) for v in point])
    for band in range(4):
        for i in range(n):
            j=(i+1)%n;start=band*n;end=((band+1)%4)*n
            if band==2:
                triangles.extend(([start+i,end+i,start+j],[end+i,end+j,start+j]))
            else:triangles.extend(([start+i,end+i,end+j],[start+i,end+j,start+j]))
    throat=dict(id='blower-transition',kind='polyhedron',bone=C,material=G,role='body',tone=.50,vertices=vertices,triangles=triangles)
    arc=[]
    stations={50:6,100:4,200:3,500:2}[d]
    for i in range(stations):
        t=i/(stations-1)
        arc.append((.052+.014*t,.001+.009*t*t))
    vane=K.ext('wheel-vane',W,G,'body',.56,'yz',arc+[(y,z+.0009) for y,z in reversed(arc)],-.0328,.0326)
    vane['repeat']={'count':24,'rotationAxis':[1,0,0],'rotationRad':math.tau/24}
    blades=[vane]
    return [wall,backcover,inlet,throat,hub,back,ring,*blades]

def air_flaps(n):
    leaves=[]
    for bone,sign in [('damper-left',1),('damper-right',-1)]:
        profile=[[round(sign*(.001+.071*math.sin(math.pi*i/(n//2))),7),round(.072*math.cos(math.pi*i/(n//2)),7)] for i in range(n//2+1)]
        leaves.append(K.ext(bone+'-leaf',bone,SS,'body',.66,'zx',profile,-.0003,.0003))
    return leaves

def author(volumes):
    fine={s['id']:deepcopy(s) for s in volumes['50']['body']}
    fine_thermal={s['id']:deepcopy(s) for s in volumes['50']['thermal']}
    for ds,v in volumes.items():
        d=int(ds);n={50:48,100:32,200:20,500:12}[d];g=.0006
        sheets=[K.slope_front('slope-front',C,SS,'body',.84,g),K.slope_side('slope-left',C,SS,'body',.8,g,1),K.slope_side('slope-right',C,SS,'body',.8,g,-1),K.top_plate('top-plate',C,SS,'body',.86,g),K.back_plate('back-plate',C,SS,'body',.7,g)]
        sheets[3]['holes']=[circle(.075,n,0,.085)]
        base=deepcopy(fine['base-panel']);base['to']=round(base['from']+g,6)
        sheets.append(base)
        rim=M['rim']('rim',.78,None,wall=g)
        shelf=K.ext('rim-return',C,SS,'body',.72,'zx',K.rect(-K.HALF_W,K.HALF_W,0,K.DEPTH),0,g,holes=[K.rect(-K.HALF_W+K.FLANGE,K.HALF_W-K.FLANGE,K.FLANGE,K.DEPTH-K.FLANGE)])
        ducts=[K.ext('chimney-lower',CH,SS,'body',.86,'zx',K.channel(K.LOWER_HW,K.LOWER_D,g,.008 if d<=100 else 0),0,K.LOWER_H),K.ext('chimney-upper',CT,SS,'body',.90,'zx',K.channel(K.UPPER_HW,K.UPPER_D,g,.006 if d<=100 else 0),-K.UPPER_HIDDEN,K.UPPER_H),K.tube('duct',CT,G,'body',.55,.075,.0005,-K.LOWER_H,K.UPPER_H+K.STUB)]
        filterids=[s for s in fine if s.startswith(('filter-mesh-','lamp-'))]
        spindle=K.rev('damper-spindle',CH,G,'detail',.36,[[-.073,0],[-.073,.001],[.073,.001],[.073,0]],[.085,.012,0],[1,0,0],slender=True)
        parts=[rim,shelf,*sheets,*ducts,*rotor(d),*air_flaps(n),spindle]
        for key in filterids:
            s=deepcopy(fine[key])
            if 'mesh-' in key:s['to']=round(s['from']+.01,6)
            else:s['role']='glazing';s['size'][1]=.002;s['transform']['translation'][1]=.005
            parts.append(s)
        updated={s['id'] for s in parts}
        v['body']=[s for s in v['body'] if s['id'] not in updated|{'filters'}]+parts
        for s in v['thermal']:
            if s['region']=='canopy' and s.get('of') in {x['id'] for x in sheets}|{'rim'}:s['layer']='shell'
            if s['region']=='chimney' and 'of'in s:s['layer']='tube'
            if d!=50 and s['id'] in ('th-slope-front','th-slope-left','th-slope-right','th-top-plate'):s['sectionFaces']=['-v']
            if s['id']=='th-top-plate' and 'of' not in s:s['holes']=[circle(.075,n,0,.085)]
        if d==100:v['thermal']=[s for s in v['thermal'] if s['region']!='filter']+[deepcopy(s) for s in fine_thermal.values() if s['region']=='filter']
        if not any(s['id']=='wheel-mass' for s in v['structure']):
            v['structure'].append(dict(id='wheel-mass',of='wheel-backplate',member=W,part='mass'))
        for leaf in air_flaps(n):
            id=leaf['id']
            if not any(s['id']=='th-'+id for s in v['thermal']):
                v['thermal'].append(dict(id='th-'+id,of=id,region='chimney',layer='tube'))
                v['structure'].append(dict(id='member-'+id,of=id,member=leaf['bone'],part='member'))
                v['structure'].append(dict(id='pivot-'+id,of='damper-spindle',member=leaf['bone'],part='joint'))
        note=TEX("The tapered shell, perforated base and chimney are folded from 0.6 mm stainless sheet. A 150 mm opening connects the hood to its air duct. The centrifugal wheel rotates about its horizontal motor shaft inside a rising scroll. Its internal dimensions are author estimates.","Haubenschrägen, durchbrochener Boden und Kamin bestehen aus 0,6-mm-Edelstahlblech. Eine Öffnung von 150 mm verbindet die Haube mit dem Luftrohr. Das Rad läuft auf der waagerechten Motorwelle im Spiralgehäuse. Die Innenmaße sind Entwurfsannahmen.")
        v['decisions']=[q for q in v['decisions'] if q!=note]+[note]
    return volumes

def roots(a):
    bones=a['rig']['bones']
    if not any(b['id']==W for b in bones):
        bones.append(dict(id=W,parent=C,title=TEX('Centrifugal blower wheel','Radiallüfterrad'),x=.105,y=.135,z=.08,angleDeg=0,lengthM=.034,joint={'type':'revolute','continuousTwist':True},mass={'kg':.16,'comX':0,'comY':0,'distribution':'point','source':'authored-estimate'}))
        main=bones[0]['mass'];remaining=round(main['kg']-.16,6);main['comX']=round((main['kg']*main['comX']-.16*.105)/remaining,7);main['comY']=round((main['kg']*main['comY']-.16*.135)/remaining,7);main['kg']=remaining
    for id,sign in [('damper-left',-1),('damper-right',1)]:
        if not any(b['id']==id for b in bones):
            bones.append(dict(id=id,parent=CH,x=.085,y=.012,angleDeg=0,lengthM=.072,title=TEX('Left backdraft leaf' if sign<0 else 'Right backdraft leaf','Linke Rückstauklappe' if sign<0 else 'Rechte Rückstauklappe'),joint={'type':'revolute','minDeg':0,'maxDeg':0,'twistMinDeg':min(0,sign*82),'twistMaxDeg':max(0,sign*82)}))
    for d,r in a['representations'].items():
        if r['status']!='authored':continue
        geom=r['geometry']['elevation']
        r.setdefault('sections',{})['elevation']={'plane':.08,'label':TEX('Longitudinal blower section','Längsschnitt durch das Gebläse')}
        cuts=[('back-cover-section',C,.144,.076,.001,.112,G),('rotor-disc-section',W,.0327,-.065,.0013,.13,G)]
        for sid,b,x,y,w,h,mat in cuts:
            if not any(s['id']==sid for s in geom):
                geom.append(dict(id=sid,bone=b,role='detail',z=9,filled=True,tone=.38,material=mat,kind='rect',x=x,y=y,w=w,h=h,penMm=.20))
    spec='bosch-dwp96bc50b'
    references=a['provenance']['references']
    if not any(q['id']==spec for q in references):
        references.append({'id':spec,'citation':'Bosch DWP96BC50B appliance data, 900 × 500 mm hood, 150 mm duct, 626 m³/h extraction, 220 W connected load and two 1.5 W LED lamps. https://media3.bosch-home.com/Documents/specsheet/en-SG/DWP96BC50B.pdf'})
    a['summary']=TEX('A 900 mm wall-mounted stainless hood with three washable cassettes, two LED lamps, a centrifugal blower and a telescopic chimney. The 150 mm duct carries up to 626 m³/h.','Eine 900 mm breite Edelstahl-Wandhaube mit drei waschbaren Kassetten, zwei LED-Leuchten, Radialgebläse und Teleskopkamin. Das 150-mm-Rohr führt bis zu 626 m³/h ab.')
    th=a['thermal'];fan=next(q for q in th['operating'] if q['id']=='fan');light=next(q for q in th['operating'] if q['id']=='light')
    fan['levels']=[{'v':v,'title':TEX(en,de)} for v,en,de in [(0,'Off','Aus'),(.4,'Low','Kleine Stufe'),(.7,'Medium','Mittlere Stufe'),(1,'Full extraction, 626 m³/h','Volle Absaugung, 626 m³/h')]]
    light['levels'][-1]['title']=TEX('Two LED lamps, 3 W','Zwei LED-Leuchten, 3 W')
    regions={r['id']:r for r in th['regions']}
    for key,source,heat in [('canopy',217,21.7),('lamps',3,3)]:
        r=regions[key];r['ratedPowerW']=source;r['operating']['heatW']=heat
    flow=626/3600;out=next(e for e in th['emitters'] if e['id']=='extract')
    out.update(volumeFlowM3S=flow,temperatureRiseK=round(195.3/(1.204*1006*flow),8),reference=spec)
    th['assumptions']=[TEX('At the 220 W connected load, the LEDs use 3 W. The model assigns 10% of the remaining 217 W to the hood casing and carries 195.3 W in the extracted air. The split and the 40% and 70% airflow stages are authored estimates.', 'Bei 220 W Anschlussleistung entfallen 3 W auf die LEDs. Das Modell gibt 10 % der verbleibenden 217 W an das Gehäuse und führt 195,3 W mit der Abluft ab. Die Aufteilung und die Luftmengenstufen von 40 % und 70 % sind Entwurfsannahmen.')]
    high=next(q for q in a['poses'] if q['id']=='extracting')
    high['description']=TEX('The blower exhausts 626 m³/h through the cassettes and chimney with both LED lamps lit.','Das Gebläse saugt bei eingeschalteten LED-Leuchten 626 m³/h durch die Kassetten und den Kamin ab.')
    if not any(q['id']=='medium' for q in a['poses']):
        med=deepcopy(next(q for q in a['poses'] if q['id']=='low'))
        med.update(id='medium',title=TEX('Medium speed','Mittlere Stufe'),description=TEX('Both lights are on and the blower operates at the middle setting.','Beide Leuchten sind an. Das Gebläse läuft auf der mittleren Stufe.'))
        med['thermal']['emitters']['extract']=.7;med['thermal']['operating']['fan']=.7;a['poses'].insert(a['poses'].index(high),med)
    for pose in a['poses']:
        opening={'off':0,'low':46,'medium':67,'extracting':80}[pose['id']]
        pose['bones']['damper-left']={'twistDeg':-opening}
        pose['bones']['damper-right']={'twistDeg':opening}
    if not any(c['id']=='blower-inspection' for c in a['clips']):
        a['clips'].append(dict(id='blower-inspection',title=TEX('Blower inspection turn','Prüfdrehung des Gebläses'),description=TEX('One slow inspection revolution of the centrifugal wheel.','Eine langsame Prüfumdrehung des Radiallüfterrads.'),durationS=2,fps=24,loop=True,rootMotion='none',contacts=[{'anchor':n,'fromS':0,'toS':2} for n in ('wall-hook','wall-rest')],phases=[{'id':'turn','fromS':0,'toS':2,'title':TEX('Inspection turn','Prüfdrehung')}],events=[],tracks=[],keys=[dict(t=i/4,pose='off',ease='linear',bones={W:{'twistDeg':i*45}})for i in range(9)]))

if __name__=='__main__':
    from sys import argv
    path=T/'🗑️generated/digital-hood-candidate.json'
    data=json.loads(CAT.read_text());asset=data['asset'];author(asset['volumes']);roots(asset)
    path.write_text(json.dumps(data,ensure_ascii=False))
    if '--publish' in argv:
        lib=run_path(str(T/'r5-range-gaps-lib.py'))
        for target in [CAT,SOURCE]:
            before=target.read_text();new=before;d=json.loads(before);a=d.get('asset',d);author(a['volumes'])
            keys=['volumes']
            if 'rig' in a:roots(a);keys+=['rig','clips','representations','provenance','summary','thermal','poses']
            decoder=json.JSONDecoder()
            start=lib['key_value'](before,'asset',0,len(before))[0] if 'asset'in d else before.index('{')
            spans=[];pos=start+1
            while True:
                while before[pos].isspace() or before[pos]==',':pos+=1
                if before[pos]=='}':break
                key,stop=decoder.raw_decode(before,pos)
                lo=before.index(':',stop)+1
                while before[lo].isspace():lo+=1
                _,hi=decoder.raw_decode(before,lo)
                if key in keys:spans.append((lo,hi,key))
                pos=hi
            for lo,hi,key in reversed(spans):
                line=before[before.rfind('\n',0,lo)+1:lo]
                new=new[:lo]+lib['render'](a[key],len(line)-len(line.lstrip()),2)+new[hi:]
            assert json.loads(new)==d and before==target.read_text()
            target.write_text(new)
