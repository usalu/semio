import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from edit_lib import apply
from gen_json import write

CLIP = Path(sys.argv[1])
P = "ventilation.volume"

G, A, F = "gross-net", "air-change", "flow"
ROWS = [
    (f"{G}.bar-gross", "abstraction", f"{G}.outline", f"{G}.bar-gross", 1.0, True),
    (f"{G}.bar-net", "abstraction", f"{G}.room-edges", f"{G}.bar-net", 1.0, True),
    (f"{G}.formula-v", "abstraction", f"{G}.symbol-v", f"{G}.slot-v", 0.8, True),
    (f"{G}.formula-ve", "abstraction", f"{G}.label-ve", f"{G}.slot-ve", 0.8, True),
    (f"{A}.flow-reading", "abstraction", f"{A}.fresh-air-sum", f"{A}.flow-reading", 0.9, True),
    (f"{A}.formula-v", "abstraction", f"{A}.label-v", f"{A}.slot-v", 1.0, True),
    (f"{A}.formula-n", "abstraction", f"{A}.label-n", f"{A}.slot-n", 1.0, True),
    (f"{A}.formula-flow", "abstraction", f"{A}.flow-reading", f"{A}.slot-flow", 1.0, True),
    *[(f"{A}.use-{i}-dot", "abstraction", f"{A}.use-{i}-glyph", f"{A}.use-{i}-dot", 1.0, False) for i in range(7)],
    *[(f"{A}.use-{i}-label", "abstraction", f"{A}.use-{i}-caption", f"{A}.use-{i}-label", 1.0, False) for i in range(7)],
    (f"{A}.renew-minutes", "concretization", f"{A}.use-6-label", f"{A}.renew-minutes", 1.5, True),
    (f"{F}.formula-v", "abstraction", f"{F}.reading-v", f"{F}.slot-v", 1.0, True),
    (f"{F}.formula-n", "abstraction", f"{F}.reading-n", f"{F}.slot-n", 1.0, True),
    (f"{F}.formula-flow", "abstraction", f"{F}.reading-flow", f"{F}.slot-flow", 1.0, True),
    (f"{F}.per-second", "transformation", f"{F}.reading-flow", f"{F}.per-second", 1.0, True),
    (f"{F}.card-heating", "transformation", f"{F}.reading-flow", f"{F}.card-heating-value", 0.9, False),
    (f"{F}.card-cooling", "transformation", f"{F}.per-second", f"{F}.card-cooling-value", 1.2, False),
    (f"{F}.spell-vn", "concretization", f"{F}.card-heating-formula", f"{F}.spell-vn", 1.2, True),
    (f"{F}.spell-h", "concretization", f"{F}.card-heating-value", f"{F}.spell-h", 1.2, True),
    (f"{F}.spell-q", "concretization", f"{F}.card-cooling-formula", f"{F}.spell-q", 1.2, True),
    (f"{F}.spell-s", "concretization", f"{F}.card-cooling-value", f"{F}.spell-s", 1.2, True),
    (f"{F}.bar-people-total", "transformation", f"{F}.card-heating-value", f"{F}.bar-total", 0.9, False),
    *[(f"{F}.person-{i}", "abstraction", f"{F}.person-{i}-still", f"{F}.person-{i}-share", 0.8, True) for i in range(4)],
    (f"{F}.cover-people", "abstraction", f"{F}.people-label", f"{F}.cover-people", 1.0, True),
    (f"{F}.cover-total", "abstraction", f"{F}.total-label", f"{F}.cover-total", 1.0, True),
]
write(CLIP.parent, P, [(r[0], *r[1:]) for r in ROWS])


def t(name, source, target, var="stage"):
    return f'{var}.transition("{P}.{name}", source={source}, target={target})'


STAGE = "        stage = _morph.stage(self, TRANSITIONS)\n"
apply(CLIP, [
    ("staffage = _manim_module.staffage\n",
     'staffage = _manim_module.staffage\n_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'),
    ("class Beat1_Innenvolumen", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("class Beat2_Luftwechselrate", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n        morphs = _morph.stage(self, TRANSITIONS)\n"),
    ("class Beat3_Volumenstrom", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("TransformFromCopy(outline, bar_e)", t(f"{G}.bar-gross", "outline", "bar_e")),
    ("TransformFromCopy(room_edges, bar_v)", t(f"{G}.bar-net", "room_edges", "bar_v")),
    ('TransformFromCopy(v_sym, items["v"])', t(f"{G}.formula-v", "v_sym", 'items["v"]')),
    ('TransformFromCopy(lbl_e, items["ve"])', t(f"{G}.formula-ve", "lbl_e", 'items["ve"]')),
    ("TransformFromCopy(snap, flow_read)", t(f"{A}.flow-reading", "snap", "flow_read", "morphs")),
    ('TransformFromCopy(v_lbl, items["v"])', t(f"{A}.formula-v", "v_lbl", 'items["v"]', "morphs")),
    ('TransformFromCopy(n_lbl, items["n"])', t(f"{A}.formula-n", "n_lbl", 'items["n"]', "morphs")),
    ('TransformFromCopy(flow_read, items["vd"])', t(f"{A}.formula-flow", "flow_read", 'items["vd"]', "morphs")),
    ("ReplacementTransform(glyph, dots[idx])", f'morphs.transition(f"{P}.{A}.use-{{idx}}-dot", source=glyph, target=dots[idx])'),
    ("ReplacementTransform(VGroup(top, val), labels[idx])", f'morphs.transition(f"{P}.{A}.use-{{idx}}-label", source=VGroup(top, val), target=labels[idx])'),
    ("TransformFromCopy(labels[6], renew)", t(f"{A}.renew-minutes", "labels[6]", "renew", "morphs")),
    ('TransformFromCopy(v_read, items["v"])', t(f"{F}.formula-v", "v_read", 'items["v"]')),
    ('TransformFromCopy(n_read, items["n"])', t(f"{F}.formula-n", "n_read", 'items["n"]')),
    ('TransformFromCopy(a_read, items["vd"])', t(f"{F}.formula-flow", "a_read", 'items["vd"]')),
    ("TransformFromCopy(a_read, per_s)", t(f"{F}.per-second", "a_read", "per_s")),
    ("ReplacementTransform(a_read, card_h[1][2])", t(f"{F}.card-heating", "a_read", "card_h[1][2]")),
    ("ReplacementTransform(per_s, card_k[1][2])", t(f"{F}.card-cooling", "per_s", "card_k[1][2]")),
    ('TransformFromCopy(card_h[1][1], items2["vn"])', t(f"{F}.spell-vn", "card_h[1][1]", 'items2["vn"]')),
    ('TransformFromCopy(card_h[1][2], items2["h"])', t(f"{F}.spell-h", "card_h[1][2]", 'items2["h"]')),
    ('TransformFromCopy(card_k[1][1], items2["q"])', t(f"{F}.spell-q", "card_k[1][1]", 'items2["q"]')),
    ('TransformFromCopy(card_k[1][2], items2["s"])', t(f"{F}.spell-s", "card_k[1][2]", 'items2["s"]')),
    ("ReplacementTransform(value_h, bar_n)", t(f"{F}.bar-people-total", "value_h", "bar_n")),
    ("TransformFromCopy(_still(person), seg)", f'stage.transition(f"{P}.{F}.person-{{i}}", source=_still(person), target=seg)'),
    ('TransformFromCopy(lbl_p, items3["p"])', t(f"{F}.cover-people", "lbl_p", 'items3["p"]')),
    ('TransformFromCopy(lbl_n, items3["n"])', t(f"{F}.cover-total", "lbl_n", 'items3["n"]')),
])
