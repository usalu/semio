import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SPEC = "en1997-en1998-target.md"
def row(key, part, clause, symbol, v, unit, note, spec_ref, evidence):
    i = put(d, "1997", key, clause, symbol, v, v, unit, note, part=part)
    d[i]["source"]["document"] = "EN 1997-2 / DIN EN 1997-2/NA"
    d[i]["specRef"] = spec_ref
    d[i]["evidence"] = evidence
GI01 = f"{SPEC} GI-01 (2.2/2.3 + Annex B.3): point spacing 20-40 m (GC1-GC2), 10-25 m (GC3, DIN 4020), marked [V] in the spec"
for gc, lo, hi, label in (("gc12", 20.0, 40.0, "GC1 and GC2"), ("gc3", 10.0, 25.0, "GC3")):
    for side, v, word in (("min", lo, "smallest"), ("max", hi, "largest")):
        row(f"2.4.2.spacing.{gc}.{side}", "2", "EN 1997-2 Annex B.3 (investigation point spacing)", f"a_{side} ({label})", v, "m",
            f"{word.capitalize()} recommended spacing of investigation points for {label}: the largest nearest-neighbour distance of the points must not exceed the maximum; points closer than the minimum are reported but never fail",
            GI01, "value from the spec text only; the printed DIN 4020 / EN 1997-2 Annex B.3 is not in the repo, so the row stays unverified until a human double-keys it")
for side, v in (("min", 15.0), ("max", 20.0)):
    row(f"2.cpt-nkt-range.{side}", "2", "EN 1997-2 Annex D (CPT)", f"N_kt,{side} (plausible range of the cone factor)", v, "",
        f"{'Lower' if side == 'min' else 'Upper'} end of the plausible range of the CPT cone factor N_kt of c_u = (q_t - sigma_v0)/N_kt; a layer value outside the range is reported as a warning, never changed",
        f"{SPEC} GI-03 (Annex D): N_kt = 15...20 (NA / project calibration)", "range from the spec text only; the German annex value is not recorded, so the row stays unverified")
save(d)
print(len(d))
