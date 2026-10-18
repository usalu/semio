import re,sys,os
# usage: lits.py <test-file> <search-dir> [<search-dir>...]  -> for each includes/toContain literal, list files under the dirs holding it
t=sys.argv[1]; dirs=sys.argv[2:]
src=open(t,encoding="utf8").read()
files=[]
for d in dirs:
    for root,_,fs in os.walk(d):
        if "node_modules" in root or "🧪️tests" in root: continue
        for f in fs:
            if f.endswith(".rs"): files.append(os.path.join(root,f))
txt={f:open(f,encoding="utf8",errors="replace").read() for f in files}
seen=set()
for m in re.finditer(r'(?:includes|toContain|indexOf)\(\s*"((?:[^"\\\n]|\\.)*)"',src):
    lit=m.group(1)
    if lit in seen: continue
    seen.add(lit)
    hit=[os.path.relpath(f,dirs[0]) for f,x in txt.items() if lit in x]
    print(("OK   " if hit else "NONE ")+repr(lit[:100]),hit[:3])
