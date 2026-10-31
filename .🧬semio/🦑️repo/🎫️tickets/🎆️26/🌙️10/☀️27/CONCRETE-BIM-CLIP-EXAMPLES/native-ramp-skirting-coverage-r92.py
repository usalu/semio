"""🪵 Inspect actual source wearing and guard faces before claiming inclined-board coverage."""
import json,sys,importlib,runpy
from pathlib import Path
from shapely.geometry import Polygon,LineString
from shapely.ops import unary_union
root=Path.cwd();base=Path(__file__).parent;sys.path.insert(0,str(root));api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');h=runpy.run_path(str(base/'native-inclined-skirting-r92.py'));g=json.loads((base/'🗑️generated/native-r92/skirting-geometry.json').read_text())['geometry'];flight=next(f for f in g['ramp']['flights'] if f['tag']=='RF-1A');plane=h['affine_plane']([[*flight['start'],flight['z0']],[*flight['end'],flight['z1']],[flight['start'][0]+1,flight['start'][1],flight['z0']]])
line=LineString([(8,14),(12,14)]);floor=next(x for x in g['details'] if x['tag']=='FIN-RAMP-INTERIOR');pieces=[]
for part in floor['parts']:
 if part.get('role')!='wearing':continue
 for face in api.part_faces(part):
  points=[api.part_vertices(part)[i] for i in face]
  try:p=h['affine_plane'](points)
  except ValueError:continue
  n,d=p
  if abs(h['height'](p,*flight['start'])-flight['z0'])<1e-9 and abs(h['height'](p,*flight['end'])-flight['z1'])<1e-9:pieces.append(Polygon([p[:2] for p in points]))
wearing=unary_union(pieces)
guard=next(x for x in g['railings'] if x['tag']=='GUARD-RM-1');cut=unary_union([h['section_xy'](api,p,plane,.045) for p in guard['parts']]);report={'flight':flight['tag'],'wearing_line':wearing.intersection(line).wkt,'guard_line':cut.intersection(line).wkt,'wearing_area':wearing.area,'wearing_bounds':wearing.bounds,'guard_bounds':cut.bounds};(base/'🗑️generated/native-r92/ramp-skirting-coverage.json').write_text(json.dumps(report,indent=2));print('[DEBUG] '+json.dumps(report))
