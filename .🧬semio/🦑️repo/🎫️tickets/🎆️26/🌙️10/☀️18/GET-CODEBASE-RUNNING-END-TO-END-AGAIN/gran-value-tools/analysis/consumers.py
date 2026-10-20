"""🎯 Consumer crates (and their files) that reference the facade tail names ordered, retirement_queue, snapshot_projection, paged_text, OriginalAliasBatch, directly or through the alias crates. Writes consumers-g5.json. Run with `cwd` = the generated folder."""
import json, re, collections
from graph import *
from tree import *
NAMES = ["ordered", "retirement_queue", "snapshot_projection", "paged_text", "OriginalAliasBatch"]
R = json.load(open("census.json"))
alias = [d for d, x in R.items() if any(n.startswith("<alias") or n == "<crate>" for n in x["prod_names"] + x["test_names"])]
print("alias/crate-name crates", [C[d]["name"] for d in alias])
FD = V + "/📦️packages/🦀️rust"
scope = set(d for d, x in R.items() if set(NAMES) & (set(x["prod_names"]) | set(x["test_names"])))
for a in alias:
    scope |= set(trans(a)) | {a}
scope &= set(R) | set(alias) | scope
print("scope crates", len(scope))
out = []
rx = re.compile(r"(?:semio_framework_value|\bvalue|__value)\s*::\s*(?:" + "|".join(NAMES) + r")\b|semio_framework_value\s*::\s*\{[^;]*\b(?:" + "|".join(NAMES) + r")\b", re.S)
for d in sorted(scope):
    if d == FD:
        continue
    files = module_tree(libfile(d, C[d]))
    hit = [f for f in files if rx.search(open(f, encoding="utf-8", errors="ignore").read())]
    if hit:
        out.append({"dir": os.path.join(ROOT, d), "name": C[d]["name"], "files": hit})
json.dump(out, open("consumers-g5.json", "w"), ensure_ascii=False, indent=0)
print("consumer crates with a hit", len(out), "files", sum(len(o["files"]) for o in out))
print(sorted(o["name"].replace("semio-", "") for o in out))
