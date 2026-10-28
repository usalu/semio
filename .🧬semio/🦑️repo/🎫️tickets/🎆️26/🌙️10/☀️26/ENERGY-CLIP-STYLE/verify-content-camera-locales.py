"""🌐 Verify actual fused native restore controls in both explicit locales."""
from pathlib import Path
import importlib
import os
import sys
import unittest

ROOT = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(Path(__file__).parent / "🗑️generated/content-camera/locales")
module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🐍️")
suite = unittest.TestSuite([module.LayeredBrowserOracleTest("test_native_fused_restore_chip_closes_video_on_release_in_each_locale")])
result = unittest.TextTestRunner(verbosity=2).run(suite)
raise SystemExit(not result.wasSuccessful())
