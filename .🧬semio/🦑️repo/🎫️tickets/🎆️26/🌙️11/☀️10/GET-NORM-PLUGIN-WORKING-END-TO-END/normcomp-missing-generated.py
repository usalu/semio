import os,re,sys
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
root=str(R)
pat=re.compile(r'(?:#\[path\s*=\s*|include!\(\s*|include_str!\(\s*|include_bytes!\(\s*)"([^"]*🤖️generated[^"]*)"')
missing=set()
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in("node_modules",".git",".cache","target",".nx")]
    for f in fs:
        if f.endswith(".rs"):
            p=os.path.join(d,f)
            try: s=open(p,encoding="utf8").read()
            except: continue
            for m in pat.finditer(s):
                t=os.path.normpath(os.path.join(d,m.group(1)))
                if not os.path.exists(t): missing.add((t,p))
for t,p in sorted(missing): print(t.replace(root+"/",""),"<-",p.replace(root+"/",""))
