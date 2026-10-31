"""🚶 Audit all fully rebuilt unpublished timber against the unchanged full-body fixture."""
import importlib,json,sys,hashlib,runpy,time,traceback
from pathlib import Path
from shapely.geometry import Polygon,LineString,box
from shapely.ops import unary_union
from shapely.affinity import rotate
R=Path.cwd();P=Path(__file__).parent;O=P/'🗑️generated/native-r95-hall';sys.path.insert(0,str(R));A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');started=time.monotonic()
candidate=json.loads((O/'skirting-candidate.json').read_text(encoding='utf8'));cache=json.loads((O/'skirting-geometry.json').read_text(encoding='utf8'));assert candidate['source_stable'] and cache['snapshot']==candidate['before'] and cache['model_canonical_sha256']==candidate['model_canonical_sha256']
diagnostic='--diagnostic-snapshot' in sys.argv
if not diagnostic:
 for name,value in candidate['after'].items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==value,('changed-input',name)
m=runpy.run_path(str(P/'native-hall-bind-r95.py'))['prepare'](A,A.load_model(),candidate);g=cache['geometry'];g['details'].extend(A._solid_geometry(m,obj) for obj in candidate['elements'])
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚶️hall');helpers=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛁️details');case=T.CASE
pose_override=float(sys.argv[sys.argv.index('--pose-angle')+1]) if '--pose-angle' in sys.argv else None
if pose_override is not None:
 import copy
 case=copy.deepcopy(case);case['operated'][0]['angle_degrees']=pose_override
old_load,old_geometry=A.load_model,A.building_geometry
A.load_model=lambda:m;A.building_geometry=lambda model=None:g
fixture_error=None
try:
 if pose_override is None:T.test_hall_body_route_retains_all_native_parts_and_actual_laundry_leaf()
except Exception:fixture_error=traceback.format_exc()
finally:A.load_model,A.building_geometry=old_load,old_geometry
inventory=[]
for family in ('pilotis','slabs','walls','fillings','screens','stairs','details','furnishings','beams','foundations','railings','services'):inventory+=g[family]
inventory+=[g['curtain_wall'],*g['ramp']['ramps']];obstacles=[];floors=[];leaf=[];fixed=[];fixed_rows=[];domain=box(*case['domain']);pose=case['operated'][0];lo,hi=case['band']
for obj in inventory:
 for part in obj['parts']+obj.get('child_parts',[]):
  if abs(part['z1'])<1e-9:floors.append(helpers._slice([part],part['z1']))
  if part['z1']<=lo or part['z0']>=hi:continue
  shape=helpers._band(part,lo,hi)
  if obj['tag']==pose['tag'] and 'frame' not in part.get('role','').lower():shape=rotate(shape,pose['angle_degrees'],origin=pose['hinge']);leaf.append(shape)
  else:fixed.append(shape);fixed_rows.append((obj['tag'],part.get('role'),shape))
  if shape.intersects(domain):obstacles.append((obj['tag'],shape))
leaf_union=unary_union(leaf)
pose_hits=[{'tag':tag,'role':role,'area':shape.intersection(leaf_union).area,'bounds':list(shape.bounds)} for tag,role,shape in fixed_rows if shape.intersection(leaf_union).area>1e-10]
(O/'hall-leaf-pose-diagnostic.json').write_text(json.dumps({'hits':pose_hits,'area':leaf_union.intersection(unary_union(fixed)).area},indent=2),encoding='utf8')
if not diagnostic:assert leaf and leaf_union.intersection(unary_union(fixed)).area<case['area_tolerance']
occupied=unary_union([shape for _,shape in obstacles]);support=unary_union(floors);rows=[]
for row in case['cases']:
 body=LineString(row['points']).buffer(row['diameter']/2+case['radial_margin'],quad_segs=128);hits=[{'tag':tag,'area':shape.intersection(body).area} for tag,shape in obstacles if shape.intersection(body).area>1e-10];missing=body.difference(support).area;rows.append({'id':row['id'],'diameter':row['diameter'],'hits':hits,'floor_missing_area':missing,'success':not hits and missing<case['area_tolerance']})
connectivity=[]
for diameter in (.77,.8,.85):
 free=domain.difference(occupied.buffer(diameter/2+.001,quad_segs=128));regions=list(free.geoms) if hasattr(free,'geoms') else [free]
 for finish_y in (17.7,19.4):
  start=LineString([[7,12.2],[8.15,12.2]]);end=LineString([[7,finish_y],[8.15,finish_y]]);connected=any(poly.intersects(start) and poly.intersects(end) for poly in regions);connectivity.append({'diameter':diameter,'end_y':finish_y,'connected':connected})
changed=[name for name,value in candidate['after'].items() if hashlib.sha256((R/name).read_bytes()).hexdigest()!=value]
report={'diagnostic_snapshot':diagnostic,'candidate_only':True,'publishable':False,'source_inputs':candidate['after'],'inputs_stable':not changed,'changed_inputs':changed,'fixture_error':fixture_error,'unchanged_required_fixture_cases':rows,'connectivity':connectivity,'candidate_operated_angle':pose_override,'original_operated_angle':T.CASE['operated'][0]['angle_degrees'],'operated_leaf_hits':pose_hits,'board_products':len(candidate['elements']),'board_cells':sum(len(obj['geometry']['parts']) for obj in candidate['elements']),'operated_leaf_fixed_overlap':unary_union(leaf).intersection(unary_union(fixed)).area,'seconds':time.monotonic()-started}
(O/('hall-route-pose-'+str(pose_override)+'.json' if pose_override is not None else 'hall-route-diagnostic.json' if diagnostic else 'hall-route.json')).write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k not in ('source_inputs','fixture_error')}),flush=True)
if fixture_error:print('[DEBUG] Existing fixture outcome '+fixture_error,flush=True)
assert all(row['success'] for row in rows) and (diagnostic or not changed)
