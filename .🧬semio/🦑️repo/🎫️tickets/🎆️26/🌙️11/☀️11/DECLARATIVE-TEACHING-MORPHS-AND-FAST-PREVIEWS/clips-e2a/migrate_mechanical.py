import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from edit_lib import apply
from gen_json import write

CLIP = Path(sys.argv[1])
P = "ventilation.mechanical"
H, D = "heat-recovery", "duct"

ROWS = [
    (f"{H}.winter-bar-share", "transformation", f"{H}.winter-bar-full", f"{H}.winter-bar-rest", 2.0, False),
    (f"{H}.winter-value-remaining", "transformation", f"{H}.winter-value-full", f"{H}.winter-value-rest", 2.0, False),
    (f"{H}.winter-formula-to-loss", "transformation", f"{H}.winter-formula", f"{H}.loss-formula", 2.0, False),
    (f"{H}.winter-box-to-loss", "transformation", f"{H}.winter-formula-box", f"{H}.loss-formula-box", 2.0, False),
    (f"{H}.season-chip-summer", "transformation", f"{H}.chip-winter", f"{H}.chip-summer", 3.0, False),
    (f"{H}.season-arrows-down", "transformation", f"{H}.arrows-up", f"{H}.arrows-down", 3.0, False),
    (f"{H}.loss-to-efficiency-formula", "transformation", f"{H}.loss-formula", f"{H}.efficiency-formula", 3.0, False),
    (f"{H}.loss-to-efficiency-box", "transformation", f"{H}.loss-formula-box", f"{H}.efficiency-formula-box", 3.0, False),
    (f"{H}.summer-bar-share", "transformation", f"{H}.summer-bar-full", f"{H}.summer-bar-rest", 2.0, False),
    (f"{H}.summer-value-remaining", "transformation", f"{H}.summer-value-full", f"{H}.summer-value-rest", 2.0, False),
    (f"{H}.efficiency-to-sensible-formula", "transformation", f"{H}.efficiency-formula", f"{H}.sensible-formula", 2.0, False),
    (f"{H}.efficiency-to-sensible-box", "transformation", f"{H}.efficiency-formula-box", f"{H}.sensible-formula-box", 2.0, False),
    (f"{H}.sensible-to-latent-formula", "transformation", f"{H}.sensible-formula", f"{H}.latent-formula", 1.6, False),
    (f"{H}.sensible-to-latent-box", "transformation", f"{H}.sensible-formula-box", f"{H}.latent-formula-box", 1.6, False),
    (f"{D}.continuity-to-area-formula", "transformation", f"{D}.continuity-formula", f"{D}.area-formula", 1.2, False),
    (f"{D}.continuity-to-area-box", "transformation", f"{D}.continuity-formula-box", f"{D}.area-formula-box", 1.2, False),
    (f"{D}.area-to-radius-formula", "transformation", f"{D}.area-formula", f"{D}.radius-formula", 1.6, False),
    (f"{D}.area-to-radius-box", "transformation", f"{D}.area-formula-box", f"{D}.radius-formula-box", 1.6, False),
]
write(CLIP.parent, P, ROWS)


def t(name, source, target):
    return f'stage.transition("{P}.{name}", source={source}, target={target})'


STAGE = "        stage = _morph.stage(self, TRANSITIONS)\n"
apply(CLIP, [
    ("fitted_staffage = _manim_module.fitted_staffage\n",
     'fitted_staffage = _manim_module.fitted_staffage\n_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'),
    ("class Beat2_Waermerueckgewinnung", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("class Beat4_Kanalquerschnitt", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("ReplacementTransform(w_full, w_rest)", t(f"{H}.winter-bar-share", "w_full", "w_rest")),
    ("ReplacementTransform(w_val, w_rest_val)", t(f"{H}.winter-value-remaining", "w_val", "w_rest_val")),
    ("ReplacementTransform(eq, lp[0])", t(f"{H}.winter-formula-to-loss", "eq", "lp[0]")),
    ("ReplacementTransform(eq_box, lp[1])", t(f"{H}.winter-box-to-loss", "eq_box", "lp[1]")),
    ("ReplacementTransform(chip_winter, chip_summer)", t(f"{H}.season-chip-summer", "chip_winter", "chip_summer")),
    ("ReplacementTransform(arrows_up, arrows_down)", t(f"{H}.season-arrows-down", "arrows_up", "arrows_down")),
    ("ReplacementTransform(lp[0], eq2)", t(f"{H}.loss-to-efficiency-formula", "lp[0]", "eq2")),
    ("ReplacementTransform(lp[1], eq2_box)", t(f"{H}.loss-to-efficiency-box", "lp[1]", "eq2_box")),
    ("ReplacementTransform(s_full, s_rest)", t(f"{H}.summer-bar-share", "s_full", "s_rest")),
    ("ReplacementTransform(s_val, s_rest_val)", t(f"{H}.summer-value-remaining", "s_val", "s_rest_val")),
    ("ReplacementTransform(eq2, sp[0])", t(f"{H}.efficiency-to-sensible-formula", "eq2", "sp[0]")),
    ("ReplacementTransform(eq2_box, sp[1])", t(f"{H}.efficiency-to-sensible-box", "eq2_box", "sp[1]")),
    ("ReplacementTransform(sp[0], lp2[0])", t(f"{H}.sensible-to-latent-formula", "sp[0]", "lp2[0]")),
    ("ReplacementTransform(sp[1], lp2[1])", t(f"{H}.sensible-to-latent-box", "sp[1]", "lp2[1]")),
    ("ReplacementTransform(cp, ap)", t(f"{D}.continuity-to-area-formula", "cp", "ap")),
    ("ReplacementTransform(cp_box, ap_box)", t(f"{D}.continuity-to-area-box", "cp_box", "ap_box")),
    ("ReplacementTransform(ap, rp)", t(f"{D}.area-to-radius-formula", "ap", "rp")),
    ("ReplacementTransform(ap_box, rp_box)", t(f"{D}.area-to-radius-box", "ap_box", "rp_box")),
])
