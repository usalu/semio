"""EN 1998 v5: registers the unverified constants of the bridge, tank and tower checks in the shared NDP registry through `🧰️ndp/ndplib.py`, then regenerates the row crates with `🧰️ndp/codegen.py`. Idempotent.
Usage: /home/deck/semio/.venv/bin/python -I en1998-v5-ndp.py"""
import pathlib, subprocess, sys

NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

d = load()
before = len(d)
rows = [
    ("en1.modal.min-mass-participation", "4.3.3.1(3)", "Σ M_eff / M of the modes taken into account", 0.90, "", "EN 1998-1 4.3.3.1(3)a: the sum of the effective modal masses is at least 90 % of the total mass; value from the author's knowledge, not read from the printed standard"),
    ("en2.seat.ground-displacement-coefficient", "3.2.2.4 Eq. 3.12", "c in d_g = c a_g S T_C T_D", 0.025, "", "EN 1998-1 3.2.2.4: design ground displacement d_g = 0.025 a_g S T_C T_D; value from the author's knowledge, not read from the printed standard"),
    ("en2.seat.ground-displacement-factor", "6.6.2", "factor of d_eg = f d_g", 2.0, "", "EN 1998-2 6.6.2: effective ground displacement d_eg for the seat length l_ov = l_m + d_eg + d_Es, taken as twice d_g for uncorrelated supports; not read from the printed standard"),
    ("en4.tank.slosh-height-coefficient", "A.3.3.2", "c in d_max = c R S_e(T_c) / g", 0.84, "", "EN 1998-4 Annex A.3.3.2: sloshing wave height of the first convective mode of a circular tank; value from the author's knowledge, not read from the printed standard"),
    ("en6.interaction.max-period-lengthening", "4.2", "T_1(k_theta) / T_1(fixed) accepted for a fixed base", 1.20, "", "design policy placeholder: EN 1998-6 gives no limit for the foundation flexibility of a tower; the ratio at which a fixed-base analysis is no longer accepted is a project decision until the printed standard is read"),
]
for key, clause, symbol, value, unit, note in rows:
    part = key.split(".")[0][2:]
    d[put(d, "1998", key, clause, symbol, value, value, unit, symbol + " = " + str(value) + " as used by the artifact", part=part, keep=True)]["evidence"] = note
save(d)
codegen = NDP_TOOLS / "codegen.py"
subprocess.run([sys.executable, "-I", str(codegen), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print("rows", before, "->", len(d))
