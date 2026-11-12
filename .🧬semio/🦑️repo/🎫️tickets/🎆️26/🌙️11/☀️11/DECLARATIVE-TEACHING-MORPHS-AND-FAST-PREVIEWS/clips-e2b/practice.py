import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from doc import DEMAND, entry as e, write
from patch import apply

P = "demand.practice."
rows = [
    e(P + "calculation.calculated-segments-to-axis", "abstraction", P + "calculation.calculated-segments", P + "calculation.calculated-axis-length", 1.8, relation="many-to-many"),
    e(P + "calculation.measured-segments-to-axis", "abstraction", P + "calculation.measured-segments", P + "calculation.measured-axis-length", 1.8, relation="many-to-many"),
    e(P + "series.small-house-to-balance-house", "continuation", P + "series.house-small", P + "series.house-balance", 1.5, relation="many-to-many"),
    e(P + "series.gas-class-chip-to-heat-pump-chip", "continuation", P + "series.chip-gas", P + "series.chip-heat-pump", 1.6, copy=True),
]
write("🧪️practice", rows)

CLIP = DEMAND / "🧪️practice/🐍️.py"
apply(CLIP, [
    ('            ReplacementTransform(calc_segs, x_seg),\n            ReplacementTransform(meas_frozen, y_seg),\n',
     f'            morph_stage.transition("{P}calculation.calculated-segments-to-axis", source=calc_segs, target=x_seg),\n            morph_stage.transition("{P}calculation.measured-segments-to-axis", source=meas_frozen, target=y_seg),\n'),
    ('ReplacementTransform(hs_small["group"], hs["group"])', f'morph_stage.transition("{P}series.small-house-to-balance-house", source=hs_small["group"], target=hs["group"])'),
    ('TransformFromCopy(chip_gas, chip_wp)', f'morph_stage.transition("{P}series.gas-class-chip-to-heat-pump-chip", source=chip_gas, target=chip_wp)'),
], scenes=("Beat1_BerechnungVsAbrechnung", "Beat3_SerienAbschluss"), name="morph_stage")
