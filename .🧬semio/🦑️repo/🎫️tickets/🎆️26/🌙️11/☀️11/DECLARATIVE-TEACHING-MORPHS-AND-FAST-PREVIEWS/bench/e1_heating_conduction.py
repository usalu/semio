import sys
sys.path.insert(0, ".")
from migrate import T, run
H = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/"
S = "heating.conduction."
def fly(prefix, key, dur):
    return T(f"{S}{prefix}.fly.{key}", "abstraction", f"{S}{prefix}.fly.{key}.readout", f"{S}{prefix}.fly.{key}.slot", "fade", dur)
tr = lambda i, *a, **k: T(S + i, *a, **k)
run(H + "🧱️conduction",
    [("def _fly_into(scene, panel, sources: dict, *, run_time: float = 1.2):", "def _fly_into(scene, stage, prefix, panel, sources: dict, *, run_time: float = 1.2):"),
     ("scene.play(*[FadeTransform(copies[key], items[key]) for key in sources], FadeIn(rest), Create(box),",
      'scene.play(*[stage.transition(f"{prefix}.{key}", source=copies[key], target=items[key]) for key in sources], FadeIn(rest), Create(box),'),
     ('_fly_into(self, p_u, {"frac": r_lbl}, run_time=1.0)', f'_fly_into(self, stage, "{S}u.fly", p_u, {{"frac": r_lbl}}, run_time=1.0)'),
     ('_fly_into(self, p_h, {"u": tiles[12][1], "a": a_lbl}, run_time=1.2)', f'_fly_into(self, stage, "{S}h.fly", p_h, {{"u": tiles[12][1], "a": a_lbl}}, run_time=1.2)'),
     ('_fly_into(self, p_q, {"dt": dt_lbl.copy().clear_updaters()}, run_time=1.0)', f'_fly_into(self, stage, "{S}q.fly", p_q, {{"dt": dt_lbl.copy().clear_updaters()}}, run_time=1.0)'),
     ('_fly_into(self, p_q, {"u": u_reads[0]}, run_time=1.0)', f'_fly_into(self, stage, "{S}compare.fly", p_q, {{"u": u_reads[0]}}, run_time=1.0)'),
     ('_fly_into(self, p_diff, {"d": ext_lbl}, run_time=1.2)', f'_fly_into(self, stage, "{S}bridge.fly", p_diff, {{"d": ext_lbl}}, run_time=1.2)'),
     ('_fly_into(self, p_det, {"psi": sum_line}, run_time=1.2)', f'_fly_into(self, stage, "{S}psi.fly", p_det, {{"psi": sum_line}}, run_time=1.2)'),
     ("ReplacementTransform(VGroup(left_read, right_read), eq_label),", f'stage.transition("{S}equilibrium.readouts-to-label", source=VGroup(left_read, right_read), target=eq_label),'),
     ("ReplacementTransform(subtitle, micro_sub),", f'stage.transition("{S}micro.subtitle-to-micro", source=subtitle, target=micro_sub),'),
     ("self.play(ReplacementTransform(grid_dots, rect), FadeIn(rect_label)", f'self.play(stage.transition("{S}slab.lattice-dots-to-slab", source=grid_dots, target=rect), FadeIn(rect_label)'),
     ('self.play(TransformFromCopy(p_u[2]["u"], u_tile), run_time=0.9)', f'self.play(stage.transition("{S}tile.u-label-to-tile", source=p_u[2]["u"], target=u_tile), run_time=0.9)'),
     ("ReplacementTransform(tile, tiles[0]), run_time=1.0)", f'stage.transition("{S}tile.tile-to-first", source=tile, target=tiles[0]), run_time=1.0)'),
     ("self.play(LaggedStart(*[TransformFromCopy(tiles[0], t) for t in tiles[1:]], lag_ratio=0.08),", f'self.play(stage.transition("{S}tile.first-to-grid", source=[tiles[0]], target=list(tiles[1:])),'),
     ("*[FadeTransform(sec, dev) for sec, dev in zip(sections, devices)],", f'stage.transition("{S}compare.sections-to-devices", source=sections, target=devices),'),
     ("LaggedStart(*[TransformFromCopy(b, s) for b, s in zip(bars, segs)], lag_ratio=0.3),", f'stage.transition("{S}sum.bars-to-stack", source=bars, target=segs),'),
     ('ReplacementTransform(corner.copy(), lines["a"]),', f'stage.transition("{S}corner.corner-to-line", source=corner, target=lines["a"]),'),
     ('self.play(TransformFromCopy(lines["a"], lines["e"]), TransformFromCopy(lines["a"], lines["i"]), FadeIn(tags),', f'self.play(stage.transition("{S}corner.line-to-outside", source=lines["a"], target=lines["e"]), stage.transition("{S}corner.line-to-inside", source=lines["a"], target=lines["i"]), FadeIn(tags),'),
     ("ReplacementTransform(chi_read, sum_line), run_time=0.9)", f'stage.transition("{S}psi.chi-to-sum", source=chi_read, target=sum_line), run_time=0.9)'),
     ("self.play(ReplacementTransform(p_det[0], p_flat[0]), ReplacementTransform(p_det[1], p_flat[1]),", f'self.play(stage.transition("{S}flat.formula-head", source=p_det[0], target=p_flat[0]), stage.transition("{S}flat.formula-box", source=p_det[1], target=p_flat[1]),')],
    [fly("u", "frac", 0.4), fly("h", "u", 0.48), fly("h", "a", 0.48), fly("q", "dt", 0.4), fly("compare", "u", 0.4), fly("bridge", "d", 0.48), fly("psi", "psi", 0.48),
     tr("equilibrium.readouts-to-label", "abstraction", S + "equilibrium.readouts", S + "equilibrium.label", "replace", 1.5),
     tr("micro.subtitle-to-micro", "continuation", S + "micro.subtitle", S + "micro.micro-subtitle", "replace", 1.0),
     tr("slab.lattice-dots-to-slab", "abstraction", S + "slab.lattice-dots", S + "slab.slab", "replace", 1.6),
     tr("tile.u-label-to-tile", "concretization", S + "tile.u-label", S + "tile.u-tile", "replace", 0.9, copy=True),
     tr("tile.tile-to-first", "continuation", S + "tile.tile", S + "tile.first-tile", "replace", 1.0),
     tr("tile.first-to-grid", "concretization", S + "tile.first-tile-seed", S + "tile.grid-tiles", "split", 2.4, relation="one-to-many", corr="pairs", pairs=[["0", "0"]], easing="linear", lag=0.192, copy=True),
     tr("compare.sections-to-devices", "concretization", S + "compare.sections", S + "compare.devices", "fade", 1.4, relation="many-to-many"),
     tr("sum.bars-to-stack", "abstraction", S + "sum.bars", S + "sum.stack-segments", "replace", 2.6, relation="many-to-many", lag=0.78, copy=True),
     tr("corner.corner-to-line", "abstraction", S + "corner.corner", S + "corner.centre-line", "replace", 1.4, copy=True),
     tr("corner.line-to-outside", "continuation", S + "corner.centre-line", S + "corner.outside-line", "replace", 1.2, copy=True),
     tr("corner.line-to-inside", "continuation", S + "corner.centre-line", S + "corner.inside-line", "replace", 1.2, copy=True),
     tr("psi.chi-to-sum", "abstraction", S + "psi.chi-readout", S + "psi.sum-line", "replace", 0.9),
     tr("flat.formula-head", "transformation", S + "flat.detailed-head", S + "flat.flat-head", "replace", 0.9),
     tr("flat.formula-box", "continuation", S + "flat.detailed-box", S + "flat.flat-box", "replace", 0.9)],
    ["Beat1_MakroUndMikro", "Beat2_RWert", "Beat3_UWertUndGradient", "Beat4_WattVergleich", "Beat5_Gebaeudehuelle", "Beat6_Waermebruecken"])
