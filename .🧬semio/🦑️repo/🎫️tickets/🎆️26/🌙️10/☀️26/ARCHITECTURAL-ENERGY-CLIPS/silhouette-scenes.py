"""🎞️ Inspect selected production scenes at settled narration clauses."""

import importlib
import json
import logging
from pathlib import Path
import sys
import traceback

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
from manim import tempconfig
from manim.utils.exceptions import EndSceneEarlyException
from PIL import Image, ImageDraw

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
R = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/silhouettes/scenes"
OUT.mkdir(parents=True, exist_ok=True)
logging.getLogger("manim").setLevel(logging.ERROR)
rows, errors, current = [], [], {}
original = V._trace_clause


def capture(scene, key, **kwargs):
    """🔍 Save real settled scene geometry with original narration and coordinates."""
    original(scene, key, **kwargs)
    if (OUT / "cancel").exists():
        raise KeyboardInterrupt
    if not scene.mobjects:
        return
    camera = scene.renderer.camera
    camera.reset()
    camera.capture_mobjects(scene.mobjects)
    target = OUT / f"{current['ordinal']:03d}-{key}.png"
    camera.get_image().save(target)
    rows.append({"source": current["ordinal"], "scene": current["name"], "clause": key, "path": str(target)})
    if key == current.get("until"):
        raise EndSceneEarlyException


V._trace_clause = capture
all_sources = R.source_plan(R.segment_plan())
requests = {"Beat3_Energieerhaltung": "law", "Beat3_Fahrplan": None, "Beat1_VerlustZuGewinn": "gain",
            "Beat2_HumanFactor": "activities", "Beat2_Primaerenergiefaktor": "umwelt"}
sources = all_sources if "--all" in sys.argv else [job for job in all_sources if job["segment"]["scene"] in requests]
for module_name in sorted({"🎓️teaching." + job["segment"]["script"].removesuffix(".py").replace("/", ".") for job in all_sources}):
    importlib.import_module(module_name)
print(f"[DEBUG] Imported all {len(all_sources)} source scenes", flush=True)
with tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 6, "skip_animations": True,
                 "write_to_movie": False, "save_last_frame": False, "disable_caching": True,
                 "progress_bar": "none", "media_dir": str(OUT / "media")}):
    for index, job in enumerate(sources, 1):
        if (OUT / "cancel").exists():
            raise KeyboardInterrupt
        segment = job["segment"]
        current = {"ordinal": job["source_ordinal"], "name": segment["scene"], "until": requests.get(segment["scene"])}
        module_name = "🎓️teaching." + segment["script"].removesuffix(".py").replace("/", ".")
        try:
            source = importlib.import_module(module_name)
            scene = getattr(source, segment["scene"])(skip_animations=True)
            scene.render()
            scene.renderer.camera.reset()
        except Exception as error:
            errors.append({**current, "error": str(error), "trace": traceback.format_exc()})
            print("[DEBUG] Scene failed: " + current["name"] + ": " + str(error), flush=True)
        print(f"[DEBUG] Source snapshots {index}/{len(sources)}: {current['name']}", flush=True)
        (OUT / "manifest.json").write_text(json.dumps({"frames": rows, "errors": errors}, indent=2), encoding="utf-8")

by_source = {}
for row in rows:
    by_source.setdefault(row["source"], []).append(row)
for ordinal, frames in by_source.items():
    selected = frames if len(frames) <= 12 else [frames[round(i * (len(frames) - 1) / 11)] for i in range(12)]
    sheet = Image.new("RGB", (1280, ((len(selected) + 3) // 4) * 204), "#171717")
    draw = ImageDraw.Draw(sheet)
    for index, row in enumerate(selected):
        x, y = index % 4 * 320, index // 4 * 204
        sheet.paste(Image.open(row["path"]).resize((320, 180)), (x, y + 24))
        draw.text((x + 8, y + 5), str(ordinal) + " " + row["clause"], fill="white")
    sheet.save(OUT / f"{ordinal:03d}-sheet.png")
(TICKET / "silhouette-scene-audit.md").write_text(
    "# Canonical Silhouette Scene Check\n\n"
    + f"Imported all {len(all_sources)} canonical source scenes. Executed selected portions of {len(sources)} production scenes with animation skipping at 640×360; captured {len(rows)} settled narration frames. "
    + f"Runtime failures: {len(errors)}. This proves settled scene execution, not full-frame video encoding.\n\n"
    + "\n".join(f"- Source {e['ordinal']} {e['name']}: {e['error']}" for e in errors) + "\n",
    encoding="utf-8")
if errors:
    raise SystemExit(1)
