"""EN 1999 v6: registers the stiffener flexural rows, re-words the ten web crippling coefficient rows with exact clause references and re-words the ring stiffener inertia factor. Usage: python -I ndp.py"""
import pathlib, subprocess, sys
NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

d = load()
before = len(d)
for pos in ("end", "interior"):
    for key, symbol in (("alpha", "α"), ("c-r", "c_r"), ("c-a", "c_a"), ("c-l", "c_l"), ("c-phi", "c_φ")):
        put(d, "1999", f"web-crippling.{pos}.{key}", "6.1.7 (web crippling)", symbol, None, None, "", f"DIN EN 1999-1-4 §6.1.7: coefficient {symbol} of R_w,Rd = α·t²·√(f_o·E)·(1 − c_r·√(r/t))·(c_a + √(c_l·s_s/t))·(c_φ + (φ/90°)²)/γ_M1 for the load position '{pos}' (coefficient table of the clause); not transcribed from the printed standard and not in the spec, kept null so no value is invented. The check reports 'awaiting verified coefficients' until all ten rows are numbers", part="1-4", locator=f"EN 1999-1-4 §6.1.7, {symbol} ({pos} load)")
put(d, "1999", "stiffener.edge-slenderness-limit", "6.7.7 (longitudinal stiffeners, edge stiffener flexure)", "λ̄_s,max", 1.0, 1.0, "", "mechanics threshold of the edge stiffener flexural check: the lip on the elastic plate foundation must not buckle elastically below yield (σ_cr,s = 2·√(K·E·I_s)/A_s >= f_o, i.e. λ̄_s <= 1); the acceptance criterion printed in EN 1999-1-1 §6.7.6/6.7.7 is NOT transcribed", part="1-1", keep=True)
put(d, "1999", "ring-stiffener.required-inertia-factor", "6.7.6 and 5 (ring stiffener stiffness)", "γ_r", None, None, "", "design margin on the elastic critical radial line load w_cr = 3·E·I/r³ of the ring plus effective shell strip (n = 2 mode): the ring needs I >= γ_r·w_Ed·r³/(3·E); the margin of EN 1999-1-1 §6.7.6 / EN 1999-1-5 is not transcribed and kept null, the check then evaluates with 1.0 and reports 'awaiting verified margin'", part="1-5")
save(d)
subprocess.run([sys.executable, "-I", str(NDP_TOOLS / "codegen.py"), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print("rows", before, "->", len(d))
