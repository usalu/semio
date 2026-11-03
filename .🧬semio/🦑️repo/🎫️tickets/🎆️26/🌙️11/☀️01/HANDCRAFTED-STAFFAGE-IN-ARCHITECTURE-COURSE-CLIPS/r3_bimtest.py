import sys, unittest, importlib.util
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py")
spec = importlib.util.spec_from_file_location("bim_tests", p)
m = importlib.util.module_from_spec(spec); sys.modules["bim_tests"] = m; spec.loader.exec_module(m)
suite = unittest.TestSuite()
for name in sys.argv[1:]:
    suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(getattr(m, name)))
unittest.TextTestRunner(verbosity=1).run(suite)
