import importlib.util
import os
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
from manim import *
from PIL import Image

target_path, target_class = os.environ["TRACE_TARGET"].split("::")
spec = importlib.util.spec_from_file_location("trace_target", target_path)
module = importlib.util.module_from_spec(spec)
sys.modules["trace_target"] = module
spec.loader.exec_module(module)
Target = getattr(module, target_class)
STOP_KEY = os.environ["STOP_KEY"]
OUT_PNG = Path(os.environ["OUT_PNG"])
original_hold = module.hold_for


def stopping_hold(scene, narration, key, *args, **kwargs):
    result = original_hold(scene, narration, key, *args, **kwargs)
    if key == STOP_KEY:
        scene.renderer.update_frame(scene)
        OUT_PNG.parent.mkdir(parents=True, exist_ok=True)
        Image.fromarray(scene.renderer.get_frame()).save(OUT_PNG)
        print("STOPPED", key, scene.renderer.num_plays, flush=True)
        os._exit(0)
    return result


module.hold_for = stopping_hold


class Trace(Target):
    pass
