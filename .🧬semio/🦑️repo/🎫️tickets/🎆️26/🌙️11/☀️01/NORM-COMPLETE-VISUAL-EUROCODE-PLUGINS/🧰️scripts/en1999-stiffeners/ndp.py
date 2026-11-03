"""EN 1999 transverse web stiffeners: registers the unverified structure constants of the stiffness and strength checks (spec A-28, EN 1999-1-1 6.7.6/6.7.7). The spec holds no values; they are recalled from EN 1993-1-5 9.1 and 9.3.3 and flagged unverified. Usage: python -I ndp.py"""
import pathlib, subprocess, sys, math
NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

d = load()
before = len(d)
RECALLED = "recalled from the intermediate transverse stiffener rules of EN 1993-1-5 9.1 and 9.3.3 (same plate girder mechanics); the value is NOT in the spec and NOT checked against the printed EN 1999-1-1 6.7.6/6.7.7"
for key, clause, symbol, value, note in (
    ("strip-factor", "6.7.6 (effective web strip of a transverse stiffener)", "15", 15.0, "web strip of 15·ε·t_w on each side of the stiffener that acts with it; " + RECALLED),
    ("inertia-factor-short", "6.7.6 (stiffener stiffness, a/h_w < aspect limit)", "1.5", 1.5, "required I_st >= factor·h_w³·t_w³/a² for panels with a/h_w below the aspect limit; " + RECALLED),
    ("inertia-factor-long", "6.7.6 (stiffener stiffness, a/h_w >= aspect limit)", "0.75", 0.75, "required I_st >= factor·h_w·t_w³ for panels with a/h_w at or above the aspect limit; " + RECALLED),
    ("aspect-limit", "6.7.6 (stiffener stiffness, panel aspect ratio)", "√2", math.sqrt(2.0), "panel aspect ratio a/h_w separating the two stiffness requirements; " + RECALLED),
    ("buckling-length-factor", "6.7.6 (stiffener strut buckling length)", "0.75", 0.75, "buckling length l = factor·h_w of the stiffener with its effective web strip as a column; " + RECALLED),
):
    put(d, "1999", f"web-stiffener.{key}", clause, symbol, value, value, "", note, part="1-1", keep=True)
save(d)
subprocess.run([sys.executable, "-I", str(NDP_TOOLS / "codegen.py"), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print("rows", before, "->", len(d))
