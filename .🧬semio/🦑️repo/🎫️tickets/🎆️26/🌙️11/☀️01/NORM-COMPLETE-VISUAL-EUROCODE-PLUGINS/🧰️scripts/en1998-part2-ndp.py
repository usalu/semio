"""EN 1998-2 bridge constants of the BR-01/02/03/06 checks, registered unverified in the shared NDP registry through `🧰️ndp/ndplib.py`; then the row crates are regenerated with `🧰️ndp/codegen.py`. Idempotent.
Usage: /home/deck/semio/.venv/bin/python -I en1998-part2-ndp.py"""
import pathlib, subprocess, sys

NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

d = load()
before = len(d)
spec = "repository spec en1997-en1998-target.md, "
rows = [
    ("en2.gamma-i.i", "2.1", "γ_I bridge importance class I", 0.85, "", spec + "BR-01"),
    ("en2.gamma-i.ii", "2.1", "γ_I bridge importance class II", 1.0, "", spec + "BR-01"),
    ("en2.gamma-i.iii", "2.1", "γ_I bridge importance class III", 1.3, "", spec + "BR-01"),
    ("en2.spectrum.vertical-ratio", "3.2", "a_vg / a_g of the bridge vertical spectrum", 0.7, "", spec + "BR-02: Type 1 bridges use 0.7, not the 0.9 of buildings"),
    ("en2.q.rc-pier", "4.1.6", "q of vertical RC piers in bending, q = value·λ(α_s)", 3.5, "", spec + "BR-03"),
    ("en2.q.shear-span-full", "4.1.6", "α_s from which λ(α_s) = 1", 3.0, "", spec + "BR-03"),
    ("en2.q.shear-span-min", "4.1.6", "smallest α_s with a ductile λ(α_s) = √(α_s/3)", 1.0, "", spec + "BR-03"),
    ("en2.combination.minor-factor", "4.1.8", "factor of the two minor components in E_x + 0.3 E_y + 0.3 E_z", 0.3, "", spec + "BR-06"),
]
for key, clause, symbol, value, unit, note in rows:
    part = key.split(".")[0][2:]
    d[put(d, "1998", key, clause, symbol, value, value, unit, symbol + " = " + str(value) + " as used by the artifact", part=part, keep=True)]["evidence"] = note + "; not read from the printed standard"
save(d)
codegen = NDP_TOOLS / "codegen.py"
subprocess.run([sys.executable, "-I", str(codegen), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print("rows", before, "->", len(d))
