#!/usr/bin/env python3
import os,re,json,subprocess,sys,collections
R="/Users/ueli/Documents/semio"
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
roots=[("1102",C+"/1102-tools"),("1101",C+"/1101-tools")]
rows=[]
for gen,root in roots:
  for lane in sorted(os.listdir(root)):
    pd=f"{root}/{lane}/patches"
    if not os.path.isdir(pd):continue
    for dp,dn,fn in os.walk(pd):
      for f in sorted(fn):
        if f.endswith(".check") or f=="index.json":continue
        p=os.path.join(dp,f)
        rel=os.path.relpath(p,C)
        sup="_superseded" in rel
        kind="diff" if f.endswith((".patch",".diff")) else "script" if f.endswith((".py",".sh")) else "plan" if f.endswith(".md") else "other"
        targets=[];hunks={}
        txt=open(p,errors="replace").read()
        if kind=="diff":
          cur=None
          for l in txt.split("\n"):
            m=re.match(r"\+\+\+ (?:b/)?(.*)",l)
            if m:cur=m.group(1).split("\t")[0];targets.append(cur);hunks[cur]=[];continue
            m=re.match(r"@@ -(\d+)(?:,(\d+))? ",l)
            if m and cur:
              a=int(m.group(1));n=int(m.group(2) or 1);hunks[cur].append((a,a+n))
        elif kind=="script":
          targets=sorted(set(re.findall(r'"([^"\n]*?(?:\.rs|\.ts|\.toml|\.json|\.py))"',txt)))
          targets=[t for t in targets if "/" in t or "🦀" in t]
        res=""
        if kind=="diff" and not sup:
          pn="-p1" if "\n--- a/" in "\n"+txt else "-p0"
          r=subprocess.run(["git","apply","--check",pn,p],cwd=R,capture_output=True,text=True)
          if r.returncode==0:res="CLEAN"
          else:
            r2=subprocess.run(["git","apply","--check","-R",pn,p],cwd=R,capture_output=True,text=True)
            res="ALREADY-APPLIED" if r2.returncode==0 else "FAIL: "+r.stderr.strip().split("\n")[0][:160]
        rows.append(dict(pn=("-p1" if "\n--- a/" in "\n"+txt else "-p0") if kind=="diff" else "",gen=gen,lane=lane,rel=rel,kind=kind,sup=sup,targets=targets,hunks=hunks,check=res,size=os.path.getsize(p),mtime=os.path.getmtime(p)))
json.dump(rows,open(C+"/🗑️generated/1102/winplan-scan.json","w"),ensure_ascii=False,indent=1)
print(len(rows))
