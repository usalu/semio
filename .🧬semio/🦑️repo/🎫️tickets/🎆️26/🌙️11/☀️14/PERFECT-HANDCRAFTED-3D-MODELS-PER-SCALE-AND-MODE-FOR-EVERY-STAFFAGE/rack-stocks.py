"""🗄️ Fit the equipment rails, zero-U power strips and carcass elevations to their working sections."""
from pathlib import Path
from copy import deepcopy
from runpy import run_path
import json
T=Path(__file__).resolve().parent
CAT=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/server-rack/🔣️.json'))
def author(a):
    fine=a['volumes']['50']
    doors=run_path(str(T/'rack-doors.py'))
    for k in doors['PUNCH']:doors['PUNCH'][k]=doors['PUNCH']['50']
    doors['projected_paper'](a)
    doorParts={s['id']:s for s in fine['body'] if s['id'] in ('door-field','door-return','rear-door-a','rear-door-a-return','rear-door-b','rear-door-b-return')}

    stock={s['id']:s for s in fine['body'] if s['id'] in ('pdu','top-frame') or s['id'].startswith(('rail-front-','rail-rear-'))}
    slug='power-strips'
    region=deepcopy(next(r for r in a['thermal']['regions']if r['id']=='blanks'))
    region.update(id=slug,title={'en':'Two upright power distribution strips','de':'Zwei senkrechte Stromverteilerleisten'},material='chassis',areaM2=.704,characteristicLengthM=1.65)
    region['layers'][0].update(id='strip-housing',material='chassis',title={'en':'Folded steel housing','de':'Gekantetes Stahlgehäuse'})
    a['thermal']['regions'][:]=[r for r in a['thermal']['regions'] if r['id']!=slug]+[region]
    for pose in a['poses']:pose['thermal']['regions'][slug]=deepcopy(pose['thermal']['regions']['blanks'])
    for d,vol in a['volumes'].items():
        if d!='50':vol['body'][:]=[deepcopy(doorParts[q['id']]) if q['id'] in doorParts else q for q in vol['body']]
        if d!='50':
            for listName in ('body','thermal','structure'):
                def caster(s):return s['id'].startswith(('caster-','th-caster-','member-caster-'))
                vol[listName][:]=[s for s in vol[listName] if not caster(s)]+[deepcopy(s) for s in fine[listName] if caster(s)]
        existing={s['id'] for s in vol['body']}
        vol['body']+=[deepcopy(s) for k,s in stock.items()if k not in existing]
        vol['thermal'][:]=[s for s in vol['thermal'] if s['id']!='th-pdu']+[dict(id='th-pdu',of='pdu',region=slug,layer='strip-housing')]
        geom=a['representations'][d]['geometry']['elevation']
        for p in geom:
            if p['id'] in ('post-left','post-right'):
                p.update(x=.1,y=.245 if p['id']=='post-left' else -.28,w=1.837,h=.035,penMm=.20)
            elif p['id'] in ('server-block','top-group') and d=='500':p.update(y=-.2413,h=.4826)
            elif p['id']=='roof' and d=='200':p.update(x=1.967,w=.033)
            elif p['id']=='base-rail' and d=='200':p.update(x=.07,w=.03)
        new=[]
        for q in ('a','b'):
            for name,bone,x,y,w,h,mat,region,z in [('side-edge','frame',.07,.2988 if q=='a' else -.3,1.897,.0012,'rack-steel','cabinet',1.7),('pdu','frame',.2,.19 if q=='a' else -.245,1.65,.055,'chassis',slug,1.3),('rail','equipment',0,.225 if q=='a' else -.2445,1.8669,.0195,'rack-steel','cabinet',2.)]:
                new.append(dict(id=name+'-'+q,kind='rect',bone=bone,role='body',z=z,filled=True,tone=.38,x=x,y=y,w=w,h=h,region=region,material=mat,penMm=.20))
        if d=='50':
            for part in new:
                if part['id'].startswith('side-edge-'):
                    part.update(y=.28 if part['id'].endswith('-a') else -.30,h=.020,w=1.895)
            side=a['representations'][d]['geometry']['front']
            edges=[]
            for bone,y,z0,z1,region in [('door',.2685,-.5,-.475,'door'),('rear-door-a',-.005,.475,.5,'rear-doors'),('rear-door-b',.0035,.475,.5,'rear-doors')]:
                edges.append(dict(id=bone+'-side-return',bone=bone,region=region,material='mesh-steel',role='body',z=3.91 if bone=='door' else .76,filled=True,tone=.22,penMm=.20,kind='rect',x=0,y=y,w=1.867,h=.0015,solid=dict(kind='extruded',sweep='w',z0=z0,z1=z1)))
            edgeIds={q['id'] for q in edges}
            side[:]=[q for q in side if q['id'] not in edgeIds]+edges
        joined=[(.10,-.28),(1.967,-.28),(1.967,.28),(.10,.28),(.10,.245),(1.937,.245),(1.937,-.245),(.10,-.245)]
        new.append(dict(id='portal-frame',kind='poly',bone='frame',role='body',z=3,filled=True,closed=True,tone=.50,points=[list(q) for q in joined],region='cabinet',material='rack-steel',penMm=.20))
        replacing={q['id']for q in new}|{'post-left','post-right','frame-head'}
        geom[:]=[q for q in geom if q['id'] not in replacing]+new
        for note in vol['decisions']:
            note['en']=note['en'].replace('Punched aperture count falls with drawing scale.','The front and rear openings keep their spacing throughout the scale series.')
            note['de']=note['de'].replace('Die Zahl der dargestellten Stanzöffnungen sinkt mit dem Maßstab.','Die Stanzöffnungen vorne und hinten behalten in allen Maßstäben ihre Teilung.')
        if d in ('200','500'):
            vol['structure'][:]=[s for s in vol['structure']if not s['id'].startswith('member-rail-')]
            vol['structure']+=[deepcopy(s)for s in fine['structure'] if s.get('of','').startswith(('rail-front-','rail-rear-'))]
if __name__=='__main__':
    doc=json.loads(CAT.read_text());author(doc['asset']);(T/'🗑️generated/digital-rack-stocks.json').write_text(json.dumps(doc,ensure_ascii=False))
