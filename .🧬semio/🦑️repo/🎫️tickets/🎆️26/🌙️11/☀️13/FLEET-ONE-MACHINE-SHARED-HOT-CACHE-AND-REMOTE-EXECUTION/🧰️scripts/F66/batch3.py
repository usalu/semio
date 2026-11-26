#!/usr/bin/env python3
"""F66 group: corrupted or unwired test text found by tsc: a `rooty.rooty..root` chain (broken rename in the unit-descriptor test), `constrouting=` (lost space after const) in the retention negative test, and the missing `linuxHostLibcV1` import in two SDK derivative tests."""
import re,sys;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import *
C=LIB+"/⚡️caching"
units=C+"/🦀️cargo/📦️units/🧪️tests/🟦️.ts"
neg=C+"/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🧪️tests/🟦️.ts"
sdk=C+"/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative"
libc=LIB+"/🎛️commands/🔧️setup/🟦️.ts"
libs=[sdk+"/📤️copy/🧪️tests/🟦️.ts",sdk+"/🧪️tests/🛂️sdk-fences/🟦️.ts"]
areas={area(f) for f in [units,neg]+libs}
got,held=wait_locks(areas,int(sys.argv[1]) if len(sys.argv)>1 else 0)
print("held",held)
try:
    for f in [units,neg]+libs:
        if area(f) in held or foreign(f):print("skip",f.split("/")[-3]);continue
        s=open(f,encoding="utf8").read();o=s
        if f==units:s=re.sub(r'\b(\w+)\.(?:rooty\.)+root\b',r'\1.root',s)
        if f==neg:s=s.replace("constrouting=await","const routing=await")
        if f in libs and "linuxHostLibcV1}" not in s.split("\n")[0:12].__str__():
            rel=os.path.relpath(libc,os.path.dirname(f))
            line='import {linuxHostLibcV1} from "'+rel+'";'
            lines=s.split("\n");at=max(i for i,l in enumerate(lines[:20]) if l.startswith("import"))
            lines.insert(at+1,line);s="\n".join(lines)
        if s!=o:write(f,s);print("edited",f.split("/")[-3])
finally:release(got)
