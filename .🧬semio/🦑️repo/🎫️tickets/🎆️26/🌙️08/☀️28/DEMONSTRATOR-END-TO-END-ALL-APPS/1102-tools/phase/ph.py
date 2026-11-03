import sys,os,difflib,subprocess,collections
sys.path.insert(0,"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1102-tools/policy")
from mk import crate_of,R
import subprocess as sp
OUT=os.path.dirname(os.path.abspath(__file__))+"/patches"
FILES=sp.run(["git","-c","core.quotepath=false","ls-files","--","*.rs","*.toml"],cwd=R,capture_output=True,text=True).stdout.split("\n")
def find(crate,suffix):
    c=[f for f in FILES if f.endswith(suffix) and crate_of(f)==crate]
    c=c if len(c)<2 else [f for f in c if '🧪️tests' not in f] or c
    assert len(c)==1,(crate,suffix,c)
    return c[0]
SP=collections.OrderedDict()
def E(crate,suffix,old,new,k=1):
    SP.setdefault(crate,[]).append((find(crate,suffix),old,new,k))
def KL(x):
    p=x.split('\n');return [q+'\n' for q in p[:-1]]+([p[-1]] if p[-1] else [])
POL=os.path.dirname(os.path.abspath(__file__))+"/../policy/patches"
SUPERSEDES={}
def policy_files(p):
    return [l[6:].strip() for l in open(p,encoding="utf8").read().split("\n") if l.startswith("+++ b/")]
def build(only=None):
    import tempfile,shutil
    for crate,edits in SP.items():
        if only and crate not in only:continue
        byf=collections.OrderedDict()
        for rel,o,n,k in edits:byf.setdefault(rel,[]).append((o,n,k))
        pp=f"{POL}/{crate}.patch"
        pf=policy_files(pp) if os.path.exists(pp) else []
        sup=bool(set(pf)&set(byf))
        T=tempfile.mkdtemp()
        touched=list(dict.fromkeys(list(byf)+(pf if sup else [])))
        for rel in touched:
            os.makedirs(os.path.dirname(f"{T}/{rel}"),exist_ok=True);shutil.copy(f"{R}/{rel}",f"{T}/{rel}")
        if sup:
            r=sp.run(["git","apply","--unsafe-paths","-p1",pp],cwd=T,capture_output=True,text=True)
            assert r.returncode==0,(crate,"policy apply",r.stderr[:300])
            SUPERSEDES[crate]=pp
        parts=[]
        for rel in touched:
            s=open(f"{R}/{rel}",encoding="utf8").read();t=open(f"{T}/{rel}",encoding="utf8").read()
            for o,n,k in byf.get(rel,[]):
                assert t.count(o)==k,(rel[-40:],t.count(o),o[:90])
                t=t.replace(o,n)
            parts.append("".join(difflib.unified_diff(KL(s),KL(t),f"a/{rel}",f"b/{rel}",n=3)))
        p=f"{OUT}/{crate}.patch";open(p,"w",encoding="utf8").write("".join(parts))
        shutil.rmtree(T)
        r=sp.run(["git","-c","core.quotepath=false","apply","--check",p],cwd=R,capture_output=True,text=True)
        print(crate,"OK" if r.returncode==0 else "FAIL "+r.stderr[:300],"SUPERSEDES-POLICY" if sup else "")
def lines(crate,suffix,a,b):
    rel=find(crate,suffix);L=open(f"{R}/{rel}",encoding="utf8").read().split("\n")
    return "\n".join(L[a-1:b])
