"""🧱 Analyze saved original source rosters without rerunning external geometry."""
import json,math,itertools,sys
from pathlib import Path
from collections import defaultdict,Counter
root=Path(__file__).parent
source=json.loads((root/'🗑️generated/faceted-brep-geometry-independent.json').read_text(encoding='utf-8'))
def sub(a,b):return tuple(x-y for x,y in zip(a,b))
def cross(a,b):return(a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])
def dot(a,b):return math.fsum(x*y for x,y in zip(a,b))
rows=[]
for model in source['rows']:
 for solid in model['solids']:
  points={};incident=defaultdict(set);edges=defaultdict(list);faceRows=[];adj=defaultdict(set)
  for face in solid['faces']:
   allp=[]
   for bound in face['bounds']:
    ring=bound['points'] if bound['orientation'] else list(reversed(bound['points']))
    for p in ring:points[p['id']]=p['coordinates'];incident[p['id']].add(face['id']);allp.append(p['coordinates'])
    for a,b in zip(ring,ring[1:]+ring[:1]):edges[tuple(sorted((a['id'],b['id'])))].append((face['id'],a['id'],b['id']))
   anchor=allp[0];normal=max((cross(sub(a,anchor),sub(b,anchor))for a,b in itertools.combinations(allp[1:],2)),key=lambda v:dot(v,v));norm=math.sqrt(dot(normal,normal));residual=max(abs(dot(sub(p,anchor),normal))/norm for p in allp)if norm else None
   span=max(math.dist(a,b)for a,b in itertools.combinations(allp,2));faceRows.append(dict(id=face['id'],bounds=len(face['bounds']),points=len(allp),span=span,maxPlaneResidual=residual,relativeResidual=residual/span if span and residual is not None else None))
  bad=[]
  for edge,uses in edges.items():
   if len(uses)!=2 or uses[0][1:]!=tuple(reversed(uses[1][1:])):bad.append(dict(edge=edge,uses=uses))
   for a,b in itertools.combinations(uses,2):adj[a[0]].add(b[0]);adj[b[0]].add(a[0])
  unseen=set(f['id']for f in solid['faces']);components=[]
  while unseen:
   stack=[unseen.pop()];component=[]
   while stack:
    f=stack.pop();component.append(f)
    for n in adj[f]&unseen:unseen.remove(n);stack.append(n)
   components.append(component)
  coords=defaultdict(list)
  for identity,p in points.items():coords[tuple(p)].append(identity)
  counts=Counter(len(v)for v in incident.values());ordered=sorted(faceRows,key=lambda r:r['relativeResidual']or 0,reverse=True)
  rows.append(dict(file=model['file'],sha256=model['sha256'],solid=solid['id'],uniquePointIds=len(points),edges=len(edges),edgeUseHistogram=dict(Counter(len(v)for v in edges.values())),nonOpposingOrNonManifoldEdges=bad,vertexFaceCountHistogram=dict(counts),verticesBelowThree=[dict(id=k,faces=sorted(v),coordinates=points[k])for k,v in incident.items()if len(v)<3],components=components,duplicateCoordinateIds=[dict(coordinates=k,ids=v)for k,v in coords.items()if len(v)>1],maxPlanarity=ordered[:5],facePlanarity=faceRows))
(root/'🗑️generated/faceted-original-topology.json').write_text(json.dumps(dict(scope='Saved original source-ID rosters; no mesh recomputation or native proof',rows=rows),indent=2)+'\n',encoding='utf-8')
for row in rows:print('[DEBUG] '+json.dumps({k:v for k,v in row.items()if k not in ['facePlanarity','components']},ensure_ascii=False));print('[DEBUG] components '+str([len(c)for c in row['components']]))
