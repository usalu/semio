import re,sys
t=open(sys.argv[1],encoding="utf8",errors="replace").read()
cur=None; rows=[]; counts={}
for line in t.splitlines():
    if line.startswith("=== "): cur=line[4:].split("/")[-2]; continue
    m=re.match(r"^\((pass|fail|skip|todo)\) (.*)$",line)
    if m:
        name=re.sub(r" \[[0-9.]+m?s\]$","",m.group(2))
        rows.append(cur+"\t"+m.group(1)+"\t"+name); continue
    m=re.match(r"^\s*(\d+) (pass|fail|skip|errors?)$",line)
    if m: counts.setdefault(cur,{})[m.group(2)]=int(m.group(1))
    if line.startswith("--- exit"): counts.setdefault(cur,{})["exit"]=line.split()[-1]
open(sys.argv[2],"w",encoding="utf8").write("\n".join(sorted(rows))+"\n")
for k,v in counts.items(): print(k,v)
