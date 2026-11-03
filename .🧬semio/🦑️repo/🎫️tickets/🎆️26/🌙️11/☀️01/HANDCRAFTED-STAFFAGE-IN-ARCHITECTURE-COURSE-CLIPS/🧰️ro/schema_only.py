import importlib.util, sys, unittest
from pathlib import Path
p = Path(sys.argv[1]).resolve()
s = importlib.util.spec_from_file_location("t", p); m = importlib.util.module_from_spec(s); sys.modules["t"] = m; s.loader.exec_module(m)
r = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(m.SchemaFirstStudy))
