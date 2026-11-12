import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from edit_lib import apply
from gen_json import write

CLIP = Path(sys.argv[1])
P = "ventilation.handoff"
S = "summary"

ROWS = [
    (f"{S}.vdot-v", "abstraction", f"{S}.card-v-symbol", f"{S}.vdot-v", 1.6, True),
    (f"{S}.vdot-n", "abstraction", f"{S}.card-n-symbol", f"{S}.vdot-n", 1.6, True),
    (f"{S}.card-vdot", "abstraction", f"{S}.vdot-formula-symbol", f"{S}.card-vdot-symbol", 1.2, True),
    *[(f"{S}.sens-{k}", "abstraction", f"{S}.card-{k}-symbol", f"{S}.sens-{k}", 1.8, True) for k in ("vdot", "c", "dt")],
    *[(f"{S}.lat-{k}", "abstraction", f"{S}.card-{k}-symbol", f"{S}.lat-{k}", 1.8, True) for k in ("vdot", "dx")],
    (f"{S}.wrg-eta", "abstraction", f"{S}.card-eta-symbol", f"{S}.wrg-eta", 1.8, True),
]
write(CLIP.parent, P, ROWS)


def t(name, source, target):
    return f'stage.transition("{P}.{S}.{name}", source={source}, target={target})'


apply(CLIP, [
    ("open_window = _manim_module.open_window\n",
     'open_window = _manim_module.open_window\n_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'),
    ("class Beat1_Zusammenfassung", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n        stage = _morph.stage(self, TRANSITIONS)\n"),
    ('TransformFromCopy(cards["v"]["sym"], it1["v"])', t("vdot-v", 'cards["v"]["sym"]', 'it1["v"]')),
    ('TransformFromCopy(cards["n"]["sym"], it1["n"])', t("vdot-n", 'cards["n"]["sym"]', 'it1["n"]')),
    ('TransformFromCopy(it1["vdot"], cards["vdot"]["sym"])', t("card-vdot", 'it1["vdot"]', 'cards["vdot"]["sym"]')),
    ('*[TransformFromCopy(cards[k]["sym"], it2[k]) for k in ("vdot", "c", "dt")]',
     f'*[stage.transition(f"{P}.{S}.sens-{{k}}", source=cards[k]["sym"], target=it2[k]) for k in ("vdot", "c", "dt")]'),
    ('*[TransformFromCopy(cards[k]["sym"], it3[k]) for k in ("vdot", "dx")]',
     f'*[stage.transition(f"{P}.{S}.lat-{{k}}", source=cards[k]["sym"], target=it3[k]) for k in ("vdot", "dx")]'),
    ('TransformFromCopy(cards["eta"]["sym"], it4["eta"])', t("wrg-eta", 'cards["eta"]["sym"]', 'it4["eta"]')),
])
