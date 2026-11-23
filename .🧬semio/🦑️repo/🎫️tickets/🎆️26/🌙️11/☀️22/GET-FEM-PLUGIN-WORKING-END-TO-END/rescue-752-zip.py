import subprocess, os, zipfile
T = os.path.dirname(os.path.abspath(__file__))
G = T + r"\🗑️generated"
def blob(p):
    return subprocess.run(["git","-c","core.quotepath=false","-C",r"C:\git\semio","show","01324745f808:"+p],capture_output=True).stdout
zf = zipfile.ZipFile(T + r"\git-semio-rescue-commit-752-only-files.zip", "w", zipfile.ZIP_DEFLATED)
idx = []
tot = 0
for line in open(G + r"\rescue-commit752.tsv", encoding="utf-8").read().split("\n"):
    if not line.startswith(("b-", "d-")): continue
    k, st, p = line.split("\t")
    data = blob(p)
    tot += len(data)
    zf.writestr(("changed-vs-751/" if k.startswith("b-") else "absent-in-C-semio/") + p, data)
    idx.append(f"{k}\t{len(data)}\t{p}")
zf.close()
open(G + r"\rescue-752-zip-index.tsv", "w", encoding="utf-8").write("\n".join(idx))
print(len(idx), tot, os.path.getsize(T + r"\git-semio-rescue-commit-752-only-files.zip"))
