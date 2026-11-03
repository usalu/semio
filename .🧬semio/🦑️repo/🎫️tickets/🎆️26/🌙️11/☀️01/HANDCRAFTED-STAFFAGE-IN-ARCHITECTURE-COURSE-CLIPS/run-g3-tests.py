import importlib.util
import sys
import unittest
from pathlib import Path

suite = unittest.TestSuite()
for arg in sys.argv[1:]:
    path = Path(arg).resolve()
    spec = importlib.util.spec_from_file_location("g3_" + str(abs(hash(arg))), path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
result = unittest.TextTestRunner(verbosity=1).run(suite)
sys.exit(0 if result.wasSuccessful() else 1)
