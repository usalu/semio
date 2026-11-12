"""Dry-run smoke of Grasshopper batch scenes under an explicit profile (skipped animations, no media output).

Usage: smoke.py <profile> <clip folder> <Scene> [language]
Prints the work counters (full.prepare / full.evaluate / preview.swap) and the played transition ids.
"""
import importlib
import json
import os
import sys
import time
import traceback
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
profile_name, clip, scene = sys.argv[1:4]
os.environ["SEMIO_TEACHING_PROFILE"] = profile_name
os.environ["SEMIO_TEACHING_LANGUAGE"] = sys.argv[4] if len(sys.argv) > 4 else "en"
os.environ.setdefault("SEMIO_TEACHING_AUDIENCE", "speaker")
out = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", ROOT / ".🧬semio/🎓️teaching/⚡️cache/tests")) / "clips-a-smoke"
engine = importlib.import_module("manim")
profiles = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
chosen = profiles.profile(profile_name)
module = importlib.import_module(f"🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.{clip}.🐍️")
instrument.reset()
started = time.time()
played = []
try:
    with engine.tempconfig(dict(media_dir=str(out), dry_run=True, skip_animations=True, disable_caching=True, pixel_width=chosen.width, pixel_height=chosen.height,
                                frame_rate=chosen.fps, progress_bar="none", verbosity="ERROR")):
        instance = getattr(module, scene)()
        instance.render()
        stage = getattr(instance, "stage", None)
        played = list(stage.played) if stage is not None else []
    print(json.dumps({"profile": profile_name, "clip": clip, "scene": scene, "seconds": round(time.time() - started, 1), "counts": instrument.snapshot()["counts"], "played": len(played)}), flush=True)
except Exception:
    print("FAILED", profile_name, clip, scene, flush=True)
    traceback.print_exc()
    sys.exit(1)
