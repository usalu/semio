import re,sys
imp='import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";\n'
for p in sys.argv[1:]:
    t=open(p,encoding='utf-8').read(); o=t
    t=re.sub(r'await Bun\.file\((new URL\((?:"[^"\n]+\.rs"|`[^`\n]*\.rs`|`[^`\n]*\$\{file\}`),\s*import\.meta\.url\))\)\.text\(\)',r'readModuleTreeSource(\1)',t)
    t=re.sub(r'Bun\.file\((new URL\(`[^`\n]*\$\{file\}`,\s*import\.meta\.url\))\)\.text\(\)',r'Promise.resolve(readModuleTreeSource(\1))',t)
    if t!=o and 'readModuleTreeSource }' not in o and 'readModuleTreeSource,' not in o and 'readModuleTreeSource ' not in o.split('\n',20)[0]:
        lines=t.split('\n'); idx=max(i for i,l in enumerate(lines[:40]) if l.startswith('import '))
        lines.insert(idx+1,imp.rstrip('\n')); t='\n'.join(lines)
    open(p,'w',encoding='utf-8').write(t); print(p[-60:], t!=o)
