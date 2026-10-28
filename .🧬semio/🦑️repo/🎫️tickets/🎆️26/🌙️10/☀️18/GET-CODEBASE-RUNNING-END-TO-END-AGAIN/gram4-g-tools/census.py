"""[gram4-g] census of the observation rows of ONE owner dump (ROWS_DUMP jsonl, lines [kind, path, fact]) and of the support rows of its derived facts (DERIVED_DUMP .support.jsonl, lines [fact, kind, path]): how many rows lie in the subtree of which OTHER project (longest project root that is a path prefix), per kind. Streams the files, keeps only counters (RSS check < 500 MB). usage: census.py <rows.jsonl> [support.jsonl] [--top N] [--depth D]"""
import json, os, re, resource, subprocess, sys
from collections import Counter, defaultdict
ROOT = "/Users/ueli/Documents/semio/"
args = [a for a in sys.argv[1:] if not a.startswith("--")]
top = int(sys.argv[sys.argv.index("--top") + 1]) if "--top" in sys.argv else 25
depth = int(sys.argv[sys.argv.index("--depth") + 1]) if "--depth" in sys.argv else 4
files = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "-z", "--", "*📋️project.json", "*Cargo.toml"], cwd=ROOT, capture_output=True, text=True).stdout.split("\0")
roots = {"."} | {os.path.dirname(f) or "." for f in files if f and not f.startswith(".🧬semio")}
trie = {}
for r in roots:
    if r == ".":
        continue
    node = trie
    for part in r.split("/"):
        node = node.setdefault(part, {})
    node["\0"] = r
def owner(path):
    if not path.startswith(ROOT):
        return "(outside)"
    parts = path[len(ROOT):].split("/")
    node, best = trie, "."
    for part in parts:
        node = node.get(part)
        if node is None:
            break
        best = node.get("\0", best)
    return best
def bucket(path):
    if not path.startswith(ROOT):
        return "(outside)"
    return "/".join(path[len(ROOT):].split("/")[:depth])
def census(name, lines, pathAt):
    kinds, byOwner, byBucket, ownKinds, total = Counter(), Counter(), Counter(), Counter(), 0
    for line in lines:
        if not line.strip():
            continue
        r = json.loads(line)
        kind, path = r[pathAt - 1], r[pathAt]
        total += 1
        kinds[kind] += 1
        if not isinstance(path, str):
            byOwner["(non-path)"] += 1
            continue
        o = owner(path)
        byOwner["root itself" if o == "." else "other project"] += 1
        byBucket[(kind, bucket(path))] += 1
        if o != ".":
            ownKinds[kind] += 1
    print(f"== {name}: {total} rows; kinds {dict(kinds.most_common())}")
    print(f"   by project: {dict(byOwner)}; in other projects by kind {dict(ownKinds.most_common())}")
    agg = Counter()
    for (kind, b), n in byBucket.items():
        agg[b] += n
    for b, n in agg.most_common(top):
        print(f"   {n:8d}  {b}  " + " ".join(f"{k}={byBucket[(k, b)]}" for k in kinds if byBucket[(k, b)]))
with open(args[0], encoding="utf8") as f:
    census("own rows", f, 1)
if len(args) > 1:
    with open(args[1], encoding="utf8") as f:
        census("support rows (all derived facts)", f, 2)
print("peak rss MB", resource.getrusage(resource.RUSAGE_SELF).ru_maxrss // 1048576)
