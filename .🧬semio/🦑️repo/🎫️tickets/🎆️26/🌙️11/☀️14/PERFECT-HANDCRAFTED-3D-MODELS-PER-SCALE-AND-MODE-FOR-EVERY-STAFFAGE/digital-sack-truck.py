"""🛞 Turn the sack-truck axle and open the steel hubs through both tyre beads."""
from copy import deepcopy
from pathlib import Path
import json,math
T=Path(__file__).resolve().parent
CAT=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/courier-with-hand-truck/🔣️.json'))
def author(asset):
    near=next(q for q in asset['volumes']['100']['body'] if q['id']=='rim')
    for d,v in asset['volumes'].items():
        body={s['id']:s for s in v['body']};shaft=body['axle']
        for k in ['p0','p1','r0','r1']:shaft.pop(k,None)
        end=[(.279,.01),(.282,.01),(.282,.0093),(.2833,.0093),(.2833,.01),(.2875,.01),(.289,.0085)] if d in ('50','100') else [(.2875,.01),(.289,.0085)]
        shaft.update(kind='revolve',profile=[[-x,r]for x,r in reversed(end)]+[list(p)for p in end],axis=[0,0,1],origin=[0,0,0])
        rim=body.get('rim')
        if not rim:
            rim=deepcopy(near);rim['profile']=[[-.0375,.0102],[-.0375,.020],[-.017,.059],[.017,.059],[.0375,.020],[.0375,.0102]]
            v['body'].append(rim);v['thermal'].append(dict(id='th-rim',of='rim',region='hand-truck'));v['structure'].append(dict(id='st-rim',of='rim',member='truck-lift',part='member'))
        rim['profile'][0][1]=rim['profile'][-1][1]=.0102
        if rim['profile'][-1]!=rim['profile'][0]:rim['profile'].append(rim['profile'][0].copy())
        rubber=body['tyre']['profile']
        if min(p[1] for p in rubber)<.056:
            rubber[0][1]=rubber[-1][1]=.056;rubber.append(rubber[0].copy())
        if d in ('50','100'):
            angles=[math.radians(30+i*300/24)for i in range(25)]
            clip=deepcopy(shaft)
            for key in ['profile','origin','axis']:clip.pop(key)
            clip.update(id='snap-ring',kind='extrude',plane='xy',role='detail',profile=[[round(r*math.cos(a),7),round(r*math.sin(a),7)]for r,seq in [(.0125,angles),(.00935,reversed(angles))] for a in seq],repeat=dict(count=2,translation=[0,0,.5653]))
            clip['from']=-.2832;clip['to']=-.2821
            if d=='50':
                for view,shapes in asset['representations'][d]['geometry'].items():
                    axis=next(q for q in shapes if q['id']=='axle')
                    for idx,side in enumerate(('near','far')):
                        z0=clip['from']+idx*.5653;z1=clip['to']+idx*.5653
                        q=deepcopy(axis);q.update(id='snap-ring-'+side,points=clip['profile'],penMm=.20,z=round(axis['z']+(.11 if idx==0 or view!='elevation' else -.02),3),solid=dict(kind='extruded',sweep='w',z0=z0,z1=z1))
                        shapes.append(q)
                    shapes.sort(key=lambda q:q['z'])
            v['body'].append(clip);v['thermal'].append(dict(id='th-snap-ring',of='snap-ring',region='hand-truck'));v['structure'].append(dict(id='st-snap-ring',of='snap-ring',member='truck-lift',part='member'))
    for shapes in asset['representations']['200']['geometry'].values():
        for s in shapes:
            if s['id']=='axle':s['penMm']=.16
if __name__=='__main__':
    doc=json.loads(CAT.read_bytes());author(doc['asset'])
    (T/'🗑️generated/digital-sack-truck-journals.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
