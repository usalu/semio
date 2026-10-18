#!/usr/bin/env python3
import sys,re,glob,os
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules"
CL=M+"/💻️client/⌨️cli"
def read(p): return open(p,encoding="utf-8").read()
def funcs(paths):
    d={}
    for p in paths:
        lines=read(p).split("\n")
        i=0
        while i<len(lines):
            m=re.match(r'^func (Test[A-Za-z0-9_]+)\(',lines[i])
            if m:
                j=i
                while j<len(lines) and lines[j]!="}": j+=1
                s=i
                while s>0 and lines[s-1].startswith("//"): s-=1
                d.setdefault(m.group(1),[]).append((p,s,j,"\n".join(lines[s:j+1]))); i=j
            i+=1
    return d
clfiles=glob.glob(CL+"/🧪️tests/*/🐹️.go")
cl=funcs(clfiles)
mfiles=[p for p in glob.glob(M+"/*/📦️packages/🐹️go/*_test.go")+glob.glob(M+"/*/*/📦️packages/🐹️go/*_test.go")+glob.glob(M+"/*/🧪️tests/*/🐹️.go")]
mo=funcs(mfiles)
# client identifiers
cid=set(['rootDir'])
for p in glob.glob(CL+"/*/🐹️.go")+clfiles:
    for m in re.finditer(r'^(?:func (?:\([^)]*\) )?|type |var |const )([A-Za-z_][A-Za-z0-9_]*)',read(p),re.M): cid.add(m.group(1))
    for m in re.finditer(r'^\t([A-Za-z_][A-Za-z0-9_]*)\s',read(p),re.M): cid.add(m.group(1))
PK=r'\b(model|languages|workspace|statutespkg|statutes|codebase|move|hooks|tree|tickets|goals|todos|events|providers|contributors|metrics|testrunner|graphql|identity|search|yaml)\.'
TOK=re.compile(r'("(?:\\.|[^"\\\n])*"|`[^`]*`|\'(?:\\.|[^\'\\\n])*\'|//[^\n]*|/\*.*?\*/)',re.S)
def mapcode(b):
    b=re.sub(PK,'',b)
    b=re.sub(r'(?<![\w.])RootDir\b','rootDir',b)
    def r(m):
        x=m.group(0)
        if x in cid: return x
        l=x[0].lower()+x[1:]
        return l if l in cid else x
    return re.sub(r'(?<![\w.])[A-Z][A-Za-z0-9_]*\b',r,b)
def mapbody(b):
    parts=TOK.split(b)
    out=[]
    for i,part in enumerate(parts):
        out.append(part if i%2==1 else mapcode(part))
    return "".join(out)
if __name__=="__main__":
    apply="--apply" in sys.argv
    for n in [a for a in sys.argv[1:] if not a.startswith("--")]:
        if n not in cl or n not in mo: print(n,"no twin"); continue
        p,s,e,body=cl[n][0]
        mp,_,_,mbody=mo[n][0]
        new=mapbody(mbody)
        print(n,"client",os.path.basename(os.path.dirname(p)),"<-",os.path.basename(mp), "same" if new==body else "differs")
        if apply and new!=body:
            fresh=funcs([p])[n][0]
            _,s,e,_=fresh
            lines=read(p).split("\n")
            lines[s:e+1]=new.split("\n")
            open(p,"w",encoding="utf-8").write("\n".join(lines))
