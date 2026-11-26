#!/usr/bin/env python3
"""F66 group: stale trailing arguments (`server.stop(true)`, `buildGraphScopeKeyV1(x,registry)`) dropped where the callee takes fewer arguments."""
import sys,collections;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import *
tsc=sys.argv[1];apply=len(sys.argv)>2
targets={("Expected 0 arguments, but got 1.","true"),("Expected 1 arguments, but got 2.","registry")}
byfile=collections.defaultdict(list)
for f,ln,col,code,msg in errors(tsc):
    if code!="TS2554":continue
    byfile[f].append((ln,col,msg))
got,held=locks({area(f) for f in byfile}) if apply else ([],set())
done=0;skip=0
try:
    for f,rows in sorted(byfile.items()):
        if area(f) in held or foreign(f):skip+=len(rows);continue
        lines=open(f,encoding="utf8").read().split("\n");ch=False
        for ln,col,msg in sorted(rows,reverse=True):
            line=lines[ln-1];i=col-1
            e=arg_end(line,i);tok=line[i:e].strip()
            if (msg,tok) not in targets:continue
            b=i
            while b>0 and line[b-1] in " \t":b-=1
            if b>0 and line[b-1]==",":b-=1
            lines[ln-1]=line[:b]+line[e:];ch=True;done+=1
        if ch and apply:write(f,"\n".join(lines))
finally:
    release(got)
print("dropped",done,"skipped(held)",skip)
