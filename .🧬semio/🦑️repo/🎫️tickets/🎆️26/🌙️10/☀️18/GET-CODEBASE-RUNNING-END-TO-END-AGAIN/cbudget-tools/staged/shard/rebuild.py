#!/usr/bin/env python3
"""⏲️ Rebuilds batch `shard` (lane cbudget) from the CURRENT live files: new files come from src/, the modified table of Bazel test classes is re-edited at its anchor and re-hashed, base/ gets the live bytes, manifest.json the hashes.
usage: python3 -I rebuild.py [--root DIR]   (default root: walk up to the directory holding 🧰️framework)
Refuses (exit 2) when an anchor no longer matches exactly once, so a peer's edit of the same lines is never overwritten silently."""
import hashlib, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
PATHS = json.load(open(os.path.join(HERE, "paths.json"), encoding="utf8"))
def sha(data): return hashlib.sha256(data).hexdigest()
def once(text, old, new):
    if text.count(old) != 1: print("ANCHOR", repr(old[:70]), text.count(old)); sys.exit(2)
    return text.replace(old, new)
RULE = """  "rules": [
    {
      "id": "course-test-shards",
      "evidence": "declared",
      "reason": "The course suite (17 files, 716 tests) needs 50 to 260 s of wall time on a quiet host and five times that at load 40 to 70; three shards keep each far inside its budget (Vitest assigns 6, 6 and 5 files).",
      "targets": ["test"],
      "projects": ["@teaching/architecture-course"],
      "shards": 3
    }
  ],
"""
def table(text):
    return once(text, '  "rules": [],\n', RULE)
EDITS = {"03.json": table}
rows = {}
os.makedirs(os.path.join(HERE, "files"), exist_ok=True); os.makedirs(os.path.join(HERE, "base"), exist_ok=True)
for name, (rel, src) in PATHS["new"].items():
    data = open(os.path.join(HERE, src), "rb").read()
    live = os.path.join(ROOT, rel)
    if os.path.exists(live) and open(live, "rb").read() != data: print("NEW FILE EXISTS WITH OTHER BYTES", rel[-60:]); sys.exit(2)
    open(os.path.join(HERE, "files", name), "wb").write(data)
    rows[rel] = {"file": name, "base": None, "new": sha(data)}
for name, rel in PATHS["modified"].items():
    base = open(os.path.join(ROOT, rel), "rb").read()
    text = base.decode("utf8")
    if "course-test-shards" in text: print("already edited", rel[-50:]); sys.exit(2)
    new = EDITS[name](text).encode("utf8")
    open(os.path.join(HERE, "base", name), "wb").write(base); open(os.path.join(HERE, "files", name), "wb").write(new)
    rows[rel] = {"file": name, "base": sha(base), "new": sha(new)}
json.dump(rows, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("rebuilt", len(rows), "files against", ROOT)
