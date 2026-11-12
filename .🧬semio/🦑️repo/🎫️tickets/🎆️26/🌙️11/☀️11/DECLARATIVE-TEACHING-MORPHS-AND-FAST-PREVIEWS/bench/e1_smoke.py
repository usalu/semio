"""Batch E1 smoke: run real scenes with skipped animations (no media) under an explicit profile, reporting the instrument counters.

Usage: e1_smoke.py <profile> <dotted module below 🎓️teaching.🏛️architecture> <Scene> [<Scene> ...]
Example: e1_smoke.py preview ⚡️energy.🔥️heating.🎬️clip.🌬️convection Beat1_LueftungImWinter
Optimization clips are ClipScene classes: pass the clip module and the Beat class; the scene calls present() itself.
"""

import importlib
import os
import sys
import time
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True

profile_name, dotted, *scenes = sys.argv[1:]
os.environ["SEMIO_TEACHING_PROFILE"] = profile_name
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
os.environ.setdefault("SEMIO_TEACHING_AUDIENCE", "speaker")
out = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", ROOT / ".🧬semio/🎓️teaching/⚡️cache/tests")) / "e1-smoke"
engine = importlib.import_module("manim")
profiles = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
chosen = profiles.profile(profile_name)
module = importlib.import_module(f"🎓️teaching.🏛️architecture.{dotted}.🐍️")
for scene in scenes:
    instrument.reset()
    started = time.time()
    with engine.tempconfig(dict(media_dir=str(out), dry_run=True, skip_animations=True, disable_caching=True, pixel_width=chosen.width, pixel_height=chosen.height,
                                frame_rate=chosen.fps, progress_bar="none", verbosity="ERROR")):
        getattr(module, scene)().render()
    print(profile_name, dotted, scene, f"{time.time() - started:.1f}s", instrument.snapshot()["counts"], flush=True)
