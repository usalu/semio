import os,re,sys
for f in sys.argv[1:]:
    s=open(f).read()
    n=len(os.path.dirname(f).split('/'))
    rel='../'*n
    if 'process.cwd()' not in s: print('none',f); continue
    s=s.replace('process.cwd()','fileURLToPath(new URL("%s", import.meta.url))'%rel,1)
    if not re.search(r'import[^;]*\bfileURLToPath\b[^;]*from "node:url"',s):
        s='import { fileURLToPath } from "node:url";\n'+s if not s.startswith('/**') and not s.startswith('//') else re.sub(r'(\nimport )',r'\nimport { fileURLToPath } from "node:url";\1',s,1)
    open(f,'w').write(s);print('fixed',f[-80:])
