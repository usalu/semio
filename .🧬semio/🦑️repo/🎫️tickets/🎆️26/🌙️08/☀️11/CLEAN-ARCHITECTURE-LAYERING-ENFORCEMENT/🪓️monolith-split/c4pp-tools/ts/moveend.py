import sys,re
# usage: moveend.py <next-hook-file> <prev-hook-file> <region-name>: move a stray leading `//#endregion <name>` line from the next hook to the end of the previous hook body (before its return/closing brace)
nf,pf,name=sys.argv[1:4]
n=open(nf,encoding="utf8").read(); p=open(pf,encoding="utf8").read()
pat=re.compile(r'^([ ]*)//#endregion '+re.escape(name)+r'\n',re.M)
mm=pat.findall(n); assert len(mm)==1,(nf,name,len(mm))
m=pat.search(n)
line=m.group(0)
n=n[:m.start()]+n[m.end():]
n=re.sub(r'\n\n\n+','\n\n',n)
idx=p.rfind("\n  return {")
if idx<0:
    idx=p.rfind("\n}\n\nexport type")
    assert idx>0
p=p[:idx+1]+line+p[idx+1:] if idx>=0 else p
open(nf,"w",encoding="utf8").write(n); open(pf,"w",encoding="utf8").write(p)
print("moved",name)
