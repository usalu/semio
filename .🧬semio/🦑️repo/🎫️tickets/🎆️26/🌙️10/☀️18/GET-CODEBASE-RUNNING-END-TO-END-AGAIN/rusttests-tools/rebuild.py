"""🔁 [rusttests] builds S0 (base) and S1 (base + edits). usage: rebuild.py <S0> <S1>"""
import os, shutil, subprocess, sys
import os as _o, sys as _s; _s.path.insert(0, _o.path.dirname(_o.path.abspath(__file__)))
from lib import *
S0, S1 = sys.argv[1:3]
for d in (S0, S1): shutil.rmtree(d, ignore_errors=True)
files = {r[0] for r in ROWS} | {HELPER, HELPER_TESTS, HELPER_FIXTURE, ROUTES}
used = set()
for rel in sorted(files):
    data, via = stacked(rel)
    if data is None: continue
    if via: used.add(via)
    for d in (S0, S1):
        p = os.path.join(d, rel); os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "wb").write(data)
open(os.path.join(S0, ".stack"), "w").write("\n".join(sorted(used)))
subprocess.run([sys.executable, "-I", os.path.join(HERE, "edit.py"), S1], check=True)
print("S0/S1 built; stack:", sorted(used))
