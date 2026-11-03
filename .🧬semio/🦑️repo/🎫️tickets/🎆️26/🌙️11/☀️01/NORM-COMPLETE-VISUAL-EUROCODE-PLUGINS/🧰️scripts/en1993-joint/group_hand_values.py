"""Hand values of the row-group effective lengths and group resistances for the Rust unit tests (geometry of `showcase_end_plate` extended to three rows). Independent of the Rust; uses the oracle's Table 6.4 / 6.5 / 6.6 functions and T-stub."""
import importlib.util, math, pathlib, sys
base = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🚦️check-outcome/🧭️families-w5/🐍️.py")
spec = importlib.util.spec_from_file_location("w5", base); W = importlib.util.module_from_spec(spec); spec.loader.exec_module(W)
tfc, twc, dc, avc, rc, fy, w, e_c, sigma, beta = 17.0, 10.0, 164.0, 2400.0, 21.0, 355.0, 110.0, 40.0, 50.0, 1.0
tp, bp, fyp = 20.0, 200.0, 355.0
m_c = (w - twc) / 2 - 0.8 * rc
ft = 0.9 * 1000.0 * 245.0 / 1.25
def row(kind, lever, pitch, col_alpha=0.0):
    return dict(kind=kind, lever=lever, m=40.0, e=40.0, mx=40.0 if kind == "Extended" else 0.0, ex=40.0 if kind == "Extended" else 0.0, alpha=6.5, pitch=pitch, n=2.0, a_s=245.0, lb=50.0, ft=ft, group=True, col_alpha=col_alpha)
def resistance(rows, first, last):
    cc, cn, pc, pn = W.group_lengths(rows, first, last, m_c, e_c)
    n_b = sum(rows[i]["n"] for i in range(first, last + 1)); sum_ft = sum(rows[i]["n"] * rows[i]["ft"] for i in range(first, last + 1))
    a_s, lb = rows[first]["a_s"], rows[first]["lb"]
    f_col, mode_col = W.t_stub(min(cc, cn), cn, tfc, fy, m_c, min(e_c, 1.25 * m_c), sum_ft, a_s, n_b, lb, 1.0)
    r0 = rows[first]; m_use = r0["mx"] if r0["kind"] == "Extended" else r0["m"]
    n_pl = min(min(r0["e"], r0["ex"]) if r0["ex"] > 0 else r0["e"], 1.25 * m_use)
    f_pl, mode_pl = W.t_stub(min(pc, pn), pn, tp, fyp, m_use, n_pl, sum_ft, a_s, n_b, lb, 1.0)
    b_t = min(cc, cn) if mode_col == 1 else cn
    f_web = W.omega_table_6_3(beta, b_t, twc, avc) * b_t * twc * fy / 1.0
    return dict(lengths=(cc, cn, pc, pn), f_col=f_col, mode_col=mode_col, f_pl=f_pl, mode_pl=mode_pl, f_web=f_web, f=min(f_col, f_pl, f_web))
three = [row("Extended", 270.0, 0.0), row("FirstBelow", 180.0, 90.0), row("Inner", 90.0, 90.0)]
unequal = [row("Extended", 270.0, 0.0), row("FirstBelow", 180.0, 80.0), row("Inner", 90.0, 100.0)]
stiff = [row("FirstBelow", 180.0, 90.0, 6.0), row("Inner", 90.0, 90.0)]
print("m_c", m_c)
for name, rows, a, b in (("three 0-2", three, 0, 2), ("three 1-2", three, 1, 2), ("unequal 0-2", unequal, 0, 2), ("stiffened 0-1", stiff, 0, 1)):
    print(name, resistance(rows, a, b))
old = [dict(r) for r in three]
cc, cn, pc, pn = W.group_lengths(three, 0, 2, m_c, e_c)
print("old plate inner (2 pi m, alpha m) instead of (2p, p): cp", pc - 180 + 2 * math.pi * 40, "nc", pn - 90 + 6.5 * 40)
tp = 10.0
unequal = [row("Extended", 270.0, 0.0), row("FirstBelow", 180.0, 70.0), row("Inner", 90.0, 110.0)]
r = resistance(three, 0, 2)
print("thin plate tp=10 three 0-2", r)
cp_old, nc_old = 682.654824574367 , 560.0
cc, cn, pc, pn = W.group_lengths(three, 0, 2, m_c, e_c)
sum_ft = 6 * ft
print("old plate lengths ft", W.t_stub(min(cp_old - 0, nc_old), nc_old, tp, fyp, 40.0, 40.0, sum_ft, 245.0, 6.0, 50.0, 1.0))
print("unequal 70/110", resistance(unequal, 0, 2)["lengths"])
