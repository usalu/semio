"""Scratch smoke for batch E2b: render demand scenes with skipped animations under an explicit profile (native engine, no media output).

Usage: smoke.py <profile> <clip folder> <Scene> [<Scene> ...]
"""

import importlib
import os
import sys
import time
import traceback
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True

profile_name, clip, *scenes = sys.argv[1:]
os.environ["SEMIO_TEACHING_PROFILE"] = profile_name
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
os.environ.setdefault("SEMIO_TEACHING_AUDIENCE", "speaker")
out = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", ROOT / ".🧬semio/🎓️teaching/⚡️cache/tests")) / "e2b-smoke"
engine = importlib.import_module("manim")
profiles = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
chosen = profiles.profile(profile_name)
module = importlib.import_module(f"🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.{clip}.🐍️")
failed = []
for scene in scenes:
    instrument.reset()
    started = time.time()
    try:
        with engine.tempconfig(dict(media_dir=str(out), dry_run=True, skip_animations=True, disable_caching=True, pixel_width=chosen.width, pixel_height=chosen.height,
                                    frame_rate=chosen.fps, progress_bar="none", verbosity="ERROR")):
            getattr(module, scene)().render()
        print(profile_name, clip, scene, f"{time.time() - started:.1f}s", instrument.snapshot()["counts"], flush=True)
    except Exception:
        failed.append(scene)
        print("FAILED", profile_name, clip, scene, flush=True)
        traceback.print_exc()
sys.exit(1 if failed else 0)
