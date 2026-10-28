"""📏 Measure the exact extents of room_section and facade relative to `point` over extreme designs, scales and seasons.

Usage: python toolkit-extents.py <en|de>
"""
import importlib, itertools, os, sys
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = sys.argv[1]
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(Path(__file__).parent / "🗑️generated/fonts")
v = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
scene = v.ClipScene()
designs = [dict(orientation=o, wwr=w, overhang=d, insulation=t) for o, w, d, t in itertools.product((90, 180, 270, 360, 450), (.1, .45, .8), (0, 1.2), (.1, .3))]
rows = []
for kind, scales in (("section", (.25, .35, .5, .7)), ("facade", (.2, .25, .3, .4, .6))):
    for s in scales:
        box = [9, -9, 9, -9]
        for design in designs:
            for season in (("winter", "summer") if kind == "section" else ("summer",)):
                trackers = scene.study_trackers(design)
                widget = scene.room_section(trackers, (0, 0, 0), s, season) if kind == "section" else scene.facade(trackers, (0, 0, 0), s, season)
                lo, hi = widget.get_corner([-1, -1, 0]), widget.get_corner([1, 1, 0])
                box = [min(box[0], lo[0]), max(box[1], hi[0]), min(box[2], lo[1]), max(box[3], hi[1])]
        rows.append(f"| {kind} | {s} | {box[0]:.2f} … {box[1]:.2f} | {box[2]:.2f} … {box[3]:.2f} |")
print("\n".join(rows))
