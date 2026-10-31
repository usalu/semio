"""🪜 Reconstruct original wearing stocks and partition them around source structural owners once."""
import importlib,sys,json,copy,math,hashlib
from pathlib import Path
from fractions import Fraction as Q
sys.path.insert(0,str(Path.cwd()))
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");o=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces")
m=a.load_model();rows=[];q=a._source_fraction
for parent,tag,depth,wear,material in [('RM-1','FIN-RAMP-INTERIOR',Q('.008'),Q('.005'),'MAT-RAMP-RESILIENT'),('RM-2','FIN-RAMP-EXTERIOR',Q('.06'),Q('.04'),'MAT-PAVING')]:
 ramp=a._find(m,parent);obj=a._find(m,tag);raw=[];owners=[]
 for flight in [e for e in m['elements'] if e['class']=='IfcRampFlight' and e['geometry']['ramp']==parent]:
  for owner in flight['geometry'].get('interface_owners',[]):
   if owner not in owners:owners.append(owner)
  ref=a._ramp_reference_stock(m,flight);g=flight['geometry'];run=math.dist(*g['path']);rise=(a._z(m,g['to_level'])-a._z(m,g['from_level']))*(g['portion'][1]-g['portion'][0]);factor=q(math.hypot(1.,a._round(rise/run))) if g['finish_direction']=='normal' else Q(1)
  for bottom,top,role,mat in [(depth,wear,'bedding','MAT-MORTAR-PUTTY' if parent=='RM-1' else 'MAT-SAND'),(wear,Q(0),'wearing',material)]:
   source=copy.deepcopy(flight);source['geometry']['structural_offsets']=copy.deepcopy(g['walking_offsets']);source['geometry']['finish_thickness']['value']=top;source['geometry']['thickness']['value']=(bottom-top)*factor;p=a._ramp_reference_stock(m,source);p.update(material=mat,role=role);raw.append(p)
 for landing in ramp['geometry']['landings']:
  if landing['ownership']!='ramp_component' or landing['owner']!=parent:continue
  z=q(a._z(m,ramp['geometry']['from_level']))+(q(a._z(m,ramp['geometry']['to_level']))-q(a._z(m,ramp['geometry']['from_level'])))*q(landing['portion'])
  for bottom,top,role,mat in [(depth,wear,'bedding','MAT-MORTAR-PUTTY' if parent=='RM-1' else 'MAT-SAND'),(wear,Q(0),'wearing',material)]:raw.append({'outline':[[q(v) for v in p] for p in landing['walking_outline']],'z0':z-bottom,'z1':z-top,'material':mat,'role':role})
 owners+=([key for key in (['25','26','28','PA','PB'] if parent=='RM-1' else ['25-FIRST','26-FIRST','28-FIRST','35','PA','PB']) if key not in owners])
 cutters=[part for owner in owners for part in a._interface_reference_parts(m,owner)]
 if parent=='RM-1':cutters+=[cell for p in o.occupied(m,'W-G-CHAUFFEUR-RETURN') for cell in a._native_material_cells(p)]
 expected=sum(o.common_union_volume(p,cutters) for p in raw);out,removed=a._subtract_native_material(raw,cutters);base=Q(0);local=[]
 for p in out:
  a._polyhedron_part(a.part_vertices(p),a.part_faces(p))
  p={k:copy.deepcopy(v) for k,v in p.items() if k in ('mesh','outline','top_outline','holes','z0','z1','material','role')}
  if 'mesh' in p:p['mesh']['vertices']=[[x,y,float(q(z)-base)] for x,y,z in p['mesh']['vertices']]
  else:p['z0']=float(q(p['z0'])-base);p['z1']=float(q(p['z1'])-base)
  if 'mesh' in p:p={k:p[k] for k in ('mesh','material','role')}
  local.append(p)
 assert abs(removed-expected)<1e-9,(tag,removed,expected)
 rows.append({'tag':tag,'parts':local,'removed':removed,'independent_removed':expected,'owners':owners,'base_level':0});print('[DEBUG] '+json.dumps({k:v for k,v in rows[-1].items() if k!='parts'}),flush=True)
(Path(__file__).parent/'🗑️generated/ramp-finish-joints-r92.json').write_text(json.dumps({'source':hashlib.sha256(Path(a.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),'rows':rows},ensure_ascii=False,indent=2),encoding='utf8')
