"""usage: nest.py <scanRoot> <oldDir>=<newDir>...  moves directories (by os.rename) and re-points every relative specifier (from "x", import("x"), {@link x |) of the files below scanRoot"""
import os,re,sys
root=os.path.abspath(sys.argv[1])
moves=[]
for a in sys.argv[2:]:
    o,n=a.split("=",1); moves.append((os.path.abspath(o),os.path.abspath(n)))
def newpath(p):
    for o,n in moves:
        if p==o or p.startswith(o+os.sep): return n+p[len(o):]
    return p
files=[]
for dp,dn,fn in os.walk(root):
    dn[:]=[d for d in dn if d!="node_modules"]
    for f in fn:
        if f.endswith((".ts",".tsx",".rs")): files.append(os.path.join(dp,f))
pat=re.compile(r'(from\s+"|import\(\s*"|\{@link\s+)(\.{1,2}/[^"\s|}]+)')
out={}
for f in files:
    s=open(f,encoding="utf8").read()
    nf=newpath(f)
    def fix(m):
        spec=m.group(2)
        tgt=os.path.normpath(os.path.join(os.path.dirname(f),spec))
        ntgt=newpath(tgt)
        r=os.path.relpath(ntgt,os.path.dirname(nf))
        if not r.startswith("."): r="./"+r
        return m.group(1)+r
    t=pat.sub(fix,s)
    out[f]=(nf,t,t!=s)
for o,n in moves:
    os.makedirs(os.path.dirname(n),exist_ok=True)
    os.rename(o,n)
for f,(nf,t,ch) in out.items():
    if ch or nf!=f:
        open(nf,"w",encoding="utf8").write(t)
print("moved",len(moves),"dirs; rewrote",sum(1 for v in out.values() if v[2]),"files")
