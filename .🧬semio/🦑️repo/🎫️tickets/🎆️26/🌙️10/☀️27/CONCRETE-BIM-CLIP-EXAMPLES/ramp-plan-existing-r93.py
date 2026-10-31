"""📋 Verify existing arrival and visibility contracts after the ramp projection integration."""
import importlib,inspect,json,sys,time,unittest
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.'
visibility=importlib.import_module(prefix+'test_visibility');arrivals=importlib.import_module(prefix+'test_ramp_arrivals');suite=unittest.defaultTestLoader.loadTestsFromModule(visibility)
for name,fn in inspect.getmembers(arrivals,inspect.isfunction):
    if name.startswith('test_'):suite.addTest(unittest.FunctionTestCase(fn))
assert suite.countTestCases()==5;start=time.monotonic();result=unittest.TextTestRunner(verbosity=2).run(suite)
(Path(__file__).parent/'🗑️generated/ramp-plan-existing-r93.json').write_text(json.dumps({'pass':result.wasSuccessful(),'tests':result.testsRun,'seconds':time.monotonic()-start,'errors':[str(row) for row in result.failures+result.errors]},indent=2),encoding='utf8');raise SystemExit(not result.wasSuccessful())
