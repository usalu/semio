#!/usr/bin/env python3
"""F68: the shared-worktree resilience test imported verifyCodeSnapshotV1 twice from the same module (a parse error: the file could not load)."""
import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
f=LIB.rsplit("/",1)[0]+"/🤝️shared-worktree/🛡️resilience/🧪️tests/🟦️.ts"
got,held=take({"replica"},5)
print("held",held)
try:
    if not held:
        s=open(f,encoding="utf8").read()
        lines=s.split("\n")
        i=next(k for k,l in enumerate(lines) if l.startswith("import {activityNamesV1"))
        assert "verifyCodeSnapshotV1," in lines[i]
        lines[i]=lines[i].replace("verifyCodeSnapshotV1,","",1)
        write(f,"\n".join(lines));print("edited")
finally:release(got)
