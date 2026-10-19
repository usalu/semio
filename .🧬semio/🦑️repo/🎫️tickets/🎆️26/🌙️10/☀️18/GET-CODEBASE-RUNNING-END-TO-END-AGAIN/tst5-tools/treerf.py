import re,sys
imp='import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";'
for p in sys.argv[1:]:
    t=open(p,encoding='utf-8').read(); o=t
    t=re.sub(r'readFileSync\((new URL\((?:"[^"\n]*\.rs"|\'[^\'\n]*\.rs\'|`[^`\n]*\.rs`),\s*[A-Za-z_.]+\))\s*,\s*"utf8"\)',r'readModuleTreeSource(\1)',t)
    if t!=o and 'readModuleTreeSource }' not in o and 'readModuleTreeSource,' not in o and '{readModuleTreeSource' not in o and 'readModuleTreeSource}' not in o:
        lines=t.split('\n'); idx=max(i for i,l in enumerate(lines[:40]) if l.startswith('import '))
        lines.insert(idx+1,imp); t='\n'.join(lines)
    open(p,'w',encoding='utf-8').write(t); print(p[-50:], t!=o)
