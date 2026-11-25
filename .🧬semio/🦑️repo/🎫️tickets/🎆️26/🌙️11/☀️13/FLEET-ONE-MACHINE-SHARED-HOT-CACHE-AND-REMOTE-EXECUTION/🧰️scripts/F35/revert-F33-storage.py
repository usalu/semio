import sys,os
src=open(sys.argv[1],encoding="utf8").read().rsplit("\nmain()",1)[0]
g={"__name__":"f33"};exec(src,g)
apply="--apply" in sys.argv
for path,old,new,marker in g["edits"]:
    if "storage/" not in path or "failure-tail" in path or "🚦️ready" in path: continue
    text=open(path,encoding="utf8").read()
    if text.count(new)!=1: print("SKIP",text.count(new),path[-40:],marker[:30]); continue
    print("revert",path[-40:],marker[:30])
    if apply: open(path,"w",encoding="utf8").write(text.replace(new,old))
nf=g["NEWFILE"][0]
if apply and os.path.exists(nf): os.remove(nf); print("removed",nf[-30:])
