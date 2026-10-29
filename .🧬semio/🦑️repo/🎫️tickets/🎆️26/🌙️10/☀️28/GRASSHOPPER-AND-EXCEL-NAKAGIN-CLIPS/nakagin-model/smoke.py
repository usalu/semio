import importlib.util
import sys
from pathlib import Path
import glob
p = Path(glob.glob("C:/git/semio/*/*/*nakagin/*.py")[0])
s = importlib.util.spec_from_file_location("nk", p)
m = importlib.util.module_from_spec(s)
sys.modules["nk"] = m
s.loader.exec_module(m)
for v in (None, "window_upgrade", "back_wall_exposed", "shared_roof_floor", ("window_upgrade", "shared_roof_floor")):
    h = m.heating("A", v)
    print(v, repr(h["h_t"]), repr(h["h"]), repr(h["annual"]), repr(h["annual_per_area"]))
print(m.climate()["period"], m.climate()["hdd_value"])
