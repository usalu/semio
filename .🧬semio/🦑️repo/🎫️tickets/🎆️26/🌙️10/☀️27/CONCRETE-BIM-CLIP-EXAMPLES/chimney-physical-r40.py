import importlib
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
t=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️chimneys')
import json,time,copy
from pathlib import Path
base=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
progress=base/'🗑️generated/chimney-physical-progress-r40.json'
progress.write_text(json.dumps({'phase':'building_geometry','state':'running'}))
api._source.cache_clear();m=api.load_model();started=time.monotonic();original_bg=api.building_geometry;frozen_geometry=original_bg(m)
api.building_geometry=lambda model=None:copy.deepcopy(frozen_geometry) if model is None or model==m else original_bg(model)
progress.write_text(json.dumps({'phase':'building_geometry','state':'PASS','seconds':time.monotonic()-started,'source_count':len(m['elements'])}))
print('Actual448product building geometry PASS',time.monotonic()-started)

import traceback
results=[]
for name in [n for n in dir(t) if n.startswith('test_')]:
 started=time.monotonic();progress.write_text(json.dumps({'phase':name,'state':'running'}))
 try:getattr(t,name)();status='PASS';error=None
 except Exception:status='FAIL';error=traceback.format_exc()
 results.append({'name':name,'status':status,'error':error,'seconds':time.monotonic()-started});progress.write_text(json.dumps(results));print(json.dumps(results[-1]))

(base/'🗑️generated'/'chimney-contracts-r40.json').write_text(json.dumps(results,indent=2))
regressions=[]
for suffix in ['🏠ground','🪜️stairs']:
 mod=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'+suffix);importlib.reload(mod)
 for name in [n for n in dir(mod) if n.startswith('test_')]:
  started=time.monotonic();progress.write_text(json.dumps({'phase':suffix+name,'state':'running'}))
  try:getattr(mod,name)();status='PASS';error=None
  except Exception:status='FAIL';error=traceback.format_exc()
  regressions.append({'module':suffix,'name':name,'status':status,'error':error,'seconds':time.monotonic()-started});progress.write_text(json.dumps(regressions));print(json.dumps(regressions[-1]))
(base/'🗑️generated/chimney-regressions-r40.json').write_text(json.dumps(regressions,indent=2))
