import re, sys, pathlib, json
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
from rewrite_lib import *
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")
T = ROOT / "⚖️masonry/📚️normative-tables/🦀️.rs"
s = T.read_text()
rows = re.findall(r'^    \("([^"]+)", "((?:[^"\\]|\\.)*)", "((?:[^"\\]|\\.)*)"\),$', s.split("pub fn is_unverified")[0], re.M)
assert len(rows) >= 45, len(rows)
d = load()
ids = {}
for i, en, de in rows:
    ids[i] = f"ndp.en1996.{i}"
    cl = {"psi": "EN 1990 Table A1.1", "durability.mortar": "Table 2.1"}.get(i, i.split(".")[0].upper().replace("M", "M-"))
    put(d, "1996", i, cl, i, None, None, "", f"{en} | {de}", part="1-1")
for k, (en, de) in {"c1": (1.5, 1.5), "c2": (1.7, 1.7), "c3": (2.0, 1.7), "c4": (2.2, 2.0), "c5": (2.5, 2.0)}.items():
    put(d, "1996", f"gamma-m.{k}", "Table 2.3 / NA.2.4.3", f"γ_M class {k[1]}", en, de, "", f"γ_M masonry class {k[1]}: EN {en} / DE {de}", part="1-1")
put(d, "1996", "gamma-m.accidental", "NA.2.4.3", "γ_M accidental", None, 1.3, "", "γ_M DE accidental 1.3; EN code uses the class value", part="1-1")
save(d)
json.dump(ids, open(pathlib.Path(__file__).parent / "en1996-ids.json", "w"), indent=1)
print(len(rows))
