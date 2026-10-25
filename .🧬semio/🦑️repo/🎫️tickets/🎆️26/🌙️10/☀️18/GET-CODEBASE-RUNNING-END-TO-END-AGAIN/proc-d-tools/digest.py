"""digest.py <census-dir> [from] — one block per failing file: counts, failing test names, first error messages."""
import sys,re,os
d=sys.argv[1];start=int(sys.argv[2]) if len(sys.argv)>2 else 0
for i,l in enumerate(open(d+"/summary.txt",encoding="utf8")):
    c=l.rstrip("\n").split("\t")
    if len(c)<5 or c[1]=="rc=0" or i<start: continue
    log=open(d+"/"+c[4].replace("log=","")+("" if c[4].endswith(".log") else ".log"),encoding="utf8",errors="replace").read()
    fails=re.findall(r"^\(fail\) (.*?)(?: \[[\d.]+ms\])?$",log,re.M)
    errs=[]
    for m in re.finditer(r"^error: (.*)$",log,re.M):
        t=m.group(1).strip()
        if t and not re.match(r"^\d+ \|",t) and t not in errs: errs.append(t[:200])
    print(f"## {i} {c[0]}  {c[1]} {c[2]} {c[3]}")
    for f in fails[:6]: print("   FAIL",f[:140])
    for e in errs[:4]: print("   ERR ",e)
