#!/usr/bin/env python3
import re,os,sys
ROOT="/Users/ueli/Documents/semio"
LIB=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/📥️invocation/🟦️.ts"
call=re.compile(r'(?:\b\w+\.)?receiveScriptProcessInvocation\(\s*process\.env\s*,\s*')
for f in sys.argv[1:]:
    p=os.path.join(ROOT,f)
    s=open(p,encoding="utf-8").read()
    n=len(call.findall(s))
    if n==0: print("skip",f[-80:]); continue
    s=call.sub("withBuildTestInvocationV1(",s)
    rel=os.path.relpath(LIB,os.path.dirname(p)).replace(os.sep,"/")
    if not rel.startswith("."): rel="./"+rel
    # drop receiveScriptProcessInvocation from imports when unused
    if "receiveScriptProcessInvocation(" not in s:
        def fix(m):
            names=[x.strip() for x in m.group(1).split(",") if x.strip() and x.strip()!="receiveScriptProcessInvocation"]
            if len(names)==len(re.split(",",m.group(1)))and "receiveScriptProcessInvocation" not in m.group(1): return m.group(0)
            return "import{"+",".join(names)+"}from"+m.group(2) if names else ""
        s=re.sub(r'import\s*\{([^}]*\breceiveScriptProcessInvocation\b[^}]*)\}\s*from\s*("[^"]+"|\'[^\']+\')\s*;?',lambda m:(("import{"+",".join([x.strip() for x in m.group(1).split(",") if x.strip() and x.strip()!="receiveScriptProcessInvocation"])+"}from"+m.group(2)+";") if [x for x in m.group(1).split(",") if x.strip() and x.strip()!="receiveScriptProcessInvocation"] else ""),s,count=1)
    if "withBuildTestInvocationV1" in s and not re.search(r'import\s*\{[^}]*withBuildTestInvocationV1',s):
        lines=s.split("\n")
        idx=0
        for i,l in enumerate(lines[:40]):
            if l.startswith("import"): idx=i
        lines.insert(idx+1,'import{withBuildTestInvocationV1}from"%s";'%rel)
        s="\n".join(lines)
    open(p,"w",encoding="utf-8").write(s)
    print("migrated",n,f[-80:])
