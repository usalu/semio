"""python3 compare-outputs.py <execlogA> <execlogB> : for every label in both logs, are the output digests (path -> sha256) identical?"""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)).replace("cinp-d-tools", "cwarm-d-tools"))
from execlog import spawns
def outs(path):
    return {r["targetLabel"]: {o["path"]: o["digest"]["hash"] for o in r.get("actualOutputs", []) if "digest" in o} for r in spawns(path)}
a, b = outs(sys.argv[1]), outs(sys.argv[2])
for label in sorted(set(a) & set(b)):
    same = a[label] == b[label]
    diff = [p for p in set(a[label]) | set(b[label]) if a[label].get(p) != b[label].get(p)]
    print(label, "IDENTICAL" if same else f"DIFFERENT {len(diff)} of {len(a[label])}", *(diff[:3] if not same else []))
print("only A:", sorted(set(a) - set(b)), "only B:", sorted(set(b) - set(a)))
