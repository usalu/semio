import sys, difflib
from jio import *
doc, raw = read(sys.argv[1], sys.argv[2])
out = dump(doc, raw)
n = 0
for l in difflib.unified_diff(raw.splitlines(), out.splitlines(), lineterm="", n=0):
    print(l[:160]); n += 1
    if n > 30: break
