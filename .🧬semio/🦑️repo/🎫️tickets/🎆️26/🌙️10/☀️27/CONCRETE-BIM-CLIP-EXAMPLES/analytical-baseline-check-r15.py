"""📐 Run independent analytical baseline classes against the current authored source."""
import importlib
import unittest

module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
suite = unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(cls) for cls in (module.BimAnalysisContract,module.BimTributaryContract))
result = unittest.TextTestRunner(verbosity=1).run(suite)
if not result.wasSuccessful():
    raise AssertionError("The independent analytical baseline failed")
