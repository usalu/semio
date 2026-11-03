import re, sys, pathlib, json
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
from rewrite_lib import *
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")
CAT = ROOT / "💡️inferences/⚖️checks/📇️catalogue/🦀️.rs"
s = CAT.read_text()
blk = s[s.index("pub const UNVERIFIED"):]
blk = blk[:blk.index("];")]
rows = re.findall(r'^    \("([^"]+)", "((?:[^"\\]|\\.)*)", "((?:[^"\\]|\\.)*)"\),$', blk, re.M)
assert len(rows) >= 25, len(rows)
d = load()
ids = {}
for i, en, de in rows:
    ids[i] = f"ndp.en1991.{i}"
    put(d, "1991", i, i.split(".")[0].upper().replace("E", "E1-").replace("E1-1-", "E1-1") if False else i.split(".")[0], i, None, None, "", f"{en} | {de}")
put(d, "1991", "density.steel", "Annex A Table A.4", "γ steel", 77.0, 78.5, "kN/m3", "steel bulk density EN 77.0 / DE 78.5", part="1-1")
for z, v in enumerate((22.5, 25.0, 27.5, 30.0), 1):
    put(d, "1991", f"vb0.zone{z}", "NA.A.1 Table NA.A.1", f"v_b,0 zone {z}", None, v, "m/s", f"basic wind velocity wind zone {z}: DE {v}", part="1-4")
for k, cl, sym, r, n, u in [("alpha-a.de-c0", "6.3.1.2 NA", "α_A c0", None, 0.5, ""), ("alpha-a.de-c1", "6.3.1.2 NA", "α_A c1", None, 10.0, "m"), ("alpha-a.de-min", "6.3.1.2 NA", "α_A min", None, 0.7, ""), ("alpha-a.de-max", "6.3.1.2 NA", "α_A max", None, 1.0, ""),
  ("alpha-a.en-a0", "6.3.1.2(10)", "A0", 10.0, None, "m2"), ("alpha-a.en-min-cd", "6.3.1.2(10)", "α_A min C D", 0.6, None, ""),
  ("partition.band1.limit", "6.3.1.2(8)", "g1", 1.0, 3.0, "kN/m"), ("partition.band1.q", "6.3.1.2(8)", "q1", 0.5, 0.8, "kN/m2"), ("partition.band2.limit", "6.3.1.2(8)", "g2", 2.0, 5.0, "kN/m"), ("partition.band2.q", "6.3.1.2(8)", "q2", 0.8, 1.2, "kN/m2"), ("partition.band3.limit", "6.3.1.2(8)", "g3", 3.0, 6.0, "kN/m"), ("partition.band3.q", "6.3.1.2(8)", "q3", 1.2, 1.6, "kN/m2"),
  ("snow.drift-mu2-cap", "Annex B", "μ2 max", 4.0, 2.0, "")]:
    put(d, "1991", k, cl, sym, r, n, u, f"{sym}: EN {r} / DE {n}", part="1-1")
save(d)
json.dump(ids, open(pathlib.Path(__file__).parent / "en1991-ids.json", "w"), indent=1)
print(len(rows))
