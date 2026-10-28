#!/usr/bin/env python3
"""scripts-scan.py <root>...: streams `📜️script.ts` files below the roots (no node_modules) that start a test runner (`runtimeTestArgumentsV1`, `"test"` argument arrays, vitest) and prints path, runner kind."""
import os,re,sys
K=re.compile(r'runtimeTestArgumentsV1|\[\s*"test"\s*,|vitest|playwright')
for r in sys.argv[1:]:
    for d,ds,fn in os.walk(r):
        ds[:]=[x for x in ds if x not in('node_modules','.git','target','dist')]
        if '📜️script.ts' in fn:
            p=os.path.join(d,'📜️script.ts');t=open(p,errors='replace').read(400000)
            m=sorted(set(K.findall(t)))
            if m: print(p+'\t'+','.join(m),flush=True)
