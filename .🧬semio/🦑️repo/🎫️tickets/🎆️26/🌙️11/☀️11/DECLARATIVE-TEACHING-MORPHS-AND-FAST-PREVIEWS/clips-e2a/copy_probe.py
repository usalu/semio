"""Probe: does a copy:true transition keep its source in the scene under both profiles, and what is visible per frame?"""

import importlib
import os
import sys
from pathlib import Path

import numpy as np

manim = importlib.import_module("manim")
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
DOC = Path("🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🔀️transitions/🔣️.json")
ID = "ventilation.volume.gross-net.bar-gross"
profile = sys.argv[1]
os.environ["SEMIO_TEACHING_PROFILE"] = profile
log = []


class Probe(manim.Scene):
    def construct(self):
        stage = morph.stage(self, DOC)
        source = manim.Square(1.0, color=manim.BLUE).shift(manim.LEFT * 3)
        target = manim.Rectangle(width=3.0, height=0.3, color=manim.ORANGE).shift(manim.RIGHT * 2 + manim.DOWN)
        self.add(source)
        animation = stage.transition(ID, source=source, target=target)
        original = animation.interpolate
        run_time = animation.run_time

        def spy(alpha):
            original(alpha)
            log.append((round(alpha, 3), source in self.mobjects, target in self.mobjects, round(float(source.get_center()[0]), 3)))

        animation.interpolate = spy
        self.play(animation)
        log.append(("end", source in self.mobjects, target in self.mobjects, round(float(source.get_center()[0]), 3)))


with manim.tempconfig({"write_to_movie": False, "disable_caching": True, "verbosity": "WARNING", "progress_bar": "none", "frame_rate": 10, "pixel_width": 96, "pixel_height": 54}):
    Probe().render()
for row in log:
    print(row)
