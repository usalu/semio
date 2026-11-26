#!/usr/bin/env python3
"""F68: the retention negative test names its workspace the way the receiver requires it: a real `tr-*` scratch directory directly below the private repository's `.tmp` (the receiver canonicalises it); the no-physical-effect claim becomes "the scratch directory stays empty"."""
import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
f=LIB+"/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🧪️tests/🟦️.ts"
got,held=take({"caching-bootstrap"},8)
print("held",held)
try:
    if not held:
        ok=all([
        edit(f,'import{existsSync}from"node:fs";\nimport{join}from"node:path";\n','import{readdirSync}from"node:fs";\n'),
        edit(f,'import{repositoryCachePartitionRootV1}from"../../../../../../../../📦️packages/🟦️typescript/💾️storage/🟦️.ts";\n',''),
        edit(f,'const workspace=join(repositoryCachePartitionRootV1(repository.root,"products"),"tr-negative-proof");expect(existsSync(workspace)).toBe(false);','const workspace=await repository.scratch("tr-");expect(readdirSync(workspace)).toEqual([]);'),
        edit(f,'expect(existsSync(workspace)).toBe(false);expect(original.control.signal.aborted)','expect(readdirSync(workspace)).toEqual([]);expect(original.control.signal.aborted)')])
        print("ok",ok)
finally:release(got)
