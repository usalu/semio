import subprocess, os, collections
G = os.path.dirname(os.path.abspath(__file__)) + r"\🗑️generated"
out = subprocess.run(["git","-c","core.quotepath=false","-C",r"C:\git\semio","ls-tree","-r","--name-only","-z","HEAD"],capture_output=True).stdout.split(b"\0")
miss = []; n = 0
for b in out:
    if not b: continue
    p = b.decode("utf-8"); n += 1
    if not os.path.lexists(os.path.join(r"C:\semio", p)): miss.append(p)
open(G + r"\rescue-tracked-missing-in-new.txt", "w", encoding="utf-8").write("\n".join(miss))
print(n, len(miss))
c = collections.Counter("/".join(p.split("/")[:2]) for p in miss)
for k, v in c.most_common(15): print(v, k.encode("ascii","replace").decode())
