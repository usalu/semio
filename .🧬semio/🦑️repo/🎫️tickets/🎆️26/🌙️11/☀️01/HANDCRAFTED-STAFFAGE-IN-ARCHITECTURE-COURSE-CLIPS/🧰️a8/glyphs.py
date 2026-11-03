"""Still of the six `_use_glyph` mini rooms of the volume clip (scale 3) for visual checks.

usage (repo root, PYTHONPATH=repo root): manim -ql -s --media_dir <dir> glyphs.py Glyphs
"""

import importlib

from manim import Scene, VGroup, config

BASE = "🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🐍️"
config.background_color = "#10181e"


class Glyphs(Scene):
    def construct(self):
        module = importlib.import_module(BASE)
        grid = VGroup(*[module._use_glyph(kind, "#7f8f99") for kind in ("lager", "passiv", "buero", "klasse", "kueche", "op")])
        grid.arrange_in_grid(rows=2, buff=0.5).scale(1.35)
        self.add(grid)
