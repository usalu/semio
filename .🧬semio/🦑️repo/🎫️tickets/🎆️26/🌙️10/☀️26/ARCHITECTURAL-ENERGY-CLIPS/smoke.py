"""🔎 Smoke-render every canonical curriculum source with the real Manim scene engine."""

import ast
import gc
import importlib
import json
import os
import sys
import time
from pathlib import Path

workspace = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(workspace))
os.environ["PYTHONDONTWRITEBYTECODE"] = "1"
output = Path(__file__).parent / "🗑️generated" / "smoke"
output.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(output)
from manim import tempconfig
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
plan = runtime.source_plan(runtime.segment_plan())
results = []
settings = {"pixel_width": 640, "pixel_height": 360, "frame_rate": 1, "skip_animations": True,
            "dry_run": True, "write_to_movie": False, "save_last_frame": False, "preview": False,
            "disable_caching": True, "progress_bar": "none", "verbosity": "ERROR", "media_dir": str(output)}
selected = set(sys.argv[1:])
with tempconfig(settings):
    for index, job in enumerate(plan, 1):
        segment = job["segment"]
        if selected and segment["scene"] not in selected:
            continue
        started = time.monotonic()
        try:
            module = importlib.import_module("🎓️teaching." + segment["script"].removesuffix(".py").replace("/", "."))
            scene = getattr(module, segment["scene"])(skip_animations=True)
            scene.render()
            result = {"scene": segment["scene"], "script": segment["script"], "status": "passed"}
            del scene
        except Exception as error:
            import traceback
            result = {"scene": segment["scene"], "script": segment["script"], "status": "failed",
                      "error": str(error), "traceback": traceback.format_exc()}
        result["seconds"] = round(time.monotonic() - started, 2)
        results.append(result)
        (output / "results.json").write_text(json.dumps(results, ensure_ascii=False, indent=2), encoding="utf-8")
        print(f"[DEBUG] {index}/{len(plan)} {result['status']}: {segment['scene']} ({result['seconds']}s)", flush=True)
        gc.collect()
failures = [entry for entry in results if entry["status"] == "failed"]
report = Path(__file__).parent / "smoke-verification.md"
prior = report.read_text(encoding="utf-8") if selected and report.exists() else "# Canonical Scene Smoke Verification\n\n"
report.write_text(prior + f"Real Manim dry renders: {len(results) - len(failures)}/{len(results)} passed. "
                  "Every selected scene constructs its full authored timeline with final animation updates, at 640×360.\n\n"
                  + "\n".join(f"- {entry['scene']}: {entry['error']}" for entry in failures) + "\n", encoding="utf-8")
print(f"[DEBUG] Smoke result: {len(results)-len(failures)}/{len(results)} passed", flush=True)
sys.exit(bool(failures))
