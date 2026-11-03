import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
P = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📏️parts/🦀️.rs")
s = P.read_text()
rows = re.findall(r'Ndp \{ id: "([^"]+)", clause: "([^"]+)", recommended: ([-\d.]+), national: ([-\d.]+), confirmed: (true|false) \}', s)
assert len(rows) == 30, len(rows)
d = load()
for i, cl, rec, nat, conf in rows:
    k = i.replace("_", "-")
    put(d, "1999", k, cl, i, float(rec), float(nat), "", f"{i} EN {rec} / DE {nat}", part="1-1")
    d[f"ndp.en1999.{k}"]["evidence"] = f"Private list before registry adoption flagged confirmed={conf}; the shared registry is the only source and verification is a human double-key."
save(d)
print([r[0] for r in rows])
