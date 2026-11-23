import os, shutil, hashlib
T = os.path.dirname(os.path.abspath(__file__))
G = T + r"\🗑️generated"
B = T + r"\git-semio-rescue-backup"
os.makedirs(B, exist_ok=True)
idx = open(B + r"\index.tsv", "a", encoding="utf-8")
n = 0
for line in open(G + r"\rescue-status-classes.tsv", encoding="utf-8").read().split("\n"):
    if not line.startswith("c-") or "\t??\t" not in line:
        continue
    k, st, p = line.split("\t")
    o = os.path.join(r"C:\git\semio", p)
    d = os.path.join(r"C:\semio", p)
    b = open(d, "rb").read()
    name = f"untracked-{n:02d}{os.path.splitext(p)[1]}"
    open(os.path.join(B, name), "wb").write(b)
    idx.write(f"{name}\t{hashlib.sha1(b).hexdigest()}\tC:\semio\{p}\n")
    shutil.copyfile(o, d)
    n += 1
print(n)
