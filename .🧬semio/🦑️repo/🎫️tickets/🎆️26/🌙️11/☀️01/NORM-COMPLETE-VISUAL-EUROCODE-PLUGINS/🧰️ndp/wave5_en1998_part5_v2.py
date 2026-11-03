import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SPEC = "📐️specs/en1997-en1998-target.md"
def row(key, clause, symbol, rec, unit, note, spec):
    i = put(d, "1998", key, clause, symbol, rec, rec, unit, note, part="5")
    d[i]["specRef"] = spec
row("en5.annex-e.permeability-limit", "EN 1998-5 Annex E", "k limit of free water movement", 5.0e-4, "m/s", "Backfill with k above the limit lets its water move against the grains (dry weight as inertia, Westergaard pressure); below it the water moves with the soil", "wave2 en1998 part 5 v2: threshold recalled from Annex E, not in the repository spec")
row("en5.sliding.gamma-rd", "EN 1998-5 5.3", "γ_Rd sliding", 1.0, "", "F_h <= (V - u) tan δ / γ_Rd with γ_Rd = 1.0", f"{SPEC} line 560 (GE-07): F_h <= (V - u) tanδ/γ_Rd with γ_Rd = 1.0")
row("en5.vertical-force.threshold", "EN 1998-5 4.1.3.3", "a_vg/a_g threshold", 0.6, "", "F_V = ±0.5 F_H if a_vg/a_g exceeds the threshold, else ±0.33 F_H", f"{SPEC} line 558 (GE-05): vertical 0.33 or 0.5 F_h, if avg/ag > 0.6")
row("en5.vertical-force.ratio-high", "EN 1998-5 4.1.3.3", "F_V/F_H for a_vg/a_g > 0.6", 0.5, "", "Vertical pseudo-static force share of the horizontal one", f"{SPEC} line 558 (GE-05)")
row("en5.vertical-force.ratio-low", "EN 1998-5 4.1.3.3", "F_V/F_H for a_vg/a_g <= 0.6", 0.33, "", "Vertical pseudo-static force share of the horizontal one", f"{SPEC} line 558 (GE-05)")
row("en5.slope.horizontal-factor", "EN 1998-5 4.1.3.3", "F_H/(α S W)", 0.5, "", "F_H = 0.5 α S W of a slice or a sliding mass", f"{SPEC} line 558 (GE-05): F_H = 0.5 α S W")
for g, v in (("b", 0.3), ("c", 0.4), ("d", 0.6)):
    row(f"en5.tie.coefficient.{g}", "EN 1998-5 5.4.2", f"tie force coefficient, ground type {g.upper()}", v, "", "N = ±coefficient α S N_Ed", f"{SPEC} line 561 (GE-08): N = ±0.3 α S N_Ed (soil B) / 0.4 (C) / 0.6 (D) [V]")
row("vertical-spectrum.avg-ratio-type2", "3.2.2.3", "a_vg/a_g type 2", 0.45, "", "Vertical component of the type 2 spectrum", f"{SPEC} line 343: Type 2 avg/ag = 0.45")
save(d)
print(len(d))
