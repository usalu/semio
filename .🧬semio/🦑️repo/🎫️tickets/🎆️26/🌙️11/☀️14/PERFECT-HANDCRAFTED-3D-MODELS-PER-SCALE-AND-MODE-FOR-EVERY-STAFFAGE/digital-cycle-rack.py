"""🚲 Author a low, open tubular city-cycle rack with twin load paths and a rear lamp."""
from pathlib import Path
from copy import deepcopy
from runpy import run_path
import json
T=Path(__file__).resolve().parent;CAT=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/cyclist/*json'))
tube=run_path(str(T/'r3-cyclist-kit.py'))['tube']
def author(a):
    fine=a['volumes']['50'];stock=[deepcopy(q) for q in fine['body'] if q['id'].startswith('carrier-')]
    lamp=deepcopy(next(q for q in fine['body'] if q['id']=='tail-lamp'));lamp['transform']['translation'][1]-=.065
    for d,v in a['volumes'].items():
        members=[]
        for original in stock:
            name=original['id']
            if d=='200' and name=='carrier-bar-3' or d=='500' and name in ('carrier-bar-1','carrier-bar-3'):continue
            pts=[]
            for i,s in enumerate(original['sections']):
                x,y,z=s['at']
                if 'leg' in name:drop=[.065,.032,0][i]
                elif 'stay' in name:drop=[.065,.039,0][i]
                else:drop=.065
                pts.append(([x,round(y-drop,6),z],s['rx'],s['ry']))
            shape=tube(name,'frame',pts,'steel',original['tone'],hint=[0,1,0] if 'bar-' in name else [0,0,1],**({'slender':True} if original.get('slender') else {}))
            members.append(shape)
        ids={q['id'] for q in v['body'] if q['id'].startswith('carrier') or q['id']=='tail-lamp'}
        pieces=members+[deepcopy(lamp)]
        v['body'][:]=[q for q in v['body']if q['id']not in ids]+pieces
        v['thermal'][:]=[q for q in v['thermal'] if q.get('of')not in ids]+[dict(id='th-'+q['id'],of=q['id'],region='bike-metal')for q in pieces]
        v['structure'][:]=[q for q in v['structure'] if q.get('of')not in ids]+[dict(id='st-'+q['id'],of=q['id'],member='frame',part='member')for q in members]
        rep=a['representations'][d]
        for view,(i,j) in {'elevation':(0,1),'front':(2,1),'plan':(0,2)}.items():
            old=rep['geometry'][view];templates={q['id']:q for q in old}
            for q in old:
                if q['id']=='tail-lamp' and view!='plan' and 'points'in q:
                    for pt in q['points']:pt[1]=round(pt[1]-.065,5)
            retained=[q for q in old if not q['id'].startswith('carrier')]
            seen=set()
            for q in members:
                points=[[s['at'][i],s['at'][j]] for s in q['sections']]
                line=tuple(tuple(p) for p in points)
                if len(set(line))<2 or line in seen or tuple(reversed(line)) in seen:continue
                seen.add(line)
                template=templates.get(q['id'],templates.get('carrier',{}))
                retained.append(dict(id=q['id'],bone='frame',material='steel',region='bike-metal',role='body',z=template.get('z',5),filled=False,tone=q['tone'],kind='poly',closed=False,points=points,penMm=.20 if 'rail-' in q['id'] else .15))
            rep['geometry'][view]=sorted(retained,key=lambda q:q['z'])
if __name__=='__main__':
    doc=json.loads(CAT.read_bytes());author(doc['asset'])
    (T/'🗑️generated/digital-cyclist-carrier.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
