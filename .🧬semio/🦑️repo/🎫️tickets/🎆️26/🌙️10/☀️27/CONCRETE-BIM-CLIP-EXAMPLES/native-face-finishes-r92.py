"""🎨 Run focused material-face treatment contracts through the native repository environment."""
import importlib,sys,unittest
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
if (Path(__file__).parent/'native-r92-cancel').exists():raise SystemExit('Native validation cancelled')
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🧪️tests.🐍️')
suite=unittest.TestSuite(unittest.FunctionTestCase(getattr(module,name)) for name in sorted(dir(module)) if name.startswith('test_'))
result=unittest.TextTestRunner(verbosity=2).run(suite)
raise SystemExit(0 if result.wasSuccessful() else 1)
