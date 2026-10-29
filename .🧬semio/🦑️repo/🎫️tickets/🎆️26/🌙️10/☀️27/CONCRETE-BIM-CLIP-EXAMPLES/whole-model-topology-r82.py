import importlib,json,time,hashlib
from pathlib import Path
from shapely.geometry import Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
base=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');out=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated');raw=(base/'🧱️model/🔣️.json').read_bytes();m=json.loads(raw);started=time.monotonic()
def hashes():return {str(p.relative_to(base)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (base/'🧱️model/🐍️.py',base/'🧱️model/🧮️analysis/🐍️.py',base/'🧱️model/🪟️curtain/🐍️.py')}
before=hashes();(out/'whole-model-topology-progress-r82.json').write_text(json.dumps({'phase':'building_geometry','source_raw':hashlib.sha256(raw).hexdigest()}));g=api.building_geometry(m);build_seconds=time.monotonic()-started
counts={};invalid=[];nonpositive=[];skipped=[];spaces=[]
from shapely.validation import explain_validity
for group,items in g.items():
 if not isinstance(items,list):continue
 for oi,obj in enumerate(items):
  if not isinstance(obj,dict):continue
  tag=obj.get('tag',str(oi))
  if group=='spaces':
   if obj.get('outline'):
    shape=Polygon(obj['outline']);spaces.append({'tag':tag,'valid':shape.is_valid,'area':shape.area})
   continue
  fields=[k for k in obj if k=='parts' or k.endswith('_parts')]
  for field in fields:
   for pi,p in enumerate(obj[field]):
    key=group+'/'+field;counts[key]=counts.get(key,0)+1
    if not p.get('outline'):
     skipped.append({'group':group,'tag':tag,'field':field,'part':pi,'reason':'No planar outline; mesh/section/path requires its own topology domain','keys':list(p)})
     continue
    shape=Polygon(p['outline'],p.get('holes',p.get('voids',[])));record={'group':group,'tag':tag,'field':field,'part':pi,'area':shape.area,'outline':p['outline'],'holes':p.get('holes',p.get('voids',[]))}
    if not shape.is_valid:invalid.append({**record,'reason':explain_validity(shape)})
    if shape.area<=0:nonpositive.append(record)
result={'source_raw_before':hashlib.sha256(raw).hexdigest(),'source_raw_after':hashlib.sha256((base/'🧱️model/🔣️.json').read_bytes()).hexdigest(),'model_snapshot_sha256':hashlib.sha256(json.dumps(m,sort_keys=True,separators=(',',':')).encode()).hexdigest(),'products':len(m['elements']),'producer_before':before,'producer_after':hashes(),'build_seconds':build_seconds,'total_seconds':time.monotonic()-started,'counts':counts,'invalid':invalid,'nonpositive':nonpositive,'skipped':skipped,'spaces':spaces,'geometry_keys':list(g)}
(out/'whole-model-topology-r82.json').write_text(json.dumps(result));print(json.dumps({k:v for k,v in result.items() if k not in ('invalid','nonpositive','skipped','spaces')}));print(json.dumps({'invalid_count':len(invalid),'nonpositive_count':len(nonpositive),'skipped_count':len(skipped),'space_count':len(spaces)}))

for group,obj in g.items():
 if not isinstance(obj,dict) or 'parts' not in obj:continue
 tag=obj.get('tag',group)
 for field in [k for k in obj if k=='parts' or k.endswith('_parts')]:
  for pi,p in enumerate(obj[field]):
   key=group+'/'+field;counts[key]=counts.get(key,0)+1
   if not p.get('outline'):
    skipped.append({'group':group,'tag':tag,'field':field,'part':pi,'reason':'No planar outline; mesh/section/path requires its own topology domain','keys':list(p)});continue
   shape=Polygon(p['outline'],p.get('holes',p.get('voids',[])));record={'group':group,'tag':tag,'field':field,'part':pi,'area':shape.area,'outline':p['outline'],'holes':p.get('holes',p.get('voids',[]))}
   if not shape.is_valid:invalid.append({**record,'reason':explain_validity(shape)})
   if shape.area<=0:nonpositive.append(record)
result.update(counts=counts,invalid=invalid,nonpositive=nonpositive,skipped=skipped,total_seconds=time.monotonic()-started,producer_after=hashes(),source_raw_after=hashlib.sha256((base/'🧱️model/🔣️.json').read_bytes()).hexdigest())
(out/'whole-model-topology-r82.json').write_text(json.dumps(result));print(json.dumps({'invalid_count':len(invalid),'nonpositive_count':len(nonpositive),'skipped_count':len(skipped),'counts':counts}))

import importlib,json
from shapely.geometry import Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();rows=[]
for e in m['elements']:
 if e['class'] not in ('IfcRamp','IfcRampFlight'):continue
 g=api.ramp_geometry(e['tag'],m)
 for field in ('parts','shared_parts','child_parts','full_parts'):
  for i,p in enumerate(g.get(field,[])):
   shape=Polygon(p['outline'],p.get('holes',[]));rows.append({'tag':e['tag'],'field':field,'part':i,'valid':shape.is_valid,'area':shape.area})
print(json.dumps({'views':len(rows),'invalid':[p for p in rows if not p['valid'] or p['area']<=0]}))
