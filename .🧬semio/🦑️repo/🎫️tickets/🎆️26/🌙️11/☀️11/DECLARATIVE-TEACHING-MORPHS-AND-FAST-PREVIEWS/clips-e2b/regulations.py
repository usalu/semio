import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from doc import DEMAND, entry as e, write
from patch import apply

CLIP = DEMAND / "⚖️regulations/🐍️.py"
source = CLIP.read_text(encoding="utf-8")
lines = source.splitlines()
start = next(i for i, l in enumerate(lines) if l.startswith("THETA_E ="))
end = next(i for i, l in enumerate(lines) if l.startswith("REST_MONTHS ="))
scope = {}
exec("\n".join(lines[start:end + 1]), scope)
n = len(scope["REST_MONTHS"])
total = 1.8
duration = total / (1 + (n - 1) * 0.1)
lag = 0.1 * duration
print("rest months", n, "duration", duration, "lag", lag)

B = "demand.regulations.balance."
write("⚖️regulations", [
    e(B + "rest-shrinks-by-credit", "transformation", B + "rest-cells", B + "rest-cells-credited", 1.6, relation="many-to-many"),
    e(B + "rest-restores-full", "transformation", B + "rest-cells-credited", B + "rest-cells-full", 0.8, relation="many-to-many"),
    e(B + "months-into-annual-bar", "abstraction", B + "rest-cells-full", B + "annual-bar", duration, relation="many-to-many", lag=lag),
    e(B + "ring-to-sum", "continuation", B + "ring-annual", B + "ring-sum", 1.0),
    e(B + "ring-to-sink", "continuation", B + "ring-sum", B + "ring-sink", 1.0),
    e(B + "ring-to-eta", "continuation", B + "ring-sink", B + "ring-eta", 1.0),
    e("demand.regulations.climate.years-to-mean", "abstraction", "demand.regulations.climate.yearly-curves", "demand.regulations.climate.mean-curves", 3.0, relation="many-to-many"),
])

apply(CLIP, [
    ('        self.play(Transform(rest, shrunk), run_time=1.6)\n',
     f'        self.play(stage.transition("{B}rest-shrinks-by-credit", source=rest, target=shrunk), run_time=1.6)\n'),
    ('self.play(FadeOut(wrong), FadeOut(warn), Transform(rest, full), run_time=0.8)',
     f'self.play(FadeOut(wrong), FadeOut(warn), stage.transition("{B}rest-restores-full", source=shrunk, target=full), run_time=0.8)'),
    ('self.play(LaggedStart(*[ReplacementTransform(c, b) for c, b in zip(rest_cells, bar["bars"])], lag_ratio=0.1),\n                  FadeIn(total), run_time=1.8)',
     f'self.play(stage.transition("{B}months-into-annual-bar", source=full, target=bar["bars"]),\n                  FadeIn(total), run_time=1.8)'),
    ('self.play(ReplacementTransform(ring, ring2), Indicate', f'self.play(stage.transition("{B}ring-to-sum", source=ring, target=ring2), Indicate'),
    ('self.play(ReplacementTransform(ring2, ring3), Indicate', f'self.play(stage.transition("{B}ring-to-sink", source=ring2, target=ring3), Indicate'),
    ('self.play(ReplacementTransform(ring3, ring4), Indicate', f'self.play(stage.transition("{B}ring-to-eta", source=ring3, target=ring4), Indicate'),
    ('        self.play(Transform(years, mean), run_time=3.0)\n        self.remove(years)\n',
     '        self.play(stage.transition("demand.regulations.climate.years-to-mean", source=years, target=mean), run_time=3.0)\n        self.remove(mean)\n'),
], scenes=("Beat2_Monatsbilanz", "Beat3_Klimadaten"))
