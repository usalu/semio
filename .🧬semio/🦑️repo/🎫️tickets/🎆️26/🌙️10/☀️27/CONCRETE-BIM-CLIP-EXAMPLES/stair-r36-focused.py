import importlib,traceback,time,json,copy
from pathlib import Path
ledger=[];progress=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/stair-focused-progress.json")
def mark(name,status,elapsed=0):
 ledger.append({"name":name,"status":status,"seconds":elapsed});progress.write_text(json.dumps(ledger))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
mark('actual447-geometry','running')
source=api.load_model();start=time.monotonic();original_bg=api.building_geometry;frozen_geometry=original_bg(source);mark('actual447-geometry','PASS',round(time.monotonic()-start,3))
api.building_geometry=lambda model=None:copy.deepcopy(frozen_geometry) if model is None or model==source else original_bg(model)
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'
for module_name,names in [('🪜️stairs',None),('🕳️cellar',None),('🛁️details',['test_stair_guards_follow_flights_with_connected_tube_and_open_arrival']),('🏛️suite',['test_upper_stair_arrival_is_supported_and_clear_of_its_guards','test_helical_tubes_have_connected_physical_sections_and_nominal_volume']),('🏛️remodel',['test_landscape_aperture_is_not_covered_by_curved_screen'])]:
 t=importlib.import_module(prefix+module_name);importlib.reload(t)
 for name in names or [n for n in dir(t) if n.startswith('test_')]:
  started=time.monotonic();mark(module_name+name,"running")
  try:getattr(t,name)();print(module_name,name,'PASS',round(time.monotonic()-started,2));mark(module_name+name,'PASS',round(time.monotonic()-started,3))
  except Exception:traceback.print_exc();mark(module_name+name,"FAIL",round(time.monotonic()-started,3))
