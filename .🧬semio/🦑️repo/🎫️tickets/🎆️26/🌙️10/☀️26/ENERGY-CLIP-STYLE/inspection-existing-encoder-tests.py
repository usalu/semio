"""🧪 Revalidate affected existing presentation contracts and native media after stream normalization."""
from pathlib import Path
import importlib
import json
import os
import sys
import unittest

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).mkdir(parents=True, exist_ok=True)
importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️")
suite = unittest.defaultTestLoader.loadTestsFromTestCase(module.PresentationContractTest)
for name in (
    "test_silent_source_preserves_every_full_frame_and_has_a_finite_silent_audio_stream",
    "test_resampled_content_uses_the_same_selected_source_frames_at_both_camera_handoffs",
    "test_edited_mp4_counts_decoded_frames_and_keeps_audio_duration_equal_to_visible_video",
    "test_encoded_chapter_preserves_body_camera_handoffs_audio_and_contextual_boundaries",
):
    suite.addTest(module.PresentationNativeTest(name))
result = unittest.TextTestRunner(verbosity=2).run(suite)
work = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
work.mkdir(parents=True, exist_ok=True)
(work / "existing-result.json").write_text(json.dumps(dict(
    tests=result.testsRun, failures=[(str(t), text) for t, text in result.failures],
    errors=[(str(t), text) for t, text in result.errors], skipped=[str(t) for t, text in result.skipped],
    passed=result.wasSuccessful()), ensure_ascii=False, indent=2), encoding="utf-8")
raise SystemExit(not result.wasSuccessful())
