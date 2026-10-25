import difflib, os, sys
R = "/Users/ueli/Documents/semio"
C = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/dbg-d"
O = C + "/🗑️generated/dbgv-d"
os.makedirs(O + "/diffs", exist_ok=True)
summary = []
alld = []
for line in open(G + "/orig/map.txt", encoding="utf-8"):
    line = line.rstrip("\n")
    if not line:
        continue
    n, f = line.split(" ", 1)
    p = R + "/" + f
    if not os.path.isfile(p):
        summary.append(f"{n}\tMISSING\t{f}")
        continue
    a = open(G + f"/orig/{n}.bak", encoding="utf-8", errors="replace").read().splitlines()
    b = open(p, encoding="utf-8", errors="replace").read().splitlines()
    d = list(difflib.unified_diff(a, b, "orig", "now", lineterm="", n=1))
    if not d:
        summary.append(f"{n}\tSAME\t{f}")
        continue
    changed = sum(1 for x in d if (x.startswith("+") or x.startswith("-")) and not x.startswith("+++") and not x.startswith("---"))
    summary.append(f"{n}\tDIFF\t{changed}\t{f}")
    alld.append(f"#### {n} {f}")
    alld.extend(d)
open(O + "/summary.txt", "w", encoding="utf-8").write("\n".join(summary) + "\n")
open(O + "/all.diff", "w", encoding="utf-8").write("\n".join(alld) + "\n")
print(len(summary), len(alld))
