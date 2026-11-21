"""🎋 Seat each grass leaf and bamboo member on the culm or terminal blade that carries its load."""
from pathlib import Path
import json
import math
from runpy import run_path
from copy import deepcopy
t=Path(__file__).resolve().parent
for slug in ('miscanthus-grass','bamboo-clump'):
 path=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/'+slug+'/🔣️.json'));doc=json.loads(path.read_text());a=doc['asset']
 for d,v in a['volumes'].items():
    body={s['id']:s for s in v['body']}
    for s in list(v['structure']):
        if slug=='bamboo-clump':
            if s['id']!='m-fol-4' or 'fol-4' not in body:continue
            bone=s['bone'];name=s['id'];s.clear();s.update(id=name,of='fol-4',member=bone,part='member')
        else:
            if s['id'] not in ('member-bl-l2b','member-bl-r2b'):continue
            bone=s['bone'];suffix=bone.removeprefix('bl-');stem=body.get('stem-'+suffix)
            leaves=[q for q in v['body']if q['bone']==bone and 'vertices' in q and q['material']=='miscanthus-blade']
            if not leaves:continue
            tip=s['sections'][-1]['at'];blade=min(leaves,key=lambda leaf:math.dist(tip,leaf['vertices'][-1]))
            assert math.dist(tip,blade['vertices'][-1])<.003
            name=s['id'];s.clear();s.update(id=name,of=stem['id']if stem else blade['id'],member=bone,part='member')
            if stem:v['structure'].append(dict(id='member-blade-'+suffix,of=blade['id'],member=bone,part='member'))
 if slug=='miscanthus-grass':
    medium=a['volumes']['200'];near=a['volumes']['100']
    mesh=run_path(str(t/'living-botanical-shells.py'))['mesh']
    small=[q for q in medium['body'] if q['id'].startswith('ribbon-')]
    gone={q['id'] for q in small};kernels={q['id']:q for q in small}
    parts=[]
    for original in near['body']:
        if not original['id'].startswith(('blade-','stem-','basal-','tiller-')):continue
        part=deepcopy(original)
        if part['id'].startswith('blade-'):
            pts=part['vertices'];side=part['triangles'][0][2];count=(len(pts)-2)//side
            rows=sorted({round(u*(count-1)/4) for u in range(5)})
            rings=[[pts[j*side+round(h*side/4)%side]for h in range(4)] for j in rows]
            originalPart=deepcopy(part);mesh(part,rings)
            def triangle_area(indices):
                x,y,z=[part['vertices'][idx] for idx in indices];u=[b-a for a,b in zip(x,y)];w=[b-a for a,b in zip(x,z)]
                return math.hypot(u[1]*w[2]-u[2]*w[1],u[2]*w[0]-u[0]*w[2],u[0]*w[1]-u[1]*w[0])
            if min(map(triangle_area,part['triangles']))<1e-10:part=originalPart
        parts.append(part)
    present={q['id']for q in parts}
    medium['body'][:]=[q for q in medium['body']if q['id']not in gone|present]+parts
    medium['thermal'][:]=[q for q in medium['thermal']if q.get('of')not in gone|present]+[deepcopy(q)for q in near['thermal']if q.get('of')in present]
    for q in medium['structure']:
        if q.get('of')not in gone:continue
        prev=kernels[q['of']];p=prev['vertices'][-1]
        leaves=[s for s in parts if s['bone']==prev['bone']and s['id'].startswith('blade-')]
        if not leaves:raise ValueError(prev['id'])
        q['of']=min(leaves,key=lambda s:math.dist(s['vertices'][-1],p))['id']
    for member in list(medium['structure']):
        if member.get('part')!='member' or not member.get('member','').startswith('bl-') or member.get('of'):continue
        bone=member['member'];leaves=[q for q in parts if q['bone']==bone and q['id'].startswith('blade-')]
        if not leaves:continue
        target=member['sections'][-1]['at'];end=min(leaves,key=lambda q:math.dist(q['vertices'][-1],target))
        stem=next((q for q in parts if q['bone']==bone and q['id'].startswith('stem-')),None)
        key=member['id'];member.clear();member.update(id=key,of=stem['id'] if stem else end['id'],member=bone,part='member')
        if stem:medium['structure'].append(dict(id='member-tip-'+bone,of=end['id'],member=bone,part='member'))
 if slug=='miscanthus-grass':
    for shapes in a['representations']['100']['geometry'].values():
        for q in shapes:
            if q['id']=='plume-1-4':q['penMm']=.20
 (t/'🗑️generated'/('digital-'+slug+'-culm.json')).write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
