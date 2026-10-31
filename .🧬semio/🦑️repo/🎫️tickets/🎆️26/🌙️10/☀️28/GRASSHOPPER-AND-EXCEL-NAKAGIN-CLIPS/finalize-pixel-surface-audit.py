"""🧪 Validate the independent oracle, then retain final exact-source attribution endpoints."""
import importlib.util
from pathlib import Path
import sys
import unittest

TICKET=Path(__file__).parent
sys.stdout.reconfigure(encoding='utf8',errors='backslashreplace')
def load(name,path):
    """📦 Load a ticket-local input without importing any production renderer."""
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    sys.modules[name]=module
    spec.loader.exec_module(module)
    return module

tests=load('pixel_ray_portable_tests',TICKET/'test-pixel-nearest-surface.py')
result=unittest.TextTestRunner(stream=sys.stdout,verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(tests))
if not result.wasSuccessful(): raise SystemExit(1)
load('final_pixel_audit',TICKET/'run-pixel-surface-audit.py')
