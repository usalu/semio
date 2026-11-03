"""🔎 `#[path = ".."] mod x;` targets that do not exist, plus `mod x;` without #[path] whose x.rs / x/mod.rs is missing."""
import os, re, sys
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SKIP = {"target", "node_modules", ".git", "dist"}
RX = re.compile(r'(?m)^\s*(?:#\[[^\]]*\]\s*)*#\[path\s*=\s*"([^"]+)"\][^\n]*?\bmod\s+(\w+)\s*;|^\s*#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')
bad = 0
for dp, dn, fn in os.walk(NORM):
    dn[:] = [d for d in dn if d not in SKIP]
    for f in fn:
        if not f.endswith(".rs"): continue
        p = os.path.join(dp, f)
        try: s = open(p, encoding="utf-8").read()
        except Exception: continue
        s = re.sub(r'(?m)^\s*//.*$', '', s)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', s):
            t = os.path.normpath(os.path.join(dp, m.group(1)))
            if not os.path.exists(t):
                print(f"{p.replace(NORM+'/', '')}: mod {m.group(3)} -> missing {m.group(1)}"); bad += 1
print("missing", bad)
