#!/usr/bin/env python3
"""📊 [audit12] per batch and file: added/removed line counts of base→new (git diff -U0) and the removed lines whose text is not re-added anywhere in the same file diff (candidate reverts of someone else's change).
usage: SHOW=n python3 -I stats.py [batch ...]   (default: every batch of compose.ORDER)"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ns = {}; exec([l for l in open(os.path.join(HERE, "compose.py"), encoding="utf8") if l.startswith("ORDER")][0], ns)
words = lambda s: set(re.findall(r"\w+", s))
for b in sys.argv[1:] or ns["ORDER"]:
    rows = json.load(open(os.path.join(C, b, "manifest.json"), encoding="utf8"))
    for rel, row in rows.items():
        base = os.path.join(C, b, "base", row["file"]) if row["base"] else "/dev/null"
        out = subprocess.run(["git", "diff", "--no-index", "--no-color", "-U0", base, os.path.join(C, b, "files", row["file"])], capture_output=True).stdout.decode("utf8", "replace").splitlines()
        add = [l[1:] for l in out if l.startswith("+") and not l.startswith("+++")]
        rem = [l[1:] for l in out if l.startswith("-") and not l.startswith("---")]
        aw = [words(a) for a in add]
        orphan = [l for l in rem if len(words(l)) > 2 and not any(len(words(l) & w) >= 0.6 * len(words(l)) for w in aw)]
        print(f"{b.split('/')[0][:-6]}/{b.split('/')[-1]} +{len(add)} -{len(rem)} orphan {len(orphan)} {rel[-80:]}")
        for l in orphan[:int(os.environ.get("SHOW", "0"))]: print("      -", l.strip()[:220])
