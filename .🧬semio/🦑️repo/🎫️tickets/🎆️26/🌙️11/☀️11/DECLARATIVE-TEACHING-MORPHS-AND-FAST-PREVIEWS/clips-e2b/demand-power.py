import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from doc import DEMAND, entry as e, write
from patch import apply

fills = {
    "energy": {"q": 0, "p": 0, "dt": 0, "u1": 0, "u2": 0, "u3": 0},
    "heat-load": {"t": 0, "v": 0, "d": 0, "phi": 0},
    "duration": {"f": 1, "res": 0, "t": 0},
    "sizing": {"q": 0},
}
rows = [
    e("demand.power.energy.readout-to-annual-sum", "transformation", "demand.power.energy.readout", "demand.power.energy.annual-sum", 1.2),
    e("demand.power.heat-load.room-frame-up", "continuation", "demand.power.heat-load.room-lower", "demand.power.heat-load.room-upper", 1.6),
    e("demand.power.duration.curve-sorted", "transformation", "demand.power.duration.curve-by-month", "demand.power.duration.curve-sorted", 4.0),
    e("demand.power.duration.area-sorted", "transformation", "demand.power.duration.area-by-month", "demand.power.duration.area-sorted", 4.0),
    e("demand.power.duration.readout-to-annual-demand", "transformation", "demand.power.duration.readout", "demand.power.duration.annual-demand", 1.0),
    e("demand.power.duration.sweep-to-rectangle", "transformation", "demand.power.duration.sweep", "demand.power.duration.rectangle", 2.0),
    e("demand.power.sizing.readout-to-annual-demand", "transformation", "demand.power.sizing.readout", "demand.power.sizing.annual-demand", 1.0),
]
for beat, keys in fills.items():
    for key, many in keys.items():
        rows.append(e(f"demand.power.{beat}.fill.{key}", "abstraction", f"demand.power.{beat}.fill.{key}.reading", f"demand.power.{beat}.fill.{key}.slot", 1.5,
                      relation="many-to-many" if many else "one-to-one"))
write("⚡️demand-power", rows)

CLIP = DEMAND / "⚡️demand-power/🐍️.py"
apply(CLIP, [
    ('def _fill_panel(scene, panel, sources, *, run_time: float = 2.0):', 'def _twin(scene, src):\n    """👯 Static on-screen copy of a readout (or of a list of readouts) that flies into a formula slot while the readout stays."""\n    twin = VGroup(*[_snap(m) for m in src]) if isinstance(src, (list, tuple)) else _snap(src)\n    scene.add(twin)\n    return twin\n\n\ndef _fill_panel(scene, stage, prefix, panel, sources, *, run_time: float = 2.0):'),
    ('    ``sources`` maps a slot key to the on-screen readout it is read from; a copy of the readout morphs into its slot.',
     '    ``sources`` maps a slot key to the on-screen readout it is read from; a copy of the readout morphs into its slot\n    through the declared transition ``prefix.key``.'),
    ('    scene.play(*[TransformFromCopy(VGroup(*[_snap(m) for m in src]) if isinstance(src, (list, tuple)) else _snap(src),\n                                   items[key]) for key, src in sources.items()], run_time=run_time * 0.75)',
     '    scene.play(*[stage.transition(f"{prefix}.{key}", source=_twin(scene, src), target=items[key]) for key, src in sources.items()], run_time=run_time * 0.75)'),
    ('_fill_panel(self, panel, {"q": q_final, "p": p_now,', '_fill_panel(self, stage, "demand.power.energy.fill", panel, {"q": q_final, "p": p_now,'),
    ('_fill_panel(self, panel, {"t": t_lbl,', '_fill_panel(self, stage, "demand.power.heat-load.fill", panel, {"t": t_lbl,'),
    ('_fill_panel(self, panel, {"f": [q_final, phi_read]', '_fill_panel(self, stage, "demand.power.duration.fill", panel, {"f": [q_final, phi_read]'),
    ('_fill_panel(self, panel, {"q": q_final})', '_fill_panel(self, stage, "demand.power.sizing.fill", panel, {"q": q_final})'),
    ('self.play(ReplacementTransform(q_read, q_final), run_time=1.2)', 'self.play(stage.transition("demand.power.energy.readout-to-annual-sum", source=q_read, target=q_final), run_time=1.2)'),
    ('self.play(ReplacementTransform(rooms[0], rooms[1]), run_time=1.6)', 'self.play(stage.transition("demand.power.heat-load.room-frame-up", source=rooms[0], target=rooms[1]), run_time=1.6)'),
    ('        self.play(Transform(curve, sorted_curve), Transform(area, sorted_area), FadeOut(months), FadeIn(hours),\n                  run_time=4.0)\n',
     '        self.play(stage.transition("demand.power.duration.curve-sorted", source=curve, target=sorted_curve),\n                  stage.transition("demand.power.duration.area-sorted", source=area, target=sorted_area), FadeOut(months), FadeIn(hours),\n                  run_time=4.0)\n        curve, area = sorted_curve, sorted_area\n'),
    ('        self.play(ReplacementTransform(q_read, q_final), run_time=1.0)\n        hold_for(self, N, "flaeche_bedarf")',
     '        self.play(stage.transition("demand.power.duration.readout-to-annual-demand", source=q_read, target=q_final), run_time=1.0)\n        hold_for(self, N, "flaeche_bedarf")'),
    ('self.play(ReplacementTransform(sweep, rect), FadeIn(phi_read), FadeIn(t_read), run_time=2.0)', 'self.play(stage.transition("demand.power.duration.sweep-to-rectangle", source=sweep, target=rect), FadeIn(phi_read), FadeIn(t_read), run_time=2.0)'),
    ('        self.play(ReplacementTransform(q_read, q_final), run_time=1.0)\n        hold_for(self, N, "bedarf")',
     '        self.play(stage.transition("demand.power.sizing.readout-to-annual-demand", source=q_read, target=q_final), run_time=1.0)\n        hold_for(self, N, "bedarf")'),
], scenes=("Beat1_LeistungUndEnergie", "Beat2_Normheizlast", "Beat3_Jahresdauerlinie", "Beat4_RichtigAuslegen"))
