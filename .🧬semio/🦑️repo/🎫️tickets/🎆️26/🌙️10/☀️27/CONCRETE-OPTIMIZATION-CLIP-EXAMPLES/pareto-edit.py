"""Apply a list of exact text replacements to the pareto scene (run with -I and a path argument)."""
import sys
from pathlib import Path

target = Path(sys.argv[1])
text = target.read_text(encoding="utf-8")
pairs = [
    ('(2.85, 1.25, 0), .55, (6.2, .9, 0), 2.35', '(2.85, .95, 0), .55, (6.2, .75, 0), 2.35'),
    ('        labels[0].move_to([s.axes.get_center()[0], s.axes.get_bottom()[1] - .52, 0])\n',
     '        labels[0].move_to([s.axes.get_center()[0], s.axes.get_bottom()[1] - .52, 0])\n        for tick in labels[2:]:\n            if tick.get_center()[0] < s.axes.get_left()[0] - .05:\n                tick.shift(mn.LEFT * .08)\n'),
    ('        self.step("grid", mn.FadeIn(total), mn.LaggedStart(*[mn.FadeIn(tile) for tile in s.tiles.values()], lag_ratio=.07),',
     '        s.plain = self.live_marker(s.axes, s.focus, lambda d: _room.result(d)["cost"], lambda d: _room.result(d)["electricity"], x_unit="CHF", y_unit="kWh/a", color=COST, readout=False)\n        self.step("grid", mn.FadeIn(total), mn.FadeOut(s.better), mn.FadeOut(s.cursor), mn.FadeIn(s.plain), mn.LaggedStart(*[mn.FadeIn(tile) for tile in s.tiles.values()], lag_ratio=.07),'),
    ('mn.FadeIn(s.ideal, run_time=.6), mn.FadeOut(s.better), seconds=3.2)', 'mn.FadeIn(s.ideal, run_time=.6), seconds=3.2)'),
    ('        plain = self.live_marker(s.axes, s.focus, lambda d: _room.result(d)["cost"], lambda d: _room.result(d)["electricity"], x_unit="CHF", y_unit="kWh/a", color=COST, readout=False)\n        self.step("dominated", mn.FadeOut(s.ideal), mn.FadeOut(s.cursor), mn.FadeIn(plain), self.set_design(s.focus, loser["design"], 1.5), seconds=1.5)\n        s.cursor = plain\n',
     '        self.step("dominated", mn.FadeOut(s.ideal), self.set_design(s.focus, loser["design"], 1.5), seconds=1.5)\n'),
    ('(2.8, -.7, 0), size=16, color=GLASS, width=4.4)', '(2.8, -1.0, 0), size=16, color=GLASS, width=4.4)'),
    ('mn.FadeOut(isoline), mn.FadeOut(bar), mn.FadeOut(s.cursor),', 'mn.FadeOut(isoline), mn.FadeOut(bar), mn.FadeOut(s.plain),'),
]
for old, new in pairs:
    if old not in text:
        raise SystemExit("missing: " + old[:70])
    text = text.replace(old, new)
target.write_text(text, encoding="utf-8")
print("ok")
