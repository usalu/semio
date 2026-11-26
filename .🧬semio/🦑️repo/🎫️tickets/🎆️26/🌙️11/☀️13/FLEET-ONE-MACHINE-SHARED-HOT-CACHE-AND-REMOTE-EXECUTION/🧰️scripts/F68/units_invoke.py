#!/usr/bin/env python3
"""F68: the Rust units test works inside its private repository, so its invocations are the private repository's (caller root = workspace), not the checkout's."""
import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
f=LIB+"/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts"
got,held=take({"caching-cargo"},5)
print("held",held)
try:
    if not held:
        s=open(f,encoding="utf8").read();n=s.count("withBuildTestInvocationV1(")
        s=s.replace("withBuildTestInvocationV1(","repository.invoke(")
        write(f,s);print("replaced",n)
finally:release(got)
