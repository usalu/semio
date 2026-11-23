import subprocess, os, collections
G = os.path.dirname(os.path.abspath(__file__)) + r"\🗑️generated"
def git(*a): return subprocess.run(["git","-c","core.quotepath=false","-C",r"C:\git\semio",*a],capture_output=True).stdout.decode("utf-8","replace")
res = collections.Counter(); nodebug = []
for line in open(G + r"\rescue-commit752.tsv", encoding="utf-8").read().split("\n"):
    if not line.startswith("b-"): continue
    k, st, p = line.split("\t")
    d = git("diff", "-U0", "b4927d2c311c", "01324745f808", "--", p)
    added = [l for l in d.split("\n") if l.startswith("+") and not l.startswith("+++")]
    dbg = [l for l in added if "[DEBUG]" in l or "DEBUG" in l]
    if dbg: res["has-debug"] += 1
    else:
        res["no-debug"] += 1; nodebug.append((p, len(added)))
print(res)
open(G + r"\rescue-752-b-nodebug.txt", "w", encoding="utf-8").write("\n".join(f"{n}\t{p}" for p, n in nodebug))
