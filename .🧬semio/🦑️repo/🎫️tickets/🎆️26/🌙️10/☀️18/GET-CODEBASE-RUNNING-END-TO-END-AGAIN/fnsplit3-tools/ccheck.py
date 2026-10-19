"""usage: ccheck.py <orig> <root> <dir>...  every comment-looking line (//, /*, *, */) of the orig must occur (modulo indentation and whitespace) in the root or in any 🟦️.* file below the dirs"""
import sys,os,re
def norm(l): return re.sub(r"\s+"," ",l.strip())
def commentish(l):
    t=l.strip()
    return t.startswith("//") or t.startswith("/*") or t.startswith("*")
def load(p): return open(p,encoding="utf8").read().split("\n")
orig=load(sys.argv[1]); root=sys.argv[2]; dirs=sys.argv[3:]
have={}
files=[root]
for d in dirs:
    for dp,dn,fn in os.walk(d):
        for f in fn:
            if f.startswith("🟦") or f.startswith("🦀"): files.append(os.path.join(dp,f))
for f in files:
    for l in load(f):
        n=norm(l); have[n]=have.get(n,0)+1
miss=[]
for l in orig:
    if commentish(l):
        n=norm(l)
        if have.get(n,0)>0: have[n]-=1
        else: miss.append(n)
print("comment lines missing:",len(miss))
for m in miss[:15]: print("  -",m[:140])
