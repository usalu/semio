#!/usr/bin/env python3
# [DEBUG] finds every exhaustive RetirementDemand literal or pattern (no `..` inside) so `items` can be added in one pass
import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
SKIP = {".git", "target", "node_modules", ".cache", ".🧬semio", "dist", "build"}
pat = re.compile(r'\bRetirementDemand\s*\{')
hits = []
for base, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if d not in SKIP and not d.startswith("claude-")]
    for f in files:
        if not f.endswith(".rs"): continue
        p = os.path.join(base, f)
        try: s = open(p, encoding="utf-8").read()
        except Exception: continue
        if "RetirementDemand" not in s: continue
        for m in pat.finditer(s):
            before = s[max(0, m.start()-40):m.start()]
            if re.search(r'(struct|->|impl|for|enum|trait|use)\s*$', before): continue
            i = m.end(); depth = 1
            while i < len(s) and depth:
                c = s[i]
                if c == '{': depth += 1
                elif c == '}': depth -= 1
                i += 1
            body = s[m.end():i-1]
            if '..' in body: continue
            if re.fullmatch(r'\s*', body): continue
            line = s.count("\n", 0, m.start()) + 1
            hits.append((p, line, body.strip()[:120]))
for h in hits: print(f"{h[0]}:{h[1]}: {h[2]}")
print(len(hits), "exhaustive", file=sys.stderr)
