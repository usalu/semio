"""EN 1998 wave 3d: unverified registry rows of the liquefaction hazard (r_u, lateral spreading, pile in liquefied soil), Table 4.1 of EN 1998-5 (degradation of the shear modulus) and the isolation limits of EN 1998-2 chapter 7 and EN 1998-1 chapter 10. Every row is flagged unverified with the evidence it rests on; nothing is read from a printed standard.
Usage: /home/deck/semio/.venv/bin/python -I en1998-w3d-ndp.py"""
import pathlib, subprocess, sys

NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

TAIL = "; UNVERIFIED: not read from the printed standard, check it against the clause before release"
ROWS = [
    ("en5.liquefaction.ru.alpha", "EN 1998-5 4.1.4", "r_u exponent α", 0.7, 0.7, "", "5", "Seed, Martin and Lysmer (1976): r_u = 0.5 + asin(2 (N/N_L)^(1/α) − 1)/π with α = 0.7 for sand; a textbook basis for the pore pressure build-up, not a provision of EN 1998-5" + TAIL),
    ("en5.liquefaction.ru.cycle-exponent", "EN 1998-5 4.1.4", "cyclic strength exponent b", 0.34, 0.34, "", "5", "CRR ∝ N^(−b) with b = 0.34 gives the cycle ratio N/N_L = F_L^(−1/b) (textbook value of Idriss and Boulanger); not a provision of EN 1998-5" + TAIL),
    ("en5.liquefaction.spreading.min-slope-percent", "EN 1998-5 4.1.4", "lowest ground slope of lateral spreading", 0.1, 0.1, "%", "5", "Youd, Hansen and Bartlett (2002) apply their lateral spreading model to gently sloping ground from 0.1 %; below it the ground is level and only oscillates; a literature basis, not a provision of EN 1998-5" + TAIL),
    ("en5.liquefaction.spreading.max-slope-percent", "EN 1998-5 4.1.4", "highest ground slope of lateral spreading", 6.0, 6.0, "%", "5", "Youd, Hansen and Bartlett (2002) apply their lateral spreading model to ground slopes up to 6 %; steeper liquefied ground fails as a flow slide and the slope stability governs; a literature basis, not a provision of EN 1998-5" + TAIL),
    ("en5.pile.liquefied.length-factor", "EN 1998-5 5.4.2", "effective length factor K of a pile in liquefied soil", 1.0, 1.0, "", "5", "K = 1 hinged-hinged Euler column over the unsupported length (Bhattacharya et al. 2004 idealisation, conservative for a restrained head); not a provision of EN 1998-5" + TAIL),
    ("en5.pile.liquefied.fixity-factor", "EN 1998-5 5.4.2", "depth to fixity below the liquefied layer in 1/β", 1.0, 1.0, "", "5", "The pile is taken as fixed one Winkler characteristic length 1/β below the liquefied layer; an engineering assumption, not a provision of EN 1998-5" + TAIL),
]
for label, vs, damping in (("a010", 0.90, 0.03), ("a020", 0.70, 0.06), ("a030", 0.60, 0.10)):
    g = label[1:]
    ROWS.append((f"en5.site-response.vs-ratio.{label}", "EN 1998-5 4.2.2 Table 4.1", f"v_s/v_s,max for a_g S/g = 0.{g[:2]}", vs, vs, "", "5", f"EN 1998-5 Table 4.1 (average soil damping ratios and reduction factors for a_g S/g = 0.10 / 0.20 / 0.30: v_s/v_s,max 0.90 / 0.70 / 0.60, G/G_max 0.80 / 0.50 / 0.36 = (v_s/v_s,max)²), recollection of the printed table" + TAIL))
    ROWS.append((f"en5.site-response.damping.{label}", "EN 1998-5 4.2.2 Table 4.1", f"soil damping ratio for a_g S/g = 0.{g[:2]}", damping, damping, "", "5", "EN 1998-5 Table 4.1 (damping ratio 0.03 / 0.06 / 0.10 for a_g S/g = 0.10 / 0.20 / 0.30), recollection of the printed table" + TAIL))
for part, fam, clause, where in (("2", "en2", "EN 1998-2 7", "bridge"), ("1", "en1", "EN 1998-1 10", "building")):
    ROWS.append((f"{fam}.isolation.min-period-ratio", clause, f"T_eff/T_fixed of an isolated {where}", 3.0, 3.0, "", part, f"Target spec SEI-53 and BR-10: T_eff ≥ 3 T_fixed for the isolated {where}" + TAIL))
    ROWS.append((f"{fam}.isolation.max-period-s", clause, f"largest T_eff of an isolated {where}", 3.0, 3.0, "s", part, f"Target spec SEI-53: T_eff ≤ 3 s for the equivalent linear analysis of the isolated {where}" + TAIL))
    ROWS.append((f"{fam}.isolation.max-damping", clause, f"largest ξ_eff of an isolated {where}", 0.30, 0.30, "", part, f"Equivalent linear analysis of isolated structures limits the effective damping to 30 % (recollection of EN 1998-1 10 and EN 1998-2 7); not in the target spec rows" + TAIL))
d = load()
for key, clause, symbol, rec, de, unit, part, evidence in ROWS:
    i = put(d, "1998", key, clause, symbol, rec, de, unit, f"{symbol} = {rec} as used by the artifact", part=part, keep=True)
    d[i]["evidence"] = evidence
    d[i]["specRef"] = "wave3d en1998: value written with its evidence note, not checked against the licensed edition"
save(d)
subprocess.run([sys.executable, "-I", str(NDP_TOOLS / "codegen.py"), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print(len(ROWS), "rows,", len(d), "entries")
