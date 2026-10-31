"""🧪 Probe whether FadeOut of a legend table removes the number discs that a morph scattered into it."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
manim = importlib.import_module("manim")
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")


def row(i):
    disc = manim.VGroup(manim.Circle(radius=0.16, fill_opacity=1), manim.Square(0.1)).move_to([-1, -i * 0.4, 0])
    return manim.VGroup(disc, manim.Square(0.2).move_to([1, -i * 0.4, 0]))


with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR"}):
    scene = manim.Scene()
    table = manim.VGroup(*[row(i) for i in range(4)])
    rest = manim.VGroup(*[manim.VGroup(*list(r)[1:]) for r in table])
    scene.add(rest)
    sources = [manim.VGroup(manim.Circle(radius=0.16, fill_opacity=1), manim.Square(0.1)).move_to([-4, 2 - i, 0]) for i in range(4)]
    scene.add(*sources)
    scene.play(morph.scatter(sources, [r[0] for r in table], lag=0.1, run_time=1.0, pairs_fn=lambda a, b: [(a[0], b[0]), (a[1], b[1])]))
    discs = [r[0] for r in table]
    print("[DEBUG] after fly", [d in scene.mobjects for d in discs])
    scene.play(manim.FadeOut(table), manim.FadeOut(rest), run_time=0.01)
    print("[DEBUG] after fadeout", [d in scene.mobjects for d in discs], len(scene.mobjects))
