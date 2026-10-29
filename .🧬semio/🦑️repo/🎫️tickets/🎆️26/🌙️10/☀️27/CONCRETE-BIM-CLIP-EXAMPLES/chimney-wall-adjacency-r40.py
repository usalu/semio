import importlib,json,numpy as np
from pathlib import Path
from shapely.geometry import Point,Polygon,LineString
from shapely.ops import unary_union
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
m=api.load_model()
base=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
r=json.loads((base/'🗑️generated/chimney-register-r40-results.json').read_text())
near={}
for storey in ['ground','first']:
 nearest={}
 for label,row in [('A',r['candidates'][0]),('B',r['candidates'][2])]:
  disk=Point(*row['cylinder'][:2]).buffer(.2,quad_segs=96)
  paths=[]
  for e in m['elements']:
   if e['class']!='IfcWall' or e['storey']!=storey:continue
   p=e['geometry'].get('path',[])
   if not p or any(isinstance(v,dict) for v in p):continue
   d=disk.distance(LineString(p).buffer(e['attributes']['thickness']['value']/2,cap_style='flat'))
   paths.append((d,e['tag']))
  nearest[label]=sorted(paths)[:3]
 near[storey]=nearest
for tag in ['W-G-ROOM-10','W-G-SERVICE','W-PARENT-BATH-S','W-SUITE-ENTRY']:
 e=next(e for e in m['elements'] if e['tag']==tag);g=api.path_wall_geometry(m,e)
 z=1.5 if e['storey']=='ground' else 4.5
 wall=unary_union([Polygon(p['outline']) for p in g['parts'] if p['z0']<=z<=p['z1']])
 near[tag]={label:Point(*row['cylinder'][:2]).buffer(.2,quad_segs=96).distance(wall) for label,row in [('A',r['candidates'][0]),('B',r['candidates'][2])]}
for tag in ['GUARD-ST-GF','GUARD-ST-FR']:
 print(tag,any(e['tag']==tag for e in m['elements']))
near['radial_envelope']={label:{'radius_to_axis':float(np.linalg.norm(np.array(row['cylinder'][:2])-np.array([6.407144828882639,9.180220372262651]))),'tube_to_finished_guard_gap':float(np.linalg.norm(np.array(row['cylinder'][:2])-np.array([6.407144828882639,9.180220372262651]))-.970-.2)} for label,row in [('A',r['candidates'][0]),('B',r['candidates'][2])]}
(base/'🗑️generated/chimney-wall-adjacency-r40.json').write_text(json.dumps(near,indent=2))
print(json.dumps(near,indent=2))

for tag in ['GUARD-ST-GF','GUARD-ST-FR']:
 g=api.solid_geometry(tag,m);foot=unary_union([Polygon(api.part_footprint(p)) for p in g['parts']])
 near[tag]={label:{'distance':Point(*row['cylinder'][:2]).buffer(.2,quad_segs=96).distance(foot),'overlap_area':Point(*row['cylinder'][:2]).buffer(.2,quad_segs=96).intersection(foot).area} for label,row in [('A',r['candidates'][0]),('B',r['candidates'][2])]}
(base/'🗑️generated/chimney-wall-adjacency-r40.json').write_text(json.dumps(near,indent=2))
print(json.dumps({k:v for k,v in near.items() if k.startswith('GUARD')},indent=2))
