# usage: outline.py <relative file under PLUG> -> items with spans
import sys
from common import *
f = sys.argv[1]
items, L, R = PT.outline(PLUG + f)
for it in items:
    print(f"{it['s']:5d}-{it['e']:5d} {it['e']-it['s']+1:5d} {it['kind']:10s} {it['head'][:110]}")
