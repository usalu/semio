"""🧿️ Run the bounded native interface profile fixture."""
import sys,unittest
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));suite=unittest.defaultTestLoader.loadTestsFromName("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧿️joints");r=unittest.TextTestRunner(verbosity=2).run(suite);sys.exit(not r.wasSuccessful())
