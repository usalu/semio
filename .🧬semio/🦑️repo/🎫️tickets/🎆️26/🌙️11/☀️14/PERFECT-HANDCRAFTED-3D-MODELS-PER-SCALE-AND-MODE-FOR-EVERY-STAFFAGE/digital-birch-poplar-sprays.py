"""🌱 Retain the measured bends and outer shoots of the drooping birch and the upright poplar crown."""
from copy import deepcopy
from pathlib import Path
from runpy import run_path
from collections import defaultdict
import json
import math
t=Path(__file__).resolve().parent
mesh=run_path(str(t/'living-botanical-shells.py'))['mesh']
def reduce_leaf(s,vertices,stations):
    part=deepcopy(s);points=s['vertices'];side=s['triangles'][0][2]
    if not 3<=side<=32 or (len(points)-2)%side: return part
    count=(len(points)-2)//side
    perimeter=sorted({round(i*side/min(side,vertices))%side for i in range(min(side,vertices))})
    rows=sorted({round(i*(count-1)/(min(count,stations)-1)) for i in range(min(count,stations))})
    mesh(part,[[points[j*side+i] for i in perimeter]for j in rows])
    return part
def forms(v):return [s for s in v['body'] if s['material'].endswith('leaf')]
def insert(v,parts,fine):
    ids={s['id']for s in parts}
    v['body'][:]=[p for p in v['body']if p['id']not in ids]+parts
    thermal=[deepcopy(q) for q in fine['thermal']if q.get('of')in ids]
    names={q['id'] for q in thermal}
    v['thermal'][:]=[p for p in v['thermal']if p.get('of')not in ids and p['id']not in names]+thermal
def extrema(parts,dirs):
    def val(s,dir):return max(sum(x*y for x,y in zip(p,dir)) for p in s['vertices'])
    return {max(parts,key=lambda s:val(s,dr))['id'] for dr in dirs}
def birch(a):
    vols=a['volumes'];fine=vols['50'];leaves={s['id']:s for s in forms(fine)}
    for d,n,stations in [('100',8,7),('200',6,6)]:
        parts=[reduce_leaf(leaf,n,stations) for leaf in leaves.values()]
        insert(vols[d],parts,fine)
    v=vols['500'];medium=vols['200']
    existing={s['id']for s in v['body']};tresses=defaultdict(list)
    for s in forms(medium):
        if s['id'].split('-')[-1].isdigit():tresses[s['id'].rsplit('-',1)[0]].append(s)
    extra=set()
    for parts in tresses.values():
        extra.update(extrema(parts,[(0,-1,0),(0,0,1),(0,0,-1),(1,-.4,0)]));extra.update(q['id'] for i,q in enumerate(parts) if i%3!=1)
    insert(v,[reduce_leaf(s,6,5) for s in forms(medium) if s['id'] in extra or s['id'] in existing],medium)
    for s in v['structure']:
        name=s['id'].removeprefix('member-')
        src=next((q for q in fine['body']if q['id']==name),None)
        if name.startswith('tip-') and src is not None and name not in existing:
            insert(v,[deepcopy(src)],fine);bone=s['bone'];s.clear();s.update(id='member-'+name,of=name,member=bone,part='member')
def poplar(a):
    vols=a['volumes'];fine=vols['50'];v=vols['100']
    ids={s['id'] for s in forms(v)}
    v['thermal'][:]=[s for s in v['thermal']if s.get('of')not in ids];v['body'][:]=[s for s in v['body']if s['id']not in ids]
    insert(v,[reduce_leaf(s,8,5) for s in forms(fine)],fine)
    med=vols['200'];coarse=vols['500'];groups=defaultdict(list)
    for q in forms(med):
        if q['id'].startswith('fol-'):groups[q['bone']].append(q)
    middleBodies={s['id']for s in forms(med)}
    med['body'][:]=[p for p in med['body']if p['id']not in middleBodies]
    med['thermal'][:]=[p for p in med['thermal']if p.get('of')not in middleBodies]
    retained=defaultdict(list)
    for leaf in forms(fine):retained[leaf['bone']].append(leaf)
    source=[]
    for pieces in retained.values():
        for i,q in enumerate(pieces):
            source.append(reduce_leaf(q,8,4))
    insert(med,source,fine)
    groups.clear()
    for q in forms(med):
        if q['id'].startswith('fol-'):groups[q['bone']].append(q)
    extra=set()
    for parts in groups.values():extra.update(extrema(parts,[(1,0,0),(0,0,-1),(0,0,1)]))
    insert(coarse,[reduce_leaf(s,6,4)for s in forms(med)if s['id']in extra],med)
if __name__=='__main__':
    for slug,author in [('multi-stem-birch',birch),('lombardy-poplar',poplar)]:
     path=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/'+slug+'/🔣️.json'));doc=json.loads(path.read_text());author(doc['asset'])
     (t/'🗑️generated'/('digital-'+slug+'-sprays.json')).write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
