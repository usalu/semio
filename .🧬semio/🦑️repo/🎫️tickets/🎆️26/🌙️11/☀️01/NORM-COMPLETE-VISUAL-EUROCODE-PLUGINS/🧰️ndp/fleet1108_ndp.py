"""🧮 Fleet-1108 NDP lane: reconciles the registry with third-party library facts and the conflict ledger.

Run: /home/deck/semio/.venv/bin/python -I fleet1108_ndp.py REGISTRY.json FIXTURE_DIR
Idempotent. Never types a library value by hand: every library-backed status comes from `verify.FACTS` and `fleet1108_probes.probes()`.
Never claims a printed-standard verification: a status flip names the library and its version only.
"""
import importlib.util, json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
TODAY = "2026-11-08"
TAG = f"fleet-1108 ndp {TODAY}"


def load(name):
    spec = importlib.util.spec_from_file_location(name, HERE / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


verify, probes = load("verify"), load("fleet1108_probes")
PEND = "to be recorded at verification"
DOC92 = "DIN EN 1992-1-1/NA"


def new(rid, clause, symbol, rec, na, unit="", doc=DOC92, code="EN 1992", part="1-1", spec="fleet-1108 ndp: library fact", written=""):
    return dict(id=rid, code=code, part=part, clause=clause, symbol=symbol, recommended=rec, naDe=na, unit=unit,
                source=dict(document=doc, edition=PEND, locator=clause), status="unverified", specRef=spec, asWritten=written or symbol, evidence="")


NEW = [
    new("ndp.en1992.d-04.class-fck.xc1", "4.4.1.2 Table 4.3N", "f_ck threshold XC1 for structural class reduction", 30.0, 30.0, "MPa", written="Structural class minus 1 from f_ck >= 30 MPa (C30/37) for XC1; the DE value follows spec en1992-target.md:81 (Table 4.3N DE)."),
    new("ndp.en1992.d-04.class-fck.xc2-xc3", "4.4.1.2 Table 4.3N", "f_ck threshold XC2, XC3 for structural class reduction", 35.0, None, "MPa", written="Structural class minus 1 from f_ck >= 35 MPa (C35/45) for XC2 and XC3."),
    new("ndp.en1992.d-04.class-fck.xc4-xd1-xd2-xs1", "4.4.1.2 Table 4.3N", "f_ck threshold XC4, XD1, XD2, XS1 for structural class reduction", 40.0, None, "MPa", written="Structural class minus 1 from f_ck >= 40 MPa (C40/50) for XC4, XD1, XD2 and XS1."),
    new("ndp.en1992.d-04.class-fck.xd3-xs2-xs3", "4.4.1.2 Table 4.3N", "f_ck threshold XD3, XS2, XS3 for structural class reduction", 45.0, None, "MPa", written="Structural class minus 1 from f_ck >= 45 MPa (C45/55) for XD3, XS2 and XS3."),
    new("ndp.en1992.cot-theta-min", "6.2.3(2) Eq. 6.7N", "cot theta lower limit", 1.0, 1.0, written="1.0 <= cot theta; the DE spec keeps the lower limit 1.0."),
    new("ndp.en1992.nu1-en-factor", "6.2.2(6) Eq. 6.6N", "nu EN factor", 0.6, None, written="EN nu = 0.6 (1 - f_ck/250); the DE annex uses nu1 = 0.75 nu2 (ids nu1-factor, nu2-offset, nu2-fck-divisor)."),
    new("ndp.en1992.nu1-en-divisor", "6.2.2(6) Eq. 6.6N", "nu EN f_ck divisor", 250.0, None, "", written="EN nu = 0.6 (1 - f_ck/250), f_ck in MPa."),
    new("ndp.en1992.nu1-strut-factor", "6.2.3(3) Eq. 6.10aN", "nu1 strut factor f_ck <= 60 MPa", 0.6, None, written="nu1 = 0.6 for f_ck <= 60 MPa, otherwise max(0.9 - f_ck/200, 0.5) (Eq. 6.10bN)."),
    new("ndp.en1992.punch-cs-vrdc-factor", "6.4.5(1) Eq. 6.52", "factor of v_Rd,c in v_Rd,cs", 0.75, 0.75, written="v_Rd,cs = 0.75 v_Rd,c + 1.5 (d/s_r) A_sw f_ywd,ef/(u_1 d) sin(alpha); the DE value follows spec en1992-target.md:246."),
    new("ndp.en1992.punch-cs-asw-factor", "6.4.5(1) Eq. 6.52", "factor of the link term in v_Rd,cs", 1.5, 1.5, written="v_Rd,cs = 0.75 v_Rd,c + 1.5 (d/s_r) A_sw f_ywd,ef/(u_1 d) sin(alpha); public sources report kappa_sw,1 = 2.5 and kappa_sw,2 = 1.4 for DE, not confirmed (print check)."),
    new("ndp.en1992.punch-fywd-ef-base", "6.4.5(1) Eq. 6.52sub1", "f_ywd,ef constant", 250.0, 250.0, "MPa", written="f_ywd,ef = min(250 + 0.25 d, f_ywd) with d in mm; the DE value follows spec en1992-target.md:246."),
    new("ndp.en1992.punch-fywd-ef-slope", "6.4.5(1) Eq. 6.52sub1", "f_ywd,ef slope in d", 0.25, 0.25, "MPa/mm", written="f_ywd,ef = min(250 + 0.25 d, f_ywd) with d in mm; the DE value follows spec en1992-target.md:246."),
    new("ndp.en1992.p-r-sup.pre-or-unbonded", "5.10.9(1)", "r_sup pre-tensioning or unbonded tendons", 1.05, None, written="r_sup = 1.05 for pre-tensioning or unbonded tendons (EN recommended)."),
    new("ndp.en1992.p-r-sup.post-bonded", "5.10.9(1)", "r_sup post-tensioning with bonded tendons", 1.1, None, written="r_sup = 1.10 for post-tensioning with bonded tendons (EN recommended)."),
    new("ndp.en1992.p-r-inf.pre-or-unbonded", "5.10.9(1)", "r_inf pre-tensioning or unbonded tendons", 0.95, None, written="r_inf = 0.95 for pre-tensioning or unbonded tendons (EN recommended)."),
    new("ndp.en1992.p-r-inf.post-bonded", "5.10.9(1)", "r_inf post-tensioning with bonded tendons", 0.9, None, written="r_inf = 0.90 for post-tensioning with bonded tendons (EN recommended)."),
    new("ndp.en1995.floor.a-limit", "7.3.3(2)", "a unit-load deflection limit", 1.5, 0.5, "mm/kN", "DIN EN 1995-1-1/NA", "EN 1995", "1-1", written="EN recommended a = 1.5 mm/kN, NA-DE a = 0.5 mm/kN (spec en1995-en1996-target.md:231); no library holds the value."),
]

UNIT_FIX = {"ndp.en1992.delta-c-dev": "mm"}

NOTES = {
    "ndp.en1992.m-01.alpha-cc-accidental": ("E92-02: EN 1.0 (EN recommended, alpha_cc default of structuralcodes and blue-prints for persistent situations; no library holds the accidental case). DE 0.85 is kept: public sources (LBV Brandenburg, Dlubal) give 0.85 and a higher value only in justified cases; spec en1992-target.md:28 says 1.0 for DE and is to be corrected. Open: NA-DE print.", "3.1.6(1)P", "Accidental situation: EN alpha_cc = 1.0 with gamma_c = 1.2; DE keeps alpha_cc = 0.85 with gamma_c = 1.3 (public sources; spec line 28 says 1.0 for DE)."),
    "ndp.en1992.m-01.gamma-c-accidental": ("E92-05: EN 1.2 (EN 1992-1-1 Table 2.1N, recommended), DE 1.3 (Table NA.2.1DE, public sources FRILO, LBV). Neither accidental value is encoded by structuralcodes or blue-prints (they hold the persistent 1.5 only), so both stay unverified. Repo is right, spec line 28 states the EN value for both annexes.", "2.4.2.4(1) Table 2.1N", "Accidental situation: EN gamma_c = 1.2, DE gamma_c = 1.3."),
    "ndp.en1992.m-01.alpha-cc-persistent": ("E92-02: persistent EN alpha_cc = 1.0 confirmed by the libraries; DE 0.85 open.", None, None),
    "ndp.en1992.m-01.alpha-ct": ("E92-02: EN alpha_ct = 1.0 confirmed by the libraries; DE 0.85 open.", None, None),
    "ndp.en1992.m-01.gamma-c-persistent": ("E92-04: gamma_c = 1.5 confirmed by the libraries for EN; DE 1.5 open.", None, None),
    "ndp.en1992.d-04.1": ("E92-13: the EN Table 4.3N strength thresholds per exposure class are now registered as ndp.en1992.d-04.class-fck.* and are executed by blue-prints (XC1 C30/37, XC2/XC3 C35/45, XC4/XD1/XD2/XS1 C40/50, XD3/XS2/XS3 C45/55). Current artifact code reads the same thresholds (materials-and-time structural_class), so the old 35 MPa conflict is resolved in code. This row keeps the Delta c_dev text: 10 mm (blue-prints DEFAULT_DELTA_C_DEV) against ndp.en1992.delta-c-dev DE 15 mm; DE Delta c_dev is open (print).", "4.4.1.3(3)", "Delta c_dev: EN 10 mm; DE open between 10 mm (spec en1992-target.md:81) and 15 mm (ndp.en1992.delta-c-dev); thresholds of Table 4.3N in ndp.en1992.d-04.class-fck.*."),
    "ndp.en1992.v-02.1": ("E92-24, E92-25: resolved in code (shear-torsion v_rd_c, vmin_coeff) through ndp.en1992.k1-shear (EN 0.15 confirmed by structuralcodes VRdc default; DE 0.12 public), crd-c-shear (EN 0.18 confirmed; DE 0.15 public), vmin-shear (EN 0.035 confirmed by structuralcodes vmin; DE numerator 0.0525 before division by gamma_c, d <= 600 mm) and vmin-shear-d800 (DE 0.0375, d > 800 mm). Open: NA-DE print.", "6.2.2(1) Eq. 6.2a, 6.3N", "EN: C_Rd,c = 0.18/gamma_c, k1 = 0.15, v_min = 0.035 k^1.5 sqrt(f_ck); DE: 0.15/gamma_c, 0.12, (0.0525/gamma_c) k^1.5 sqrt(f_ck) for d <= 600 mm and (0.0375/gamma_c) for d > 800 mm, linear in between."),
    "ndp.en1992.v-03.1": ("E92-26: resolved in code (shear-torsion cot_theta_max). EN 1 <= cot theta <= 2.5 confirmed by structuralcodes (ids cot-theta-min, cot-theta-max). DE cot theta <= min(3.0, (1.2 + 1.4 sigma_cp/f_cd)/(1 - V_Rd,cc/V_Ed)) with V_Rd,cc = 0.5 * 0.48 f_ck^(1/3) (1 - 1.2 sigma_cp/f_cd) b_w z: public single source (Infograph), ids cot-theta-a, cot-theta-b, vrdcc-c, vrdcc-k. Open: NA-DE print.", "6.2.3(2) Eq. 6.7N", "EN: 1.0 <= cot theta <= 2.5; DE: 1.0 <= cot theta <= min(3.0, (1.2 + 1.4 sigma_cp/f_cd)/(1 - V_Rd,cc/V_Ed))."),
    "ndp.en1992.e-03.1": ("E92-82: resolved in code (shear-torsion nu1). EN nu = 0.6 (1 - f_ck/250) confirmed by structuralcodes (ids nu1-en-factor, nu1-en-divisor); DE nu1 = 0.75 nu2, nu2 = min(1.1 - f_ck/500, 1.0) is public (KIT, FRILO), ids nu1-factor, nu2-offset, nu2-fck-divisor. Open: NA-DE print.", "6.2.2(6), 6.2.3(3)", "EN nu = 0.6 (1 - f_ck/250); DE nu1 = 0.75 nu2 with nu2 = min(1.1 - f_ck/500, 1.0); worked example f_ck = 30 MPa gives nu2 = 1.0 and nu1 = 0.75 for DE."),
    "ndp.en1992.e-05.1": ("E92-83: the worked example C_Rd,c = 0.18/1.5 = 0.12 is the EN value, confirmed by structuralcodes VRdc (CRdc = 0.18/gamma_c) and blue-prints Form 6.47. The DE shear value 0.15/1.5 = 0.10 is a different NDP (ndp.en1992.crd-c-shear), so the repo is consistent: 0.12 for EN, 0.10 for DE.", "6.2.2(1)", "Worked example: EN C_Rd,c = 0.18/1.5 = 0.12; DE shear C_Rd,c = 0.15/1.5 = 0.10."),
    "ndp.en1992.p-03.2": ("E92-39, E92-41: resolved in code (punching-and-struts c_rd_c_numerator, v_rd_c_punch). EN C_Rd,c = 0.18/gamma_c and k1 = 0.1 confirmed by blue-prints Form 6.47 (documented recommended values; ids crd-c-punch, k1-punch). DE 0.18/gamma_c for flat slabs, 0.18/gamma_c (0.1 u0/d + 0.6) for interior columns with u0/d < 4 (ids punch-small-u0.*) and v_min as in shear: public (Dlubal, same vendor), open for the NA-DE print. The values of this row were a wrong mix of k1 and C_Rd,c and are removed.", "6.4.4(1) Eq. 6.47", "EN: C_Rd,c = 0.18/gamma_c, k1 = 0.1, v_min = 0.035 k^1.5 sqrt(f_ck); DE: C_Rd,c = 0.18/gamma_c (reduced for interior columns with u0/d < 4), k1 = 0.1, v_min with kappa1 = 0.0525 / 0.0375."),
    "ndp.en1992.p-04.1": ("E92-43: resolved in code (resistance-and-serviceability-checks, v_Rd,cs). The EN formula v_Rd,cs = 0.75 v_Rd,c + 1.5 (d/s_r) A_sw f_ywd,ef/(u_1 d) sin(alpha) with f_ywd,ef = min(250 + 0.25 d, f_ywd) is executed by blue-prints Form 6.52 and 6.52sub1 (ids punch-cs-vrdc-factor, punch-cs-asw-factor, punch-fywd-ef-base, punch-fywd-ef-slope). DE kappa_sw,1 = 2.5 and kappa_sw,2 = 1.4 are reported by one public source and are not applied by the artifact: open (print).", "6.4.5(1) Eq. 6.52", "v_Rd,cs = 0.75 v_Rd,c + 1.5 (d/s_r) A_sw f_ywd,ef (1/(u_1 d)) sin(alpha), f_ywd,ef = min(250 + 0.25 d, f_ywd); v_Rd,max = 1.4 v_Rd,c,u1 for DE."),
    "ndp.en1992.b-01.1": ("E92-47: resolved in code (materials-and-time f_ctd = alpha_ct f_ctk,0.05/gamma_c, detailing f_bd_from). EN f_bd = 2.25 eta1 eta2 f_ctd confirmed by blue-prints Form 8.2 (id fbd-eta), alpha_ct = 1.0 by structuralcodes (id m-01.alpha-ct); DE alpha_ct = 0.85 public only.", "8.4.2(2) Eq. 8.2", "f_bd = 2.25 eta1 eta2 f_ctd with f_ctd = alpha_ct f_ctk,0.05/gamma_c; EN alpha_ct = 1.0, DE alpha_ct = 0.85."),
    "ndp.en1992.f-03.1": ("E92-59: no conflict. Table 6.3N gives the characteristic range Delta sigma_Rsk = 162.5 MPa (straight bars, N* = 10^6); the artifact divides by gamma_s,fat = 1.15 (ids fatigue-straight, gamma-s-fat) to the design value 141.3 MPa. No library holds Table 6.3N; stays unverified.", "6.8.4 Table 6.3N", "Delta sigma_Rsk(N*) = 162.5 MPa for straight bars (N* = 10^6, k1 = 5, k2 = 9); design value = 162.5/gamma_s,fat."),
    "ndp.en1992.fa-03.1": ("E92-80: resolved in code (fastenings k1_cone reads ndp.en1992.cone-k-*): cast-in 8.9 cracked / 12.7 uncracked, post-installed 7.7 / 11.0 (EN 1992-4). No library holds EN 1992-4; stays unverified, print check.", "EN 1992-4 7.2.1.4", "N0Rk,c = k1 sqrt(f_ck) h_ef^1.5 with k1 = 7.7 / 11.0 (post-installed cracked / uncracked) and 8.9 / 12.7 (cast-in cracked / uncracked)."),
    "ndp.en1992.e-12.1": ("E92-86: the artifact (fire-design) holds BEAM_CONT 80/15, 120/25, 150/35, 200/45, 240/60 and a separate simply-supported row set; the spec lists the continuous-beam set under 'column 2 simply supported'. No library holds EN 1992-1-2 tables; unresolved between Table 5.5 and 5.6 labelling, print check.", "EN 1992-1-2 Tables 5.5, 5.6", "Beam fire table: simply supported beams and continuous beams use separate rows; the b_min/a set 80/15 to 240/60 is the continuous-beam set."),
    "ndp.en1995.t-03.1": ("E95-14, E95-15, E95-16, E95-21: the registry splits gamma_M per material (ndp.en1995.gamma-m.*): EN recommended solid 1.3, glulam 1.25, LVL 1.2, plywood 1.2, OSB 1.2, CLT 1.25; DE 1.3 for all per public source L1 (CLT DE value open). No library holds EN 1995-1-1 Table 2.3; stays unverified. Spec line 43 lists the EN recommended values for DE and is to be corrected.", "2.4.1 Table 2.3", "gamma_M per material: EN recommended solid 1.3, glulam 1.25, LVL/plywood/OSB 1.2, CLT 1.25; DE 1.3 (public single source)."),
    "ndp.en1995.t-13.1": ("E95-26: resolved in code (cross-sections k_c90_factor): 1.25 / 1.5 solid continuous / discrete, 1.5 / 1.75 glulam continuous / discrete (l <= 400 mm), 1.0 otherwise. No library holds EN 1995-1-1 6.1.5; stays unverified.", "6.1.5(4)", "k_c,90 = 1.25 (solid, continuous), 1.5 (solid, discrete), 1.5 (glulam, continuous), 1.75 (glulam, discrete), 1.0 otherwise."),
    "ndp.en1995.t-15.1": ("E95-29: resolved in code (k_cr_value): 1.0 for LVL and CLT, 0.67 (EN) or 2.5/f_v,k (DE, ids k-cr.en-constant, k-cr.de-numerator) for solid timber and glulam. No library holds EN 1995-1-1 6.1.7; stays unverified.", "6.1.7(2)", "k_cr = 0.67 (EN) or 2.5/f_v,k (DE) for solid timber and glulam, 1.0 for LVL and other products."),
    "ndp.en1995.t-63.1": ("E95-44: the artifact (timber structure-and-members) uses 8 Hz for EN and 6 Hz for DE; registry id ndp.en1995.floor.f1-limit holds EN 8 / DE 6. The artifact reads literals instead of the id: request logged. No library holds the value (blue-prints en_1995_1_1_2004 covers formulas 7.5 to 7.7 only); DE 6 Hz stays unverified.", "7.3.3(2)", "f1 lower limit: EN 8 Hz, DE 6 Hz (ndp.en1995.floor.f1-limit)."),
    "ndp.en1995.t-64.1": ("E95-45, E95-46: the artifact uses a = 1.5 mm/kN (EN) and 0.5 mm/kN (DE), as the spec. The new registry id ndp.en1995.floor.a-limit holds both; the artifact reads literals: request logged. No library holds the value; stays unverified.", "7.3.3(2)", "a = 1.5 mm/kN (EN recommended), 0.5 mm/kN (DE) (ndp.en1995.floor.a-limit)."),
    "ndp.en1995.e3-bearing-perpendicular.1": ("E96-63: depends on E95-44; the artifact threshold for DE is 6 Hz (ndp.en1995.floor.f1-limit), so the example f1 = 7.08 Hz passes the DE stage. Stays unverified.", None, None),
    "ndp.en1996.m-01.1": ("E96-01: the shape factor delta is computed from EN 772-1 Table A.1 by bilinear interpolation (artifact materials, id ndp.en1996.m01.delta). No library holds EN 772-1; the spec example delta = 1.15 for 238 x 240 mm is not reproduced by the table lookup: print check.", "3.1.2.2 Table A.1 of EN 772-1", "delta by interpolation of EN 772-1 Table A.1 (unit height and least horizontal dimension)."),
    "ndp.en1996.m-02.2": ("E96-06 to E96-09: the registry holds the EN Table 3.3 K values per mortar, material and group (ids k.gp.*, k.tl.*): clay GP G1 0.55, G2 0.45, G3 0.35, G4 0.35; clay thin layer G1 0.75, G2 0.70, G3 0.50, G4 0.35. They agree with the public reproduction EN6-33; no library holds EN 1996-1-1 Table 3.3. DE K values (own f_k tables of the NA) are open.", "3.6.1.2 Table 3.3", "f_k = K f_b^0.7 f_m^0.3: EN K per Table 3.3 by material, group and mortar type; the DE NA gives its own f_k tables."),
    "ndp.en1996.m-03.1": ("E96-15, E96-16, E96-18: the registry holds EN Table 3.4 f_vk0 per mortar band, material and mortar type (ids fvk0.gp.*, fvk0.tl.*, fvk0.lw.*): clay GP M10-M20 0.30, M2.5-M9 0.20, M1-M2 0.10; thin layer clay 0.30; lightweight 0.15. They agree with public reproduction EN6-34; no library holds the table. DE values (0.08 to 0.26) are public (ZAPF) and open.", "3.6.2 Table 3.4", "f_vk0 by mortar and unit material per Table 3.4; DE NA Table NA.5 differs."),
    "ndp.en1996.m-03.3": ("E96-21: the artifact computes f_vlt from the registry (ids fvlt.f-b-factor EN 0.065, fvlt.de-k 0.45, fvlt.de-share.g1/g2/g34 = 0.032 / 0.026 / 0.020): the old 0.045 f_b is no longer used. Public source ZAPF; no library; stays unverified.", "3.6.2(3)", "EN f_vlt = 0.065 f_b; DE f_vlt = 0.45 f_bt,cal sqrt(1 + sigma/f_bt,cal), f_bt,cal = 0.020 / 0.026 / 0.032 f_b for hollow / perforated / solid units."),
    "ndp.en1996.m-04.1": ("E96-22, E96-24, E96-25: registry ids fxk.x1.* and fxk.x2.* hold f_xk1 / f_xk2 by material and f_m class. No library; spec and registry still differ for clay f_m < 5 (0.10 both) and calcium silicate (registry 0.05 / 0.10 f_xk1, 0.20 / 0.40 f_xk2): print check.", "3.6.3", "f_xk1, f_xk2 by unit material and mortar strength class (EN Table 3.6 and NA-DE)."),
    "ndp.en1996.m-07.1": ("E96-36: the registry holds EN gamma_M by class 1.5 / 1.7 / 2.0 / 2.2 / 2.5 (ids gamma-m.c1 to c5) matching the designed-mortar Category I row; DE 1.5 / 1.7 / 1.7 / 2.0 / 2.0 and accidental 1.3. No library; stays unverified. The spec text '1.7 / 2.0' is a mix of designed and prescribed mortar.", "2.4.3 Table 2.3", "gamma_M by masonry class: EN Category I designed mortar 1.5 to 2.5; DE per NA."),
    "ndp.en1996.m-10.1": ("E96-38: rho_2 is read through ndp.en1996.m10.rho2 in the artifact (vertical-load); the old fixed 0.75 is gone. No library; print check.", "5.5.1.2", "rho_2 = 1.0 (timber floors) and 0.9 per NA."),
    "ndp.en1996.m-80.1": ("E96-54: no library; print check of the clear storey height limit (DE 3.0 m, 2.75 m for t < 240 mm) against ndp.en1996.limit.simplified.height-thin 2.75.", "NA 4.5", "Clear storey height limits of the simplified method."),
    "ndp.en1996.m-81.1": ("E96-57, E96-58: no library; print check of Phi_1 = 0.85 - 0.0011 (h_ef/t)^2 and the 4.2 m span threshold of Phi_2.", "NA 6.1", "Phi_1 = 0.85 - 0.0011 (h_ef/t)^2; Phi_2 = 1.6 - l_f/6 for spans above 4.2 m."),
    "ndp.en1996.m-90.1": ("E96-60: the registry holds the REI table (ids fire.clay.rei30 to rei240); clay group 1 rows 100 / 100 / 150 / 175 / 240 / 240 equal the spec. No library; print check.", "EN 1996-1-2 Table", "REI t_min for clay group 1: 100, 100, 150, 175, 240, 240 mm."),
    "ndp.en1996.x1-fk-for-clay-group-1-g.4": ("E96-67: no library; print check of the REI90 row (150 / 175 mm).", None, None),
}

DOWNGRADE_NOTE = "fleet-1108 ndp 2026-11-08: UNVERIFIED. No third-party library holds EN 14080 Table 5 (blue-prints holds EN 338:2016 only); the earlier verified flag cited public secondary sources only. Public values (Hasslacher HNT, Leporello 3, civilaxis) agree with this registry value."


def main():
    registry, outdir = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    doc = json.loads(registry.read_text(encoding="utf-8"))
    by_id = {e["id"]: e for e in doc["entries"]}
    for entry in NEW:
        by_id.setdefault(entry["id"], entry)
    doc["entries"] = sorted(by_id.values(), key=lambda e: e["id"])
    for rid, unit in UNIT_FIX.items():
        by_id[rid]["unit"] = unit

    lib = probes.probes()
    for rid, p in lib.items():
        if rid in by_id:
            verify.fact(rid, p["value"], p["unit"], p["scope"], f"{p['cite']} [{p['kind']}]")

    stats = dict(verified=0, annex=0, downgraded=0, noted=0)
    for e in doc["entries"]:
        rid = e["id"]
        if e["status"] == "verified" and e["code"] == "EN 1995" and not verify.FACTS.get(rid):
            e["status"], e["evidence"] = "unverified", DOWNGRADE_NOTE + " | previous: " + e["evidence"]
            e["source"]["edition"] = PEND
            stats["downgraded"] += 1
        cls, c = verify.classify(e)
        if not c or c["inferred"]:
            continue
        cite = f"{c['cite']}; library value {c['value']:g} {c['unit']}".rstrip()
        if cls == "verified" and e["status"] != "verified":
            old = e["evidence"]
            e["evidence"] = f"{TAG}: VERIFIED by {cite}. Earlier public or spec values are superseded by the library value."
            e["status"] = "verified"
            if m := re.fullmatch(r"ndp\.en1995\.strength\.([cd]\d+)\.\w+", rid):
                e["source"]["edition"] = "EN 338:2016"
                e["asWritten"] = f"{e['symbol']} = {e['recommended']:g} {e['unit']} (EN 338:2016 Table {'1' if m.group(1)[0] == 'c' else '3'})"
            elif e["source"]["edition"] == PEND:
                e["source"]["edition"] = "EN edition of the cited library table"
            stats["verified"] += 1
        elif cls == "en_confirmed_na_open" and "annexStatus" not in e:
            e["annexStatus"] = dict(
                en=dict(status="verified", evidence=f"EN recommended {e['recommended']:g} confirmed by {cite}."),
                naDe=dict(status="unverified", evidence="NA-DE value stays open: no third-party library carries the German national annex and the print is not checked."))
            e["evidence"] = (e["evidence"] + " | " if e["evidence"] else "") + f"{TAG}: per-annex status in annexStatus, EN verified by {c['cite']}, NA-DE unverified."
            stats["annex"] += 1

    for rid, (note, locator, written) in NOTES.items():
        e = by_id.get(rid)
        if e is None:
            raise SystemExit(f"unknown id {rid}")
        marker = f"{TAG}: {note}"
        if marker not in e["evidence"]:
            e["evidence"] = (e["evidence"] + " | " if e["evidence"] else "") + marker
            stats["noted"] += 1
        if locator:
            e["source"]["locator"] = e["clause"] = locator
        if written:
            e["asWritten"] = written
    for rid in ("ndp.en1992.p-03.2",):
        by_id[rid]["recommended"] = by_id[rid]["naDe"] = None
        by_id[rid]["symbol"] = "C_Rd,c, k1, v_min (punching)"

    fixture = []
    for e in doc["entries"]:
        per = e.get("annexStatus")
        en_ok = e["status"] == "verified" or (per and per["en"]["status"] == "verified")
        cands = verify.FACTS.get(e["id"])
        if en_ok and cands and not cands[0]["inferred"] and e["recommended"] is not None:
            c = cands[0]
            fixture.append(dict(id=e["id"], value=float(c["value"]), unit=c["unit"], scope=c["scope"], kind="documented" if c["cite"].endswith("[documented]") else "executed",
                                library=c["cite"].split(" ")[0] + " " + c["cite"].split(" ")[1], cite=c["cite"]))
    registry.write_text(json.dumps(doc, ensure_ascii=False, indent=1), encoding="utf-8")
    outdir.mkdir(parents=True, exist_ok=True)
    (outdir / "🔣️.json").write_text(json.dumps(dict(
        description="Registry rows whose EN recommended value is independently encoded by a third-party library (blue-prints, structuralcodes, groundhog). Generated by 🧰️ndp/fleet1108_ndp.py from the installed library versions; do not edit by hand. kind executed = the library computes the number, documented = the library docstring states the recommended value.",
        rows=sorted(fixture, key=lambda r: r["id"])), ensure_ascii=False, indent=1), encoding="utf-8")
    print(stats, "fixture rows", len(fixture), "entries", len(doc["entries"]))


if __name__ == "__main__":
    main()
