"""⏱️ Observe the actual facade oracle without changing its operators or tolerances."""
import hashlib,importlib,inspect,json,time,traceback
from pathlib import Path
import shapely.ops
from shapely.geometry.base import BaseGeometry
ticket=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES");gen=ticket/'🗑️generated';clip=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');source=clip/'🧱️model/🔣️.json'
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️facade')
started=time.monotonic();index=0;captured=False;events=[]
original_union=shapely.ops.unary_union;originals={name:getattr(BaseGeometry,name) for name in ('difference','symmetric_difference','intersection')}
def context():
 frame=inspect.currentframe().f_back
 while frame:
  if 'raw' in frame.f_locals and 'owned' in frame.f_locals and 'primary' in frame.f_locals:
   return frame.f_locals
  frame=frame.f_back
 return {}
def operation(name,args,call):
 global index,captured
 index+=1;number=index;local=context();z=local.get('z');begin=time.monotonic()
 if not captured and local:
  snapshot={k:local[k] for k in ('raw','owned','primary')};snapshot['source']=hashlib.sha256(source.read_bytes()).hexdigest()
  (gen/'facade-profile-parts-r80.json').write_text(json.dumps(snapshot),encoding='utf8');captured=True
 count=len(args[0]) if name=='unary_union' else len(args)
 marker={'debug':'[DEBUG] compact facade oracle operation','index':number,'operation':name,'phase':'start','elapsed':begin-started,'z':z,'count':count}
 (gen/'facade-profile-marker-r80.json').write_text(json.dumps(marker),encoding='utf8')
 result=call();seconds=time.monotonic()-begin
 marker.update(phase='end',seconds=seconds);events.append(marker)
 (gen/'facade-profile-marker-r80.json').write_text(json.dumps(marker),encoding='utf8')
 (gen/'facade-profile-events-r80.json').write_text(json.dumps(events),encoding='utf8')
 return result
def union(geometries,*args,**kwargs):
 geometries=list(geometries)
 return operation('unary_union',(geometries,),lambda:original_union(geometries,*args,**kwargs))
def binary(name):
 def wrapped(self,other,*args,**kwargs):return operation(name,(self,other),lambda:originals[name](self,other,*args,**kwargs))
 return wrapped
shapely.ops.unary_union=union
for name in originals:setattr(BaseGeometry,name,binary(name))
try:
 module.test_auxiliary_union_matches_geos_difference_of_primary_structure();record={'pass':True}
except Exception:record={'pass':False,'traceback':traceback.format_exc()}
finally:
 shapely.ops.unary_union=original_union
 for name,fn in originals.items():setattr(BaseGeometry,name,fn)
record.update(seconds=time.monotonic()-started,operations=index,source=hashlib.sha256(source.read_bytes()).hexdigest(),scope='Actual method unchanged operators/tolerances; instrumentation timings include compact marker writes')
(gen/'facade-profile-result-r80.json').write_text(json.dumps(record),encoding='utf8');print(record)
