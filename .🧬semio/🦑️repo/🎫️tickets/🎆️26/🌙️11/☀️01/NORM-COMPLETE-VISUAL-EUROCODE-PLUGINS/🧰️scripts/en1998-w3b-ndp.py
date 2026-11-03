"""EN 1998 wave 3b: registers the unverified detailing constants of the DCM/DCH critical regions (EN 1998-1 5.4.3, 5.5.3) in the shared NDP registry through `🧰️ndp/ndplib.py`, flags the NA-DE T_R row with the NA clause that has to be read, then regenerates the row crates with `🧰️ndp/codegen.py`. Idempotent.
Usage: /home/deck/semio/.venv/bin/python -I en1998-w3b-ndp.py"""
import pathlib, subprocess, sys

NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

NOTE = "; value from the author's knowledge, not read from the printed standard (unverified)"
d = load()
before = len(d)
rows = [
    ("en1.rc.column.critical-length.min-m", "5.4.3.2.2(4)", "l_cr lower bound of columns", 0.45, "m", "EN 1998-1 5.4.3.2.2: l_cr = max{h_c; l_cl/6; 0.45 m}, 0.45 m" + NOTE),
    ("en1.rc.column.critical-length.clear-length-divisor", "5.4.3.2.2(4)", "divisor of l_cl in l_cr", 6.0, "", "EN 1998-1 5.4.3.2.2: l_cr = max{h_c; l_cl/6; 0.45 m}, divisor 6" + NOTE),
    ("en1.rc.column.critical-length.full-height-slenderness", "5.4.3.2.2(5)", "l_cl/h_c below which the whole column is critical", 3.0, "", "EN 1998-1 5.4.3.2.2: if l_cl/h_c < 3 the whole height of the column is a critical region" + NOTE),
    ("en1.rc.beam.critical-length.depth-factor.dcm", "5.4.3.1.2(6)", "l_cr / h_w of DCM beams", 1.0, "", "EN 1998-1 5.4.3.1.2: critical region length l_cr = h_w" + NOTE),
    ("en1.rc.beam.critical-length.depth-factor.dch", "5.5.3.1.2", "l_cr / h_w of DCH beams", 1.5, "", "EN 1998-1 5.5.3.1.2: critical region length l_cr = 1.5 h_w" + NOTE),
    ("en1.rc.hoop-diameter.min-m.dcm", "5.4.3.2.2(11)a", "minimum hoop diameter d_bw DCM", 0.006, "m", "EN 1998-1 5.4.3.2.2(11)a: d_bw >= 6 mm" + NOTE),
    ("en1.rc.hoop-diameter.min-m.dch", "5.5.3.2.2", "minimum hoop diameter d_bw DCH", 0.006, "m", "EN 1998-1 5.5.3.2.2: d_bw >= 6 mm" + NOTE),
    ("en1.rc.hoop-diameter.dch-longitudinal-factor", "5.5.3.2.2", "d_bw >= factor d_bL,max sqrt(f_ydL/f_ydw) of DCH columns", 0.4, "", "EN 1998-1 5.5.3.2.2: d_bw >= 0.4 d_bL,max sqrt(f_ydL/f_ydw)" + NOTE),
    ("en1.rc.column.hoop-spacing.core-divisor.dcm", "5.4.3.2.2(11)b", "s <= b_0 / divisor DCM", 2.0, "", "EN 1998-1 5.4.3.2.2(11)b: s = min{b_0/2; 175 mm; 8 d_bL}" + NOTE),
    ("en1.rc.column.hoop-spacing.core-divisor.dch", "5.5.3.2.2", "s <= b_0 / divisor DCH", 3.0, "", "EN 1998-1 5.5.3.2.2: s = min{b_0/3; 125 mm; 6 d_bL}" + NOTE),
    ("en1.rc.column.hoop-spacing.max-m.dcm", "5.4.3.2.2(11)b", "absolute hoop spacing limit of DCM columns", 0.175, "m", "EN 1998-1 5.4.3.2.2(11)b: 175 mm" + NOTE),
    ("en1.rc.column.hoop-spacing.max-m.dch", "5.5.3.2.2", "absolute hoop spacing limit of DCH columns", 0.125, "m", "EN 1998-1 5.5.3.2.2: 125 mm" + NOTE),
    ("en1.rc.column.hoop-spacing.bar-diameter-factor.dcm", "5.4.3.2.2(11)b", "s <= factor d_bL DCM", 8.0, "", "EN 1998-1 5.4.3.2.2(11)b: 8 d_bL" + NOTE),
    ("en1.rc.column.hoop-spacing.bar-diameter-factor.dch", "5.5.3.2.2", "s <= factor d_bL DCH", 6.0, "", "EN 1998-1 5.5.3.2.2: 6 d_bL" + NOTE),
    ("en1.rc.beam.hoop-spacing.depth-divisor", "5.4.3.1.2(6)", "s <= h_w / divisor of beams", 4.0, "", "EN 1998-1 5.4.3.1.2 and 5.5.3.1.3: s = min{h_w/4; 24 d_bw; 225 mm (DCM) or 175 mm (DCH); 8 d_bL (DCM) or 6 d_bL (DCH)}" + NOTE),
    ("en1.rc.beam.hoop-spacing.hoop-diameter-factor", "5.4.3.1.2(6)", "s <= factor d_bw of beams", 24.0, "", "EN 1998-1 5.4.3.1.2 and 5.5.3.1.3: 24 d_bw" + NOTE),
    ("en1.rc.beam.hoop-spacing.max-m.dcm", "5.4.3.1.2(6)", "absolute hoop spacing limit of DCM beams", 0.225, "m", "EN 1998-1 5.4.3.1.2: 225 mm" + NOTE),
    ("en1.rc.beam.hoop-spacing.max-m.dch", "5.5.3.1.3", "absolute hoop spacing limit of DCH beams", 0.175, "m", "EN 1998-1 5.5.3.1.3: 175 mm" + NOTE),
    ("en1.rc.beam.hoop-spacing.bar-diameter-factor.dcm", "5.4.3.1.2(6)", "s <= factor d_bL of DCM beams", 8.0, "", "EN 1998-1 5.4.3.1.2: 8 d_bL" + NOTE),
    ("en1.rc.beam.hoop-spacing.bar-diameter-factor.dch", "5.5.3.1.3", "s <= factor d_bL of DCH beams", 6.0, "", "EN 1998-1 5.5.3.1.3: 6 d_bL" + NOTE),
    ("en1.rc.restrained-bar-spacing.max-m.dcm", "5.4.3.2.2(11)c", "largest spacing of restrained longitudinal bars DCM", 0.2, "m", "EN 1998-1 5.4.3.2.2(11)c: the distance between consecutive longitudinal bars engaged by hoops or cross-ties is at most 200 mm" + NOTE),
    ("en1.rc.restrained-bar-spacing.max-m.dch", "5.5.3.2.2", "largest spacing of restrained longitudinal bars DCH", 0.15, "m", "EN 1998-1 5.5.3.2.2: the distance between consecutive longitudinal bars engaged by hoops or cross-ties is at most 150 mm" + NOTE),
    ("en1.rc.column.nu-d-max.dcm", "5.4.3.2.1(3)P", "nu_d upper limit of DCM columns", 0.65, "", "EN 1998-1 5.4.3.2.1(3)P: nu_d = N_Ed/(A_c f_cd) <= 0.65; value from the repository target spec SEI-50, not read from the printed standard (unverified)"),
    ("en1.rc.column.nu-d-max.dch", "5.5.3.2.1(3)P", "nu_d upper limit of DCH columns", 0.55, "", "EN 1998-1 5.5.3.2.1(3)P: nu_d = N_Ed/(A_c f_cd) <= 0.55; value from the repository target spec SEI-50, not read from the printed standard (unverified)"),
    ("en1.rc.confinement.mu-phi-coefficient", "5.4.3.2.2(8) Eq. 5.15", "factor 30 of alpha omega_wd >= 30 mu_phi nu_d eps_sy,d b_c/b_0 - 0.035", 30.0, "", "EN 1998-1 5.4.3.2.2(8) Eq. (5.15): alpha omega_wd >= 30 mu_phi nu_d eps_sy,d b_c/b_0 - 0.035" + NOTE),
    ("en1.rc.confinement.offset", "5.4.3.2.2(8) Eq. 5.15", "offset 0.035 of Eq. 5.15", 0.035, "", "EN 1998-1 5.4.3.2.2(8) Eq. (5.15): the constant 0.035" + NOTE),
]
for key, clause, symbol, value, unit, note in rows:
    d[put(d, "1998", key, clause, symbol, value, value, unit, symbol + " = " + str(value) + " as used by the artifact", part="1", keep=True)]["evidence"] = note
na = d["ndp.en1998.en3.na.return-period"]
na["asWritten"] = "NA-DE fixes the return period to T_R = 475 yr for every limit state (a_gR of Table NA.1); UNVERIFIED: the value rests on the repository target spec only. The row names DIN EN 1998-1/NA, but the return periods of EN 1998-3 are an NDP of EN 1998-3:2005 clause 2.1 (the recommended T_NCR 2475, T_SDR 475, T_DLR 225 yr), so the clause to read is the National Annex to EN 1998-3 (DIN EN 1998-3/NA), its NDP to clause 2.1(2)P-(4) on the return periods of the three limit states; until it is read the recommended periods stay the limit-state values under EN and the 475 yr rule is applied under NA-DE as a flagged assumption"
na["evidence"] = "NDP to read: National Annex to EN 1998-3, clause 2.1 (return periods T_R per limit state NC, SD, DL); not verified, not read from the printed National Annex"
na["source"]["document"] = "DIN EN 1998-3/NA (to be identified); today cited as DIN EN 1998-1/NA:2023-11"
na["source"]["locator"] = "NA to EN 1998-3, 2.1(2)P-(4)"
save(d)
codegen = NDP_TOOLS / "codegen.py"
subprocess.run([sys.executable, "-I", str(codegen), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print("rows", before, "->", len(d))
