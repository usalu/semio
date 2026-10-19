import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
bad = []
n = 0
for top in sys.argv[1:]:
    for dp, dn, fn in os.walk(os.path.join(ROOT, top)):
        dn[:] = [d for d in dn if d not in ("node_modules", "target", ".git", "dist")]
        for f in fn:
            if not f.endswith(".rs"): continue
            p = os.path.join(dp, f)
            try: t = open(p, encoding="utf-8").read()
            except Exception: continue
            for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;', t):
                n += 1
                q = os.path.normpath(os.path.join(dp, m.group(1)))
                if not os.path.exists(q): bad.append((p[len(ROOT)+1:], m.group(1)))
print(n, "mounts", len(bad), "missing")
for b in bad: print(b[0][-150:], "->", b[1])
