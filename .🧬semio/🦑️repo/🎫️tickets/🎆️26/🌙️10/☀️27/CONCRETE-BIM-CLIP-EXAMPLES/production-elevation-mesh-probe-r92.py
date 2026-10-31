"""🔎 Compare structural mesh silhouette with production vertical-prism fallback."""
import hashlib,json
from pathlib import Path
from shapely.geometry import Polygon,box,mapping
from shapely.ops import unary_union
T=Path(__file__).parent;R=next(p for p in T.parents if (p/'AGENTS.md').exists() and (p/'package.json').exists());B=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip';raw=(B/'🧱️model/🔣️.json').read_bytes();m=json.loads(raw);rows=[]
for e in m['elements']:
 if e['class'] not in ('IfcBeam','IfcColumn','IfcFooting'):continue
 for i,p in enumerate(e['geometry'].get('parts',[])):
  if 'mesh' not in p:continue
  verts=p['mesh']['vertices'];faces=p['mesh']['faces']
  for axis in (0,1):
   projected=[Polygon([(verts[j][axis],verts[j][2]) for j in f]) for f in faces];actual=unary_union([q for q in projected if q.area]);bounds=actual.bounds;fallback=box(*bounds);extra=fallback.difference(actual)
   if extra.area>0:rows.append({'tag':e['tag'],'part':i,'horizontal_axis':'XY'[axis],'actual_projected_area':actual.area,'fallback_bbox_area':fallback.area,'false_silhouette_area':extra.area,'witness':list(extra.representative_point().coords)[0],'actual_projection':mapping(actual)})
out={'source':hashlib.sha256(raw).hexdigest(),'scope':'Source structural convex mesh projections from actual face polygons; production fallback rectangle diagnosis, no full native/render capture','rows':rows};(T/'🗑️generated/production-elevation-mesh-probe-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps({'source':out['source'],'rows':[{k:v for k,v in q.items() if k!='actual_projection'} for q in rows]}))
