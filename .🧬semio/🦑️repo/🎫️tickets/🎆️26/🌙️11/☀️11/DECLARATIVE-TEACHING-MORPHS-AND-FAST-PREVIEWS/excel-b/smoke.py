"""Skipped-animation dry run of one clip scene under an explicit profile (no media): proves stage.transition wiring and the work counters.

Usage: smoke.py <profile> <language> <dotted module> <Scene> [<Scene> ...]
Env SEMIO_TEACHING_OUTPUT_ROOT chooses the scratch root. Exit 1 when a scene fails or when the counters break the profile contract (preview: full.* == 0; final: full.prepare and full.evaluate > 0).
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

profile_name, language, dotted, *scenes = sys.argv[1:]
os.environ["SEMIO_TEACHING_PROFILE"] = profile_name
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ.setdefault("SEMIO_TEACHING_AUDIENCE", "speaker")
out = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", ROOT / ".🧬semio/🎓️teaching/⚡️cache/tests")) / "excel-b-smoke"
engine = importlib.import_module("manim")
profiles = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
chosen = profiles.profile(profile_name)
module = importlib.import_module(dotted)
failed = []
for scene in scenes:
    instrument.reset()
    started = time.time()
    try:
        with engine.tempconfig(dict(media_dir=str(out), dry_run=True, skip_animations=True, disable_caching=True, pixel_width=chosen.width, pixel_height=chosen.height,
                                    frame_rate=chosen.fps, progress_bar="none", verbosity="ERROR")):
            getattr(module, scene)().render()
        counts = instrument.snapshot()["counts"]
        heavy = {k: v for k, v in counts.items() if k.startswith("full.")}
        ok = (not heavy) if profile_name == "preview" else (heavy.get("full.prepare", 0) > 0 and heavy.get("full.evaluate", 0) > 0)
        print("OK" if ok else "CONTRACT", profile_name, language, scene, f"{time.time() - started:.1f}s", counts, flush=True)
        if not ok:
            failed.append(scene)
    except Exception:
        failed.append(scene)
        print("FAILED", profile_name, language, scene, flush=True)
        traceback.print_exc()
sys.exit(1 if failed else 0)
