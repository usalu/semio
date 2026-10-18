import re, subprocess, sys, collections, os
# 📊️ jobuse.py : per symbol, how many tracked .rs files use it via semio_framework_job paths (use-blocks, inline paths, aliases, globs).
R = "/Users/ueli/Documents/semio"
os.chdir(R)
out = subprocess.run(["git","-c","core.quotepath=false","grep","-l","semio_framework_job","--","*.rs"],capture_output=True,text=True).stdout.split("\n")
files = [f for f in out if f]
sym = collections.defaultdict(set)
globs=set(); aliases=set()
def flat(block):
    names=[]; depth=0; cur=""; 
    for ch in block:
        if ch=="{": depth+=1; cur+=ch
        elif ch=="}": depth-=1; cur+=ch
        elif ch=="," and depth==0: names.append(cur.strip()); cur=""
        else: cur+=ch
    if cur.strip(): names.append(cur.strip())
    return names
def head(n):
    n=re.sub(r"\s+"," ",n)
    m=re.match(r"([A-Za-z_][A-Za-z0-9_]*)(::(.*))?",n)
    return m
for f in files:
    try: t=open(f,encoding="utf8").read()
    except Exception: continue
    for m in re.finditer(r"use\s+semio_framework_job(?:\s+as\s+(\w+))?\s*;",t): aliases.add(f)
    for m in re.finditer(r"use\s+semio_framework_job::\{",t):
        i=m.end(); depth=1; j=i
        while depth and j<len(t):
            depth+= (t[j]=="{") - (t[j]=="}"); j+=1
        for n in flat(t[i:j-1]):
            n=re.sub(r"\s+"," ",n.strip())
            if not n: continue
            if n=="*": globs.add(f); continue
            mm=re.match(r"(?:self|[A-Za-z_]\w*)",n)
            if mm: sym[mm.group(0)].add(f)
    for m in re.finditer(r"use\s+semio_framework_job::(\*|\w+)",t):
        if m.group(1)=="*": globs.add(f)
        else: sym[m.group(1)].add(f)
    for m in re.finditer(r"semio_framework_job::(\w+)",t):
        sym[m.group(1)].add(f)
res=sorted(sym.items(),key=lambda kv:-len(kv[1]))
for k,v in res: print(len(v),k)
print("GLOBS",len(globs),"ALIASES",len(aliases),file=sys.stderr)
import json
json.dump({k:sorted(v) for k,v in sym.items()},open(sys.argv[1],"w"),ensure_ascii=False)
json.dump({"globs":sorted(globs),"aliases":sorted(aliases)},open(sys.argv[1]+".misc","w"),ensure_ascii=False)
