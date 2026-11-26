#!/usr/bin/env python3
"""F66 groups: (1) `import X from "m" with {type:"text"}` of a TypeScript/JS module is typed as the module, so the source text is read with `Bun.file(new URL(rel,import.meta.url)).text()`; (2) object-literal defaults that a typed `...options` spread always overwrites (TS2783) are dropped."""
import re,sys,collections;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import *
tsc=sys.argv[1];minutes=int(sys.argv[2]) if len(sys.argv)>2 else 0;apply=len(sys.argv)>3
by=collections.defaultdict(list)
for f,ln,col,code,msg in errors(tsc):
    if code in("TS1192","TS2783"):by[f].append((ln,col,code))
TXT=re.compile(r'import\s+(\w+)\s+from\s*"([^"]+)"\s*with\s*\{\s*type\s*:\s*"text"\s*\}\s*;?')
got,held=wait_locks({area(f) for f in by},minutes) if apply else ([],set())
n1=n2=0
try:
    for f,rows in sorted(by.items()):
        if area(f) in held or foreign(f):print("skip",f.split("/")[-2]);continue
        lines=open(f,encoding="utf8").read().split("\n");ch=False
        for ln,col,code in sorted(rows,reverse=True):
            line=lines[ln-1]
            if code=="TS1192":
                m=TXT.search(line)
                if not m:continue
                lines[ln-1]=line[:m.start()]+f'const {m.group(1)}=await Bun.file(new URL("{m.group(2)}",import.meta.url)).text();'+line[m.end():];ch=True;n1+=1
            else:
                i=col-1;e=arg_end(line,i)
                if not re.match(r'\w+\s*:',line[i:]):continue
                if line[e:e+1]!=",":continue
                lines[ln-1]=line[:i]+line[e+1:];ch=True;n2+=1
        if ch and apply:write(f,"\n".join(lines))
finally:release(got)
print("text imports",n1,"dead defaults",n2)
