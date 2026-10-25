"""Compares two Bazel execution logs spawn by spawn: python3 -I diff-execlogs.py <logA> <logB>
Per target label: action digest, cacheHit, changed input paths (digest differs / only on one side), env, args and output digests."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from execlog import spawns

def index(path):
    return {row["targetLabel"]: row for row in spawns(path)}

def digest(entry):
    return entry.get("digest", {}).get("hash", "")

def main(left_path, right_path):
    left, right = index(left_path), index(right_path)
    for label in sorted(set(left) | set(right)):
        a, b = left.get(label), right.get(label)
        if a is None or b is None:
            print(label, "only in", "A" if b is None else "B")
            continue
        print("==", label, a["mnemonic"], "A spawn", digest(a)[:12], "B spawn", digest(b)[:12], "B cacheHit", b.get("cacheHit"), "B runner", b.get("runner"))
        ia = {i["path"]: digest(i) for i in a["inputs"]}
        ib = {i["path"]: digest(i) for i in b["inputs"]}
        changed = sorted(p for p in ia.keys() & ib.keys() if ia[p] != ib[p])
        only_a = sorted(ia.keys() - ib.keys())
        only_b = sorted(ib.keys() - ia.keys())
        print("   inputs", len(ia), len(ib), "changed", len(changed), "only A", len(only_a), "only B", len(only_b))
        for p in changed[:15]: print("   changed", p[:140])
        for p in only_a[:8]: print("   onlyA", p[:140])
        for p in only_b[:8]: print("   onlyB", p[:140])
        if a["commandArgs"] != b["commandArgs"]: print("   args differ", a["commandArgs"][-3:], b["commandArgs"][-3:])
        if a["environmentVariables"] != b["environmentVariables"]: print("   env differ")
        oa = {o["path"]: digest(o) for o in a.get("actualOutputs", [])}
        ob = {o["path"]: digest(o) for o in b.get("actualOutputs", [])}
        print("   outputs", len(oa), len(ob), "differing", sum(1 for p in oa.keys() & ob.keys() if oa[p] != ob[p]), "onlyA", len(oa.keys() - ob.keys()), "onlyB", len(ob.keys() - oa.keys()))

main(sys.argv[1], sys.argv[2])
