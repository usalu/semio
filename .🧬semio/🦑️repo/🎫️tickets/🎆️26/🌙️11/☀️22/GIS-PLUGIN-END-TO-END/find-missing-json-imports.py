"""🔍️ Lists static JSON imports in TS sources whose target file does not exist."""
import os, re, sys
root = sys.argv[1]
pat = re.compile(r'''from\s*["'](\.{1,2}/[^"']+\.json)["']''')
for d, _, fs in os.walk(root):
    if "node_modules" in d or "/dist" in d: continue
    for f in fs:
        if not f.endswith(".ts"): continue
        p = os.path.join(d, f)
        try: s = open(p, encoding="utf-8").read()
        except Exception: continue
        for m in pat.finditer(s):
            t = os.path.normpath(os.path.join(d, m.group(1)))
            if not os.path.exists(t): print(p, "->", m.group(1))
