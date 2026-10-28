#!/usr/bin/env python3
"""usage: doc-emoji-check.py <dir>... : lists every TypeScript/Python/JSON-free file of the directories whose docstrings (/** E ...) start with a repeated or missing emoji (unique per file rule)."""
import os, re, sys, collections
bad = 0
for base in sys.argv[1:]:
    for d, _, fs in os.walk(base):
        for f in fs:
            if not f.endswith(".ts"): continue
            p = os.path.join(d, f)
            seen = collections.defaultdict(list)
            for n, line in enumerate(open(p, encoding="utf-8"), 1):
                m = re.match(r"\s*/\*\* (\S+) ", line)
                if m: seen[m.group(1)].append(n)
                elif re.match(r"\s*/\*\* ", line): print("no emoji", p, n); bad += 1
            for e, lines in seen.items():
                if len(lines) > 1: print("dup", e, p, lines); bad += 1
print("problems:", bad)
