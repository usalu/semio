import sys, re
sys.path.insert(0, ".")
from rcrlib import *
name = sys.argv[1]
t = TARGETS[name]
src = read(f"{ROOT}/{t['lib']}")
ls = src.split("\n")
its = top_items(src)
for it in its:
    h = f"{it['first_line']+1:5}-{it['end']+1:5} {it['end']-it['first_line']+1:4} {it['kind']:6} {it['vis']:9} {it['name'][:60]}"
    print(h)
