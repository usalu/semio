"""🔎 `include_str!`/`include_bytes!`/`include!` literal paths (relative to the file) that do not exist, in the norm artifact trees."""
import os, re
N = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
SKIP = {"target", "node_modules", ".git", "dist"}
bad = 0
for dp, dn, fn in os.walk(N):
    dn[:] = [d for d in dn if d not in SKIP]
    for f in fn:
        if not f.endswith(".rs"): continue
        p = os.path.join(dp, f)
        try: s = open(p, encoding="utf-8").read()
        except Exception: continue
        if "include" not in s: continue
        for m in re.finditer(r'\binclude(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)', re.sub(r'(?m)^\s*//.*$', '', s)):
            t = os.path.normpath(os.path.join(dp, m.group(1)))
            if not os.path.exists(t):
                print(f"{p.replace(N + '/', '')}: missing {m.group(1)}"); bad += 1
print("missing", bad)
