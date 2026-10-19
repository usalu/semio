import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
out = open(sys.argv[1], "w", encoding="utf-8")
n = 0
for top in ["🧰️framework", "✏️s"]:
    for dp, dn, fn in os.walk(os.path.join(ROOT, top)):
        dn[:] = [d for d in dn if d not in ("node_modules", "target", ".git", "dist")]
        for f in fn:
            if f.endswith(".rs") or f.endswith(".rs.in"):
                p = os.path.join(dp, f)
                try: t = open(p, encoding="utf-8").read()
                except Exception: continue
                out.write(p[len(ROOT) + 1:] + "\t" + re.sub(r"\s+", "", t) + "\n"); n += 1
print(n)
