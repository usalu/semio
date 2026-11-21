"""🔧 Shape the meter stampings, bin plinth and floor nozzle from their physical sections at each drawing scale."""
from pathlib import Path
from copy import deepcopy
from math import sin,cos,pi
import sys,json
T=Path(__file__).resolve().parent
def meter(a):
    v=a['volumes']['200'];fine=a['volumes']['50']['body']
    form=next(p for p in v['body']if p['id']=='housing');keep={k:form[k]for k in ('id','bone','material','role','tone')};form.clear()
    form.update(keep,kind='loft',capStart=True,capEnd=True,sections=[dict(at=[0,h,0],rx=x,ry=z,exponent=5.5)for h,x,z in [(-.10,.0785,.049),(-.082,.089,.0595),(0,.0915,.062),(.077,.089,.0595),(.095,.0785,.049)]])
    seam=deepcopy(next(p for p in fine if p['id']=='seam'));seam['sections']=[seam['sections'][0],dict(at=[0,0,0],rx=.0965,ry=.067,exponent=5.5),seam['sections'][-1]]
    v['body']=[s for s in v['body'] if s['id']!='seam']+[seam];v['thermal']=[s for s in v['thermal'] if s['id']!='th-seam']+[dict(id='th-seam',of='seam',region='shell')]
    drawings=a['representations']['200']['geometry']
    for view in ('front','elevation'):
        higher=a['representations']['50']['geometry'][view]
        edge=deepcopy(next(q for q in higher if q['id']=='seam'))
        edge['penMm']=.2
        drawings[view]=[q for q in drawings[view] if q['id']!='seam']+[edge]
    shape=next(q for q in drawings['elevation']if q['id']=='housing')
    edge=[(q['rx'],q['at'][1])for q in form['sections']]
    for k in ['x','y','w','h','r']:shape.pop(k,None)
    shape.update(kind='poly',closed=True,points=[list(q)for q in edge]+[[-x,y] for x,y in reversed(edge)])
    v['decisions'].append({'en':'Five pressed sections retain the taper of the two metal shells. The folded seam projects beyond both stampings.','de':'Fünf Pressquerschnitte bilden die verjüngten Blechschalen ab. Der gefalzte Mittelstoß ragt über beide Schalen hinaus.'})
def rounded(radius,half,steps):
    out=[]
    for x,z,deg in [(half-radius,-half+radius,-90),(half-radius,half-radius,0),(-half+radius,half-radius,90),(-half+radius,-half+radius,180)]:
        for j in range(steps+1):
            a=pi/180*(deg+90*j/steps);out.append([round(x+radius*cos(a),7),round(z+radius*sin(a),7)])
    return out
def bin_body(a):
    a['thermal']['minimumAirPassageM']=.014
    fine={p['id']:p for p in a['volumes']['50']['body']}
    for scale,steps in [('100',3),('200',2),('500',1)]:
        v=a['volumes'][scale]
        feet=[deepcopy(s)for k,s in fine.items()if k.startswith('foot-')]
        pedestal=deepcopy(fine['pedestal']);shoulder=deepcopy(fine['pedestal-shoulder']);floor=deepcopy(fine['floor'])
        for s in [pedestal,shoulder]:
            s.update(profile=rounded(.030,.200,steps),holes=[list(reversed(rounded(.026,.196,steps)))])
        floor['profile']=[[z,round(x+.2,7)] for z,x in rounded(.028,.198,steps)]
        if 'slab' in {s['id']for s in v['body']}:pedestal['id']='slab'
        pieces=feet+[pedestal,shoulder,floor];ids={s['id'] for s in pieces}
        v['body']=[s for s in v['body'] if s['id'] not in ids]+pieces
        wall=next(s for s in v['body']if s['id']=='wall');wall['from']=.025
        v['thermal']=[s for s in v['thermal'] if s['id'].removeprefix('th-') not in ids]+[dict(id='th-'+s['id'],of=s['id'],region='bin-hdpe',layer='hdpe-wall')for s in pieces]
        for st in v['structure']:
            if st['id']=='member-slab':st.update(of=pedestal['id'])
        if scale in ('100','200','500'):
            source=fine
            parts=[]
            stride={'100':1,'200':2,'500':3}[scale]
            for id in ('lid-cap','lid-end-l','lid-end-r','lid-rail','lid-rail-l','lid-rail-r','lid-skirt','rim-seat','rim-bead'):
                lid=deepcopy(source[id])
                if id in ('rim-seat','rim-bead'):
                    for ring in [lid['profile'],*lid.get('holes',[])]:
                        ring[:]=[q for j,q in enumerate(ring) if j%7 in (0,6) or (j%7)%stride==0]
                elif id=='lid-cap':
                    profile=lid['profile']
                    keep={0,1,len(profile)-1}
                    for n in range(1,len(profile)-1):
                        u,q,w=profile[n-1:n+2]
                        if any((q[dim]-u[dim])*(w[dim]-q[dim])<=0 and (q[dim]!=u[dim] or q[dim]!=w[dim]) for dim in [0,1]):keep.add(n)
                    lid['profile']=[q for j,q in enumerate(profile)if j in keep or j%stride==0]
                parts.append(lid)
            ceiling={p['id'] for p in parts}|{'lid','rim'}
            v['body']=[p for p in v['body'] if p['id'] not in ceiling]+parts
            v['thermal']=[p for p in v['thermal'] if p['id'].removeprefix('th-') not in ceiling]
            skins={p['of']:p for p in a['volumes']['50']['thermal'] if 'of' in p}
            v['thermal'] +=[deepcopy(skins[p['id']]) for p in parts]
            for m in v['structure']:
                if m.get('of')=='lid':m['of']='lid-cap'
                if m.get('of')=='rim':m['of']='rim-seat'
            wall['to']=.94
        geom=a['representations'][scale]['geometry']
        for view in ('front','elevation'):
            fineDrawing=a['representations']['50']['geometry'][view]
            feet=[deepcopy(q)for q in fineDrawing if q['id'] in ('pedestal-foot-l','pedestal-foot-r')]
            for q in feet:q['penMm']=.22
            geom[view]=[q for q in geom[view] if q['id'] not in ('pedestal-foot-l','pedestal-foot-r','bearing-floor')]+feet
            geom[view].append(dict(id='bearing-floor',bone='base-plate',kind='rect',role='body',z=1.15,filled=True,tone=.32,x=.002 if view=='elevation' else -.198,y=0 if view=='elevation' else .025,w=.396,h=.0075,region='bin-hdpe',material='hdpe-plastic',penMm=.2))
        v['decisions'].append({'en':'The pedestal carries the 7.5 mm floor at 25 mm above the ground. The four pads are 8.5 mm thick. The rolled shoulder and 4 mm pedestal wall retain their working sections.','de':'Der Sockel trägt den 7,5 mm starken Boden 25 mm über dem Aufstand. Die vier Füße sind 8,5 mm stark. Sockelschulter und 4-mm-Sockelwand behalten ihren Querschnitt.'})
def vacuum(a):
    fine={p['id']:p for p in a['volumes']['50']['body']}
    shell=next(r for r in a['thermal']['regions']if r['id']=='body-shell');shell['areaM2']=.293
    a['thermal']['assumptions'].append({'en':'The shell exposes 0.293 m² below the independent lid. The lid contributes about 0.178 m² to the same canister exterior and has its own heat field.','de':'Unter dem separaten Deckel gibt die Schale über 0,293 m² Wärme ab. Der Deckel trägt rund 0,178 m² zur Außenfläche bei und hat ein eigenes Temperaturfeld.'})
    for shapes in a['representations']['50']['geometry'].values():
        for shape in shapes:
            if shape['id'] in ('caster-left','caster-right'):shape['penMm']=.15
            if shape['id'] in ('grip-swivel','brush-rear','hose-end'):shape['penMm']=.20
    for d,v in a['volumes'].items():
        tube=next(p for p in v['body'] if p['id']=='wand-tube')
        end=.3 if d=='50' else .74
        tube['profile']=[[0,.0165],[0,.0175],[end,.0175],[end,.0165],[0,.0165]]
        if d=='50':
            inner=next(p for p in v['body'] if p['id']=='wand-inner')
            inner['profile']=[[.27,.015],[.27,.016],[.74,.016],[.74,.015],[.27,.015]]
            collar=next(p for p in v['body']if p['id']=='wand-collar')
            collar['profile']=[[.28,.0175],*collar['profile'][1:-1],[.318,.016],[.3,.016],[.3,.0175],[.28,.0175]]
        if d=='500':
            for shapes in a['representations'][d]['geometry'].values():
                for q in shapes:
                    if q['id']=='wand-tube':
                        q.update(y=-.0175,h=.035,w=.74)
                        if q.get('solid'):q['solid'].update(z0=-.0175,z1=.0175)
    for d,index in [('200',[0,1,3,5,7,9]),('500',[0,1,3,5,8])]:
        v=a['volumes'][d]
        for p in v['body']+v['thermal']:
            if 'hose-' in p['id'] and p.get('kind')=='capsule':
                p['r0']=p['r1']=.018
        nozzle=next(p for p in v['body'] if p['id']=='nozzle')
        pts=fine['nozzle']['profile']
        nozzle['profile']=[pts[j] for j in index if j<len(pts)];nozzle['from']=-.1425;nozzle['to']=.1425
        add=[deepcopy(fine[k]) for k in ['brush','brush-rear','nozzle-neck']]
        if d=='500':
            neck=add[-1];neck['sections']=[neck['sections'][0],neck['sections'][-1]]
        ids={q['id']for q in add};v['body']=[q for q in v['body']if q['id'] not in ids]+add
        for q in [nozzle,*add]:
            name='th-'+q['id'];surface=next((p for p in v['thermal']if p['id']==name),None)
            if surface:
                region,layer=surface['region'],surface.get('layer');surface.clear();surface.update(id=name,of=q['id'],region=region)
                if layer:surface['layer']=layer
            else:v['thermal'].append(dict(id=name,of=q['id'],region='nozzle',layer='noz'))
        for view,shapes in a['representations'][d]['geometry'].items():
            fineDraw=a['representations']['50']['geometry'][view]
            brushes=[deepcopy(q) for q in fineDraw if q['id']=='brush']
            for brush in brushes:brush['penMm']=.2
            for q in shapes:
                if d=='500' and q['bone'].startswith('hose-') and q['id'].endswith('-tube'):
                    q.update(y=-.018,h=.036)
                    if q.get('solid'):q['solid'].update(z0=-.018,z1=.018)
                if q['id']=='nozzle':
                    q['d']='M '+' L '.join(' '.join(str(n)for n in point) for point in nozzle['profile'])+' Z'
                    q.pop('regions',None);q['region']='nozzle'
            shapes[:]=[q for q in shapes if q['id'] not in ('brush','brush-rear')]+brushes
        v['decisions'].append({'en':'The ten hose sections remain 36 mm outside diameter. The floor head has a 285 mm working width, two 8 mm brush strips and a swivel neck. The 35 mm steel wand has a continuous bore.','de':'Die zehn Schlauchabschnitte behalten 36 mm Außendurchmesser. Der 285 mm breite Bodenkopf hat zwei 8-mm-Bürstenleisten und einen drehbaren Anschluss. Das 35-mm-Stahlrohr hat einen durchgehenden Saugkanal.'})
def author(a):
    {'gas-meter':meter,'waste-bin':bin_body,'vacuum-cleaner':vacuum}[a['id']](a)
    for notes in [a['thermal']['assumptions'],*[v['decisions'] for v in a['volumes'].values()]]:
        notes[:]=list({json.dumps(q,sort_keys=True):q for q in notes}.values())
if __name__=='__main__':
    for slug in sys.argv[1:]:
        path=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/'+slug+'/🔣️.json'));d=json.loads(path.read_text());author(d['asset']);(T/('🗑️generated/digital-'+slug+'.json')).write_text(json.dumps(d,ensure_ascii=False))
