"""Find uses of removed os-pack names through a module alias (`use ...os_pack as X;`, bare `use ...::os_pack;`, `self` in a brace import) and through glob imports of os_pack."""
import re, sys
from orclib import *
files = []
for n in ("tracked", "untracked"):
    files += [x for x in open(G + "/orc-rs-%s.z" % n, "rb").read().decode().split("\0") if x]
names = set(sys.argv[1].split(","))
alias_pat = re.compile(r"use\s+(?:[A-Za-z_][A-Za-z0-9_]*::)*(?:semio_framework_os_pack|os_pack)(?:\s+as\s+([A-Za-z_]\w*))?\s*;|use\s+(?:[A-Za-z_][A-Za-z0-9_]*::)*(?:semio_framework_os_pack|os_pack)::\{[^}]*\bself(?:\s+as\s+(\w+))?[^}]*\}\s*;")
glob_pat = re.compile(r"use\s+(?:[A-Za-z_][A-Za-z0-9_]*::)*(?:semio_framework_os_pack|os_pack)::\*\s*;")
for f in files:
    if f.startswith(".🧬semio/") or "/🗑️generated/" in f: continue
    try: t = open(ROOT + "/" + f, errors="replace").read()
    except Exception: continue
    if "os_pack" not in t: continue
    aliases = set()
    for m in alias_pat.finditer(t):
        a = m.group(1) or m.group(2) or "os_pack"
        aliases.add(a)
    globs = bool(glob_pat.search(t))
    for a in aliases:
        if a == "os_pack" and False: pass
        for m in re.finditer(r"(?<![\w:])" + re.escape(a) + r"::([A-Za-z_]\w*)(?:::([A-Za-z_]\w*))?", t):
            if m.group(1) in names:
                print("%s:%d\talias=%s\t%s" % (f, t.count("\n", 0, m.start()) + 1, a, m.group(0)))
    if globs:
        for m in re.finditer(r"(?<![\w:.])(" + "|".join(sorted(names)) + r")\b", t):
            print("%s:%d\tGLOB\t%s" % (f, t.count("\n", 0, m.start()) + 1, m.group(0)))
