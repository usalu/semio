#!/usr/bin/env python3
import sys,re
path=sys.argv[1]; names=set(sys.argv[2:])
lines=open(path).read().split("\n")
out=[];i=0;removed=[]
while i<len(lines):
    m=re.match(r'^func (\([^)]*\) )?([A-Za-z0-9_]+)\(',lines[i])
    if m and m.group(2) in names:
        j=i
        while j<len(lines) and lines[j]!="}": j+=1
        k=len(out)
        while k>0 and out[k-1].startswith("//"): k-=1
        del out[k:]
        if out and out[-1]=="" and j+1<len(lines) and lines[j+1]=="": j+=1
        removed.append(m.group(2)); i=j+1; continue
    out.append(lines[i]); i+=1
open(path,"w").write("\n".join(out))
print(path, "removed", removed, "missing", sorted(names-set(removed)))
