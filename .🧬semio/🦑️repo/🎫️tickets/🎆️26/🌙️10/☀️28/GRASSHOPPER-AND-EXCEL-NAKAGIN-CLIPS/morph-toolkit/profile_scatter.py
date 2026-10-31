"""🔬 Profile a 140-source scatter: build, begin and one interpolation."""
import cProfile
import importlib
import pstats
import sys
import time
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
manim = importlib.import_module("manim")
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")


def capsule(x):
    box = manim.RoundedRectangle(width=1.0, height=0.5, corner_radius=0.2, fill_color=manim.BLUE, fill_opacity=0.8)
    window = manim.Square(0.2, fill_color=manim.YELLOW, fill_opacity=0.6).shift(0.25 * manim.LEFT)
    return manim.VGroup(box, window).shift(x * manim.RIGHT)


sources = [capsule(i * 0.05) for i in range(140)]
targets = [morph.token(str(i), kind="item").move_to([(i % 14) * 0.3 - 2, (i // 14) * 0.3 - 2, 0]) for i in range(140)]
t = time.perf_counter()
anim = morph.scatter(sources, targets)
print("[DEBUG] build", time.perf_counter() - t)
with manim.tempconfig({"write_to_movie": False, "skip_animations": True, "progress_bar": "none", "verbosity": "ERROR"}):
    scene = manim.Scene()
    scene.add(*sources)
    profiler = cProfile.Profile()
    profiler.enable()
    t = time.perf_counter()
    scene.play(anim)
    print("[DEBUG] play", time.perf_counter() - t)
    profiler.disable()
pstats.Stats(profiler).sort_stats("cumulative").print_stats(18)
