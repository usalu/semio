import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SIT = {"p": "persistent", "t": "transient", "a": "accidental"}
def e(key, cl, sym, r, n, unit=""): put(d, "1997", key, cl, sym, r, n, unit, f"{sym}: EN {r} / DE {n}", part="1")
for k, g, q in (("a1", 1.35, 1.5), ("a2", 1.0, 1.3)):
    e(f"{k}.g", "Table A.3/A.4", f"γ_G {k.upper()}", g, g); e(f"{k}.q", "Table A.3/A.4", f"γ_Q {k.upper()}", q, q)
e("a-acc.g", "EN 1997-1 2.4.7", "γ_G accidental", 1.0, 1.0); e("a-acc.q", "EN 1997-1 2.4.7", "γ_Q accidental", 1.0, 1.0)
for k, v in (("m1", (1.0, 1.0, 1.0, 1.0)), ("m2", (1.25, 1.25, 1.4, 1.0))):
    for s, x in zip(("phi", "c", "cu", "gamma"), v): e(f"{k}.{s}", "Table A.4", f"γ_{s} {k.upper()}", x, x)
R = {"r1": (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.25, 1.1), "r2": (1.4, 1.1, 1.4, 1.1, 1.1, 1.1, 1.15, 1.1), "r3": (1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.1, 1.1), "r4": (1.0, 1.0, 1.0, 1.3, 1.3, 1.3, 1.6, 1.1)}
for k, v in R.items():
    for s, x in zip(("rv", "rh", "re", "b", "s", "t", "st", "a"), v): e(f"{k}.{s}", "Tables A.5-A.14", f"γ_{s} {k.upper()}", x, x)
for s, (g, q) in {"p": (1.35, 1.50), "t": (1.20, 1.30), "a": (1.10, 1.15)}.items():
    e(f"de.act.{s}.g", "DIN 1054 Table A 2.1", f"γ_G {SIT[s]}", None, g); e(f"de.act.{s}.q", "DIN 1054 Table A 2.1", f"γ_Q {SIT[s]}", None, q)
for s, (r, f, q) in {"p": (1.40, 1.25, 1.30), "t": (1.30, 1.15, 1.20), "a": (1.20, 1.10, 1.10)}.items():
    e(f"de.res.{s}.rv", "DIN 1054 Table A 2.3", f"γ_R,v {SIT[s]}", None, r); e(f"de.res.{s}.re", "DIN 1054 Table A 2.3", f"γ_R,e {SIT[s]}", None, r)
    e(f"de.soil.{s}.f", "DIN 1054 Table A 2.2", f"γ_φ=γ_c=γ_cu {SIT[s]}", None, f); e(f"de.geo3.{s}.q", "DIN 1054 GZ 1C", f"γ_Q GEO-3 {SIT[s]}", None, q)
for s, x in zip(("rh", "b", "s", "t", "st", "a"), (1.10, 1.10, 1.10, 1.10, 1.15, 1.10)): e(f"de.res.{s}", "DIN 1054 Table A 2.3", f"γ_{s} DE", None, x)
U = {"p": ((0.90, 1.00, 1.50, 1.35, 0.90, 1.10, 1.50, 0.90), (0.90, 1.00, 1.50, 1.80, 0.95, 1.05, 1.50, 0.90)),
     "t": ((0.90, 1.00, 1.50, 1.35, 0.90, 1.10, 1.50, 0.90), (0.90, 1.00, 1.30, 1.45, 0.95, 1.05, 1.30, 0.90)),
     "a": ((1.0,) * 8, (0.95, 1.00, 1.15, 1.30, 0.95, 1.00, 1.15, 0.95))}
for s, (en, de) in U.items():
    for sym, a, b in zip(("gstb", "gdst", "qdst", "hyd", "equstb", "equdst", "equq", "hydstb"), en, de):
        e(f"upl.{s}.{sym}", "Tables A.1, A.2, A.17 / A 2.1", f"{sym} {SIT[s]}", a, b)
save(d)
print(sum(k.startswith("ndp.en1997.") for k in d))
