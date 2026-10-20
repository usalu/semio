# 🔬 Lists the scopes of a list file that have no inventory output yet. usage: python3 missing.py <listfile> <outdir>
import os,sys
for s in open(sys.argv[1],encoding="utf8").read().split("\n"):
    if not s: continue
    n=s.replace("✏️s/","").replace("/","__")
    if not os.path.isfile(f"{sys.argv[2]}/inv-{n}.json"): print(s)
