import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
rows = [
    ("settlement", "Table NA.A.1.1", "Settlements", (1.0, 1.0, 1.0)),
    ("variable_other", "Table NA.A.1.1", "Other variable actions", (0.8, 0.7, 0.5)),
    ("wind_rail", "Table A2.3", "Rail wind", (0.75, 0.5, 0.0)),
    ("rail_hslm", "Table A2.3", "Rail HSLM", (1.0, 1.0, 0.0)),
]
for key, clause, label, values in rows:
    for n, v in enumerate(values):
        put(d, "1990", f"psi.{key}.{n}", clause, f"ψ{n} {key}", v, v, "", f"ψ{n} of {label}: EN {v} / DE {v} (wave 2 v2, to be double-keyed)")
save(d)
d = load()
d.pop("ndp.en1990.gP", None)
for key, note in (("gP_sup", "unfavourable"), ("gP_inf", "favourable")):
    put(d, "1990", key, "A1.3.1", f"γP,{note}", 1.0, 1.0, "", f"γP {note}: 1.00 for global effects (EN 1992-1-1 2.4.2.2); local effects are owned by the material part (wave 2 v2)")
save(d)
