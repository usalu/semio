"""🎨 Run the neutral native architecture/service covering contract."""
import importlib,sys,unittest
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
if (Path(__file__).parent/'native-r92-cancel').exists():raise SystemExit('Native validation cancelled')
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🎨️coverings')
result=unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(module.test_architectural_covering_dispatch_uses_source_domains_and_native_material))
raise SystemExit(0 if result.wasSuccessful() else 1)
