"""✅ Final static checks over every touched file: path length, rustc parse of each .rs, emoji docstring of files new to HEAD, no leftover [DEBUG] logs."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftclib import ROOT

T = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "ftc-touched.txt")
files = []
for l in open(T, encoding="utf-8"):
    l = l.strip()
    if l and l not in files and os.path.exists(os.path.join(ROOT, l)):
        files.append(l)
gone = sum(1 for l in open(T, encoding="utf-8") if l.strip() and not os.path.exists(os.path.join(ROOT, l.strip())))
head = set(subprocess.run(["git", "-C", ROOT, "ls-tree", "-r", "--name-only", "-z", "HEAD", "--", "🧰️framework"], capture_output=True).stdout.decode("utf-8").split("\0"))
bad = collections = 0
long_ = parse = emoji = dbg = 0
new = 0
for rel in files:
    if len(rel.encode("utf-8")) > 240:
        long_ += 1
        print("LONG", len(rel.encode("utf-8")), rel)
    p = os.path.join(ROOT, rel)
    data = open(p, "rb").read().decode("utf-8", "replace")
    if "[DEBUG]" in data:
        dbg += 1
        print("DEBUG", rel)
    if rel.endswith(".rs"):
        r = subprocess.run(["rustc", "+nightly-2026-07-20", "-Zunpretty=normal", "--edition", "2021", "--crate-type", "lib", p], capture_output=True, cwd=ROOT)
        err = r.stderr.decode("utf-8", "replace")
        if r.returncode != 0 and "error" in err:
            first = [x for x in err.splitlines() if x.startswith("error")][:2]
            parse += 1
            print("PARSE", rel, first)
        if rel not in head:
            new += 1
            first = data.split("\n", 1)[0]
            if not (first.startswith("//! ") and ord(first[4]) > 0x2000):
                emoji += 1
                print("NO-EMOJI-DOC", rel, first[:60])
print(f"files {len(files)} (absent {gone}); new-to-HEAD rs {new}; long {long_}; parse {parse}; emoji {emoji}; debug {dbg}")
