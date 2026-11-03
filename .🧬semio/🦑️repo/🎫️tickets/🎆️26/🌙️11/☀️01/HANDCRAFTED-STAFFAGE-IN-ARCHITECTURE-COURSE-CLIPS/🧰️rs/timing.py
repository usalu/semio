import importlib, sys, time, unittest
from pathlib import Path
ROOT = Path.cwd(); sys.path.insert(0, str(ROOT))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
original = runtime.ToolSession.run
start = time.time()
def run(self, command, **kw):
    t = time.time()
    try:
        return original(self, command, **kw)
    finally:
        label = " ".join(Path(str(c)).name[:24] for c in command[:6])
        print(f"[TIMING] t={t - start:6.1f}s dur={time.time() - t:6.1f}s {label}", file=sys.stderr, flush=True)
runtime.ToolSession.run = run
sys.argv = ["x", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️.PresentationNativeTest.test_encoded_chapter_preserves_body_camera_handoffs_audio_and_contextual_boundaries"]
unittest.main(module=None)
