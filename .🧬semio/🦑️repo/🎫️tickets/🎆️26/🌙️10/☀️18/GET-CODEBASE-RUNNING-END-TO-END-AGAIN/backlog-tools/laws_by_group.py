"""⚖️ [backlog] assigns every law file of the cumulative run to the landing group(s) whose files sit in or beside its module, writes backlog-tools/laws/<group>.txt (+ all.txt). A group's laws are what the gate runs after landing it. usage: laws_by_group.py <tests-list>"""
import json, os, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
groups = {l.split()[0]: l.split()[1:] for l in open(C + "/backlog-tools/groups.txt") if l.strip()}
def files_of(bid):
    fs = set()
    p = f"{C}/backlog-tools/merge/{bid}.json"
    if os.path.exists(p): fs |= {r["rel"] for r in json.load(open(p, encoding="utf8"))["files"]}
    p = f"{C}/backlog-tools/files/{bid}.txt"
    if os.path.exists(p): fs |= {l.strip() for l in open(p, encoding="utf8") if l.strip()}
    return fs
tests = [l.strip() for l in open(sys.argv[1]) if l.strip()]
os.makedirs(C + "/backlog-tools/laws", exist_ok=True); out = {g: [] for g in groups}
for t in tests:
    mod = t.split("/🧪️tests")[0] if "/🧪️tests" in t else os.path.dirname(t)
    for g, ids in groups.items():
        fs = set().union(*[files_of(b) for b in ids])
        if t in fs or any(f.startswith(mod + "/") or os.path.dirname(f) == mod for f in fs): out[g].append(t)
for g, v in out.items(): open(f"{C}/backlog-tools/laws/{g}.txt", "w", encoding="utf8").write("\n".join(v) + "\n"); print(g, len(v))
open(C + "/backlog-tools/laws/all.txt", "w", encoding="utf8").write("\n".join(tests) + "\n")
