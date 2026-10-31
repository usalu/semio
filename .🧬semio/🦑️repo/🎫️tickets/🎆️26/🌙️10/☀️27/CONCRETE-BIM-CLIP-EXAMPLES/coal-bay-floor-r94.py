import importlib,sys,pathlib,unittest
sys.path.insert(0,str(pathlib.Path.cwd()))
m=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪵️coal-bay')
r=unittest.TextTestRunner(verbosity=2).run(unittest.TestSuite([unittest.FunctionTestCase(m.test_coal_bay_indoor_finished_floor_supports_all_cupboard_feet_and_front)]))
sys.exit(not r.wasSuccessful())
