import importlib, json, pathlib, sys, unittest
sys.path.insert(0,str(pathlib.Path.cwd()))
m=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🧪️snapshot.🐍️')
s=unittest.TestSuite(unittest.FunctionTestCase(getattr(m,n)) for n in dir(m) if n.startswith('test_'))
r=unittest.TextTestRunner(verbosity=2).run(s)
sys.exit(not r.wasSuccessful())
