"""Registers the EN 1993-1-5 Annex A, B and C rows of the stiffened plate (wave 2 v5) in the NDP registry through the ticket ndplib and regenerates the row crates with the ticket codegen. Every row is unverified; the values are the formula structure as encoded, written from memory, never checked against the print edition."""
import json, pathlib, subprocess, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"))
from ndplib import *
d = load()
NOTE = "Written from memory by the en1993 artifact agent (wave 2 v5); NOT double-keyed from the print edition."
ROWS = [
    ("v-376.a1", "Annex A.1", "Smeared equal stiffeners: k_sigma,p = 2((1+alpha^2)^2+gamma-1)/(alpha^2(psi+1)(1+delta)) for alpha <= 4 sqrt(gamma), else 4(1+sqrt(gamma))/((psi+1)(1+delta)); gamma = I_sl/I_p with I_p = b t^3/10.92, delta = sum A_sl/(b t); sigma_cr,p = k_sigma,p sigma_E",
     "Formula structure as encoded; I_sl taken as the gross second moment of the whole stiffened plate; reduces to the Navier orthotropic plate solution, checked in the oracle. Open: validity range of psi and the exact definition of I_sl."),
    ("v-376.a2", "Annex A.2.2.1", "Single stiffener: a_c = 4.33 (I_sl b1^2 b2^2/(t^3 b))^(1/4); sigma_cr,sl = 1.05 E sqrt(I_sl t^3 b)/(A_sl b1 b2) for a >= a_c, else pi^2 E I_sl/(A_sl a^2) + E t^3 b a^2/(4 pi^2 (1-nu^2) A_sl b1^2 b2^2); governing = min(sigma_cr,sl, 4 sigma_E of the wider sub-panel)",
     "Constants 4.33 and 1.05 and the min combination with the sub-panel stress as encoded. Open: adjacent plate strips of A_sl and I_sl, two stiffeners (A.2.2.2) not encoded."),
    ("v-376.b1", "Annex B", "Non-uniform members: not encoded",
     "No formula of Annex B is encoded; the open item is listed so a verified transcription can replace the finite element route (Annex C) for tapered panels."),
    ("v-376.c1", "Annex C", "Finite element cross-check: ratio of the FE load factor alpha_cr to sigma_cr,p/sigma_Ed of Annex A; mesh, imperfection, material and partial factor rules of C.5 to C.9 are not encoded",
     "Only the ratio is reported (information row); no limit of the standard is applied."),
]
for key, clause, symbol, note in ROWS:
    put(d, "1993", key, clause, symbol, None, None, "", f"{note} {NOTE}", part="1-5", locator=clause)
    d[f"ndp.en1993.{key}"]["source"]["document"] = "EN 1993-1-5:2006"
    d[f"ndp.en1993.{key}"]["specRef"] = "wave2 en1993 v5: formula structure as used by the artifact"
    d[f"ndp.en1993.{key}"]["evidence"] = NOTE
save(d)
print(len(d))
REG = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp"
subprocess.run([sys.executable, "-I", str(pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp/codegen.py"), REG + "/🔣️.json", REG + "/🗃️table.rs"], check=True)
