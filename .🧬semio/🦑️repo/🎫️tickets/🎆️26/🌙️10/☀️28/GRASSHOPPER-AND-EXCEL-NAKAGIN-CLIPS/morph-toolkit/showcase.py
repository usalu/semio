"""🎞️ Morph toolkit showcase: capsule to tree leaf to path chip and back, cell and face, 20 squares to a number, scatter, tether, retext.

Usage: python showcase.py [name ...]   Output: 🗑️generated/morph-toolkit/<n>-<name>-<25|50|75>.png (quarter frames of each animation).
"""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
os.environ.setdefault("SEMIO_TEACHING_CACHE_DIR", str(TICKET / "🗑️generated/fonts/morph-toolkit"))
OUTPUT = TICKET / "🗑️generated/morph-toolkit"
OUTPUT.mkdir(parents=True, exist_ok=True)
m = importlib.import_module("manim")
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
only = set(sys.argv[1:])
CYAN, YELLOW, GREEN, ORANGE = art.P_CYAN, art.P_YELLOW, art.P_GREEN, art.P_ORANGE


def capsule(color=CYAN, at=m.ORIGIN):
    """💊 Detailed capsule: shell, window, door."""
    shell = m.RoundedRectangle(width=1.5, height=0.8, corner_radius=0.28, fill_color=color, fill_opacity=0.55, stroke_color=art.P_WHITE, stroke_width=2)
    window = m.Circle(radius=0.2, fill_color=art.P_DEEP_DARK, fill_opacity=0.9, stroke_color=art.P_WHITE, stroke_width=1.5).shift(0.35 * m.LEFT)
    door = m.Rectangle(width=0.22, height=0.5, fill_color=art.P_WHITE, fill_opacity=0.5, stroke_width=1).shift(0.45 * m.RIGHT)
    return m.VGroup(shell, window, door).move_to(at)


def face(color=ORANGE, at=m.ORIGIN):
    """🧱 A hatched wall face."""
    box = m.Rectangle(width=1.5, height=0.8, fill_color=color, fill_opacity=0.35, stroke_color=color, stroke_width=2)
    hatch = m.VGroup(*[m.Line(box.get_corner(m.DL) + m.RIGHT * x, box.get_corner(m.DL) + m.RIGHT * x + m.UP * 0.8, stroke_width=1.2, color=color) for x in (0.3, 0.6, 0.9, 1.2)])
    return m.VGroup(box, hatch).move_to(at)


class Showcase(m.Scene):
    """🎞️ All sequences in one scene."""

    n = 0

    def construct(self):
        self.camera.background_color = art.P_DEEP_DARK
        if not only or "chain" in only:
            self.chain()
        if not only or "cell" in only:
            self.cell_face()
        if not only or "gather" in only:
            self.gather_number()
        if not only or "scatter" in only:
            self.scatter_leaves()
        if not only or "retext" in only:
            self.retext_tether()

    def chain(self):
        """⛓ Capsule to tree leaf to path chip and back."""
        cap = capsule(CYAN, 3 * m.LEFT)
        self.add(cap)
        leaf = morph.token("3", kind="item", color=CYAN, size=1.6).move_to(m.UP * 1.5 + m.RIGHT)
        chip = morph.token("{0;1;3;1}", kind="path", color=CYAN, size=1.5).move_to(m.RIGHT * 2.5 + m.DOWN)
        back = capsule(CYAN, 3 * m.LEFT)
        self.play(morph.morph(cap, leaf, ghost=0.25, path_arc=-0.8, run_time=1.8, dip=0.15), name="capsule-leaf")
        self.play(morph.morph(leaf, chip, path_arc=0.8, run_time=1.6, dip=0.15), name="leaf-chip")
        self.play(morph.morph(chip, back, path_arc=-0.6, run_time=1.8, dip=0.15), name="chip-capsule")
        self.clear()

    def cell_face(self):
        """🧮 Cell and face."""
        cell = morph.token("B2 = 12", kind="cell", color=GREEN, size=2.0).move_to(3 * m.LEFT)
        wall = face(GREEN, 2.5 * m.RIGHT + m.UP * 0.5)
        self.add(cell)
        self.play(morph.morph(cell, wall, path_arc=0.7), name="cell-face")
        cell2 = morph.token("B2 = 12", kind="cell", color=GREEN, size=2.0).move_to(3 * m.LEFT)
        self.play(morph.morph(wall, cell2, path_arc=0.7), name="face-cell")
        self.clear()

    def gather_number(self):
        """🧲 20 squares into a number."""
        squares = [m.Square(0.34, fill_color=ORANGE, fill_opacity=0.8, stroke_color=art.P_WHITE, stroke_width=1.5).move_to([(i % 10) * 0.5 - 2.25, 1.8 - (i // 10) * 0.5, 0]) for i in range(20)]
        badge = morph.token("20", kind="number", color=YELLOW, size=2.4).move_to(m.DOWN * 1.2)
        self.add(*squares)
        self.play(morph.gather(squares, badge), name="gather")
        outs = [m.Square(0.34, fill_color=ORANGE, fill_opacity=0.8, stroke_color=art.P_WHITE, stroke_width=1.5).move_to([(i % 10) * 0.5 - 2.25, 1.8 - (i // 10) * 0.5, 0]) for i in range(20)]
        self.play(morph.split(badge, outs), name="split")
        self.clear()

    def scatter_leaves(self):
        """🎆 Capsules to leaves."""
        caps = [capsule(CYAN, [(i % 4) * 1.8 - 6 + 0.3, 1.5 - (i // 4) * 1.1, 0]).scale(0.55) for i in range(12)]
        leaves = [morph.token(str(i), kind="item", color=CYAN).move_to([(i % 3) * 1.1 + 1, 1.7 - (i // 3) * 0.7, 0]) for i in range(12)]
        self.add(*caps)
        self.play(morph.scatter(caps, leaves, ghost=0.2), name="scatter")
        self.clear()

    def retext_tether(self):
        """✏ Tether and retext."""
        a = morph.token("A1", kind="cell", color=YELLOW, size=1.6).move_to(3 * m.LEFT + m.UP)
        b = morph.token("plane", kind="plane", color=GREEN, size=1.6).move_to(3 * m.RIGHT + m.DOWN)
        line = morph.tether(a, b)
        self.add(a, b, line)
        self.play(b.animate.shift(2 * m.UP), name="tether-follow")
        t1 = m.Text("A1+B2*C3", font_size=48, color=art.P_WHITE)
        t2 = m.Text("B2*C3+A1", font_size=48, color=YELLOW).move_to(m.DOWN * 2)
        self.clear()
        self.add(t1)
        self.play(morph.retext(t1, t2), name="retext")
        self.clear()

    def play(self, *animations, name="anim", **kwargs):
        """▶ Play and keep the quarter frames."""
        self.n += 1
        self.tag, self.marks = f"{self.n:02d}-{name}", {0.25, 0.5, 0.75}
        return super().play(*animations, **kwargs)

    def update_to_time(self, t):
        """⏱ Save quarter frames while playing."""
        super().update_to_time(t)
        duration = getattr(self, "duration", 0) or 1
        for mark in sorted(self.marks):
            if t >= mark * duration:
                self.marks.discard(mark)
                self.renderer.update_frame(self, ignore_skipping=True)
                self.renderer.camera.get_image().save(OUTPUT / f"{self.tag}-{int(mark * 100)}.png")

    def wait(self, *args, **kwargs):
        """⏸ No waiting."""
        return None


with m.tempconfig({"pixel_width": 960, "pixel_height": 540, "frame_rate": 10, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR", "progress_bar": "none"}):
    Showcase().render()
print("[DEBUG] showcase frames in", OUTPUT, flush=True)
