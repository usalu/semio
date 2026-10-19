"""usage: tidy.py <file>...  removes the blank lines the import pruning left between two imports and collapses runs of blank lines to one"""
import re,sys
def isimport_end(l):
    t=l.strip()
    return t.startswith("import ") and t.endswith(";") or (t.endswith(";") and ' from "' in t) or t.startswith("// #endregion") 
for p in sys.argv[1:]:
    s=open(p,encoding="utf8").read()
    lines=s.split("\n")
    out=[]
    for i,l in enumerate(lines):
        if l.strip()=="":
            j=i+1
            while j<len(lines) and lines[j].strip()=="": j+=1
            prev=next((x for x in reversed(out) if x.strip()!=""),None)
            nxt=lines[j] if j<len(lines) else None
            if prev is not None and nxt is not None and (isimport_end(prev) and (nxt.startswith("import ") or nxt.startswith("// #"))) :
                continue
            if out and out[-1].strip()=="":
                continue
        out.append(l)
    open(p,"w",encoding="utf8").write("\n".join(out))
    print("tidied",p[-60:],len(lines),"->",len(out))
