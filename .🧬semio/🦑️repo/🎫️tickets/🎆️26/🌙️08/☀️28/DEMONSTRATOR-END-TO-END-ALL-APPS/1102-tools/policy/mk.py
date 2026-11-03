import os,sys,subprocess,difflib,json
R="/Users/ueli/Documents/semio"
OUT=os.path.dirname(os.path.abspath(__file__))+"/patches"
def ctx(rel,needle,w=260):
    s=open(f"{R}/{rel}",encoding="utf8").read()
    i=0
    while True:
        i=s.find(needle,i)
        if i<0:break
        print("...",s[max(0,i-w):i+len(needle)+w].replace("\n","\\n"),"...\n");i+=1
def KL(x):
    p=x.split('\n');return [q+'\n' for q in p[:-1]]+([p[-1]] if p[-1] else [])
def build(name,spec):
    parts=[]
    for rel,edits in spec.items():
        s=open(f"{R}/{rel}",encoding="utf8").read();t=s
        for e in edits:
            old,new=e[0],e[1];k=e[2] if len(e)>2 else 1
            assert t.count(old)==k,(rel[-50:],t.count(old),old[:80])
            t=t.replace(old,new)
        d=list(difflib.unified_diff(KL(s),KL(t),f"a/{rel}",f"b/{rel}",n=3))
        parts.append("".join(d))
    p=f"{OUT}/{name}.patch";open(p,"w",encoding="utf8").write("".join(parts))
    r=subprocess.run(["git","-c","core.quotepath=false","apply","--check",p],cwd=R,capture_output=True,text=True)
    print(name,"APPLY-CHECK",("OK" if r.returncode==0 else "FAIL "+r.stderr[:300]))
    open(f"{OUT}/{name}.check","w").write("OK" if r.returncode==0 else "FAIL "+r.stderr)

import re as _re
NAMES={}
def crate_of(rel):
    if rel in NAMES:return NAMES[rel]
    d=os.path.dirname(f"{R}/{rel}")
    while len(d)>len(R):
        for c in (d+"/Cargo.toml",d+"/📦️packages/🦀️rust/Cargo.toml"):
          if os.path.exists(c):
            m=_re.search(r'^\s*name\s*=\s*"([^"]+)"',open(c,encoding="utf8").read(),_re.M)
            if m:return m.group(1)
        d=os.path.dirname(d)
    return "unknown"
def plan(specs):
    rows=[]
    for name,(spec,fixed) in specs.items():
        cr={crate_of(r) for r in spec}
        build(name,spec)
        rows.append((name,sorted(cr),sorted(spec.keys()),fixed,open(f"{OUT}/{name}.check").read()[:2]))
    json.dump(rows,open(f"{OUT}/index.json","w"),ensure_ascii=False,indent=1)
