"""🖥️ Assemble the monitor around its horizontal hinge, open bezel, cover glass and four-point VESA plate."""
from pathlib import Path
from copy import deepcopy
from math import cos,sin,pi
import json
T=Path(__file__).resolve().parent
CAT=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/height-adjustable-desk/🔣️.json'))
PIVOT=[.6,.305,.177]
JOINT='screen-tilt'
CASE='equipment-housing'
GLASS='display-glass'
def text(en,de):return {'en':en,'de':de}
def box(id,size,centre,tone=.24,bone=JOINT,material=CASE,role='body'):
    return dict(id=id,kind='box',bone=bone,material=material,role=role,tone=tone,size=size,transform={'translation':centre})
def outline(w,h,cy,r,steps):
    points=[]
    for cx,y,angle in [(w/2-r,cy+h/2-r,0),(-w/2+r,cy+h/2-r,90),(-w/2+r,cy-h/2+r,180),(w/2-r,cy-h/2+r,270)]:
        for j in range(steps+1):
            a=pi/180*(angle+90*j/steps);points.append([round(cx+r*cos(a),7),round(y+r*sin(a),7)])
    return points
def pieces(scale):
    count={'50':6,'100':4,'200':2,'500':1}[scale]
    front=outline(.614,.368,-.004,.004,count)
    aperture=outline(.598,.336,.004,.002,count)
    bezel=dict(id='monitor-panel',kind='extrude',bone=JOINT,material=CASE,role='body',tone=.18,plane='xy',profile=front,holes=[list(reversed(aperture))],**{'from':-.062,'to':-.047})
    cover=box('monitor-screen',[.598,.336,.002],[0,.004,-.0607],.08,material=GLASS,role='glazing')
    display=box('monitor-backplane',[.6,.338,.009],[0,.004,-.0552],.23)
    deck=box('monitor-rear-panel',[.607,.361,.003],[0,-.004,-.0485])
    base=box('monitor-base',[.25,.018,.20],[.6,.034,.16],.26,'top')
    neck=box('monitor-neck',[.06,.317,.025],[.6,.2015,.1875],.30,'top')
    if scale in ('50','100'):base['cornerRadius']=.004;neck['cornerRadius']=.006
    rings=[[-.047,.26,.150,5],[-.038,.256,.147,5],[-.027,.24,.14,4],[-.015,.212,.12,4],[-.009,.18,.10,4]]
    indices={'50':[0,1,2,3,4],'100':[0,2,3,4],'200':[0,2,4],'500':[0,4]}[scale]
    bulge=dict(id='monitor-bulge',kind='loft',bone=JOINT,material=CASE,role='body',tone=.24,capStart=True,capEnd=True,sections=[{'at':[0,-.015,rings[i][0]],'rx':rings[i][1],'ry':rings[i][2],'exponent':rings[i][3]}for i in indices])
    mount=box('monitor-vesa-plate',[.116,.116,.0025],[0,0,-.00775],.31,material='steel-desk')
    ear=box('monitor-hinge-ear',[.009,.033,.017],[-.042,0,-.001],.31,material='steel-desk');ear['repeat']={'count':2,'translation':[.084,0,0]}
    pin=dict(id='monitor-hinge',kind='revolve',bone=JOINT,material='steel-desk',role='body',tone=.32,profile=[[-.05,.0045],[-.047,.006],[.047,.006],[.05,.0045]],origin=[0,0,0],axis=[1,0,0])
    bolts=[dict(id='vesa-screw-'+name,kind='revolve',bone=JOINT,material='steel-desk',role='detail',tone=.27,profile=[[-.009,.002],[0,.002],[0,.0038],[.002,.0038]],origin=[x,y,-.0065],axis=[0,0,1])for name,x,y in [('bl',-.05,-.05),('br',.05,-.05),('tl',-.05,.05),('tr',.05,.05)]]
    return [base,neck,bezel,cover,display,deck,bulge,mount,ear,pin,*bolts]
def drawing(s,profile,lo,hi,region,pen):
    return dict(id=s['id'],bone=s['bone'],role=s['role'],z=3.5 if region=='display-front' else 3,filled=True,tone=s['tone'],kind='poly',points=profile,closed=True,region=region,material=s['material'],penMm=pen,solid={'kind':'extruded','sweep':'w','z0':lo,'z1':hi})
def author(a):
    if all(any(p['id']=='monitor-screen' and p['bone']==JOINT for p in v['body']) for v in a['volumes'].values()):return
    for collection,key in [(a['rig']['bones'],JOINT),(a['materials'],GLASS),(a['thermal']['regions'],'display-front'),(a['clips'],'adjust-screen-tilt')]:
        collection[:]=[q for q in collection if q['id']!=key]
    a['thermal']['assumptions'][:]=[q for q in a['thermal']['assumptions'] if not q['en'].startswith('The 25 W display releases')]
    a['rig']['bones'].append(dict(id=JOINT,parent='top',title=text('Display tilt spindle','Neigeachse des Monitors'),x=PIVOT[0],y=PIVOT[1],z=PIVOT[2],angleDeg=0,lengthM=.05,joint={'type':'revolute','minDeg':0,'maxDeg':0,'twistMinDeg':-5,'twistMaxDeg':20}))
    mat=deepcopy(next(m for m in json.loads(next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/laptop/🔣️.json')).read_text())['asset']['materials'] if m['id']==GLASS));a['materials'].append(mat)
    housing=next(r for r in a['thermal']['regions']if r['id']=='monitor-housing');front=deepcopy(housing)
    housing.update(title=text('Monitor enclosure and stand','Monitorgehäuse und Standfuß'),areaM2=round(.65-.598*.336,6),ratedPowerW=17)
    housing['operating']['heatW']=17
    front.update(id='display-front',title=text('Display cover glass','Deckglas des Bildschirms'),material=GLASS,areaM2=round(.598*.336,7),ratedPowerW=8,heatCapacityJM2K=4000,characteristicLengthM=.336)
    front['layers']=[dict(id='glass',title=text('2 mm cover glass','2 mm starkes Deckglas'),kind='glass',material=GLASS,thicknessM=.002,resistanceM2KW=.002,heatCapacityJM2K=4000,source='authored-estimate')]
    front['operating']['heatW']=8;a['thermal']['regions'].append(front)
    a['thermal']['assumptions'].append(text('The 25 W display releases an estimated 8 W through the cover glass and 17 W through the rear enclosure. The stand carries the full 8 kg monitor load to the worktop.','Der Monitor gibt von 25 W geschätzt 8 W über das Deckglas und 17 W über das Rückgehäuse ab. Der Fuß trägt die gesamte Monitorlast von 8 kg in die Tischplatte ein.'))
    for pose in a['poses']:
        pose['thermal']['regions']['display-front']=deepcopy(pose['thermal']['regions']['monitor-housing'])
    a['clips'].append(dict(id='adjust-screen-tilt',title=text('Adjust screen tilt','Bildschirm neigen'),description=text('The display tilts on the horizontal axle of its stand while the desk stays seated.','Der Bildschirm schwenkt auf der waagerechten Achse des Fußes bei Sitzhöhe.'),durationS=4,fps=24,loop=True,rootMotion='none',phases=[{'id':'adjust','fromS':0,'toS':4,'title':text('Tilt adjustment','Neigungsverstellung')}],events=[],tracks=[],contacts=[{'anchor':q['id'],'fromS':0,'toS':4} for q in a['anchors'] if q['kind']=='contact'],keys=[{'t':i,'pose':'seated-height','ease':'smooth','bones':{JOINT:{'twistDeg':v}}}for i,v in [(0,0),(1,-5),(2.5,20),(4,0)]]))
    for scale,v in a['volumes'].items():
        model=pieces(scale);v['body']=[s for s in v['body'] if not s['id'].startswith('monitor-')]+model
        v['thermal']=[s for s in v['thermal'] if not s['id'].startswith('th-monitor-')]
        for s in model:
            if s['id']=='monitor-backplane':continue
            isFront=s['id']=='monitor-screen'
            entry=dict(id='th-'+s['id'],of=s['id'],region='display-front' if isFront else 'monitor-housing')
            if s['material']==GLASS:entry.update(layer='glass',sectionFaces=['+w'])
            elif s['material']==CASE:entry['layer']='housing'
            v['thermal'].append(entry)
        for m in v['structure']:
            if m['id']=='mass-monitor':m['bone']=JOINT;m['transform']['translation']=[0,-.034,-.032]
        v['decisions'].append(text('The 15 mm bezel surrounds a 2 mm display glass. The contoured rear cover bears on a four-screw VESA plate. The hinge supports the display on the neck while its angle changes.','Der 15 mm tiefe Rahmen fasst ein 2 mm starkes Deckglas. Das profilierte Rückgehäuse sitzt auf einer VESA-Platte mit vier Schrauben. Das Gelenk trägt das neigbare Display am Standfuß.'))
        r=a['representations'][scale];pen=.25 if scale=='50' else .2
        src={s['id']:s for s in model}
        for view,items in r['geometry'].items():
            items[:]=[s for s in items if not s['id'].startswith('monitor-')]
            for item in items:
                if scale=='100' and item['id'] in ('top-bracket-left','top-bracket-right'):item['penMm']=.2
            for id in (['monitor-base','monitor-neck'] if scale=='500' else ['monitor-base','monitor-neck','monitor-hinge']):
                component=src[id]
                if id=='monitor-hinge':
                    profile=[[-.05,-.006],[.05,-.006],[.05,.006],[-.05,.006]]
                    low,high=.171,.183
                else:
                    w,h,depth=component['size'];cx,cy,cz=component['transform']['translation']
                    profile=[[cx-w/2,cy-h/2],[cx+w/2,cy-h/2],[cx+w/2,cy+h/2],[cx-w/2,cy+h/2]]
                    low,high=cz-depth/2,cz+depth/2
                items.append(drawing(component,profile,low,high,'monitor-housing',pen))
            cover=src['monitor-screen'];w,h,z=cover['size'];cx,cy,cz=cover['transform']['translation']
            items.append(drawing(src['monitor-panel'],src['monitor-panel']['profile'],.115,.130,'monitor-housing',pen))
            items.append(drawing(cover,outline(w,h,cy,.002,2),PIVOT[2]+cz-z/2,PIVOT[2]+cz+z/2,'display-front',pen))
            if view!='elevation':
                bulge=src['monitor-bulge'];items.append(drawing(bulge,outline(.49,.27,-.015,.012,2),.130,.168,'monitor-housing',pen))
if __name__=='__main__':
    doc=json.loads(CAT.read_text());author(doc['asset']);(T/'🗑️generated/digital-monitor-candidate.json').write_text(json.dumps(doc,ensure_ascii=False))
