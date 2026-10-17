"""[DEBUG] Report visible mobjects whose bbox leaves |x| <= LIMIT after each play (viewer crops the 16:9 frame)."""
import os, sys
import numpy as np
from manim import Scene
from manim.__main__ import main

LIMIT = float(os.environ.get("X_LIMIT", "6.4"))
seen = set()
orig = Scene.play


def visible(m):
    try:
        return m.get_fill_opacity() > 0.02 or m.get_stroke_opacity() > 0.02
    except AttributeError:
        return False


def play(self, *a, **k):
    out = orig(self, *a, **k)
    t = round(float(getattr(self.renderer, "time", 0.0)), 1)
    for top in self.mobjects:
        for m in top.get_family():
            if m.submobjects or not m.has_points() or not visible(m):
                continue
            lo, hi = m.get_left()[0], m.get_right()[0]
            if lo < -LIMIT or hi > LIMIT:
                key = (type(self).__name__, type(m).__name__, round(lo, 1), round(hi, 1))
                if key not in seen:
                    seen.add(key)
                    print(f"[DEBUG][EXTENT] {type(self).__name__} t={t}s {type(m).__name__} x=[{lo:.2f},{hi:.2f}] top={type(top).__name__}@({top.get_center()[0]:.2f},{top.get_center()[1]:.2f}) w={top.width:.2f}", flush=True)
    return out


Scene.play = play
sys.argv = ["manim", *sys.argv[1:]]
main()
