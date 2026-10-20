"""📊 Markdown table of the value family: production lines of the module tree, direct and transitive dependents, commits and longest chain above. Run with `cwd` = the generated folder."""
from graph import *
from tree import *
import json, sys
sys.setrecursionlimit(100000)
churn = json.load(open("churn.json")) if os.path.exists("churn.json") else {}
memo = {}
def height(x):
    if x in memo: return memo[x]
    memo[x] = 0
    memo[x] = 1 + max((height(y) for y in rev.get(x, ())), default=0)
    return memo[x]
rows = []
for d in fam:
    c = C[d]
    files = [f for f in module_tree(libfile(d, c)) if "🧪️" not in f and f.endswith(".rs")]
    lines = sum(len(open(f, encoding="utf-8", errors="ignore").read().splitlines()) for f in files)
    rows.append((len(trans(d)), len(rev.get(d, ())), height(d), lines, churn.get(c["name"], 0), c["name"].replace("semio-framework-", ""), "proc-macro" if c["proc_macro"] else ""))
rows.sort(reverse=True)
print("| crate | prod lines | direct | transitive | chain above | commits |")
print("| --- | ---: | ---: | ---: | ---: | ---: |")
for t, dr, h, ln, cm, nm, pm in rows:
    print(f"| `{nm}`{' (proc-macro)' if pm else ''} | {ln:,} | {dr:,} | {t:,} | {h} | {cm} |")
