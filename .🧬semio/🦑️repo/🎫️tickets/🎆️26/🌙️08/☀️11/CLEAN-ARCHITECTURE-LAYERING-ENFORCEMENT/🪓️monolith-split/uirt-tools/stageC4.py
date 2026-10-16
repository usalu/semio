"""🔓️ Stage C4: widens the internal (non-exported) reconcile definitions that the laws and the facade tests reference."""
import sys,re
sys.path.insert(0,".")
from widen import *
from exported_private import EXPORTED
R=f"{REPO}/{RT}"
core=[f"{R}/♻️reconcile/🦀️.rs",f"{R}/📤️output/🦀️.rs"]
laws=[f"{R}/🧪️tests/🔬️reconcile-unit/🦀️.rs",f"{R}/📏️ownership/🧪️tests/📏️ownership/🦀️.rs",f"{R}/📃️document/🧪️tests/📃️document/🦀️.rs",f"{R}/📤️output/🧪️tests/📤️output/🦀️.rs",f"{R}/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs",f"{R}/🧪️tests/🔬️present-unit/🦀️.rs"]
fac=[f"{R}/🔄️transaction/🦀️.rs",f"{R}/🧪️tests/🔬️transaction-unit/🦀️.rs"]
oth=Others(laws+fac)
apply="--apply" in sys.argv
for p in core:
    ch=widen_file(p,oth,apply=apply,skip_types=set(EXPORTED))
    print("==",p.split("runtime/")[-1],len(ch))
    for c in ch: print("  ",c[0],c[1][:110])
