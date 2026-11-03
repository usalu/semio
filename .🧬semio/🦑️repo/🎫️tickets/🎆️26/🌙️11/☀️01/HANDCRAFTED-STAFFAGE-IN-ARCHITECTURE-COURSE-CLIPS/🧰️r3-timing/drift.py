"""Measure a Motion's elapsed against Manim's renderer time across wait/play/remove/add."""
import importlib
import json
import sys
import tempfile
from pathlib import Path

import numpy as np
from manim import Scene, FadeIn, FadeOut, config, Circle

sys.path.insert(0, str(Path(sys.argv[1])))
T = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️")
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
config.disable_caching = True
config.frame_rate = 15
config.write_to_movie = False
config.dry_run = True


class Probe(Scene):
    def construct(self):
        root = tempfile.mkdtemp()
        folder = T.synthetic(Path(root))
        S._INDEXES.pop(folder, None)
        motion = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
        self.add(motion)
        log = []
        def mark(label):
            log.append({"label": label, "renderer": float(self.renderer.time), "elapsed": float(motion.elapsed)})
        mark("start")
        self.wait(1.0); mark("wait1.0")
        self.wait(0.33); mark("wait0.33")
        self.play(FadeIn(Circle()), run_time=0.7); mark("play0.7")
        self.play(FadeOut(Circle()), run_time=0.41); mark("play0.41")
        self.remove(motion); self.wait(1.0); self.add(motion); self.wait(0.5); mark("removed1.0-readd-0.5")
        print("[DEBUG]", json.dumps(log, indent=1))


Probe().render()
