import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
f=LIB+"/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts"
got,held=take({"caching-cargo"},5)
try:
    if not held:
        s=open(f,encoding="utf8").read();ls=s.split("\n")
        k=[i for i,l in enumerate(ls) if l.startswith("import {withBuildTestInvocationV1}")]
        assert len(k)==1
        del ls[k[0]];write(f,"\n".join(ls));print("removed")
finally:release(got)
