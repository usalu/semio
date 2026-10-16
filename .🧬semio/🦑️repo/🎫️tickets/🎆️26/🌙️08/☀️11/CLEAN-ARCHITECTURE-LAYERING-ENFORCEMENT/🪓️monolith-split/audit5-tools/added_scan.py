"""🔎️ Read-only scan of added diff lines (last-day commits and uncommitted diff) for debug leftovers and compatibility shims."""
import re, subprocess, sys, os
ROOT = "/Users/ueli/Documents/semio"
GEN = sys.argv[1]
GIT = ["git", "--no-optional-locks", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false"]
DEBUG = re.compile(r"\[DEBUG\]")
DBG_MACRO = re.compile(r"\bdbg!\(|eprintln!\(\"DEBUG|console\.log\(\"DEBUG")
SHIM = [
    ("deprecated", re.compile(r"#\[\s*deprecated|#\[\s*allow\(deprecated\)")),
    ("legacy", re.compile(r"legacy", re.I)),
    ("compat", re.compile(r"\bcompat(ib\w*)?\b", re.I)),
    ("shim", re.compile(r"\bshim\b", re.I)),
    ("pub-use-as", re.compile(r"\bpub(\(\w+\))? use [^;]* as \w+")),
    ("use-as", re.compile(r"^\+\s*use [^;]* as \w+")),
]
def diff_added(args):
    out = subprocess.run(GIT + args, cwd=ROOT, capture_output=True, text=True, encoding="utf-8", errors="replace").stdout
    cur = None
    for raw in out.splitlines():
        if raw.startswith("+++ "):
            cur = raw[6:] if raw.startswith("+++ b/") else raw[4:]
        elif raw.startswith("+") and not raw.startswith("+++"):
            yield cur, raw[1:]
hits = {"debug": [], "dbg": [], "shim": []}
def consider(src, path, line):
    if path is None or not re.search(r"\.(rs|ts|tsx|toml)$", path):
        return
    if DEBUG.search(line):
        hits["debug"].append((src, path, line.strip()[:200]))
    if DBG_MACRO.search(line):
        hits["dbg"].append((src, path, line.strip()[:200]))
    for name, rx in SHIM:
        if rx.search(line):
            hits["shim"].append((src, path, name, line.strip()[:200]))
            break
for c in ["36fbe9da786", "1bae6115bc5", "f856dff0813"]:
    for path, line in diff_added(["show", "--format=", "-U0", c]):
        consider("commit:" + c, path, line)
for path, line in diff_added(["diff", "HEAD", "-U0"]):
    consider("worktree", path, line)
untracked = [l for l in open(os.path.join(GEN, "audit5r-untracked.txt"), encoding="utf-8").read().splitlines() if l]
for path in untracked:
    full = os.path.join(ROOT, path)
    if not os.path.isfile(full) or not re.search(r"\.(rs|ts|tsx|toml)$", path):
        continue
    for line in open(full, encoding="utf-8", errors="replace"):
        consider("untracked", path, line.rstrip("\n"))
with open(os.path.join(GEN, "audit5r-added-hits.tsv"), "w", encoding="utf-8") as f:
    for key, rows in hits.items():
        for row in rows:
            f.write(key + "\t" + "\t".join(str(x) for x in row) + "\n")
for key, rows in hits.items():
    print(key, len(rows))
    srcs = {}
    for row in rows:
        srcs[row[0]] = srcs.get(row[0], 0) + 1
    print("  by source", srcs)
