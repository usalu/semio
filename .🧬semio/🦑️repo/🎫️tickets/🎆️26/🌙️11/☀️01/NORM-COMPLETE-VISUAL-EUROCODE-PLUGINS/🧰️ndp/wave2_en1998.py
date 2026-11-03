import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
S = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs").read_text()
d = load()
combos = re.findall(r'Self::(AR|BR|CR|BT|CT|CS) => \(([\d.]+), ([\d.]+), ([\d.]+), ([\d.]+)\),', S)
assert len(combos) == 6, combos
alt = {"BT": "spec S 1.00 (DIN EN 1998-1/NA:2011); code S 1.25", "CT": "spec S 1.25 (DIN EN 1998-1/NA:2011); code S 1.50"}
for c, *vals in combos:
    for sym, v in zip(("S", "T_B", "T_C", "T_D"), vals):
        k = f"na4.{c.lower()}.{sym.lower().replace('_', '')}"
        put(d, "1998", k, "Table NA.4", f"{sym} {c}", None, float(v), "s" if sym != "S" else "", f"{sym} of ground combination {c}: DE {v}", part="1")
        if sym == "S" and c in alt:
            d[f"ndp.en1998.{k}"]["evidence"] = "conflict: " + alt[c] + "; review-en1998-en1999.md item 18"
en = re.search(r"fn en_ground_params.*?\n}\n", S, re.S).group(0)
for typ, blk in (("t32", en.split("} else {")[0]), ("t33", en.split("} else {")[1])):
    for g, *vals in re.findall(r"'?([A-E_])'? => \(([\d.]+), ([\d.]+), ([\d.]+), ([\d.]+)\)", blk):
        g = "b" if g == "_" else g.lower()
        for sym, v in zip(("S", "T_B", "T_C", "T_D"), vals):
            put(d, "1998", f"{typ}.{g}.{sym.lower().replace('_', '')}", "Table 3.2" if typ == "t32" else "Table 3.3", f"{sym} ground {g.upper()}", float(v), None, "s" if sym != "S" else "", f"{sym} ground class {g.upper()} spectrum type {1 if typ == 't32' else 2}: EC {v}", part="1")
rows = [("vertical-spectrum.avg-ratio", "3.2.2.3", "a_vg/a_g", 0.9, 0.9, ""), ("en.torsion.simplified-bound", "4.3.3.2.4", "δ max", 1.6, 1.6, ""),
 ("en1.timber.max-diameter-mm", "8.3", "d max", 12.0, 12.0, "mm"), ("en1.timber.min-thickness-factor", "8.3", "t/d min", 8.0, 8.0, ""),
 ("en1.masonry.slenderness-max", "Table 9.2", "h_ef/t_ef max", 12.0, 12.0, ""), ("en1.masonry.min-thickness-m", "Table 9.2", "t_ef min", 0.24, 0.24, "m"),
 ("en2.pier.gamma-0", "EN 1998-2 5.6.3.3", "γ_0", 1.35, 1.35, ""), ("en3.chord-rotation.sd-factor", "EN 1998-3 A.1", "SD factor", 0.75, 0.75, ""),
 ("en5.liquefaction.min-safety-factor", "EN 1998-5 4.1.4", "F_L min", 1.25, 1.25, "")]
for k, cl, sym, r, n, u in rows:
    put(d, "1998", k, cl, sym, r, n, u, f"{sym} = {r} as used by the artifact")
d["ndp.en1998.en1.timber.min-thickness-factor"]["evidence"] = "private list noted alternative 10"
d["ndp.en1998.en2.pier.gamma-0"]["evidence"] = "private list noted alternative 1.25"
put(d, "1998", "ref.liquefaction.youd-2001", "EN 1998-5 Annex B (chart), Youd et al. 2001", "reference model", "r_d, CRR 7.5, MSF, fines correction", "r_d, CRR 7.5, MSF, fines correction", "", "Chart of EN 1998-5 Annex B replaced by Youd et al. 2001 formulas")
save(d)
print(len(combos), [k for k in d if k.startswith("ndp.en1998.t3")][:3], sum(k.startswith("ndp.en1998.t3") for k in d))
