#!/usr/bin/env python3
"""R34 bundle: unified diffs (live now -> tested overlay) of the files the package changes, with the baseline and result sha256 of each. usage: build-bundle.py <overlay root>"""
import os, sys, json, hashlib, difflib
HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = "/Users/Shared/semio"
MODULES = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
SW = MODULES + "/🤝️shared-worktree"
FILES = [SW + "/" + f for f in [
    "🌿️git/🟦️.ts", "🌿️git/🧪️tests/🟦️.ts",
    "🔁️replica/🟦️.ts", "🔁️replica/⌨️command/🟦️.ts", "🔁️replica/🧬️schema/🔣️.json", "🔁️replica/📖️texts/🔣️.json",
    "🔁️replica/🧪️tests/🟦️.ts", "🔁️replica/🧪️tests/🌿️git.ts",
    "🛡️resilience/🟦️.ts", "🛡️resilience/🧬️schema/🔣️.json", "🛡️resilience/🧪️tests/🟦️.ts"]]
def sha(b): return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p, "rb") as h: return h.read()
overlay = os.path.abspath(sys.argv[1])
bundle = {"patches": {}, "baseline": {}, "result": {}, "full": {}, "new": {}}
for rel in FILES:
    live, new = rd(LIVE + "/" + rel), rd(overlay + "/" + rel)
    bundle["baseline"][rel], bundle["result"][rel] = sha(live), sha(new)
    if live == new: continue
    diff = "".join(difflib.unified_diff(live.decode("utf8").splitlines(True), new.decode("utf8").splitlines(True), "a/f", "b/f", n=3))
    bundle["patches"][rel] = diff
json.dump(bundle, open(HERE + "/R34-bundle.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(bundle["patches"]), "patches;", sum(d.count("\n") for d in bundle["patches"].values()), "diff lines")
for rel, d in bundle["patches"].items(): print("  ", rel[len(MODULES) + 1:], "+%d -%d" % (sum(1 for l in d.splitlines() if l.startswith("+") and not l.startswith("+++")), sum(1 for l in d.splitlines() if l.startswith("-") and not l.startswith("---"))))
