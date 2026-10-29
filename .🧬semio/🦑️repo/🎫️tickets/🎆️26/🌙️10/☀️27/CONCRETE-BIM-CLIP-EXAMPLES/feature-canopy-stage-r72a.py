"""⏱️ Locate the exact unmodified canopy-test stage without changing production or tolerances."""
import importlib,json,time
from pathlib import Path
from unittest.mock import patch
import shapely.ops
out=Path(__file__).parent/'🗑️generated/features-r72a-canopy-stage.json';events=[]
def emit(stage,**detail):
    events.append({'stage':stage,'seconds':time.monotonic()-started,**detail});out.write_text(json.dumps(events))
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel')
original_geometry=module.model.building_geometry;original_union=shapely.ops.unary_union;count=0;started=time.monotonic()
def geometry(*args,**kwargs):
    emit('building_geometry start');value=original_geometry(*args,**kwargs);emit('building_geometry done');return value
def union(shapes,*args,**kwargs):
    global count
    count+=1;index=count;emit('union start',index=index,parts=len(shapes));value=original_union(shapes,*args,**kwargs);emit('union done',index=index);return value
with patch.object(module.model,'building_geometry',geometry),patch.object(shapely.ops,'unary_union',union):module.test_covered_garden_canopy_preserves_open_terrace()
emit('test done')
