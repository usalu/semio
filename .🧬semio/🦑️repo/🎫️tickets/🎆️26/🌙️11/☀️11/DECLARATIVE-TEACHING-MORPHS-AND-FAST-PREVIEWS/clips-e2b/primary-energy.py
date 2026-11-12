import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from doc import DEMAND, entry as e, write
from patch import apply

P = "demand.primary-energy."
B, F, W, R = P + "boundaries.", P + "factor.", P + "worked-example.", P + "reference."
rows = [
    e(B + "symbol-qp-into-formula", "abstraction", B + "card-qp-symbol", B + "formula-qp", 1.4, copy=True),
    e(B + "symbol-qe-into-formula", "abstraction", B + "card-qe-symbol", B + "formula-qe", 1.4, copy=True),
]
for k in ("qp", "eq", "qe", "dot", "fp"):
    rows.append(e(f"{F}formula-{k}-to-non-renewable", "transformation" if k == "fp" else "continuation", f"{F}formula-{k}", f"{F}formula-non-renewable-{k}", 1.4))
rows += [
    e(W + "note-to-unit-legend", "transformation", W + "schematic-note", W + "unit-legend", 1.0),
    e(W + "heat-bar-splits-ambient", "transformation", W + "heat-bar", W + "heat-bar-split", 1.6),
]
for tag, name in (("g", "gas"), ("w", "heat-pump")):
    for k, what in (("qe", "final-energy"), ("fp", "factor"), ("qp", "primary-energy")):
        rows.append(e(f"{W}{name}-{what}-into-formula", "abstraction", f"{W}{name}-{what}-static", f"{W}{name}-{what}-term", 1.8, copy=True))
rows += [
    e(R + "design-house-copies-to-reference", "continuation", R + "design-house", R + "reference-house", 1.8, relation="many-to-many", copy=True),
    e(R + "dimension-copies-to-reference", "continuation", R + "design-dimensions", R + "reference-dimensions", 1.8, relation="many-to-many", copy=True),
    e(R + "envelope-into-ht", "abstraction", R + "design-envelope", R + "ht-term", 1.4, relation="many-to-many", copy=True),
    e(R + "share-reading-into-formula", "abstraction", R + "share-reading", R + "share-term", 1.2, copy=True),
]
write("🏭️primary-energy", rows)

CLIP = DEMAND / "🏭️primary-energy/🐍️.py"
apply(CLIP, [
    ('TransformFromCopy(row_cards[2]["sym"], items["qp"]),', f'stage.transition("{B}symbol-qp-into-formula", source=row_cards[2]["sym"], target=items["qp"]),'),
    ('TransformFromCopy(row_cards[1]["sym"], items["qe"]),', f'stage.transition("{B}symbol-qe-into-formula", source=row_cards[1]["sym"], target=items["qe"]),'),
    ('*[ReplacementTransform(items[k], items2[k]) for k in ("qp", "eq", "qe", "dot", "fp")]',
     f'*[stage.transition(f"{F}formula-{{k}}-to-non-renewable", source=items[k], target=items2[k]) for k in ("qp", "eq", "qe", "dot", "fp")]'),
    ('ReplacementTransform(schematic_note, legend_unit)', f'stage.transition("{W}note-to-unit-legend", source=schematic_note, target=legend_unit)'),
    ('ReplacementTransform(wp["rect_h"], split_bar)', f'stage.transition("{W}heat-bar-splits-ambient", source=wp["rect_h"], target=split_bar)'),
    ('        for p, tag in ((gas, "g"), (wp, "w")):\n            copies += [\n                TransformFromCopy(p["static"]["e"]["num"], items[f"{tag}_qe"]),\n                TransformFromCopy(p["static"]["f"], items[f"{tag}_fp"]),\n                TransformFromCopy(p["static"]["q"]["num"], items[f"{tag}_qp"]),\n',
     f'        for p, tag, name in ((gas, "g", "gas"), (wp, "w", "heat-pump")):\n            copies += [\n                stage.transition(f"{W}{{name}}-final-energy-into-formula", source=p["static"]["e"]["num"], target=items[f"{{tag}}_qe"]),\n                stage.transition(f"{W}{{name}}-factor-into-formula", source=p["static"]["f"], target=items[f"{{tag}}_fp"]),\n                stage.transition(f"{W}{{name}}-primary-energy-into-formula", source=p["static"]["q"]["num"], target=items[f"{{tag}}_qp"]),\n'),
    ('TransformFromCopy(design["group"], reference["group"]),', f'stage.transition("{R}design-house-copies-to-reference", source=design["group"], target=reference["group"]),'),
    ('TransformFromCopy(dims_d, dims_r)', f'stage.transition("{R}dimension-copies-to-reference", source=dims_d, target=dims_r)'),
    ('TransformFromCopy(envelope_d, items["ht"])', f'stage.transition("{R}envelope-into-ht", source=envelope_d, target=items["ht"])'),
    ('TransformFromCopy(frozen["ee"], items2["ee"])', f'stage.transition("{R}share-reading-into-formula", source=frozen["ee"], target=items2["ee"])'),
], scenes=("Beat1_DreiBilanzgrenzen", "Beat2_Primaerenergiefaktor", "Beat3_Rechenbeispiel", "Beat4_Referenzgebaeude"))
