"""restore.py <file...>: copies the pre-edit snapshot back over the live file (own edits only)."""
import os, shutil, sys, hashlib
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "orig")
idx = {l.split("\t")[1]: l.split("\t")[0] for l in open(os.path.join(D, "index.tsv")).read().splitlines()}
for p in sys.argv[1:]:
    p = os.path.abspath(p)
    shutil.copy2(os.path.join(D, idx[p] + ".orig"), p); print("restored", p[-50:])
