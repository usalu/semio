"""Poisson's ratio of concrete (EN 1992-1-1 3.1.3(4)) and masonry (EN 1996-1-1 3.7.3) as unverified registry rows; open request line 1852.
Usage: /home/deck/semio/.venv/bin/python -I en1998-poisson-ndp.py"""
import pathlib, subprocess, sys

NDP_TOOLS = pathlib.Path(__file__).resolve().parent.parent / "🧰️ndp"
sys.path.insert(0, str(NDP_TOOLS))
from ndplib import load, put, save, REG

TAIL = "; UNVERIFIED: recollection, not read from the printed standard, check it against the clause before release"
ROWS = [
    ("1992", "3.1.poisson-uncracked", "EN 1992-1-1 3.1.3(4)", "Poisson's ratio of uncracked concrete", 0.2, "1", "EN 1992-1-1 3.1.3(4): the Poisson ratio may be taken as 0.2 for uncracked concrete" + TAIL),
    ("1992", "3.1.poisson-cracked", "EN 1992-1-1 3.1.3(4)", "Poisson's ratio of cracked concrete", 0.0, "1", "EN 1992-1-1 3.1.3(4): the Poisson ratio may be taken as 0 for cracked concrete" + TAIL),
    ("1996", "3.7.poisson-implied", "EN 1996-1-1 3.7.3", "Poisson's ratio of masonry implied by G = 0.4 E", 0.25, "1", "EN 1996-1-1 3.7.3(1): shear modulus G = 0.4 E, hence nu = E/(2 G) - 1 = 0.25; a derived value, not printed" + TAIL),
]
d = load()
for fam, key, clause, symbol, value, part, note in ROWS:
    i = put(d, fam, key, clause, symbol, value, value, "", note, part=part, keep=True)
    d[i]["evidence"] = note
    d[i]["specRef"] = "open-9899: value written with its evidence note, not checked against the licensed edition"
save(d)
subprocess.run([sys.executable, "-I", str(NDP_TOOLS / "codegen.py"), str(REG), str(REG.parent / "🗃️table.rs")], check=True)
print(len(ROWS), "rows,", len(d), "entries")
