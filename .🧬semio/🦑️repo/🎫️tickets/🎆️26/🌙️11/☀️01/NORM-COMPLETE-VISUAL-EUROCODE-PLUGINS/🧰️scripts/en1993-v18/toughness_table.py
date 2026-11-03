"""EN 1993-1-10 Table 2.1 as registry rows (en1993 v18): every grade x sub-grade x stress level x reference temperature cell. The repo spec text (en1993-target.md V-900) prints no cell value, so only the 49 cells of the sigma_Ed = 0.75 f_y(t) column that the artifact already carried (written from memory, unverified) keep their numbers; every other cell is registered Unset and unverified with a note naming the source needed. Idempotent."""
import pathlib, subprocess, sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp")
from ndplib import *
GRADES = {"S235": ["JR", "J0", "J2"], "S275": ["JR", "J0", "J2", "N", "NL", "M", "ML"], "S355": ["JR", "J0", "J2", "K2", "N", "NL", "M", "ML"], "S420": ["N", "NL", "M", "ML"], "S460": ["N", "NL", "M", "ML", "Q", "QL", "QL1"]}
LEVELS = [("75", "0.75"), ("50", "0.50"), ("25", "0.25")]
TEMPS = [10, 0, -10, -20, -30, -40, -50]
d = load()
old = {i: e for i, e in d.items() if i.startswith("ndp.en1993.1-10.t21.") and ".r" not in i}
for i in old: del d[i]
SRC = "Printed EN 1993-1-10:2005 Table 2.1 (t_max by steel grade, sub-grade, reference temperature T_Ed and sigma_Ed/f_y(t)); the repo spec text lists no cell value and the pairing of grade and sub-grade is likewise unverified"
n_set = n_unset = 0
for g, subs in GRADES.items():
    for s in subs:
        for lk, lt in LEVELS:
            for t in TEMPS:
                key = f"1-10.t21.{g.lower()}-{s.lower()}.r{lk}.t{t}"
                prev = old.get(f"ndp.en1993.1-10.t21.{g.lower()}-{s.lower()}.t{t}") if lk == "75" else None
                v = prev["recommended"] if prev else None
                sym = f"t_max {g}-{s} sigma_Ed {lt} f_y(t) {t} C"
                note = ("Maximum thickness for brittle fracture avoidance; value written from memory by the artifact, not double-keyed. Source needed: " if v is not None else "Cell not transcribed (no value in the repo). Source needed: ") + SRC
                put(d, "1993", key, "Table 2.1", sym, v, v, "mm", note, part="1-10", locator="Table 2.1")
                if v is None: n_unset += 1
                else: n_set += 1
save(d)
print("cells with value", n_set, "unset", n_unset, "kept old", len(old))
assert n_set == len(old), (n_set, len(old))
REG = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp"
subprocess.run([sys.executable, "-I", str(pathlib.Path(__file__).resolve().parent.parent.parent / "🧰️ndp/codegen.py"), REG + "/🔣️.json", REG + "/🗃️table.rs"], check=True)
