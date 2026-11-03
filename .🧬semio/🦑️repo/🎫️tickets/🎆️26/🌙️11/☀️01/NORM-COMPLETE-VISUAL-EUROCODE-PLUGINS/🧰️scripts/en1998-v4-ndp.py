"""EN 1998 v4: registers the two unverified constants of the overturning and time-history checks in the shared NDP registry through `🧰️ndp/ndplib.py`, then regenerates the row crates with `🧰️ndp/codegen.py`. Idempotent.
Usage: /home/deck/semio/.venv/bin/python -I en1998-v4-ndp.py"""
import pathlib, subprocess, sys

NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

d = load()
before = len(d)
rows = [
    ("en1.overturning.stabilising-factor", "4.4.2 (overturning stability)", "γ stabilising on G_k in the seismic combination", 1.0, "", "spec SEI-46 line 463: overturning moment stability M_ov,Ed <= M_stab,Rd with stabilising factor 1.0, marked [V]; not read from the printed standard"),
    ("en1.time-history.zero-period-ratio", "3.2.3.1.2(4)", "mean zero-period acceleration over a_g·S", 1.0, "", "spec SEI-06 line 374: the mean of the zero-period spectral accelerations of the records is not smaller than a_g·S; not read from the printed standard"),
]
for key, clause, symbol, value, unit, note in rows:
    d[put(d, "1998", key, clause, symbol, value, value, unit, symbol + " = " + str(value) + " as used by the artifact", part="1", keep=True)]["evidence"] = note
save(d)
codegen = NDP_TOOLS / "codegen.py"
subprocess.run([sys.executable, "-I", str(codegen), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print("rows", before, "->", len(d))
