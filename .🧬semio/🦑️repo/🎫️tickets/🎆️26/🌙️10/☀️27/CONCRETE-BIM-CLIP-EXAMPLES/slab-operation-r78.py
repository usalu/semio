import importlib,json,time,unittest
from pathlib import Path
from shapely.geometry import mapping
from shapely.geometry.base import BaseGeometry
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️partitions')
out=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated');registry={};refs=[];ordinal=0;slab=None
registryfile=out/'slab-operation-registry-r78.jsonl';registryfile.write_text('')
def register(shape):
 key=id(shape)
 if key not in registry:
  registry[key]=len(refs);refs.append(shape)
  with registryfile.open('a',encoding='utf8') as f:f.write(json.dumps({'id':registry[key],'geometry':mapping(shape)})+'\n')
 return registry[key]
def mark(op,operands):
 global ordinal
 ids=[register(p) for p in operands];ordinal+=1
 (out/'slab-operation-marker-r78.json').write_text(json.dumps({'op':op,'operands':ids,'ordinal':ordinal,'slab':slab}),encoding='utf8')
originals={name:getattr(BaseGeometry,name) for name in ('intersection','difference','covers')}
def wrapper(name,original):
 def invoke(left,right,*args,**kwargs):
  mark(name,[left,right]);return original(left,right,*args,**kwargs)
 return invoke
for name,original in originals.items():setattr(BaseGeometry,name,wrapper(name,original))
original_union=module.unary_union
def union(values,*args,**kwargs):
 values=list(values);mark('unary_union',values);return original_union(values,*args,**kwargs)
module.unary_union=union
original_slab=module.api._slab_geometry
def geometry(model,source):
 global slab
 slab=source['tag'];return original_slab(model,source)
module.api._slab_geometry=geometry
started=time.monotonic()
try:
 result=unittest.TextTestRunner().run(unittest.defaultTestLoader.loadTestsFromName('SlabPartitionContract.test_source_region_surfaces_and_sleeves_match_independent_geos',module))
 record={'seconds':time.monotonic()-started,'operations':ordinal,'geometries':len(refs),'tests':result.testsRun,'failures':[(str(t),msg) for t,msg in result.failures],'errors':[(str(t),msg) for t,msg in result.errors]}
 (out/'slab-operation-result-r78.json').write_text(json.dumps(record));print(json.dumps(record))
finally:
 for name,original in originals.items():setattr(BaseGeometry,name,original)
 module.unary_union=original_union;module.api._slab_geometry=original_slab
