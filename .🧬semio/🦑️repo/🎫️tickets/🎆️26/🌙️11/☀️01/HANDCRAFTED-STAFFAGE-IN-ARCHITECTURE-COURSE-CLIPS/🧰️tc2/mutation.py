import importlib
import os
import sys
import unittest

sys.path.insert(0, os.getcwd())

root = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage."
S = importlib.import_module(root + "🐍️")
tests = importlib.import_module(root + "🧪️tests.🐍️")
S._Tree.stop = lambda self, grace=3.0: self.process.kill()
S._Tree.close = lambda self: None
suite = unittest.defaultTestLoader.loadTestsFromName("Cancellation.test_a_grandchild_of_the_command_dies_with_it", tests)
result = unittest.TextTestRunner(verbosity=1).run(suite)
sys.exit(0 if result.wasSuccessful() else 3)
