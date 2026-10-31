"""🧪 Run the presentation lifecycle (real animation setup, interpolation, cleanup, Cairo frame) for chosen Grasshopper scenes in one language, without video output.

Usage: python lifecycle.py <en|de> <Scene> [Scene ...]
"""
import importlib.util
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language, *names = sys.argv[1:]
spec = importlib.util.spec_from_file_location("gh_presentation", ROOT / "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation/🐍️.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
case = module.GrasshopperPresentationContract("test_native_lifecycle")
output = TICKET / "🗑️generated/layout-fixes/lifecycle"
output.mkdir(parents=True, exist_ok=True)
for folder, scene, _ in module.discover():
    if scene.__name__ in names:
        started = time.time()
        try:
            case.lifecycle(scene.__name__, scene, language, output)
            print(f"[DEBUG] OK {scene.__name__} {language} {time.time() - started:.0f}s", flush=True)
        except Exception as error:
            print(f"[DEBUG] FAIL {scene.__name__} {language} {type(error).__name__}: {error}", flush=True)
            import traceback

            traceback.print_exc()
