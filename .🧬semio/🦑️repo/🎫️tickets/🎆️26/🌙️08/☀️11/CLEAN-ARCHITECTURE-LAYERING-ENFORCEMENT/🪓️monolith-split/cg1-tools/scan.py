import re, sys, os
# 🔎️ scan.py <filelist> <regex> [<regex>...] : prints "path:line:snippet" for every file in the list whose text matches any regex (ASCII-safe replacement for ugrep on emoji paths).
os.chdir("/Users/ueli/Documents/semio")
files=[l.rstrip("\n") for l in open(sys.argv[1]) if l.strip()]
pats=[re.compile(p) for p in sys.argv[2:]]
for f in files:
    try: t=open(f,encoding="utf8",errors="replace").read()
    except Exception: continue
    for i,line in enumerate(t.split("\n"),1):
        for p in pats:
            if p.search(line):
                print(f"{f}:{i}:{line[:200]}"); break
