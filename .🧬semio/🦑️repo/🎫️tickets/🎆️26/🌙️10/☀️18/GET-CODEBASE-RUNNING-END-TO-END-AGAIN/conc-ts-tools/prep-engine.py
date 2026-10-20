#!/usr/bin/env python3
"""Copies the staged engine to /private/tmp/conc-ts/engine with the validator import made absolute (scratch runs outside the repo tree)."""
import os,shutil
C=os.path.dirname(os.path.abspath(__file__)); S=os.path.join(C,'staged','🧩️units'); D='/private/tmp/conc-ts/engine/🧩️units'
shutil.rmtree(D,ignore_errors=True); shutil.copytree(S,D)
v='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts'
for r,_,fs in os.walk(D):
    for f in fs:
        if f.endswith('.ts'):
            p=os.path.join(r,f); t=open(p).read()
            t=t.replace('"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts"','"%s"'%v)
            open(p,'w').write(t)
print('engine ->',D)
