"""cstep: `Ok(NAME(args))` -> `NAME(args)` for fns that now answer Result<InteractiveJobCloseStep, ValueError> themselves.
usage: unwrap_ok.py [--write] names(comma) listfile"""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
write='--write' in sys.argv
args=[a for a in sys.argv[1:] if not a.startswith('--')]
names=args[0].split(',')
pat=re.compile(r'\bOk\s*\(\s*((?:[\w]+::)*(?:[\w]+\.)*(?:self\.)?(?:[\w]+\.)*(?:'+'|'.join(names)+r'))\s*\(')
for p in open(args[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read()
    if not any(re.search(n,s) for n in names): continue
    m=mask(s);out=s;changed=0
    for r in reversed(list(pat.finditer(m))):
        po=r.end()-1;pc=match_brace(m,po)
        if pc<0: continue
        j=pc+1
        while j<len(m) and m[j] in ' \t\r\n': j+=1
        if j<len(m) and m[j]==')':
            inner=s[r.start(1):pc+1]
            out=out[:r.start()]+inner+out[j+1:]
            changed+=1
    if changed:
        print(p,changed)
        if write: open(p,'w',encoding='utf-8').write(out)
