#!/usr/bin/env python3
import sys,re,glob,difflib,os
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules"
def find(paths,name):
    out=[]
    for p in paths:
        lines=open(p,encoding="utf-8").read().split("\n")
        i=0
        while i<len(lines):
            m=re.match(r'^func (\([^)]*\) )?([A-Za-z0-9_]+)\(',lines[i])
            if m and m.group(2).lower()==name.lower():
                j=i
                while j<len(lines) and lines[j]!="}" and not (lines[i].rstrip().endswith("}") and j==i): 
                    if j==i and lines[i].rstrip().endswith("}"): break
                    j+=1
                s=i
                while s>0 and lines[s-1].startswith("//"): s-=1
                out.append((p,s+1,"\n".join(lines[s:j+1])))
                i=j
            i+=1
    return out
PK=r'\b(model|languages|workspace|statutespkg|statutes|codebase|move|hooks|tree|tickets|goals|todos|events|providers|contributors|metrics|testrunner|graphql|identity|search|yaml)\.'
for name in sys.argv[1:]:
    cl=find(glob.glob(M+"/💻️client/⌨️cli/*/🐹️.go"),name)
    mo=find([p for p in glob.glob(M+"/*/📦️packages/🐹️go/*.go")+glob.glob(M+"/*/*/📦️packages/🐹️go/*.go") if "_test" not in p and "💻️client" not in p],name)
    print("=====",name,"client",[(os.path.basename(os.path.dirname(p)),l) for p,l,_ in cl],"module",[(p.split("/")[-4],os.path.basename(p),l) for p,l,_ in mo])
    if cl and mo:
        a=re.sub(PK,'',cl[0][2]).split("\n"); b=re.sub(PK,'',mo[0][2]).split("\n")
        a=[re.sub(r'\b(rootDir)\b','RootDir',x) for x in a]
        for l in difflib.unified_diff(a,b,"client","module",lineterm="",n=1): print(l[:200])
