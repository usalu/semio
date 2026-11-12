import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from edit_lib import apply
from gen_json import write

CLIP = Path(sys.argv[1])
P = "ventilation.heat-flow"
A, S, L = "air-heat", "sensible", "latent"

ROWS = [
    (f"{A}.cube-warms", "transformation", f"{A}.cube", f"{A}.cube-warm", 3.0, False, "linear", "many-to-many"),
    (f"{A}.joules-per-m3-k", "abstraction", f"{A}.reading-q", f"{A}.c-joules", 0.9, True),
    (f"{A}.wh-per-m3-k", "abstraction", f"{A}.reading-wh", f"{A}.c-wh", 0.9, True),
    (f"{S}.formula-dtheta", "abstraction", f"{S}.name-dtheta", f"{S}.slot-dtheta", 1.0, True),
    (f"{S}.season-chip-summer", "transformation", f"{S}.chip-winter", f"{S}.chip-summer", 3.2, False),
    (f"{S}.label-gain", "transformation", f"{S}.label-loss", f"{S}.label-gain", 3.2, False),
    (f"{S}.guides-summer", "transformation", f"{S}.guides-winter", f"{S}.guides-summer", 3.2, False),
    (f"{L}.formula-dx", "abstraction", f"{L}.label-dx", f"{L}.slot-dx", 1.0, True),
]
write(CLIP.parent, P, ROWS)


def t(name, source, target):
    return f'stage.transition("{P}.{name}", source={source}, target={target})'


STAGE = "        stage = _morph.stage(self, TRANSITIONS)\n"
apply(CLIP, [
    ("open_window = _manim_module.open_window\n",
     'open_window = _manim_module.open_window\n_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'),
    ("class Beat1_Luftwaerme", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("class Beat2_SensibleWaerme", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("class Beat3_LatenteLast", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("*[Transform(a, b) for a, b in zip(cube, warm)]", t(f"{A}.cube-warms", "cube", "warm")),
    ("_hide(self, cube, cube_label,", "_hide(self, warm, cube_label,"),
    ("TransformFromCopy(q_snap, c_j)", t(f"{A}.joules-per-m3-k", "q_snap", "c_j")),
    ("TransformFromCopy(wh_snap, c_wh)", t(f"{A}.wh-per-m3-k", "wh_snap", "c_wh")),
    ('TransformFromCopy(dt_name[0], items["dt"])', t(f"{S}.formula-dtheta", "dt_name[0]", 'items["dt"]')),
    ("ReplacementTransform(chip_w, chip_s)", t(f"{S}.season-chip-summer", "chip_w", "chip_s")),
    ("Transform(q_name[1], gain)", t(f"{S}.label-gain", "q_name[1]", "gain")),
    ('ReplacementTransform(guides, _guides([(house, "summer")]))', t(f"{S}.guides-summer", "guides", '_guides([(house, "summer")])')),
    ('TransformFromCopy(dx_lbl, items["dx"])', t(f"{L}.formula-dx", "dx_lbl", 'items["dx"]')),
])
