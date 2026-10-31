"""🛝️ Validate declarative original ramp finishes against actual materialized source evidence."""
import sys,importlib,json,copy,hashlib,time,threading,_thread,faulthandler,traceback
from pathlib import Path
from types import SimpleNamespace
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,MultiPoint
from shapely.ops import unary_union
from jsonschema import Draft202012Validator
ROOT=Path.cwd();BASE=Path(__file__).parent;sys.path.insert(0,str(ROOT));P='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.'
a=importlib.import_module(P+'🐍️');f=importlib.import_module(P+'🎨️finishes.🛝️ramps');paths=[Path(a.__file__),Path(f.__file__),Path(a.__file__).with_name('🔣️.json'),Path(a.__file__).parent/'🧬️schema/🔣️.schema.json']
def hashes():return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
stack=(BASE/'🗑️generated/native-r92/ramp-finish.stack.log').open('w',encoding='utf8');faulthandler.enable(stack)
stopped=threading.Event()
def observe():
    """⏳ Keep expensive native preparation observable and cooperatively cancellable."""
    while not stopped.wait(10):
        print('[DEBUG] original ramp finish preflight working',flush=True)
        frame=sys._current_frames().get(threading.main_thread().ident)
        if frame is not None:traceback.print_stack(frame,file=stack);stack.flush()
        if (BASE/'🗑️generated/native-r92/ramp-finish.cancel').exists():_thread.interrupt_main();return
threading.Thread(target=observe,daemon=True).start()
before=hashes();m=a.load_model();schema=json.loads(paths[-1].read_text(encoding='utf8'));rows=[];start=time.monotonic()
for parent,tag,wear,bed,material in [('RM-1','FIN-RAMP-INTERIOR',.005,.003,'MAT-RAMP-RESILIENT'),('RM-2','FIN-RAMP-EXTERIOR',.04,.02,'MAT-PAVING')]:
 obj=a._find(m,tag);old=a.solid_geometry(tag,m);owners=[]
 for flight in [e for e in m['elements'] if e['class']=='IfcRampFlight' and e['geometry']['ramp']==parent]:
  owners.extend(key for key in flight['geometry'].get('interface_owners',[]) if key not in owners)
 owners.extend(key for key in (['25','26','28','PA','PB'] if parent=='RM-1' else ['25-FIRST','26-FIRST','28-FIRST','35','PA','PB']) if key not in owners)
 spec={'ramp':parent,'layers':[{'role':'wearing','material':material,'depth':{'value':wear,'unit':'m','basis':'inferred' if parent=='RM-1' else 'sourced','source':'SAB2024-p139 interior approximately8mm;5+3mm allocation inferred; p192 exterior4cm pavers'}},{'role':'bedding','material':'MAT-MORTAR-PUTTY' if parent=='RM-1' else 'MAT-SAND','depth':{'value':bed,'unit':'m','basis':'inferred' if parent=='RM-1' else 'sourced','source':'SAB2024 interior8mm split inferred5+3mm; exterior2cm sand'}}],'interface_owners':owners,'wall_owners':['W-G-CHAUFFEUR-RETURN'] if parent=='RM-1' else []}
 candidate=copy.deepcopy(obj);candidate['geometry']={'kind':'ramp_finish','base_level':0,'ramp_finish':spec};candidate_model=copy.deepcopy(m);next(e for e in candidate_model['elements'] if e['tag']==tag)['geometry']=copy.deepcopy(candidate['geometry'])
 Draft202012Validator(schema).validate(candidate_model)
 print('[DEBUG] original ramp finish begin '+tag,flush=True)
 new=f.geometry(SimpleNamespace(**vars(a)),candidate_model,candidate);layers=[]
 for role in ('wearing','bedding'):
  op=[p for p in old['parts'] if p['role']==role];np_=[p for p in new['parts'] if p['role']==role]
  oldv=sum(ConvexHull(np.asarray(a.part_vertices(p),float)).volume for p in op);newv=sum(ConvexHull(np.asarray(a.part_vertices(p),float)).volume for p in np_)
  before_shape=unary_union([MultiPoint(np.asarray(a.part_vertices(p),float)[:,:2]).convex_hull for p in op]);after_shape=unary_union([MultiPoint(np.asarray(a.part_vertices(p),float)[:,:2]).convex_hull for p in np_]);delta=before_shape.symmetric_difference(after_shape).area
  assert abs(oldv-newv)<1e-10,(tag,role,oldv,newv)
  assert delta<1e-9,(tag,role,delta)
  layers.append({'role':role,'old_volume':oldv,'new_volume':newv,'coverage_difference':delta,'old_parts':len(op),'new_parts':len(np_)})
 for part in new['parts']:a._polyhedron_part(a.part_vertices(part),a.part_faces(part))
 rows.append({'tag':tag,'geometry':candidate['geometry'],'layers':layers,'parts':len(new['parts']),'elapsed':time.monotonic()-start});print('[DEBUG] original ramp finish PASS '+json.dumps(rows[-1]),flush=True)
after=hashes();report={'stable':before==after,'before':before,'after':after,'rows':rows};(BASE/'🗑️generated/native-r92/ramp-finish-preflight.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');stopped.set();assert report['stable']
                                                                              