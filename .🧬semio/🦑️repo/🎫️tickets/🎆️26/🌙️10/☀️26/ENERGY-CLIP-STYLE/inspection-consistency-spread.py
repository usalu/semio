"""🔎 Probe only the current spread builder and native entrance without a movie encoder."""
from pathlib import Path
import hashlib
import importlib
import json
import os
import sys
import time

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
OUTPUT = TICKET / "🗑️generated/ik/spread"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
MODULE = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔗️consistency.🐍️")
ENGINE = MODULE._engine
PATH = Path(MODULE.__file__)
before = hashlib.sha256(PATH.read_bytes()).hexdigest()
results = []
with ENGINE.tempconfig(dict(media_dir=str(OUTPUT), write_to_movie=False, pixel_width=480,
                            pixel_height=270, frame_rate=4, disable_caching=True)):
    for language in ("en", "de"):
        os.environ["SEMIO_TEACHING_LANGUAGE"] = language
        MODULE._visuals._art.set_vo_language(language)
        scene = MODULE.Beat6_Consistency()
        MODULE._visuals._fonts.apply_scene_style(scene)
        scene._prepare()
        captured = {}
        original_show = scene.show
        def show(key, **kwargs):
            captured["types"] = [type(item).__name__ for item in kwargs["new"]]
            assert all(isinstance(item, ENGINE.Mobject) for item in kwargs["new"])
            return original_show(key, **kwargs)
        def step(key, animation, **kwargs):
            captured["key"] = key
            animation._setup_scene(scene)
            animation.begin()
            for alpha in (0, 0.25, 0.5, 0.75, 1):
                animation.update_mobjects(0.05)
                animation.interpolate(alpha)
            animation.finish()
            animation.clean_up_from_scene(scene)
        scene.show, scene.step = show, step
        scene._spread()
        assert captured["key"] == "spread"
        results.append(dict(language=language, **captured))
after = hashlib.sha256(PATH.read_bytes()).hexdigest()
result = dict(checked=time.time(), source_before=before, source_after=after, results=results,
              stable=before == after)
(OUTPUT / "result.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
print(json.dumps(result, ensure_ascii=False))
assert before == after, "Authored leaf changed during probe"
