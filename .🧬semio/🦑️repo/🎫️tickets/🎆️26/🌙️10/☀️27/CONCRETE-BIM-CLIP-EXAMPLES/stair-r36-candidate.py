import importlib,json,copy,traceback
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
source=api.load_model();delta=json.loads(Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/stair-r36-delta.json").read_text())
patch={e['tag']:e for e in delta['changed']}
source['elements']=[patch.get(e['tag'],e) for e in source['elements'] if e['tag'] not in delta['remove']]+delta['add']
source['materials']+=delta['materials'];source['types']+=delta['types']
t=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪜️stairs');importlib.reload(t)
original=api.load_model;api.load_model=lambda:copy.deepcopy(source)
for name in [n for n in dir(t) if n.startswith('test_')]:
 try:getattr(t,name)();print(name,'PASS')
 except Exception:traceback.print_exc()
api.load_model=original
