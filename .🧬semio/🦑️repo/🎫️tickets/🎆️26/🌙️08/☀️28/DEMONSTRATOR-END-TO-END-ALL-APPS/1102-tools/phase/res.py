import subprocess,re,sys,os
sys.path.insert(0,"../policy")
from mk import crate_of,R
B=open("../policy/blockers.md",encoding="utf8").read().split("\n")[4:]
files=subprocess.run(["git","-c","core.quotepath=false","ls-files","--","*.rs"],cwd=R,capture_output=True,text=True).stdout.split("\n")
rows=[]
for l in B:
    p=[x.strip() for x in l.split("|")]
    if len(p)<5 or "test-support" in p[3]:continue
    cr,fl,cls=p[1],p[2],p[3]
    path,ln=fl.rsplit(":",1)
    c=[f for f in files if f.endswith("/"+path) and crate_of(f)==cr]
    rows.append((cr,c,int(ln),cls))
for r in rows:print(r[0],r[2],r[3][:12],r[1])
