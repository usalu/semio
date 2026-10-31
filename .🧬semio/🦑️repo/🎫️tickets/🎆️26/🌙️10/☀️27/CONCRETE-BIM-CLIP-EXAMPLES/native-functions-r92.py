"""🧪 Run selected canonical native standalone contracts without the unrelated suite."""
import sys,importlib,inspect,unittest
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
suite=unittest.TestSuite()
for name in sys.argv[1:]:
 module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'+name)
 suite.addTests(unittest.FunctionTestCase(f) for key,f in inspect.getmembers(module,inspect.isfunction) if key.startswith('test_') and f.__module__==module.__name__)
result=unittest.TextTestRunner(verbosity=2).run(suite)
raise SystemExit(not result.wasSuccessful())
