"""🧪 Run only bounded neutral renderer/revision gates through the registered locked Python selector."""
import sys,json,time,unittest,hashlib,importlib,importlib.metadata,os
from pathlib import Path
root=Path.cwd();ticket=Path(__file__).parent;sys.path.insert(0,str(root));started=time.perf_counter()
os.environ['SEMIO_TEACHING_OUTPUT_ROOT']=str(ticket/'🗑️generated/native-r92/terrain')
modules=['🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🗂️revisions','🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧪️assembly.🐍️','🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧪️ground.🐍️','🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.📏️geometry','🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🧹️fixtures.🐍️','🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🧱️layers.🧪️tests.🐍️']
versions={name:importlib.metadata.version(name) for name in ('manim','numpy','scipy','shapely','jsonschema')}
lock=root/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock'
metadata={'executable':sys.executable,'python':sys.version,'pid':os.getpid(),'versions':versions,'lock_sha256':hashlib.sha256(lock.read_bytes()).hexdigest(),'scope':'focused revision/assembly/terrain/source-snapshot only; no full building_geometry or lifecycle','normal_registered_profile_mib':2048}
(ticket/'🗑️generated/native-r92/renderer-focused-r94-start.json').write_text(json.dumps(metadata,ensure_ascii=True,indent=2),encoding='utf-8')
print('[DEBUG] focused provenance '+json.dumps(metadata),flush=True)
suite=unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromName(name) for name in modules)
snapshot_contract=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🧪️snapshot.🐍️')
for name in sorted(dir(snapshot_contract)):
    if name.startswith('test_') and callable(getattr(snapshot_contract,name)):suite.addTest(unittest.FunctionTestCase(getattr(snapshot_contract,name)))
private=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️surface-producers')
suite.addTest(unittest.FunctionTestCase(private.test_private_slab_and_wall_cutter_surface_replacement_invalidates_cached_values))
quantity_contract=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📦️disjoint')
for name in sorted(dir(quantity_contract)):
    if name.startswith('test_') and callable(getattr(quantity_contract,name)):suite.addTest(unittest.FunctionTestCase(getattr(quantity_contract,name)))
result=unittest.TextTestRunner(verbosity=2).run(suite)
files={Path(module.__file__) for name,module in tuple(sys.modules.items()) if name.startswith('🎓️teaching.') and getattr(module,'__file__',None)}
model_api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
report={**metadata,'loaded_model_source_sha256':hashlib.sha256(model_api._source().encode('utf-8')).hexdigest(),'success':result.wasSuccessful(),'tests':result.testsRun,'failures':[(str(t),s) for t,s in result.failures],'errors':[(str(t),s) for t,s in result.errors],'duration_seconds':time.perf_counter()-started,'source_hashes':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files) if p.is_relative_to(root) and p.is_file()}}
out=ticket/'🗑️generated/native-r92/renderer-focused-r94.json';out.write_text(json.dumps(report,ensure_ascii=True,indent=2),encoding='utf-8');print('[DEBUG] focused result '+str(out),flush=True)
raise SystemExit(0 if result.wasSuccessful() else 1)
