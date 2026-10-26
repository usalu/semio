"""🧪 Check skipped-render timing with trackers, updaters, and authored hold logic."""
import json
import os
from pathlib import Path
import sys
from manim import BLUE, GREEN, Dot, RIGHT, Scene, ValueTracker

TICKET = Path(__file__).resolve().parent
CASES = json.loads((TICKET / "recovery-cases.json").read_text(encoding="utf-8"))
sys.path.insert(0, os.environ["RECOVERY_TEACHING_ROOT"])
from manim_visuals import begin_vo_beat, hold_for


class RecoveryFixture(Scene):
    """🎞️ Advance two transforms and one narration-budget wait."""

    def construct(self):
        """⏱️ Record exact renderer clocks and object state at every boundary."""
        tracker = ValueTracker(0)
        moving = Dot(color=BLUE).add_updater(lambda dot: dot.move_to(tracker.get_value() * RIGHT))
        clock = Dot(color=GREEN).shift(-3 * RIGHT)
        clock.add_updater(lambda dot, dt: dot.shift(CASES["updater_speed"] * dt * RIGHT))
        self.add(moving, clock)
        begin_vo_beat(self, "RecoveryFixture")
        self._vo_caption_t0 = 0.0
        records = []

        def record():
            records.append({"plays": self.renderer.num_plays, "time": self.renderer.time, "tracker": float(tracker.get_value()), "moving": moving.get_center().tolist(), "clock": clock.get_center().tolist()})
            Path(os.environ["RECOVERY_TRACE"]).write_text(json.dumps(records, indent=2), encoding="utf-8")

        self.play(tracker.animate.set_value(1), run_time=CASES["animation_durations"][0])
        record()
        self.play(tracker.animate.set_value(2), run_time=CASES["animation_durations"][1])
        record()
        hold_for(self, CASES["narration"], "line")
        record()
