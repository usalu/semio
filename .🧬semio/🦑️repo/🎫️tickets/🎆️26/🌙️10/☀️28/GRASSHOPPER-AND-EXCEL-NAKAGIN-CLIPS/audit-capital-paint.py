"""🔍 Isolate physical ordering from the material-underpaint path in the permanent head control."""
import importlib.util
from pathlib import Path
import sys
import unittest
from unittest.mock import patch
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
source=root/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/👁️occlusion/🐍️.py'
spec=importlib.util.spec_from_file_location('capital_paint_probe',source)
module=importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
with patch.object(module.V,'_paint_ordered',lambda items,view:[item.draw(view) for item in items]):
    print('[DEBUG] Direct physical faces without compound underpaint',flush=True)
    result=unittest.TextTestRunner(verbosity=2).run(unittest.TestSuite([module.Occlusion('test_closed_capital_has_no_pixels_of_contained_frame')]))
if not result.wasSuccessful():
    sys.exit(1)
