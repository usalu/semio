"""Run the real-setup lifecycle of the migrated Grasshopper scenes under both profiles and report the transitions each scene played."""
import importlib
import sys
import traceback
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "AGENTS.md").exists())
sys.path.insert(0, str(ROOT))
OUT = Path(sys.argv[1])
names = sys.argv[2].split(",")
languages = sys.argv[3].split(",")
profiles = sys.argv[4].split(",")
mod = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🎭️presentation.🐍️")
stage_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")
case = mod.GrasshopperPresentationContract("test_native_lifecycle")
scenes = {scene.__name__: scene for _, scene, _ in mod.discover()}
played = []
original = stage_module.Stage.transition


def spy(self, transition_id, source=None, target=None):
    played.append(transition_id)
    return original(self, transition_id, source, target)


stage_module.Stage.transition = spy
failed = 0
for name in names:
    for language in languages:
        for profile in profiles:
            played.clear()
            try:
                case.lifecycle(name, scenes[name], language, OUT / f"{name}-{language}-{profile}", profile)
                print(f"OK {name} {language} {profile}: {len(played)} transitions", flush=True)
            except Exception:
                failed += 1
                print(f"FAIL {name} {language} {profile}", flush=True)
                traceback.print_exc()
sys.exit(1 if failed else 0)
