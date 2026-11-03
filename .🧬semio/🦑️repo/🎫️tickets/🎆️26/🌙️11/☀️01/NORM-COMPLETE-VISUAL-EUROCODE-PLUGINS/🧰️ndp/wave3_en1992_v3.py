import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
UNV = "Unverified (en1992 v3): value as implemented by the EN 1992 artifact; no normative table was read. "
ROWS = [
    ("psi-fire-lead", "EN 1990 6.5.3 / EN 1992-1-2 2.4.2", "psi_fi selector", 1.0, 1.0, "", "1-2", "Combination of the leading variable action in fire: 1 = frequent value psi_1,1, 2 = quasi-permanent value psi_2,1. The check uses 1.", UNV + "EN 1990 6.5.3 (2) allows psi_1,1 or psi_2,1; the NA choice is not read."),
    ("fire-zones", "EN 1992-1-2 B.2", "n", 5.0, 5.0, "", "1-2", "Number of parallel zones of the zone method.", UNV + "Implementation constant 5."),
    ("fire-zone-exponent", "EN 1992-1-2 B.2", "e_az", 1.0, 1.0, "", "1-2", "Exponent of a_z = w [1 - (k_c,m / k_c(theta_M))^e]; spec R-04 has no exponent, implemented 1.", UNV + "EN 1992-1-2 Eq. B.6 is quoted from memory with exponent 1.3 [V]; switch the row only after reading the print."),
    ("ld-flange-factor", "EN 1992-1-1 Table 7.4N note 2", "k_flange", 0.8, 0.8, "", "1-1", "Factor on the span-to-depth limit for flanged sections with b_f/b_w above the ratio row.", UNV + "Applied from memory; the NA-DE may differ."),
    ("ld-flange-ratio", "EN 1992-1-1 Table 7.4N note 2", "b_f/b_w", 3.0, 3.0, "", "1-1", "Flange to web width ratio above which the flange factor applies.", UNV + "From memory."),
    ("col-link-spacing-de", "EN 1992-1-1 9.5.3(3) / NA-DE", "k_s,cl", 20.0, 12.0, "", "1-1", "Factor on the smallest longitudinal bar diameter for the largest link spacing of columns (EN 20, NA-DE 12).", UNV + "NA-DE 12 phi_min from spec DT-09 text; not read in DIN EN 1992-1-1/NA."),
    ("col-link-spacing-max", "EN 1992-1-1 9.5.3(3) / NA-DE", "s_cl,t,max", 400.0, 400.0, "mm", "1-1", "Absolute cap of the link spacing of columns.", UNV + "A later review quotes 300 mm for NA-DE 9.5.3(3) [V]; the implemented 400 mm is kept for both annexes until the NA is read."),
    ("col-link-near-factor", "EN 1992-1-1 9.5.3(4)", "k_near", 0.6, 0.6, "", "1-1", "Reduction of the link spacing near beams and above or below slabs and at laps.", UNV + "0.6 as in EN 1992-1-1 9.5.3(4); the NA-DE distance rules are not read."),
    ("lq-04-wk-selfhealing", "EN 1992-3 7.3.1(105)", "w_k,lim", 0.2, 0.2, "mm", "3", "Crack width limit with self-healing when h_D/h is not above the head ratio.", UNV + "Spec LQ-04; EN 1992-3 7.3.1(105) states a different rule than the spec."),
    ("lq-04-wk-above", "EN 1992-3 Table 7.105DE", "w_k,lim", 0.1, 0.1, "mm", "3", "Crack width limit for h_D/h above the head ratio (NA-DE tightness class 1).", UNV + "Spec LQ-04 / NA-DE Table 7.105DE [V]; the EN value is not read, 0.1 kept for both annexes as implemented."),
    ("lq-04-head-ratio", "EN 1992-3 7.3.1(105)", "h_D/h", 5.0, 5.0, "", "3", "Hydrostatic head ratio up to which self-healing applies.", UNV + "Spec LQ-04."),
    ("na-de-edition", "DIN EN 1992-1-1/NA", "edition year", 0.0, 2013.0, "year", "1-1", "Edition year of the national annex the DE values are taken from (cited: DIN EN 1992-1-1/NA:2013-04).", UNV + "The current edition list was not checked; check the print edition and its amendments before verifying any NA-DE row."),
]
d = load()
for key, clause, symbol, rec, de, unit, part, written, ev in ROWS:
    i = put(d, "1992", key, clause, symbol, rec, de, unit, written, part=part)
    d[i]["evidence"] = ev
    d[i]["specRef"] = "wave3 en1992 v3: constant formerly hard-coded in the checks"
d["ndp.en1992.bridge-alpha-cc-lt"]["evidence"] = UNV + "NA-DE zu EN 1992-2 3.1.6 long-term factor 0.8 for prestressed bridges [V]; wired in BR-01."
save(d)
print(len(ROWS), len(d))
