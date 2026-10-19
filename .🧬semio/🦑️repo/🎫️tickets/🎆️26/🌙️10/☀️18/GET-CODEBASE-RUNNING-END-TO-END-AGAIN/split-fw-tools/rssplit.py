#!/usr/bin/env python3
"""Rust include!-fragment splitter. usage: rssplit.py spec.json [--apply]
spec: {source, root, fragments:[{dir, doc, ranges:[[a,b]..]}], rootExtra?: [lines appended after leftover]}
 - every original line not in a fragment range stays in the root (original order).
 - fragments go to <rootdir>/<dir>/🦀️.rs with a first-line `/// doc`; include!("<dir>/🦀️.rs"); lines are appended to the root.
 - include_str!("../x") / include!("../x") inside fragments get one extra `../`.
"""
import json, os, re, sys
from collections import Counter
spec = json.load(open(sys.argv[1]))
apply = "--apply" in sys.argv
src = open(spec["source"], encoding="utf-8").read().split("\n")
if src and src[-1] == "":
    src.pop()
n = len(src)
taken = {}
for f in spec["fragments"]:
    for a, b in f["ranges"]:
        for k in range(a - 1, b):
            if k in taken:
                raise SystemExit("line %d assigned twice" % (k + 1))
            taken[k] = f["dir"]
rootdir = os.path.dirname(spec["root"])
REL = re.compile(r'(include_str!|include!|include_bytes!)\("(\.\./[^"]*)"\)')
def rebase(l):
    return REL.sub(lambda m: '%s("../%s")' % (m.group(1), m.group(2)), l)
root_lines = [src[k] for k in range(n) if k not in taken]
while root_lines and not root_lines[-1].strip():
    root_lines.pop()
outs = {}
for f in spec["fragments"]:
    lines = []
    prev = None
    for a, b in f["ranges"]:
        if lines and lines[-1].strip():
            lines.append("")
        lines.extend(rebase(l) for l in src[a - 1:b])
    while lines and not lines[0].strip():
        lines.pop(0)
    while lines and not lines[-1].strip():
        lines.pop()
    for old, new in f.get("replace", []):
        c = sum(l.count(old) for l in lines)
        if c == 0: print("REPLACE MISS", old[:60])
        lines = [l.replace(old, new) for l in lines]
    out = [f["doc"], ""] + lines
    outs[os.path.join(rootdir, f["dir"], "🦀️.rs")] = out
tail = []
for f in spec["fragments"]:
    tail += ["", 'include!("%s/🦀️.rs");' % f["dir"]]
outs[spec["root"]] = root_lines + tail
# oracle
orig = Counter(l for l in src if l.strip())
new = Counter()
for p, out in outs.items():
    new.update(l for l in out if l.strip())
inc = Counter('include!("%s/🦀️.rs");' % f["dir"] for f in spec["fragments"])
docs = Counter(f["doc"] for f in spec["fragments"])
rebased = Counter()
for l in src:
    if REL.search(l) and any(src.index(l) is not None for _ in [0]):
        pass
new2 = new - inc - docs
missing = orig - new2
extra = new2 - orig
miss = {l: c for l, c in missing.items() if not REL.search(l)}
ext = {l: c for l, c in extra.items() if not REL.search(l)}
print("MISSING", sum(miss.values()), "EXTRA", sum(ext.values()), "(rebased include lines excluded:", sum(c for l, c in missing.items() if REL.search(l)), ")")
for l, c in list(miss.items())[:10]: print("  -", c, l[:120])
for l, c in list(ext.items())[:10]: print("  +", c, l[:120])
tests_o = sum(1 for l in src if l.strip() == "#[test]")
tests_n = sum(1 for out in outs.values() for l in out if l.strip() == "#[test]")
print("tests:", tests_o, tests_n)
for p, out in outs.items():
    print("%5d %s" % (len(out), p[-90:]))
if apply:
    for p, out in outs.items():
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write("\n".join(out) + "\n")
    print("written")
