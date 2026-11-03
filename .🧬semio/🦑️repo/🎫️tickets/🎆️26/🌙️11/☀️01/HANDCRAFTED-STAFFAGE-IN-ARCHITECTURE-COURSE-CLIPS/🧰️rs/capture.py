import importlib, json, shutil, sys, unittest
from pathlib import Path
ROOT = Path.cwd(); sys.path.insert(0, str(ROOT))
OUT = ROOT / sys.argv[1]
OUT.mkdir(parents=True, exist_ok=True)
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
original = runtime.ToolSession.run
count = [0]
def run(self, command, **kw):
    if any("render.mjs" in str(c) for c in command) and "--sources" not in command:
        path = command[command.index("--input") + 1]
        frames = len(json.loads(Path(path).read_text(encoding="utf-8"))["frames"])
        count[0] += 1
        shutil.copy2(path, OUT / f"batch{count[0]}-{frames}.json")
        if frames > 5:
            raise SystemExit("captured")
    return original(self, command, **kw)
runtime.ToolSession.run = run
sys.argv = ["x", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️.PresentationNativeTest.test_encoded_chapter_preserves_body_camera_handoffs_audio_and_contextual_boundaries"]
unittest.main(module=None)
