"""🌳 Run staffage source guards without the rendering runtime."""

import importlib.util
from pathlib import Path
import sys
import types
import unittest

root = Path.cwd()
visuals = root / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals"
name = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️"
module = types.ModuleType(name)
module.__file__ = str(visuals / "🐍️.py")
sys.modules[name] = module
spec = importlib.util.spec_from_file_location("staffage_guard", visuals / "🧪️tests/🐍️staffage_only.py")
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)
tests = (
    "test_rule_a_flags_primitive_built_objects_and_passes_staffage_built_ones",
    "test_rule_a_no_clip_builds_a_physical_object_from_primitives",
    "test_rule_a_allowlist_has_a_reason_and_no_stale_entries",
    "test_physical_pictograms_are_found_the_way_the_fixture_says",
    "test_no_clip_renders_physical_pictograms",
)
suite = unittest.TestSuite(guard.StaffageOnlyTest(name) for name in tests)
result = unittest.TextTestRunner(verbosity=2).run(suite)
sys.exit(not result.wasSuccessful())
