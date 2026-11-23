"""🔬 Prints the native pointer events of the focus phase per target. usage: --script this.py"""
import importlib, os, sys, unittest
sys.path.insert(0, os.getcwd())
module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🧪️tests.🐍️")
original = unittest.TestCase.assertEqual
def spy(self, first, second, msg=None):
    if first == ["pointerdown", "pointerdown", "pointerup", "click"]:
        print("[DEBUG] duplicated pointerdown", first)
    return original(self, first, second, msg)
unittest.TestCase.assertEqual = spy
cls = module.CursorNativeFrameTest
renderer = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🖌️render.🐍️")
frame_batch = renderer.frame_batch
def traced(requests, settings, *a, **k):
    result = frame_batch(requests, settings, *a, **k)
    for index, frame in enumerate(result):
        for event in frame["browser"]["pointer_events"]:
            print("[DEBUG]", requests[0]["phase"], index, event)
    return result
renderer.frame_batch = traced
result = unittest.TextTestRunner(verbosity=2).run(unittest.TestSuite([cls("test_four_fps_transitions_capture_current_native_clicks_at_every_required_level")]))
sys.exit(0 if result.wasSuccessful() else 1)
