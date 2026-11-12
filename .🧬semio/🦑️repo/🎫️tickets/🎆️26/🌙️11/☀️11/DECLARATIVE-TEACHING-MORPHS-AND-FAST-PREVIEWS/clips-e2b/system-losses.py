import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from doc import DEMAND, entry as e, write
from patch import apply

X = "demand.system-losses.expense."
G = "demand.system-losses.generation."
C = "demand.system-losses.chain."
rows = []
for k in ("ue", "ve", "sp"):
    rows.append(e(f"{X}{k}-symbol-into-product", "abstraction", f"{X}{k}-symbol", f"{X}{k}-product-term", 1.6, copy=True))
rows.append(e(G + "gas-label-to-condensing", "transformation", G + "gas-boiler-label", G + "condensing-boiler-label", 2.4))
rows.append(e(G + "parts-into-spf", "abstraction", G + "heat-and-power-labels", G + "spf-fraction", 2.0, relation="many-to-many"))
rows.append(e(C + "product-symbol-into-ep", "abstraction", C + "product-symbol", C + "ep-term", 1.6, copy=True))
for k in ("ue", "ve", "sp", "eg"):
    rows.append(e(f"{C}{k}-symbol-into-term", "abstraction", f"{C}{k}-symbol", f"{C}{k}-term", 1.6, copy=True))
rows += [
    e(C + "generator-label-to-gas-boiler", "transformation", C + "generator-label", C + "gas-boiler-label", 1.8),
    e(C + "formula-box-to-gas-numbers", "continuation", C + "formula-box-symbols", C + "formula-box-gas", 1.8),
    e(C + "formula-to-gas-numbers", "transformation", C + "formula-symbols", C + "formula-gas", 1.8, relation="many-to-many"),
    e(C + "gas-label-to-heat-pump-label", "transformation", C + "gas-boiler-label", C + "heat-pump-label", 3.0),
    e(C + "formula-box-to-heat-pump", "continuation", C + "formula-box-gas", C + "formula-box-heat-pump", 3.0),
    e(C + "formula-to-heat-pump-numbers", "transformation", C + "formula-gas", C + "formula-heat-pump", 3.0, relation="many-to-many"),
    e(C + "final-energy-gas-to-heat-pump", "transformation", C + "final-energy-gas", C + "final-energy-heat-pump", 3.0),
    e(C + "primary-factor-gas-to-heat-pump", "transformation", C + "primary-factor-gas", C + "primary-factor-heat-pump", 3.0),
    e(C + "formula-box-to-bridge", "continuation", C + "formula-box-heat-pump", C + "formula-box-bridge", 1.8),
    e(C + "formula-to-bridge", "abstraction", C + "formula-heat-pump", C + "formula-bridge", 1.8, relation="many-to-many"),
]
write("♨️system-losses", rows)

CLIP = DEMAND / "♨️system-losses/🐍️.py"
apply(CLIP, [
    ('*[ReplacementTransform(final[k]["symbol"].copy(), items[i])\n                    for k, i in (("ue", "eu"), ("ve", "ed"), ("sp", "es"))],',
     f'*[stage.transition(f"{X}{{k}}-symbol-into-product", source=final[k]["symbol"], target=items[i])\n                    for k, i in (("ue", "eu"), ("ve", "ed"), ("sp", "es"))],'),
    ('ReplacementTransform(gas_row["label"], cond_label)', f'stage.transition("{G}gas-label-to-condensing", source=gas_row["label"], target=cond_label)'),
    ('ReplacementTransform(sources, items["frac"]),', f'stage.transition("{G}parts-into-spf", source=sources, target=items["frac"]),'),
    ('ReplacementTransform(product["symbol"].copy(), items["ep"]),\n                  *[ReplacementTransform(rows[k]["symbol"].copy(), items[i])\n',
     f'stage.transition("{C}product-symbol-into-ep", source=product["symbol"], target=items["ep"]),\n                  *[stage.transition(f"{C}{{k}}-symbol-into-term", source=rows[k]["symbol"], target=items[i])\n'),
    ('ReplacementTransform(rows["eg"]["label"], gas_label),\n                  ReplacementTransform(eq_box, eq2_box), ReplacementTransform(eq, eq2), run_time=1.8)',
     f'stage.transition("{C}generator-label-to-gas-boiler", source=rows["eg"]["label"], target=gas_label),\n                  stage.transition("{C}formula-box-to-gas-numbers", source=eq_box, target=eq2_box),\n                  stage.transition("{C}formula-to-gas-numbers", source=eq, target=eq2), run_time=1.8)'),
    ('            ReplacementTransform(gas_label, pump_label),\n', f'            stage.transition("{C}gas-label-to-heat-pump-label", source=gas_label, target=pump_label),\n'),
    ('            ReplacementTransform(eq2_box, eq3_box), ReplacementTransform(eq2, eq3),\n            ReplacementTransform(out_e, out_e_wp), ReplacementTransform(out_x, out_x_wp), run_time=3.0)',
     f'            stage.transition("{C}formula-box-to-heat-pump", source=eq2_box, target=eq3_box),\n            stage.transition("{C}formula-to-heat-pump-numbers", source=eq2, target=eq3),\n            stage.transition("{C}final-energy-gas-to-heat-pump", source=out_e, target=out_e_wp),\n            stage.transition("{C}primary-factor-gas-to-heat-pump", source=out_x, target=out_x_wp), run_time=3.0)'),
    ('self.play(ReplacementTransform(eq3_box, bridge_box), ReplacementTransform(eq3, bridge),',
     f'self.play(stage.transition("{C}formula-box-to-bridge", source=eq3_box, target=bridge_box),\n                  stage.transition("{C}formula-to-bridge", source=eq3, target=bridge),'),
], scenes=("Beat2_UebergabeVerteilungSpeicherung", "Beat3_Erzeugung", "Beat4_Anlagenaufwandszahl"))
