import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")
TABLE = ROOT / "💡️inferences/⚖️checks/📜️ndp-tables/🦀️.rs"
SPECIAL = {"alpha_cc": "m-01.alpha-cc-persistent", "alpha_cc_acc": "m-01.alpha-cc-accidental", "alpha_ct": "m-01.alpha-ct", "gamma_c": "m-01.gamma-c-persistent", "gamma_c_acc": "m-01.gamma-c-accidental"}
def ident(sym): return "ndp.en1992." + SPECIAL.get(sym, sym.lower().replace("_", "-"))
src = TABLE.read_text()
rows = re.findall(r'row\("([^"]+)", "([^"]+)", ([-\d.]+), ([-\d.]+), (true|false), (true|false), "([^"]*)", "([^"]*)"\)', src)
assert len(rows) >= 100, len(rows)
d = load()
ids = {}
for sym, cl, ec, de, ecv, dev, ne, nd in rows:
    i = ident(sym); assert i not in ids.values(), i
    ids[sym] = i
    key = i[len("ndp.en1992."):]
    put(d, "1992", key, cl, sym, float(ec), float(de), "", ne, part="1-1")
    d[i]["evidence"] = (d[i]["evidence"] + " " if d[i]["evidence"] else "") + f"Private table before registry adoption flagged EC verified={ecv}, DE verified={dev}. DE note: {nd}"
d["ndp.en1992.m-01.alpha-cc-accidental"]["naDe"] = 0.85
d["ndp.en1992.m-01.gamma-c-accidental"].update(recommended=1.2, naDe=1.3)
d["ndp.en1992.m-01.alpha-cc-accidental"]["recommended"] = 1.0
save(d)
import json
json.dump(ids, open(pathlib.Path(__file__).parent / "en1992-ids.json", "w"), ensure_ascii=False, indent=1)
print(len(ids))
