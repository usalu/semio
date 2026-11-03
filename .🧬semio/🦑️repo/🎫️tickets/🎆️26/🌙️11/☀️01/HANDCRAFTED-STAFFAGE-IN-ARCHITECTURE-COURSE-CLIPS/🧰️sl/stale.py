import importlib

import numpy as np
from manim import FadeIn, Scene, ValueTracker, VGroup, Circle

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


def readout(fn, at, edge="left"):
    holder = VGroup()
    state = {"text": None}

    def refresh(mob):
        text = fn()
        if text == state["text"]:
            return
        state["text"] = text
        mob.become(V.math_label(text, np.array(at), size=24, color="#ffffff", edge=edge))

    holder.add_updater(refresh)
    refresh(holder)
    return holder


class Stale(Scene):
    def construct(self):
        t = ValueTracker(0.0)
        lbl = readout(lambda: rf"\text{{Wärme}}\;{V.de_num(t.get_value())}\,\mathrm{{kWh}}", (2.0, 0.0, 0.0), "right")
        self.play(FadeIn(Circle(radius=0.2)), run_time=0.5)
        self.add(lbl)
        self.play(t.animate.set_value(100.0), Circle().animate.scale(1.0), run_time=2.0)
        self.wait(0.5)
