"""EN 1999 v5: registers the unverified coefficients of the web crippling (1-4 6.1.7), transverse web load (1-1 6.7.5) and ring stiffener (1-5) checks. Coefficients that are neither in the repo nor in the spec stay null (the check then reports the formula structure without a resistance); the sibling-standard structure constants are flagged as recalled. Usage: python -I ndp.py"""
import pathlib, subprocess, sys
NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

d = load()
before = len(d)
NULL_NOTE = "coefficient of the web crippling resistance R_w,Rd of EN 1999-1-4 not transcribed from the printed standard and not in the spec; kept null so no value is invented"
for pos in ("end", "interior"):
    for key, symbol in (("alpha", "α"), ("c-r", "c_r"), ("c-a", "c_a"), ("c-l", "c_l"), ("c-phi", "c_φ")):
        put(d, "1999", f"web-crippling.{pos}.{key}", "6.1.7 (web crippling)", symbol, None, None, "", NULL_NOTE + f" (load position {pos})", part="1-4", keep=True)
RECALLED = "structure constant recalled from the transverse-load rules of EN 1993-1-5 §6 (same form as the web rule of EN 1999-1-1 §6.7.5); NOT checked against the printed EN 1999-1-1"
for key, symbol, value in (("kf-base", "k_F,0", 6.0), ("kf-slope", "k_F,1", 2.0), ("fcr-factor", "F_cr factor", 0.9), ("chi-const", "χ_F constant", 0.5), ("m2-factor", "m_2 factor", 0.02)):
    put(d, "1999", f"transverse-load.{key}", "6.7.5 (transverse load on the web)", symbol, value, value, "", RECALLED, part="1-1", keep=True)
put(d, "1999", "ring-stiffener.effective-width-factor", "5 (ring stiffener, effective shell width)", "b_e/√(r·t)", 0.78, 0.78, "", "effective shell strip beside a ring stiffener b_e = factor·√(r·t), recalled from the shell rules of EN 1993-1-6; NOT checked against the printed EN 1999-1-5", part="1-5", keep=True)
put(d, "1999", "ring-stiffener.required-inertia-factor", "5 (ring stiffener stiffness)", "I_r,min factor", None, None, "", "required second moment of a ring stiffener of EN 1999-1-5 not transcribed and not in the spec; kept null so no value is invented", part="1-5", keep=True)
save(d)
subprocess.run([sys.executable, "-I", str(NDP_TOOLS / "codegen.py"), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print("rows", before, "->", len(d))
