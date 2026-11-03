import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
Q = {"rc.frame": (5.85, 3.9), "rc.wall": (4.4, 3.0), "rc.dual": (5.2, 3.6), "steel.frame": (6.5, 4.0), "steel.wall": (5.0, 4.0), "composite": (6.0, 4.0), "timber": (5.0, 2.5), "masonry": (2.5, 2.5)}
for k, (h, m) in Q.items():
    for cls, v in (("dch", h), ("dcm", m), ("low", 1.5)):
        put(d, "1998", f"en1.q-max.{k}.{cls}", "Tables 5.1/6.2/7.2/8.1/9.1", f"q_max {k} {cls.upper()}", v, v, "", f"Upper bound of the behaviour factor q for {k}, ductility class {cls.upper() if cls != 'low' else 'DCL or unspecified'}; simplified from the system tables as used by the artifact", part="1")
put(d, "1998", "en1.q-max.default", "Tables 5.1/6.2/7.2/8.1/9.1", "q_max default", 1.5, 1.5, "", "Upper bound of q for an unknown material or system", part="1")
for k, v in (("brittle", 0.005), ("isolated", 0.010), ("ductile", 0.0075)):
    put(d, "1998", f"en1.drift-theta.{k}", "4.4.3.2", f"drift limit ratio {k}", v, v, "", f"Interstorey drift limit d_r nu / h for the limit class {k}; the code maps every other class to ductile", part="1")
save(d); print(len(d))
