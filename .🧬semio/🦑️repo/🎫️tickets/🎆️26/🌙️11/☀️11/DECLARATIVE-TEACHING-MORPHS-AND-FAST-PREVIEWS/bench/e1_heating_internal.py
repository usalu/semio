import sys
sys.path.insert(0, ".")
from migrate import T, run
H = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/"
P = "heating.internal-gains.sum."
run(H + "👥️internal-gains",
    [("*[ReplacementTransform(tok, target) for tok, target in zip(tokens, targets)],",
      f'stage.transition("{P}source-tokens-into-formula", source=tokens, target=targets),'),
     ("self.play(ReplacementTransform(subtitle, density_sub), run_time=0.7)",
      f'self.play(stage.transition("{P}subtitle-to-density", source=subtitle, target=density_sub), run_time=0.7)'),
     ("self.play(ReplacementTransform(row, row2), ReplacementTransform(box, box2), run_time=1.4)",
      f'self.play(stage.transition("{P}formula-to-density", source=row, target=row2), stage.transition("{P}formula-box-to-density", source=box, target=box2), run_time=1.4)')],
    [T(P + "source-tokens-into-formula", "abstraction", "heating.internal-gains.source-tokens", "heating.internal-gains.formula-terms", "replace", 1.6, relation="many-to-many"),
     T(P + "subtitle-to-density", "continuation", "heating.internal-gains.subtitle", "heating.internal-gains.density-subtitle", "replace", 0.7),
     T(P + "formula-to-density", "transformation", "heating.internal-gains.sum-formula", "heating.internal-gains.density-formula", "replace", 1.4),
     T(P + "formula-box-to-density", "continuation", "heating.internal-gains.sum-formula-box", "heating.internal-gains.density-formula-box", "replace", 1.4)],
    ["Beat5_SummeUndDichte"])
