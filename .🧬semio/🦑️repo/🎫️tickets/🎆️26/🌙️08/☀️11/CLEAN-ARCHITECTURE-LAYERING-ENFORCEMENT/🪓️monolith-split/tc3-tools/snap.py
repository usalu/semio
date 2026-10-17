"""Pre-edit snapshot: snap.py <file...>; copies to orig/ once, records index.tsv with sha256."""
import os, shutil, sys, hashlib
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "orig")
os.makedirs(D, exist_ok=True)
idx = os.path.join(D, "index.tsv")
seen = {l.split("\t")[1].rstrip("\n") for l in open(idx)} if os.path.exists(idx) else set()
for p in sys.argv[1:]:
    p = os.path.abspath(p)
    if p in seen or not os.path.exists(p): continue
    key = hashlib.sha256(p.encode()).hexdigest()[:16]
    shutil.copy2(p, os.path.join(D, key + ".orig"))
    sha = hashlib.sha256(open(p, "rb").read()).hexdigest()
    open(idx, "a").write(f"{key}\t{p}\t{sha}\n")
    print("snap", p[-60:])
