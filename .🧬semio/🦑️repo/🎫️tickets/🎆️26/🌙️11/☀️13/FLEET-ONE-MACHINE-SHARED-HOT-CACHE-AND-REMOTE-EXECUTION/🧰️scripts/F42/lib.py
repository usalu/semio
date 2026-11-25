import os,tempfile,sys
R="/Users/ueli/Documents/semio/"
ID=R+"🧰️framework/🔨️modules/🏃️process/🔖️identity/🟦️.ts"
def read(p):return open(p,encoding="utf8").read()
def write(p,s):
    fd,t=tempfile.mkstemp(dir=os.path.dirname(os.path.abspath(p)),prefix=".tmp-");os.write(fd,s.encode());os.close(fd);os.rename(t,p)
def imp(frm):
    r=os.path.relpath(ID,os.path.dirname(frm))
    return r if r.startswith(".") else "./"+r
def edit(p,pairs,add_import=None):
    s=read(p)
    for old,new in pairs:
        n=s.count(old)
        if n!=1:sys.exit(f"{p}: anchor count {n}: {old[:90]}")
        s=s.replace(old,new)
    if add_import:
        names=add_import
        line='import {%s} from "%s";\n'%(",".join(names),imp(p))
        # insert after last top-level import line
        lines=s.split("\n");last=-1
        for i,l in enumerate(lines):
            if l.startswith("import ") :last=i
        lines.insert(last+1,line.rstrip("\n"))
        s="\n".join(lines)
    write(p,s)
    print("ok",p.replace(R,""))
