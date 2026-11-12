import sys
sys.path.insert(0, ".")
from migrate import T, run
H = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/"
S = "heating.solar-gains."
fly = lambda scene, key: T(f"{S}{scene}.fly.{key}", "abstraction", f"{S}{scene}.fly.{key}.readout", f"{S}{scene}.fly.{key}.slot", "fade", 0.48)
run(H + "☀️solar-gains",
    [("def _fly_into(scene, panel, sources, *, run_time: float = 1.2):", "def _fly_into(scene, stage, prefix, panel, sources, *, run_time: float = 1.2):"),
     ("scene.play(*[FadeTransform(copies[key], items[key]) for key in copies], run_time=run_time * 0.4)",
      'scene.play(*[stage.transition(f"{prefix}.{key}", source=copies[key], target=items[key]) for key in copies], run_time=run_time * 0.4)'),
     ('_fly_into(self, panel, {"g": g_lbl, "a": a_side, "ff": ff_lbl})', f'_fly_into(self, stage, "{S}ff.fly", panel, {{"g": g_lbl, "a": a_side, "ff": ff_lbl}})'),
     ('_fly_into(self, panel, {"g": g_sym})', f'_fly_into(self, stage, "{S}g.fly", panel, {{"g": g_sym}})'),
     ('_fly_into(self, panel, {"fsh": sym})', f'_fly_into(self, stage, "{S}fsh.fly", panel, {{"fsh": sym}})'),
     ("self.play(TransformFromCopy(opening, glass), ReplacementTransform(a_lbl, ff_lbl),",
      f'self.play(stage.transition("{S}ff.glass-from-opening", source=opening, target=glass), stage.transition("{S}ff.area-label-to-frame-factor", source=a_lbl, target=ff_lbl),'),
     ("FadeTransform(tag_summer, tag_winter),", f'stage.transition("{S}season.tag-summer-to-winter", source=tag_summer, target=tag_winter),'),
     ("*[Transform(r, c) for r, c in zip(direct, blocked)],", f'stage.transition("{S}blind.rays-blocked", source=direct, target=blocked),'),
     ("FadeOut(residual), FadeOut(direct), run_time=1.2)", "FadeOut(residual), FadeOut(blocked), run_time=1.2)"),
     ("*[ReplacementTransform(tok, slot) for tok, slot in zip(tokens, slots)],", f'stage.transition("{S}main.tokens-into-formula", source=tokens, target=slots),')],
    [fly("ff", "g"), fly("ff", "a"), fly("ff", "ff"), fly("g", "g"), fly("fsh", "fsh"),
     T(S + "ff.glass-from-opening", "concretization", S + "ff.opening-outline", S + "ff.glass-pane", "replace", 1.0, copy=True),
     T(S + "ff.area-label-to-frame-factor", "transformation", S + "ff.area-label", S + "ff.frame-factor-label", "replace", 1.0),
     T(S + "season.tag-summer-to-winter", "transformation", S + "season.tag-summer", S + "season.tag-winter", "fade", 3.0),
     T(S + "blind.rays-blocked", "transformation", S + "blind.direct-rays", S + "blind.blocked-rays", "replace", 1.6, relation="many-to-many"),
     T(S + "main.tokens-into-formula", "abstraction", S + "main.factor-tokens", S + "main.formula-slots", "replace", 1.6, relation="many-to-many")],
    ["Beat2_BestrahlungUndFlaeche", "Beat3_GWert", "Beat4_SaisonaleWinkel", "Beat5_Verschattung", "Beat8_Hauptgleichung"])
