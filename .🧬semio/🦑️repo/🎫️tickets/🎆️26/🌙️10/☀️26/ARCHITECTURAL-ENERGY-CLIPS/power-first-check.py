"""🎞️ Verify the reordered physics scenes with real Manim frames and bilingual captions."""

import importlib
import os
from pathlib import Path
import sys
import time

ticket = Path(__file__).resolve().parent
workspace = next(parent for parent in ticket.parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(workspace))
output = ticket / "🗑️generated" / "power-first"
output.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(output)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(output / "fonts")
os.environ["VO_TRACE"] = "1"

import av
from manim import tempconfig

physics = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
names = sys.argv[1:] or ["Beat1_Leistung"]
if len(names) != 1:
    raise SystemExit("Run one canonical scene per process, matching the curriculum renderer.")
results = []
settings = {"pixel_width": 640, "pixel_height": 360, "frame_rate": 2, "preview": False,
            "disable_caching": True, "progress_bar": "none", "verbosity": "ERROR", "media_dir": str(output),
            "video_dir": str(output / "v"), "images_dir": str(output / "i"),
            "text_dir": str(output / "t"), "tex_dir": str(output / "x")}
with tempconfig(settings):
    for name in names:
        started = time.monotonic()
        beat = getattr(physics, name)
        for key, en, de in beat.NARRATION:
            if key in {"rate", "unit", "vacuum", "shower", "zeros", "meter", "product", "energy", "power"}:
                visuals.caption_bar(en)
                visuals.caption_bar(de)
        print(f"[DEBUG] Rendering {name}", flush=True)
        scene = beat()
        scene.render()
        movie = Path(scene.renderer.file_writer.movie_file_path)
        marks = visuals._VO_TRACE.get(name, {})
        ordered = {"Beat1_Leistung": ["rate", "unit", "area"],
                   "Beat2_EnergieImAlltag": ["meter", "product"],
                   "Beat8_Zusammenfassung": ["power", "energy"]}[name]
        assert all(marks[first]["start"] < marks[second]["start"] for first, second in zip(ordered, ordered[1:]))
        samples = {key: float(mark["start"]) + min(2, float(mark["window"]) / 2)
                   for key, mark in marks.items() if key in {"rate", "unit", "meter", "product", "power", "energy"}}
        count = 0
        with av.open(movie) as container:
            for frame in container.decode(video=0):
                count += 1
                assert (frame.width, frame.height) == (640, 360)
                for key, seconds in list(samples.items()):
                    if float(frame.time) >= seconds:
                        frame.to_image().save(output / f"{name}-{key}.png")
                        del samples[key]
        assert count > 0 and not samples
        elapsed = time.monotonic() - started
        results.append(f"- {name}: {count} decoded frames, 640×360, nominal 2 fps; {elapsed:.2f} s render/check time.")
        print(f"[DEBUG] Verified {name}: {count} frames in {elapsed:.2f}s", flush=True)

report = ticket / "power-first-render.md"
prior = report.read_text(encoding="utf-8") if report.exists() else "# Power-First Render Verification\n\n"
report.write_text(prior + "\n".join(results) + "\n", encoding="utf-8")
